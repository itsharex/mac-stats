//! Deterministic pre-routing: skip the LLM planning step for unambiguous patterns.
//!
//! Screenshot + URL → BROWSER_SCREENSHOT; "run <command>" → RUN_CMD;
//! "fetch <URL>" → FETCH_URL; "search <query>" → BRAVE_SEARCH / PERPLEXITY_SEARCH;
//! ticket → REDMINE_API.

use tracing::info;

use crate::commands::redmine_helpers::{
    extract_ticket_id, is_redmine_time_entries_request, redmine_request_for_routing,
    redmine_time_entries_range,
};
use crate::commands::reply_helpers::{
    extract_last_prefixed_argument, extract_screenshot_recommendation, extract_url_from_question,
};

/// Try to pre-route the question to a tool without asking the LLM.
///
/// Returns `Some(recommendation_string)` when the question unambiguously maps to a tool,
/// or `None` when the LLM planning step is needed.
pub(crate) fn compute_pre_routed_recommendation(
    question: &str,
    request_for_verification: &str,
    is_verification_retry: bool,
) -> Option<String> {
    extract_screenshot_recommendation(question).or_else(|| {
        let run_cmd_rec = try_pre_route_run_cmd(question);
        if run_cmd_rec.is_some() {
            return run_cmd_rec;
        }
        let fetch_rec = try_pre_route_fetch_url(question);
        if fetch_rec.is_some() {
            return fetch_rec;
        }
        let search_rec = try_pre_route_web_search(question);
        if search_rec.is_some() {
            return search_rec;
        }
        try_pre_route_redmine(question, request_for_verification, is_verification_retry)
    })
}

/// "run <command>" / "RUN_CMD: <command>" → `RUN_CMD: <arg>`.
fn try_pre_route_run_cmd(question: &str) -> Option<String> {
    if !crate::commands::run_cmd::is_local_cmd_allowed() {
        return None;
    }
    let q = question.trim();
    let q_lower = q.to_lowercase();
    let cmd_rest = if let Some(cmd) = extract_last_prefixed_argument(q, "RUN_CMD:") {
        cmd
    } else if q_lower.starts_with("run command:") {
        q[12..].trim().to_string()
    } else if q_lower.starts_with("run ") {
        q[4..].trim().to_string()
    } else {
        String::new()
    };
    if cmd_rest.is_empty() {
        return None;
    }
    let rec = format!("RUN_CMD: {}", cmd_rest);
    info!(
        "Agent router: pre-routed to RUN_CMD (run command): {}",
        crate::logging::ellipse(&cmd_rest, 60)
    );
    Some(rec)
}

/// "fetch <URL>" / "FETCH_URL: <URL>" / "get the page at <URL>" → `FETCH_URL: <url>`.
///
/// Only triggers when the question contains a URL and clear fetch/read intent.
/// Does NOT trigger for browser/navigate/screenshot patterns (handled upstream).
fn try_pre_route_fetch_url(question: &str) -> Option<String> {
    let q = question.trim();
    let q_lower = q.to_lowercase();

    // Skip if the question looks like a browser/navigate task (screenshot pre-route
    // already ran, but we also avoid "navigate to" / "open in browser" patterns).
    if q_lower.contains("screenshot")
        || q_lower.contains("navigate")
        || q_lower.contains("click")
        || q_lower.contains("scroll")
        || (q_lower.contains("open") && q_lower.contains("browser"))
    {
        return None;
    }

    // Explicit FETCH_URL: prefix
    if let Some(arg) = extract_last_prefixed_argument(q, "FETCH_URL:") {
        if let Some(url) = extract_url_from_question(&arg) {
            info!(
                "Agent router: pre-routed to FETCH_URL (explicit prefix): {}",
                crate::logging::ellipse(&url, 80)
            );
            return Some(format!("FETCH_URL: {url}"));
        }
    }

    // Must contain a URL for the remaining keyword-based detection
    let url = extract_url_from_question(q)?;

    let has_fetch_intent = q_lower.contains("fetch ")
        || q_lower.contains("get the page")
        || q_lower.contains("get the content")
        || q_lower.contains("get the html")
        || q_lower.contains("read the page")
        || q_lower.contains("read the url")
        || q_lower.contains("read the site")
        || q_lower.contains("what's on ")
        || q_lower.contains("what is on ")
        || q_lower.contains("summarize the page")
        || q_lower.contains("summarize this page")
        || q_lower.contains("summarize this url")
        || q_lower.contains("summarize the url")
        || q_lower.contains("summarize the site")
        || q_lower.contains("summarise the page")
        || q_lower.contains("summarise this url");

    if has_fetch_intent {
        info!(
            "Agent router: pre-routed to FETCH_URL (keyword + URL): {}",
            crate::logging::ellipse(&url, 80)
        );
        return Some(format!("FETCH_URL: {url}"));
    }

    None
}

/// "search for <query>" / "google <query>" / "BRAVE_SEARCH: <query>" → web search tool.
///
/// Routes to BRAVE_SEARCH when Brave is configured, or PERPLEXITY_SEARCH when only
/// Perplexity is configured. Explicit "PERPLEXITY_SEARCH: <query>" always routes to
/// Perplexity (when configured). "research <query>" prefers Perplexity over Brave.
///
/// Skips pre-routing when the question contains multi-step indicators (browser actions,
/// "and then", "send to", etc.) that need LLM planning.
fn try_pre_route_web_search(question: &str) -> Option<String> {
    let q = question.trim();
    let q_lower = q.to_lowercase();

    // Skip multi-step / compound requests that need LLM planning.
    if q_lower.contains("and then ")
        || q_lower.contains("after that ")
        || q_lower.contains("send to ")
        || q_lower.contains("post to ")
        || q_lower.contains("screenshot")
        || q_lower.contains("navigate")
        || q_lower.contains("click")
    {
        return None;
    }

    let brave_ok = crate::commands::brave::get_brave_api_key().is_some();
    let perplexity_ok = crate::commands::perplexity::is_perplexity_configured().unwrap_or(false);

    if !brave_ok && !perplexity_ok {
        return None;
    }

    // Explicit "PERPLEXITY_SEARCH: <query>"
    if perplexity_ok {
        if let Some(arg) = extract_last_prefixed_argument(q, "PERPLEXITY_SEARCH:") {
            let query = arg.trim().to_string();
            if !query.is_empty() {
                info!(
                    "Agent router: pre-routed to PERPLEXITY_SEARCH (explicit prefix): {}",
                    crate::logging::ellipse(&query, 80)
                );
                return Some(format!("PERPLEXITY_SEARCH: {query}"));
            }
        }
    }

    // Explicit "BRAVE_SEARCH: <query>"
    if brave_ok {
        if let Some(arg) = extract_last_prefixed_argument(q, "BRAVE_SEARCH:") {
            let query = arg.trim().to_string();
            if !query.is_empty() {
                info!(
                    "Agent router: pre-routed to BRAVE_SEARCH (explicit prefix): {}",
                    crate::logging::ellipse(&query, 80)
                );
                return Some(format!("BRAVE_SEARCH: {query}"));
            }
        }
    }

    // Keyword-based search intent detection.
    // Extract the search query from the question after the keyword.
    let search_query = extract_search_query(&q_lower, q);
    if let Some((query, is_research)) = search_query {
        if query.is_empty() {
            return None;
        }
        // "research" prefers Perplexity; "search" / "google" / "look up" prefers Brave.
        let (tool, label) = if is_research && perplexity_ok {
            ("PERPLEXITY_SEARCH", "research keyword")
        } else if brave_ok {
            ("BRAVE_SEARCH", "search keyword")
        } else {
            ("PERPLEXITY_SEARCH", "search keyword (Brave unavailable)")
        };
        info!(
            "Agent router: pre-routed to {} ({}): {}",
            tool,
            label,
            crate::logging::ellipse(&query, 80)
        );
        return Some(format!("{tool}: {query}"));
    }

    None
}

/// Extract a search query from keyword patterns. Returns `(query, is_research)`.
/// `is_research` is true for "research ..." patterns (prefers Perplexity).
fn extract_search_query(q_lower: &str, q_original: &str) -> Option<(String, bool)> {
    // Ordered by specificity: longer patterns first to avoid partial matches.
    let patterns: &[(&str, bool)] = &[
        ("search the web for ", false),
        ("search the internet for ", false),
        ("web search for ", false),
        ("web search ", false),
        ("search online for ", false),
        ("search for ", false),
        ("look up ", false),
        ("lookup ", false),
        ("google ", false),
        ("research ", true),
        ("search ", false),
    ];

    for &(pattern, is_research) in patterns {
        if let Some(pos) = q_lower.find(pattern) {
            let before = q_lower[..pos].trim();
            if !before.is_empty()
                && !before.ends_with("please")
                && !before.ends_with("can you")
                && !before.ends_with("could you")
                && !before.ends_with("pls")
            {
                continue;
            }
            let query = q_original[pos + pattern.len()..].trim().to_string();
            let query = query.trim_end_matches('?').trim().to_string();
            if !query.is_empty() {
                return Some((query, is_research));
            }
            // A matching pattern with empty query means the user typed
            // the keyword but no search terms — stop here instead of
            // falling through to a shorter, less specific pattern.
            return None;
        }
    }
    None
}

/// Ticket / time-entries patterns → `REDMINE_API: GET /...`.
fn try_pre_route_redmine(
    question: &str,
    request_for_verification: &str,
    is_verification_retry: bool,
) -> Option<String> {
    if !crate::redmine::is_configured() {
        return None;
    }
    let q = question.trim();
    let redmine_request =
        redmine_request_for_routing(q, request_for_verification, is_verification_retry);
    let redmine_request_lower = redmine_request.to_lowercase();

    if is_redmine_time_entries_request(redmine_request) {
        let (from, to) = redmine_time_entries_range(redmine_request);
        let rec = format!(
            "REDMINE_API: GET /time_entries.json?from={}&to={}&limit=100",
            from, to
        );
        info!(
            "Agent router: pre-routed to REDMINE_API for time entries ({}..{})",
            from, to
        );
        return Some(rec);
    }

    let ticket_id = extract_ticket_id(&redmine_request_lower);
    let wants_update = redmine_request_lower.contains("update")
        || redmine_request_lower.contains("add comment")
        || redmine_request_lower.contains("with the next steps")
        || redmine_request_lower.contains("post a comment")
        || redmine_request_lower.contains("write ")
        || redmine_request_lower.contains("put ");
    ticket_id
        .filter(|_| {
            redmine_request_lower.contains("ticket")
                || redmine_request_lower.contains("issue")
                || redmine_request_lower.contains("redmine")
        })
        .filter(|_| !wants_update)
        .map(|id| {
            let rec = format!(
                "REDMINE_API: GET /issues/{}.json?include=journals,attachments",
                id
            );
            info!("Agent router: pre-routed to REDMINE_API for ticket #{}", id);
            rec
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_url_explicit_prefix() {
        let r = try_pre_route_fetch_url("FETCH_URL: https://example.com");
        assert_eq!(r, Some("FETCH_URL: https://example.com".to_string()));
    }

    #[test]
    fn fetch_url_explicit_prefix_case_insensitive() {
        let r = try_pre_route_fetch_url("fetch_url: https://example.com/page");
        assert_eq!(r, Some("FETCH_URL: https://example.com/page".to_string()));
    }

    #[test]
    fn fetch_url_keyword_fetch() {
        let r = try_pre_route_fetch_url("fetch https://example.com");
        assert_eq!(r, Some("FETCH_URL: https://example.com".to_string()));
    }

    #[test]
    fn fetch_url_keyword_get_the_page() {
        let r = try_pre_route_fetch_url("get the page at https://docs.rs/tokio");
        assert_eq!(r, Some("FETCH_URL: https://docs.rs/tokio".to_string()));
    }

    #[test]
    fn fetch_url_keyword_get_the_content() {
        let r = try_pre_route_fetch_url("get the content of https://example.com/api");
        assert_eq!(r, Some("FETCH_URL: https://example.com/api".to_string()));
    }

    #[test]
    fn fetch_url_keyword_read_the_page() {
        let r = try_pre_route_fetch_url("read the page https://example.com");
        assert_eq!(r, Some("FETCH_URL: https://example.com".to_string()));
    }

    #[test]
    fn fetch_url_keyword_summarize() {
        let r = try_pre_route_fetch_url("summarize the page at https://blog.example.com/post");
        assert_eq!(
            r,
            Some("FETCH_URL: https://blog.example.com/post".to_string())
        );
    }

    #[test]
    fn fetch_url_keyword_summarise_british() {
        let r = try_pre_route_fetch_url("summarise this url https://example.com");
        assert_eq!(r, Some("FETCH_URL: https://example.com".to_string()));
    }

    #[test]
    fn fetch_url_keyword_whats_on() {
        let r = try_pre_route_fetch_url("what's on https://news.example.com?");
        assert_eq!(
            r,
            Some("FETCH_URL: https://news.example.com".to_string())
        );
    }

    #[test]
    fn fetch_url_no_url_returns_none() {
        assert_eq!(try_pre_route_fetch_url("fetch some data"), None);
    }

    #[test]
    fn fetch_url_no_intent_returns_none() {
        assert_eq!(
            try_pre_route_fetch_url("tell me about https://example.com"),
            None
        );
    }

    #[test]
    fn fetch_url_screenshot_skipped() {
        assert_eq!(
            try_pre_route_fetch_url("take a screenshot of https://example.com"),
            None
        );
    }

    #[test]
    fn fetch_url_navigate_skipped() {
        assert_eq!(
            try_pre_route_fetch_url("navigate to https://example.com and find the price"),
            None
        );
    }

    #[test]
    fn fetch_url_click_skipped() {
        assert_eq!(
            try_pre_route_fetch_url("fetch https://example.com and click the button"),
            None
        );
    }

    #[test]
    fn fetch_url_open_in_browser_skipped() {
        assert_eq!(
            try_pre_route_fetch_url("open https://example.com in the browser"),
            None
        );
    }

    #[test]
    fn fetch_url_strips_trailing_punctuation() {
        let r = try_pre_route_fetch_url("fetch https://example.com.");
        assert_eq!(r, Some("FETCH_URL: https://example.com".to_string()));
    }

    #[test]
    fn fetch_url_http_scheme() {
        let r = try_pre_route_fetch_url("fetch http://localhost:8080/api");
        assert_eq!(
            r,
            Some("FETCH_URL: http://localhost:8080/api".to_string())
        );
    }

    // --- extract_search_query tests ---

    #[test]
    fn search_query_search_for() {
        let r = extract_search_query("search for rust async patterns", "search for rust async patterns");
        assert_eq!(r, Some(("rust async patterns".to_string(), false)));
    }

    #[test]
    fn search_query_search_plain() {
        let r = extract_search_query("search latest rust release", "search latest rust release");
        assert_eq!(r, Some(("latest rust release".to_string(), false)));
    }

    #[test]
    fn search_query_google() {
        let r = extract_search_query("google best restaurants in berlin", "google best restaurants in berlin");
        assert_eq!(r, Some(("best restaurants in berlin".to_string(), false)));
    }

    #[test]
    fn search_query_look_up() {
        let r = extract_search_query("look up tauri v2 documentation", "look up tauri v2 documentation");
        assert_eq!(r, Some(("tauri v2 documentation".to_string(), false)));
    }

    #[test]
    fn search_query_lookup_no_space() {
        let r = extract_search_query("lookup tokio runtime", "lookup tokio runtime");
        assert_eq!(r, Some(("tokio runtime".to_string(), false)));
    }

    #[test]
    fn search_query_web_search() {
        let r = extract_search_query("web search for climate change 2026", "web search for climate change 2026");
        assert_eq!(r, Some(("climate change 2026".to_string(), false)));
    }

    #[test]
    fn search_query_web_search_no_for() {
        let r = extract_search_query("web search rust tauri", "web search rust tauri");
        assert_eq!(r, Some(("rust tauri".to_string(), false)));
    }

    #[test]
    fn search_query_search_the_web_for() {
        let r = extract_search_query(
            "search the web for apple silicon m4",
            "search the web for apple silicon m4",
        );
        assert_eq!(r, Some(("apple silicon m4".to_string(), false)));
    }

    #[test]
    fn search_query_search_the_internet_for() {
        let r = extract_search_query(
            "search the internet for ollama models",
            "search the internet for ollama models",
        );
        assert_eq!(r, Some(("ollama models".to_string(), false)));
    }

    #[test]
    fn search_query_search_online_for() {
        let r = extract_search_query("search online for weather berlin", "search online for weather berlin");
        assert_eq!(r, Some(("weather berlin".to_string(), false)));
    }

    #[test]
    fn search_query_research_is_research() {
        let r = extract_search_query("research quantum computing advances", "research quantum computing advances");
        assert_eq!(r, Some(("quantum computing advances".to_string(), true)));
    }

    #[test]
    fn search_query_with_please_prefix() {
        let r = extract_search_query(
            "please search for openai news",
            "please search for openai news",
        );
        assert_eq!(r, Some(("openai news".to_string(), false)));
    }

    #[test]
    fn search_query_with_can_you_prefix() {
        let r = extract_search_query(
            "can you search for tauri plugins",
            "can you search for tauri plugins",
        );
        assert_eq!(r, Some(("tauri plugins".to_string(), false)));
    }

    #[test]
    fn search_query_with_could_you_prefix() {
        let r = extract_search_query(
            "could you google macos 15 features",
            "could you google macos 15 features",
        );
        assert_eq!(r, Some(("macos 15 features".to_string(), false)));
    }

    #[test]
    fn search_query_strips_trailing_question_mark() {
        let r = extract_search_query(
            "search for what is serde?",
            "search for what is serde?",
        );
        assert_eq!(r, Some(("what is serde".to_string(), false)));
    }

    #[test]
    fn search_query_empty_after_keyword_returns_none() {
        assert_eq!(extract_search_query("search for ", "search for "), None);
    }

    #[test]
    fn search_query_no_match_returns_none() {
        assert_eq!(
            extract_search_query("tell me about rust", "tell me about rust"),
            None
        );
    }

    #[test]
    fn search_query_embedded_search_not_at_start() {
        assert_eq!(
            extract_search_query(
                "i want to search for something and then send it",
                "i want to search for something and then send it",
            ),
            None
        );
    }

    #[test]
    fn search_query_pls_prefix() {
        let r = extract_search_query("pls search for new iphone", "pls search for new iphone");
        assert_eq!(r, Some(("new iphone".to_string(), false)));
    }

    #[test]
    fn search_query_longer_pattern_preferred() {
        let r = extract_search_query(
            "search the web for tauri v2",
            "search the web for tauri v2",
        );
        assert_eq!(r, Some(("tauri v2".to_string(), false)));
    }

    #[test]
    fn search_query_case_preserved_in_output() {
        let r = extract_search_query(
            "search for Rust Async Patterns",
            "search for Rust Async Patterns",
        );
        assert_eq!(r, Some(("Rust Async Patterns".to_string(), false)));
    }
}
