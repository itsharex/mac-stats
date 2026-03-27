# CLOSED — browser-use graceful browser shutdown (2026-03-21)

## Goal

Ensure mac-stats closes the CDP browser session on process exit via both Tauri `RunEvent::Exit` and signal paths (`ctrlc` for SIGINT/SIGTERM/SIGHUP), matching **browser-use-style** safety: `close_browser_session()` runs, headless Chrome may receive SIGTERM, visible/user Chrome is not killed.

## References

- `src-tauri/src/lib.rs` — `ctrlc::set_handler`, `RunEvent::Exit` → `close_browser_session()`
- `src-tauri/src/browser_agent/mod.rs` — `close_browser_session()`, `BROWSER_INTENTIONAL_STOP`
- `docs/029_browser_automation.md` — “App shutdown” paragraph

## Acceptance criteria

1. **Build:** `cargo check` in `src-tauri/` succeeds.
2. **Tests:** `cargo test` in `src-tauri/` succeeds (no new failures attributable to browser shutdown paths).
3. **Static verification:** Source still registers shutdown hooks and calls `close_browser_session` from both paths (spot-check via repository search or read).

## Verification commands

```bash
cd src-tauri && cargo check
cd src-tauri && cargo test
```

Optional spot-check:

```bash
rg -n "close_browser_session|RunEvent::Exit|ctrlc::set_handler" src-tauri/src/lib.rs
rg -n "pub fn close_browser_session" src-tauri/src/browser_agent/mod.rs
```

## Test report

**Date:** 2026-03-27 (session date from operator environment).

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test` — **pass** (854 unit tests in `mac_stats` library crate; 0 failed; 1 doc-test ignored)

**Static spot-check**

- `src-tauri/src/lib.rs`: `ctrlc::set_handler` invokes `close_browser_session()`; `RunEvent::Exit` path logs and calls `close_browser_session()`.
- `src-tauri/src/browser_agent/mod.rs`: `pub fn close_browser_session` present at line ~4267.

**Outcome:** All acceptance criteria satisfied for this verification pass. End-to-end “quit app with live CDP session” was not exercised in automation here (manual/operator smoke if desired).
