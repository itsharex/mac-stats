//! One initial **BROWSER_NAVIGATE** (no `new_tab`), then **five** sequential **`new_tab`** navigations — each is a
//! separate process call, so the agent-router **stale-batch guard** does not apply and
//! **`try_enforce_browser_tab_limit`** runs after every successful navigation.
//!
//! Prereq: Chromium with CDP on the configured port (default **9222**), same as other `examples/`.
//!
//! Run from `src-tauri/`:
//!
//! ```text
//! MAC_STATS_BROWSER_MAX_PAGE_TABS=3 cargo run --example managed_tab_cap_smoke
//! ```
//!
//! Or set **`browserMaxPageTabs`** in `~/.mac-stats/config.json` instead of the env var.
//! Use **`RUST_LOG=mac_stats=debug`** (or **`mac_stats=info`**) to see **`managed tab cap`** lines on stderr.
//!
//! If you see **`The event waited for never came`** on an **empty** debug Chrome window, open **one** normal
//! tab manually (or start Chrome with a non-empty window), then re-run — CDP tab creation can time out when
//! the browser has zero page targets.

use mac_stats::browser_agent::{
    navigate_and_get_state, navigate_and_get_state_with_options, switch_tab_to_index,
};
use mac_stats::config::Config;

fn tab_count_from_tabs_line(snapshot: &str) -> Option<usize> {
    for line in snapshot.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("Tabs:") {
            let rest = rest.trim();
            if rest.is_empty() {
                return Some(0);
            }
            return Some(rest.split('|').count());
        }
    }
    None
}

fn main() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_new("warn,mac_stats=debug")
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .try_init();

    let cap = Config::browser_max_page_tabs();
    if cap == 0 {
        eprintln!(
            "Set MAC_STATS_BROWSER_MAX_PAGE_TABS (e.g. 3) or browserMaxPageTabs in config.json; cap must be > 0."
        );
        std::process::exit(2);
    }

    println!("managed_tab_cap_smoke: cap={cap} (from config/env)");

    println!("Step A: first navigation https://example.com/ (no new_tab when tabs already exist)");
    let s1 = match navigate_and_get_state("https://example.com/") {
        Ok(s) => s,
        Err(e) if e.contains("no open tabs") || e.contains("automatic tab creation failed") => {
            println!("--- empty CDP window: retrying first step with new_tab=true ---");
            navigate_and_get_state_with_options("https://example.com/", true).unwrap_or_else(|e2| {
                eprintln!("navigate failed: {e2}");
                std::process::exit(1);
            })
        }
        Err(e) => {
            eprintln!("navigate failed: {e}");
            std::process::exit(1);
        }
    };
    let n1 = tab_count_from_tabs_line(&s1).unwrap_or_else(|| {
        eprintln!("could not parse Tabs: line:\n{s1}");
        std::process::exit(1);
    });
    println!("--- tab_count={n1} ---\n");

    let mut last_snapshot = s1;
    let mut last_count = n1;

    for i in 1..=5 {
        println!("Step B{i}: new_tab navigate https://example.com/");
        let sn =
            navigate_and_get_state_with_options("https://example.com/", true).unwrap_or_else(|e| {
                eprintln!("navigate new_tab failed: {e}");
                std::process::exit(1);
            });
        let nn = tab_count_from_tabs_line(&sn).unwrap_or_else(|| {
            eprintln!("could not parse Tabs: line:\n{sn}");
            std::process::exit(1);
        });
        println!("--- tab_count={nn} (expect <= {cap} after enforcement) ---\n");
        if nn > cap {
            eprintln!("FAIL: tab_count {nn} exceeds cap {cap}");
            std::process::exit(1);
        }
        last_snapshot = sn;
        last_count = nn;
    }

    if !last_snapshot.contains("(active)") {
        eprintln!("FAIL: final snapshot missing (active) marker");
        std::process::exit(1);
    }
    for j in 0..last_count {
        switch_tab_to_index(j).unwrap_or_else(|e| {
            eprintln!("FAIL: BROWSER_SWITCH_TAB index {j}: {e}");
            std::process::exit(1);
        });
        println!("--- BROWSER_SWITCH_TAB index={j} ok ---");
    }
    println!("DONE: managed tab cap smoke passed (final_tabs={last_count}, cap={cap})");
}
