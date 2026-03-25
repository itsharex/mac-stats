//! Browser tool dispatch handlers for the agent router tool loop.
//!
//! Extracted from `commands/ollama.rs` to keep modules small and cohesive.

use std::path::PathBuf;
use std::sync::OnceLock;

use tracing::info;

use crate::commands::browser_helpers::{
    append_latest_browser_state_guidance, extract_browser_navigation_target,
    should_use_http_fallback_after_browser_action_error,
};
use crate::config::Config;

pub(crate) struct BrowserScreenshotResult {
    pub message: String,
    pub attachment_path: Option<PathBuf>,
}

fn send_status(tx: Option<&tokio::sync::mpsc::UnboundedSender<String>>, msg: &str) {
    if let Some(tx) = tx {
        let _ = tx.send(msg.to_string());
    }
}

fn browser_tools_disabled_guard() -> Option<String> {
    if Config::browser_tools_enabled() {
        return None;
    }

    static LOGGED: OnceLock<()> = OnceLock::new();
    LOGGED.get_or_init(|| {
        // Avoid leaking URLs/targets; this message is stable and policy-friendly.
        crate::mac_stats_info!(
            "browser/tools_disabled",
            "Browser tools disabled in config (browserToolsEnabled=false)"
        );
    });

    Some("Browser tools disabled in config".to_string())
}

fn nav_url_changed_hint_if_navigation_timeout(err: &str) -> Option<bool> {
    // Error returned by `browser_agent::navigate_and_get_state_inner`.
    if err.contains("Navigation failed: timeout after") {
        crate::browser_agent::take_last_nav_timeout_url_changed_hint()
    } else {
        None
    }
}

fn append_browser_readiness_context(
    base: String,
    cdp_used: bool,
    nav_url_changed: Option<bool>,
) -> String {
    if let Some(ctx) = crate::browser_agent::format_last_browser_error_context(
        cdp_used,
        nav_url_changed,
    ) {
        crate::mac_stats_debug!(
            "browser/tools",
            "Browser tool error context: {}",
            crate::logging::ellipse(&ctx, 200)
        );
        format!("{}\n{}", base, ctx)
    } else {
        base
    }
}

pub(crate) async fn handle_browser_screenshot(
    arg: &str,
    request_id: &str,
    status_tx: Option<&tokio::sync::mpsc::UnboundedSender<String>>,
) -> BrowserScreenshotResult {
    if let Some(msg) = browser_tools_disabled_guard() {
        return BrowserScreenshotResult {
            message: msg,
            attachment_path: None,
        };
    }

    let url_arg = arg.trim().to_string();
    let is_current = url_arg.is_empty() || url_arg.eq_ignore_ascii_case("current");
    if !is_current {
        info!(
            "Agent router: rejecting BROWSER_SCREENSHOT: {} — use NAVIGATE first, then SCREENSHOT: current",
            crate::logging::ellipse(&url_arg, 60)
        );
        BrowserScreenshotResult {
            message: format!(
                "BROWSER_SCREENSHOT only works on the current page. Use BROWSER_NAVIGATE: {} first, then BROWSER_SCREENSHOT: current. Never use BROWSER_SCREENSHOT: <url>.",
                url_arg
            ),
            attachment_path: None,
        }
    } else {
        send_status(status_tx, "📸 Taking screenshot of current page");
        match tokio::task::spawn_blocking(crate::browser_agent::take_screenshot_current_page).await
        {
            Ok(Ok(path)) => {
                if let Some(tx) = status_tx {
                    let _ = tx.send(format!("ATTACH:{}", path.display()));
                }
                BrowserScreenshotResult {
                    message: format!(
                        "Screenshot of current page saved to: {}.\n\nTell the user the screenshot was taken; the app will attach it in Discord.",
                        path.display()
                    ),
                    attachment_path: Some(path),
                }
            }
            Ok(Err(e)) => {
                info!(
                    "Agent router [{}]: BROWSER_SCREENSHOT (current) failed: {}",
                    request_id,
                    crate::logging::ellipse(&e, 200)
                );
                let base_msg = format!(
                    "Screenshot of current page failed: {}. (Use BROWSER_NAVIGATE and BROWSER_CLICK first with CDP; then BROWSER_SCREENSHOT: current. Chrome may need to be on port 9222.)",
                    e
                );
                let msg = append_browser_readiness_context(base_msg, true, None);
                BrowserScreenshotResult {
                    message: msg,
                    attachment_path: None,
                }
            }
            Err(e) => BrowserScreenshotResult {
                message: format!("Screenshot task error: {}", e),
                attachment_path: None,
            },
        }
    }
}

pub(crate) async fn handle_browser_navigate(
    arg: &str,
    request_id: &str,
    status_tx: Option<&tokio::sync::mpsc::UnboundedSender<String>>,
) -> String {
    if let Some(msg) = browser_tools_disabled_guard() {
        return msg;
    }

    let raw_arg = arg.trim().to_string();
    if raw_arg.is_empty() {
        return "BROWSER_NAVIGATE requires a URL (e.g. BROWSER_NAVIGATE: https://www.example.com). Please try again with a URL.".to_string();
    }
    if let Some(url_arg) = extract_browser_navigation_target(&raw_arg) {
        let new_tab = raw_arg
            .split_whitespace()
            .any(|w| w.eq_ignore_ascii_case("new_tab"));
        send_status(
            status_tx,
            &format!(
                "🧭 Navigating to {}…{}",
                url_arg,
                if new_tab { " (new tab)" } else { "" }
            ),
        );
        info!(
            "Agent router [{}]: BROWSER_NAVIGATE: URL sent to CDP: {} new_tab={}",
            request_id, url_arg, new_tab
        );
        match tokio::task::spawn_blocking({
            let u = url_arg.clone();
            move || crate::browser_agent::navigate_and_get_state_with_options(&u, new_tab)
        })
        .await
        {
            Ok(Ok(state_str)) => state_str,
            Ok(Err(cdp_err)) => {
                info!(
                    "BROWSER_NAVIGATE CDP failed, ensuring Chrome on 9222 and retrying: {}",
                    crate::logging::ellipse(&cdp_err, 120)
                );
                tokio::task::spawn_blocking(|| crate::browser_agent::ensure_chrome_on_port(9222))
                    .await
                    .ok();
                match tokio::task::spawn_blocking({
                    let u = url_arg.clone();
                    move || crate::browser_agent::navigate_and_get_state_with_options(&u, new_tab)
                })
                .await
                {
                    Ok(Ok(state_str)) => state_str,
                    Ok(Err(cdp_err2)) => {
                        info!(
                            "BROWSER_NAVIGATE CDP retry failed, trying HTTP fallback: {}",
                            crate::logging::ellipse(&cdp_err2, 120)
                        );
                        match tokio::task::spawn_blocking(move || {
                            crate::browser_agent::navigate_http(&url_arg)
                        })
                        .await
                        {
                            Ok(Ok(state_str)) => state_str,
                            Ok(Err(http_err)) => {
                                let nav_url_changed =
                                    nav_url_changed_hint_if_navigation_timeout(&cdp_err2);
                                let base = format!(
                                    "BROWSER_NAVIGATE failed (CDP: {}). HTTP fallback also failed: {}",
                                    crate::logging::ellipse(&cdp_err2, 80),
                                    http_err
                                );
                                append_browser_readiness_context(base, false, nav_url_changed)
                            }
                            Err(e) => format!("BROWSER_NAVIGATE HTTP fallback task error: {}", e),
                        }
                    }
                    Err(e) => {
                        format!("BROWSER_NAVIGATE CDP retry task error: {}", e)
                    }
                }
            }
            Err(e) => format!("BROWSER_NAVIGATE task error: {}", e),
        }
    } else {
        append_latest_browser_state_guidance(&format!(
            "BROWSER_NAVIGATE requires a concrete URL. The step {:?} was not executed because it did not contain a grounded browser target. This was an agent planning/parsing issue, not evidence about the site.",
            raw_arg
        ))
    }
}

pub(crate) async fn handle_browser_go_back(
    request_id: &str,
    status_tx: Option<&tokio::sync::mpsc::UnboundedSender<String>>,
) -> String {
    if let Some(msg) = browser_tools_disabled_guard() {
        return msg;
    }

    send_status(status_tx, "🔙 Going back…");
    info!("Agent router [{}]: BROWSER_GO_BACK", request_id);
    match tokio::task::spawn_blocking(crate::browser_agent::go_back).await {
        Ok(Ok(state_str)) => state_str,
        Ok(Err(e)) => {
            let base = format!("BROWSER_GO_BACK failed: {}", e);
            let base = append_browser_readiness_context(base, true, None);
            append_latest_browser_state_guidance(&base)
        }
        Err(e) => format!("BROWSER_GO_BACK task error: {}", e),
    }
}

pub(crate) async fn handle_browser_go_forward(
    request_id: &str,
    status_tx: Option<&tokio::sync::mpsc::UnboundedSender<String>>,
) -> String {
    if let Some(msg) = browser_tools_disabled_guard() {
        return msg;
    }

    send_status(status_tx, "⏩ Going forward…");
    info!("Agent router [{}]: BROWSER_GO_FORWARD", request_id);
    match tokio::task::spawn_blocking(crate::browser_agent::go_forward).await {
        Ok(Ok(state_str)) => state_str,
        Ok(Err(e)) => {
            let base = format!("BROWSER_GO_FORWARD failed: {}", e);
            let base = append_browser_readiness_context(base, true, None);
            append_latest_browser_state_guidance(&base)
        }
        Err(e) => format!("BROWSER_GO_FORWARD task error: {}", e),
    }
}

/// Optional arg: `nocache` or `hard` (case-insensitive) for cache-bypass reload; empty for normal refresh.
pub(crate) async fn handle_browser_reload(
    arg: &str,
    request_id: &str,
    status_tx: Option<&tokio::sync::mpsc::UnboundedSender<String>>,
) -> String {
    if let Some(msg) = browser_tools_disabled_guard() {
        return msg;
    }

    let tok = arg.split_whitespace().next().unwrap_or("").to_ascii_lowercase();
    let ignore_cache = matches!(tok.as_str(), "nocache" | "hard" | "bypass");
    let status_msg = if ignore_cache {
        "🔄 Reloading page (cache bypass)…"
    } else {
        "🔄 Reloading page…"
    };
    send_status(status_tx, status_msg);
    info!(
        "Agent router [{}]: BROWSER_RELOAD ignore_cache={}",
        request_id, ignore_cache
    );
    match tokio::task::spawn_blocking(move || {
        crate::browser_agent::reload_current_tab(ignore_cache)
    })
    .await
    {
        Ok(Ok(state_str)) => state_str,
        Ok(Err(e)) => {
            let base = format!("BROWSER_RELOAD failed: {}", e);
            let base = append_browser_readiness_context(base, true, None);
            append_latest_browser_state_guidance(&base)
        }
        Err(e) => format!("BROWSER_RELOAD task error: {}", e),
    }
}

pub(crate) async fn handle_browser_click(
    arg: &str,
    status_tx: Option<&tokio::sync::mpsc::UnboundedSender<String>>,
) -> String {
    if let Some(msg) = browser_tools_disabled_guard() {
        return msg;
    }

    let index_arg = arg.split_whitespace().next().unwrap_or("").trim();
    let index = index_arg.parse::<u32>().ok();
    let status_msg = match index {
        Some(idx) => {
            let label = crate::browser_agent::get_last_element_label(idx);
            if let Some(l) = label {
                format!(
                    "🖱️ Clicking element {} ({})",
                    idx,
                    crate::logging::ellipse(&l, 30)
                )
            } else {
                format!("🖱️ Clicking element {}", idx)
            }
        }
        None => format!(
            "🖱️ Clicking element {}",
            if index_arg.is_empty() { "?" } else { index_arg }
        ),
    };
    send_status(status_tx, &status_msg);
    match index {
        Some(idx) => {
            info!("BROWSER_CLICK: index {}", idx);
            match tokio::task::spawn_blocking(move || {
                crate::browser_agent::click_by_index(idx)
            })
            .await
            {
                Ok(Ok(state_str)) => state_str,
                Ok(Err(cdp_err)) => {
                    if should_use_http_fallback_after_browser_action_error(
                        "BROWSER_CLICK",
                        &cdp_err,
                    ) {
                        match tokio::task::spawn_blocking(move || {
                            crate::browser_agent::click_http(idx)
                        })
                        .await
                        {
                            Ok(Ok(state_str)) => state_str,
                            Ok(Err(e)) => {
                                let base = format!("BROWSER_CLICK failed: {}", e);
                                let base = append_browser_readiness_context(base, false, None);
                                append_latest_browser_state_guidance(&base)
                            }
                            Err(e) => format!("BROWSER_CLICK task error: {}", e),
                        }
                    } else {
                        let base = format!("BROWSER_CLICK failed: {}", cdp_err);
                        let base = append_browser_readiness_context(base, true, None);
                        append_latest_browser_state_guidance(&base)
                    }
                }
                Err(e) => append_latest_browser_state_guidance(&format!(
                    "BROWSER_CLICK task error: {}",
                    e
                )),
            }
        }
        None => append_latest_browser_state_guidance(
            "BROWSER_CLICK requires a numeric index (e.g. BROWSER_CLICK: 3). Use the index from the Current page Elements list.",
        ),
    }
}

pub(crate) async fn handle_browser_input(
    arg: &str,
    status_tx: Option<&tokio::sync::mpsc::UnboundedSender<String>>,
) -> String {
    if let Some(msg) = browser_tools_disabled_guard() {
        return msg;
    }

    let mut parts = arg.trim().splitn(2, |c: char| c.is_whitespace());
    let index_arg = parts.next().unwrap_or("").trim();
    let index_for_status = index_arg.parse::<u32>().ok();
    let status_msg = match index_for_status {
        Some(idx) => {
            let label = crate::browser_agent::get_last_element_label(idx);
            if let Some(l) = label {
                format!(
                    "✍️ Typing into element {} ({})…",
                    idx,
                    crate::logging::ellipse(&l, 30)
                )
            } else {
                format!("✍️ Typing into element {}…", idx)
            }
        }
        None => format!(
            "✍️ Typing into element {}…",
            if index_arg.is_empty() { "?" } else { index_arg }
        ),
    };
    send_status(status_tx, &status_msg);
    let text = parts.next().unwrap_or("").trim().to_string();
    let index = index_arg.parse::<u32>().ok();
    match index {
        Some(idx) => {
            info!("BROWSER_INPUT: index {} ({} chars)", idx, text.len());
            let text_clone = text.clone();
            match tokio::task::spawn_blocking(move || {
                crate::browser_agent::input_by_index(idx, &text_clone)
            })
            .await
            {
                Ok(Ok(state_str)) => state_str,
                Ok(Err(cdp_err)) => {
                    if should_use_http_fallback_after_browser_action_error(
                        "BROWSER_INPUT",
                        &cdp_err,
                    ) {
                        match tokio::task::spawn_blocking(move || {
                            crate::browser_agent::input_http(idx, &text)
                        })
                        .await
                        {
                            Ok(Ok(state_str)) => state_str,
                            Ok(Err(e)) => {
                                let base = format!("BROWSER_INPUT failed: {}", e);
                                let base = append_browser_readiness_context(base, false, None);
                                append_latest_browser_state_guidance(&base)
                            }
                            Err(e) => format!("BROWSER_INPUT task error: {}", e),
                        }
                    } else {
                        let base = format!("BROWSER_INPUT failed: {}", cdp_err);
                        let base = append_browser_readiness_context(base, true, None);
                        append_latest_browser_state_guidance(&base)
                    }
                }
                Err(e) => append_latest_browser_state_guidance(&format!(
                    "BROWSER_INPUT task error: {}",
                    e
                )),
            }
        }
        None => append_latest_browser_state_guidance(
            "BROWSER_INPUT requires a numeric index and text (e.g. BROWSER_INPUT: 4 search query). Use the index from the Current page Elements list.",
        ),
    }
}

pub(crate) async fn handle_browser_keys(
    arg: &str,
    status_tx: Option<&tokio::sync::mpsc::UnboundedSender<String>>,
) -> String {
    if let Some(msg) = browser_tools_disabled_guard() {
        return msg;
    }

    let chord = arg.trim().to_string();
    if chord.is_empty() {
        return append_latest_browser_state_guidance(
            "BROWSER_KEYS requires a chord: use + between parts, no spaces inside (e.g. BROWSER_KEYS: Escape, BROWSER_KEYS: Meta+f, BROWSER_KEYS: Ctrl+Enter). Allowlisted keys: Enter, Escape, Tab, Backspace, F5, or one letter a–z with at least one of Meta, Ctrl, Alt, Shift. Sends keys to the **page** only (not Chrome chrome UI). CDP required — no HTTP fallback.",
        );
    }
    send_status(
        status_tx,
        &format!("⌨️ Keys: {}…", crate::logging::ellipse(&chord, 40)),
    );
    info!("BROWSER_KEYS: {}", crate::logging::ellipse(&chord, 80));
    let chord_clone = chord.clone();
    match tokio::task::spawn_blocking(move || {
        crate::browser_agent::dispatch_browser_keys(&chord_clone)
    })
    .await
    {
        Ok(Ok(state_str)) => state_str,
        Ok(Err(e)) => {
            let base = format!("BROWSER_KEYS failed: {}", e);
            let base = append_browser_readiness_context(base, true, None);
            append_latest_browser_state_guidance(&base)
        }
        Err(e) => append_latest_browser_state_guidance(&format!("BROWSER_KEYS task error: {}", e)),
    }
}

pub(crate) async fn handle_browser_scroll(
    arg: &str,
    status_tx: Option<&tokio::sync::mpsc::UnboundedSender<String>>,
) -> String {
    if let Some(msg) = browser_tools_disabled_guard() {
        return msg;
    }

    let scroll_arg = if arg.trim().is_empty() {
        "down".to_string()
    } else {
        arg.trim().to_string()
    };
    send_status(
        status_tx,
        &format!("📜 Scrolling {}…", crate::logging::ellipse(&scroll_arg, 20)),
    );
    match tokio::task::spawn_blocking(move || crate::browser_agent::scroll_page(&scroll_arg)).await
    {
        Ok(Ok(state_str)) => state_str,
        Ok(Err(e)) => {
            info!(
                "BROWSER_SCROLL failed: {}",
                crate::logging::ellipse(&e, 200)
            );
            let base = format!("BROWSER_SCROLL failed: {}", e);
            append_browser_readiness_context(base, true, None)
        }
        Err(e) => format!("BROWSER_SCROLL task error: {}", e),
    }
}

pub(crate) async fn handle_browser_extract() -> String {
    if let Some(msg) = browser_tools_disabled_guard() {
        return msg;
    }

    match tokio::task::spawn_blocking(crate::browser_agent::extract_page_text).await {
        Ok(Ok(text)) => text,
        Ok(Err(_cdp_err)) => {
            match tokio::task::spawn_blocking(crate::browser_agent::extract_http).await {
                Ok(Ok(text)) => text,
                Ok(Err(e)) => format!(
                    "{}",
                    append_browser_readiness_context(
                        format!(
                            "BROWSER_EXTRACT failed: {}. (Navigate to a page first with BROWSER_NAVIGATE.)",
                            e
                        ),
                        false,
                        None
                    )
                ),
                Err(e) => format!("BROWSER_EXTRACT task error: {}", e),
            }
        }
        Err(e) => format!("BROWSER_EXTRACT task error: {}", e),
    }
}

pub(crate) async fn handle_browser_search_page(
    arg: &str,
    status_tx: Option<&tokio::sync::mpsc::UnboundedSender<String>>,
) -> String {
    if let Some(msg) = browser_tools_disabled_guard() {
        return msg;
    }

    let pattern = arg.trim().to_string();
    if pattern.is_empty() {
        return "BROWSER_SEARCH_PAGE requires a search pattern (e.g. BROWSER_SEARCH_PAGE: Ralf Röber). Use to find specific text on the current page.".to_string();
    }
    send_status(
        status_tx,
        &format!(
            "🔍 Searching page for \"{}\"…",
            crate::logging::ellipse(&pattern, 30)
        ),
    );
    match tokio::task::spawn_blocking(move || crate::browser_agent::search_page_text(&pattern))
        .await
    {
        Ok(Ok(result)) => result,
        Ok(Err(e)) => {
            info!(
                "BROWSER_SEARCH_PAGE failed: {}",
                crate::logging::ellipse(&e, 200)
            );
            let base = format!(
                "BROWSER_SEARCH_PAGE failed: {}. (Navigate to a page first with BROWSER_NAVIGATE.)",
                e
            );
            append_browser_readiness_context(base, true, None)
        }
        Err(e) => format!("BROWSER_SEARCH_PAGE task error: {}", e),
    }
}
