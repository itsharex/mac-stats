# Plan: Lean Chrome Helper Processes (Browser / Navigate)

## Problem

Chrome processes spawned by mac-stats for BROWSER_NAVIGATE / BROWSER_* (and when the user or scheduler triggers browser use) run in the background and consume a lot of CPU. We want to keep this as lean as possible.

---

## Why Do We Have Multiple Chrome Processes? (debug.log review)

Reviewing `~/.mac-stats/debug.log` shows a clear pattern:

- **"reusing existing session"** is logged, then **within the same second or a few seconds** we see **"user requested headless — launching headless Chrome"** and **"Started Chrome. PID: XXXXX"** with a **new PID** every time.
- Many distinct PIDs over a short period: e.g. 51453, 52581, 52992, 54264, 54419, 55618, 72250, 73128, 87420, 62703, 62981, 92947, 94149, 96133, 96727, 97499, 97871, 652, 988, 2315, 3778, 5026.

So we are **launching a new Chrome process very often** instead of reusing one.

### Root causes

1. **Connection error + retry**  
   When a CDP operation fails with a connection error (e.g. "connection is closed"), we call `clear_browser_session_on_error()` and set `BROWSER_SESSION` to `None`. The **retry** then calls `get_or_create_browser()` again, sees no session, and launches a **new** Chrome. The **old** Chrome is only killed when the previous `Browser` handle is dropped (when the first attempt returns). So we can briefly have two processes, or the old process can outlive the session if something else still holds a clone.

2. **Concurrent requests (race)**  
   Two flows can use the browser at the same time (e.g. Discord message + scheduler task, or two Discord messages). Each runs in its own `spawn_blocking`. So two threads can call `get_or_create_browser()`:
   - Thread A: lock, see `None` (or expired), set `None`, drop guard, call `launch_via_headless_chrome()` (takes several seconds).
   - Thread B: lock, still sees `None` (A has not stored yet), drop guard, call `launch_via_headless_chrome()`.
   - Both spawn a new Chrome. Both then store their `Browser`; the second store overwrites the first, so the first `Browser` is dropped and its process is killed. So we can have **two Chrome processes alive at once** until the overwrite, and we **replace the session** with a new process instead of reusing the one the first thread just created.

3. **Session cleared, then immediate reuse log from another path**  
   One call clears the session (e.g. after error); another call (or retry) had already logged "reusing existing session" before the clear, or a concurrent call reuses a stale handle. So logs show "reuse" then "launch" in quick succession.

### Conclusion

- **Multiple Chrome processes** come from (a) connection-error clear + retry launching a new Chrome while the old one may still be shutting down or still held, and (b) **concurrent** `get_or_create_browser()` calls both deciding to launch, so two processes are started and only one is kept in the session.
- To have **at most one** Chrome and reuse it, we need to:
  - **Serialize** the "create browser" path (e.g. a single lock or guard so only one thread can be in `launch_via_headless_chrome()` at a time), and/or
  - Avoid clearing the session on connection error in a way that causes every retry to spawn a new process; or ensure that after a clear we don’t have two concurrent launches.
  - Optionally **shorten the idle timeout** so we don’t keep a long-lived session that then gets invalidated (e.g. Chrome crash) and replaced by a new launch.

## Current Behaviour

1. **Visible Chrome** (user asks for “browser” or doesn’t say “headless”):  
   We spawn Chrome via `launch_chrome_on_port(9222)` with only:
   - `--remote-debugging-port=9222`
   - `--window-size=W,H`
   - `--no-first-run`
   - `--no-default-browser-check`  
   No other flags → full Chrome with extensions, GPU, sync, background networking, etc., and many helper processes.

2. **Headless Chrome** (user says “headless” or Discord/scheduler/task):  
   We use the `headless_chrome` crate’s `LaunchOptions::default_builder()` with only `window_size` set. The crate already adds:
   - Its `DEFAULT_ARGS` (e.g. `--disable-background-networking`, `--disable-extensions`, `--disable-sync`, `--disable-default-apps`, …)
   - `--headless`, `--disable-gpu`, and (if sandbox disabled) `--no-sandbox`  
   So headless is already relatively lean; we can still add a few extra args if needed.

3. **Session lifetime**:  
   Browser session is cached until idle for `Config::browser_idle_timeout_secs()` — currently **hardcoded 3600** (1 hour). Chrome can sit there for an hour after last use, with all helper processes still running.

4. **Existing Chrome on 9222**:  
   If the user (or something else) already started Chrome with `--remote-debugging-port=9222`, we connect to it and do not control its flags. This plan only affects **Chrome instances launched by mac-stats**.

---

## Proposed Changes

### 0. Single Chrome process: serialize launch and avoid races (multiple-process fix)

**Where:** `src-tauri/src/browser_agent/mod.rs` — `get_or_create_browser()` and optionally callers.

**What:**

- **Serialize the "create" path** so only one thread can be inside `launch_via_headless_chrome()` (or the visible-Chrome launch path) at a time. For example:
  - A dedicated `OnceLock<Mutex<()>>` or a static mutex that is held for the entire "create new browser" block (from "session is None/expired" until we store the new browser). So: lock → check session again inside the lock; if still need to create, create while holding the lock (or hold a "launch in progress" lock so a second thread waits and then re-checks session).
  - Or: one global mutex for "obtain or create browser": every caller takes the mutex, then either reuses existing or creates one and stores it; no dropping the lock between "see None" and "store new".
- **Avoid duplicate launches on connection error + retry:** When we clear the session on connection error, the retry will create a new browser. Ensure the old `Browser` (and thus its process) is dropped before we launch the new one (e.g. don’t hold a clone across the retry), and ensure only one thread can create at a time so we don’t get two threads both creating after a clear.

**Goal:** At most one mac-stats-launched Chrome process at a time; new launches only when the session is truly missing or expired, not due to races or repeated clears.

**Implemented (current):**
- **Kill orphaned headless Chrome:** `kill_orphaned_browser_processes()` finds Chrome processes whose command line contains `rust-headless-chrome-profile` (headless_chrome crate temp dir) and sends them `SIGTERM`. Called (1) at app startup in `lib.rs` setup, and (2) immediately before launching a new headless Chrome in `get_or_create_browser()`.
- **Serialize creation:** A static `LAUNCH_MUTEX` is held for the entire "create new browser" path. After acquiring it we re-check the session (another thread may have just created one); only if still needed do we call `kill_orphaned_browser_processes()` and then `launch_via_headless_chrome()`.

---

### 1. Visible Chrome (mac-stats-launched on 9222) — add lean flags

**Where:** `src-tauri/src/browser_agent/mod.rs` → `launch_chrome_on_port()` (macOS and non-macOS).

**What:** When we spawn Chrome for the debugging port, add a minimal set of flags to reduce background work and helper process load, while keeping CDP and basic page rendering working:

- `--disable-extensions`
- `--disable-background-networking`
- `--disable-sync`
- `--disable-default-apps`
- `--no-first-run` (already present)
- `--no-default-browser-check` (already present)
- Optionally: `--disable-background-timer-throttling`, `--disable-renderer-backgrounding` (often used in automation to avoid throttling).

**Avoid (for visible mode):**

- `--disable-gpu` in visible mode can make the window sluggish or cause odd rendering; leave GPU on for visible Chrome unless we see high CPU from GPU process and decide otherwise.
- `--headless` — not used here; this path is for visible Chrome.

**Risk:** Some sites or features might rely on extensions or sync; we use this Chrome only for automation (navigate, click, screenshot), so disabling extensions and sync is acceptable.

---

### 2. Headless Chrome (headless_chrome crate) — optional extra args

**Where:** `src-tauri/src/browser_agent/mod.rs` → `launch_via_headless_chrome()`.

**What:** The crate already adds many lean defaults. We can add a few extra args via `LaunchOptions::default_builder().args([...])` to further reduce load, e.g.:

- `--disable-software-rasterizer`
- `--disable-dev-shm-usage` (often already in crate’s DEFAULT_ARGS)
- `--mute-audio`

We will only add flags that are supported and that we can verify don’t break navigate/click/screenshot.

**Optional:** Consider `sandbox(false)` to get `--no-sandbox` and slightly fewer isolation processes (security trade-off; many automation setups use it). Document the choice.

---

### 3. Shorter default idle timeout + configurable timeout

**Where:**  
- `src-tauri/src/config/mod.rs` → `browser_idle_timeout_secs()`  
- Optionally `config.json` and env (e.g. `MAC_STATS_BROWSER_IDLE_TIMEOUT_SECS`).

**What:**

- Make idle timeout **configurable** (e.g. from `config.json` and/or env), with a **lower default** so we don’t keep Chrome around for an hour when unused.
- Suggested default: **300** (5 minutes) or **600** (10 minutes). Current 3600 (1 hour) keeps helper processes alive much longer than needed for typical “run a few browser steps then stop” usage.

**Behaviour:** After the last BROWSER_* use, if there is no activity for `browser_idle_timeout_secs`, we drop the cached session and the Chrome process exits (when we launched it). Next BROWSER_* will reconnect or relaunch.

---

### 4. Documentation and config

**Where:**  
- `docs/` (e.g. existing browser/CDP docs or a short “browser automation” note).  
- `config` / `Config` if we add new keys.

**What:**

- Document that mac-stats-launched Chrome (visible and headless) is started with lean flags to reduce CPU.
- Document the new config key and env for browser idle timeout and the new default.
- Mention that if the user starts Chrome themselves on 9222, we don’t change its flags.

---

## Summary Table

| Item | Action | Goal |
|------|--------|------|
| **Multiple Chrome processes** | Serialize browser creation (lock around "create" path); ensure only one thread can launch at a time; avoid duplicate launches on clear+retry | At most one Chrome process; no races leading to multiple PIDs |
| Visible Chrome (launch_chrome_on_port) | Add lean flags (no extensions, no background networking, no sync, no default apps, etc.) | Fewer helper processes and less background CPU |
| Headless Chrome (launch_via_headless_chrome) | Keep crate defaults; optionally add a few extra args | Slightly leaner if needed |
| Idle timeout | Configurable; default 300 or 600 s instead of 3600 | Close browser sooner when unused → less time with helper processes running |
| Docs / config | Document flags, timeout, and multi-process fix | Clear for users and future changes |

---

## Out of Scope (for this plan)

- Changing how we detect or connect to an existing Chrome on 9222 (we don’t control its flags).
- Switching to a different browser or automation stack.
- Changing when we choose headless vs visible (that stays as today: e.g. “headless” in question or from_remote → headless).

---

## Sign-off

If you’re happy with this plan, we can implement in this order:

1. **Single Chrome process:** Serialize browser creation so only one thread can launch at a time; prevent races and duplicate launches (fix for multiple Chrome PIDs seen in debug.log).
2. Config: make `browser_idle_timeout_secs` configurable and lower default (e.g. 300 or 600).
3. Visible Chrome: add lean flags in `launch_chrome_on_port()`.
4. Headless Chrome: optionally add extra `.args()` in `launch_via_headless_chrome()`.
5. Docs and any config schema notes.

Please confirm or adjust (e.g. default timeout value, or which flags to include/avoid) and we can proceed.
