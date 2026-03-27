//! One-shot navigate to a slow httpbin URL (same CDP stack as `BROWSER_NAVIGATE`, without the tool dispatcher).
//!
//! **Quick timeout check:** Chrome must be listening on the configured CDP port (default 9222).
//! ```text
//! cd src-tauri
//! MAC_STATS_BROWSER_NAVIGATION_TIMEOUT_SECS=5 cargo run --example httpbin_delay_nav
//! ```
//! Expect exit **1** and stderr containing `Navigation failed: timeout after 5s` (minimum timeout is 5s in config).

use mac_stats::browser_agent::navigate_and_get_state;

fn main() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_new("warn,mac_stats=info")
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .try_init();

    // `/delay/10` stays in-flight longer than the minimum 5s navigation timeout when env is set to 5.
    match navigate_and_get_state("https://httpbin.org/delay/10") {
        Ok(state) => println!("ok (state {} chars)", state.len()),
        Err(e) => {
            eprintln!("navigate failed: {e}");
            std::process::exit(1);
        }
    }
}
