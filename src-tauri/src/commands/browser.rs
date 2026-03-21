//! Browser / URL fetch support for Ollama and AI tasks.
//!
//! Provides server-side page fetch (no CORS). Used by the Ollama tool protocol
//! (FETCH_URL) and can be invoked from the frontend.

use std::net::{IpAddr, Ipv6Addr, ToSocketAddrs};
use std::time::Duration;
use tracing::{info, warn};
use url::Url;

/// Max response body size (chars) to avoid huge strings (e.g. 500 KB of text).
const MAX_BODY_CHARS: usize = 500_000;

/// Browser-like User-Agent so servers that block bots/scrapers allow the request (avoids 403).
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

/// Extract the first URL-like token from text (e.g. FETCH_URL arg that may contain extra words).
/// Takes the first substring that starts with http:// or https:// and runs until first whitespace or newline.
/// Returns None if no such substring exists.
pub fn extract_first_url(arg: &str) -> Option<String> {
    let s = arg.trim();
    let start = s.find("https://").or_else(|| s.find("http://"))?;
    let rest = &s[start..];
    let end = rest.find(|c: char| c.is_whitespace()).unwrap_or(rest.len());
    let url = rest[..end].trim_end_matches(['.', ',', ';', ':']);
    if url.is_empty() {
        return None;
    }
    Some(url.to_string())
}

/// Validate and normalize URL for fetch. Returns clear error for invalid or IDN URLs.
fn validate_fetch_url(url: &str) -> Result<Url, String> {
    let parsed = Url::parse(url).map_err(|e| {
        let err_str = e.to_string();
        if err_str.to_lowercase().contains("international domain name") || err_str.contains("IDN") {
            "Invalid URL: international domain names (IDN) are not supported. Use punycode (e.g. xn--...) or a different URL.".to_string()
        } else {
            format!("Invalid URL for FETCH_URL: {}", e)
        }
    })?;
    match parsed.scheme() {
        "http" | "https" => {}
        _ => return Err("URL must use http or https".to_string()),
    }
    Ok(parsed)
}

// ---------------------------------------------------------------------------
// SSRF protection: block private/loopback/link-local/metadata URLs
// ---------------------------------------------------------------------------

fn is_blocked_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            v4.is_loopback()
                || v4.is_private()
                || v4.is_link_local()
                || v4.is_unspecified()
                || v4.is_broadcast()
        }
        IpAddr::V6(v6) => {
            v6.is_loopback()
                || v6.is_unspecified()
                || is_ipv6_link_local(v6)
                || is_ipv6_unique_local(v6)
                || v6.to_ipv4_mapped().map_or(false, |v4| {
                    v4.is_loopback() || v4.is_private() || v4.is_link_local() || v4.is_unspecified()
                })
        }
    }
}

fn is_ipv6_link_local(addr: &Ipv6Addr) -> bool {
    (addr.segments()[0] & 0xffc0) == 0xfe80
}

fn is_ipv6_unique_local(addr: &Ipv6Addr) -> bool {
    (addr.segments()[0] & 0xfe00) == 0xfc00
}

/// Validate that a URL does not target a private/loopback/link-local/metadata network.
/// Rejects URLs with userinfo (credentials). Resolves the hostname to IPs and rejects if
/// any resolved IP is on the blocklist. Hosts listed in `allowed_hosts` bypass the IP check.
pub fn validate_url_no_ssrf(url: &Url, allowed_hosts: &[String]) -> Result<(), String> {
    if !url.username().is_empty() || url.password().is_some() {
        return Err(
            "URL contains credentials (userinfo) and was blocked for security".to_string(),
        );
    }
    let host = url.host_str().ok_or("URL has no host")?;
    if allowed_hosts.iter().any(|h| h.eq_ignore_ascii_case(host)) {
        info!("SSRF guard: host '{}' is in ssrfAllowedHosts, skipping IP check", host);
        return Ok(());
    }
    let port = url
        .port()
        .unwrap_or(if url.scheme() == "https" { 443 } else { 80 });
    let addr_str = format!("{}:{}", host, port);
    let addrs: Vec<std::net::SocketAddr> = addr_str
        .to_socket_addrs()
        .map_err(|e| format!("DNS resolution failed for '{}': {}", host, e))?
        .collect();
    if addrs.is_empty() {
        return Err(format!(
            "DNS resolution for '{}' returned no addresses",
            host
        ));
    }
    for addr in &addrs {
        if is_blocked_ip(&addr.ip()) {
            warn!(
                "SSRF guard: blocked URL {} — host '{}' resolves to private/loopback address {}",
                url, host, addr.ip()
            );
            return Err(format!(
                "URL targets a private network ({}) and was blocked to prevent SSRF",
                addr.ip()
            ));
        }
    }
    Ok(())
}

/// Build a reqwest redirect policy that blocks redirects to private/loopback networks.
fn ssrf_redirect_policy(allowed_hosts: Vec<String>) -> reqwest::redirect::Policy {
    reqwest::redirect::Policy::custom(move |attempt| {
        let url = attempt.url();
        if !url.username().is_empty() || url.password().is_some() {
            return attempt.error(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Redirect with credentials blocked (SSRF guard)",
            ));
        }
        if let Some(host) = url.host_str() {
            if !allowed_hosts.iter().any(|h| h.eq_ignore_ascii_case(host)) {
                let port = url
                    .port()
                    .unwrap_or(if url.scheme() == "https" { 443 } else { 80 });
                let addr_str = format!("{}:{}", host, port);
                if let Ok(addrs) = addr_str.to_socket_addrs() {
                    for addr in addrs {
                        if is_blocked_ip(&addr.ip()) {
                            return attempt.error(std::io::Error::new(
                                std::io::ErrorKind::PermissionDenied,
                                format!(
                                    "Redirect to private network ({}) blocked (SSRF guard)",
                                    addr.ip()
                                ),
                            ));
                        }
                    }
                }
            }
        }
        if attempt.previous().len() > 10 {
            attempt.stop()
        } else {
            attempt.follow()
        }
    })
}

/// Fetch a URL and return the response body as text.
/// Extracts first URL from arg (task-002), validates, uses same timeout/SSL policy as website monitors.
/// Used by Ollama FETCH_URL flow and by the fetch_page Tauri command.
pub fn fetch_page_content(url: &str) -> Result<String, String> {
    let raw = url.trim();
    let url_str = extract_first_url(raw).ok_or_else(|| {
        "Invalid URL for FETCH_URL: no http:// or https:// URL found. Provide a single URL only.".to_string()
    })?;
    let parsed = validate_fetch_url(&url_str)?;
    let allowed_hosts = crate::config::Config::ssrf_allowed_hosts();
    validate_url_no_ssrf(&parsed, &allowed_hosts)?;
    let url = parsed.as_str();

    info!("Fetch page: GET {}", url);

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(15))
        .danger_accept_invalid_certs(true)
        .redirect(ssrf_redirect_policy(allowed_hosts))
        .build()
        .map_err(|e| format!("HTTP client: {}", e))?;

    let resp = client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .header(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        )
        .header("Accept-Language", "en-US,en;q=0.9")
        .send()
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = resp.status();
    if !status.is_success() {
        let code = status.as_u16();
        let reason = status.canonical_reason().unwrap_or("");
        warn!("Fetch page failed: {} {} for URL {}", code, reason, url);
        return Err(format!("HTTP {}: {}", code, reason));
    }

    let body = resp.text().map_err(|e| format!("Read body: {}", e))?;

    let body = if body.chars().count() > MAX_BODY_CHARS {
        crate::logging::ellipse(&body, MAX_BODY_CHARS)
    } else {
        body
    };
    let n = body.chars().count();
    info!("Fetch page: fetched {} chars from {}", n, url);
    Ok(body)
}

/// Tauri command: fetch a URL and return body as text (for frontend or tools).
#[tauri::command]
pub async fn fetch_page(url: String) -> Result<String, String> {
    let url = url.clone();
    tokio::task::spawn_blocking(move || fetch_page_content(&url))
        .await
        .map_err(|e| format!("Task join: {}", e))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ssrf_blocks_loopback_ipv4() {
        let url = Url::parse("http://127.0.0.1/secret").unwrap();
        assert!(validate_url_no_ssrf(&url, &[]).is_err());
    }

    #[test]
    fn ssrf_blocks_loopback_ipv6() {
        let url = Url::parse("http://[::1]/secret").unwrap();
        assert!(validate_url_no_ssrf(&url, &[]).is_err());
    }

    #[test]
    fn ssrf_blocks_private_10() {
        let url = Url::parse("http://10.0.0.1/admin").unwrap();
        assert!(validate_url_no_ssrf(&url, &[]).is_err());
    }

    #[test]
    fn ssrf_blocks_private_172() {
        let url = Url::parse("http://172.16.0.1/").unwrap();
        assert!(validate_url_no_ssrf(&url, &[]).is_err());
    }

    #[test]
    fn ssrf_blocks_private_192() {
        let url = Url::parse("http://192.168.1.1/").unwrap();
        assert!(validate_url_no_ssrf(&url, &[]).is_err());
    }

    #[test]
    fn ssrf_blocks_metadata_endpoint() {
        let url = Url::parse("http://169.254.169.254/latest/meta-data/").unwrap();
        assert!(validate_url_no_ssrf(&url, &[]).is_err());
    }

    #[test]
    fn ssrf_blocks_userinfo() {
        let url = Url::parse("http://evil@127.0.0.1/").unwrap();
        let err = validate_url_no_ssrf(&url, &[]).unwrap_err();
        assert!(err.contains("credentials"));
    }

    #[test]
    fn ssrf_blocks_userinfo_with_password() {
        let url = Url::parse("http://user:pass@example.com/").unwrap();
        let err = validate_url_no_ssrf(&url, &[]).unwrap_err();
        assert!(err.contains("credentials"));
    }

    #[test]
    fn ssrf_allows_public_url() {
        let url = Url::parse("https://www.example.com/page").unwrap();
        assert!(validate_url_no_ssrf(&url, &[]).is_ok());
    }

    #[test]
    fn ssrf_allowlist_bypasses_check() {
        let url = Url::parse("http://127.0.0.1:3000/api").unwrap();
        let allowed = vec!["127.0.0.1".to_string()];
        assert!(validate_url_no_ssrf(&url, &allowed).is_ok());
    }

    #[test]
    fn ssrf_allowlist_case_insensitive() {
        let url = Url::parse("http://MyLocalService:8080/").unwrap();
        let allowed = vec!["mylocalservice".to_string()];
        assert!(validate_url_no_ssrf(&url, &allowed).is_ok());
    }

    #[test]
    fn is_blocked_ip_loopback() {
        assert!(is_blocked_ip(&IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)));
        assert!(is_blocked_ip(&IpAddr::V6(std::net::Ipv6Addr::LOCALHOST)));
    }

    #[test]
    fn is_blocked_ip_private_ranges() {
        assert!(is_blocked_ip(&IpAddr::V4(std::net::Ipv4Addr::new(10, 0, 0, 1))));
        assert!(is_blocked_ip(&IpAddr::V4(std::net::Ipv4Addr::new(172, 16, 0, 1))));
        assert!(is_blocked_ip(&IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 0, 1))));
    }

    #[test]
    fn is_blocked_ip_link_local() {
        assert!(is_blocked_ip(&IpAddr::V4(std::net::Ipv4Addr::new(169, 254, 169, 254))));
    }

    #[test]
    fn is_blocked_ip_public_is_ok() {
        assert!(!is_blocked_ip(&IpAddr::V4(std::net::Ipv4Addr::new(8, 8, 8, 8))));
        assert!(!is_blocked_ip(&IpAddr::V4(std::net::Ipv4Addr::new(1, 1, 1, 1))));
    }
}
