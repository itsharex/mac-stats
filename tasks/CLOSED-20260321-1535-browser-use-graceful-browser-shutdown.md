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

### Tester run — 2026-03-27 (local)

**Note:** Operator asked to test `tasks/UNTESTED-20260321-1535-browser-use-graceful-browser-shutdown.md`; that path was absent. The same task existed as `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`; workflow followed by renaming `CLOSED-` → `TESTING-`, re-running verification, appending this report, then renaming back to `CLOSED-`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed, 0 ignored in `mac_stats` lib tests; 1 doc-test ignored)
- `rg -n "close_browser_session|RunEvent::Exit|ctrlc::set_handler" src-tauri/src/lib.rs` — **pass** (matches at lines 236–239, 1681–1686)
- `rg -n "pub fn close_browser_session" src-tauri/src/browser_agent/mod.rs` — **pass** (line 4266)

**Outcome:** All acceptance criteria satisfied. **Final filename:** `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`.

### Tester run — 2026-03-27 (local, operator session)

**Note:** Operator requested `tasks/UNTESTED-20260321-1535-browser-use-graceful-browser-shutdown.md`; that filename was not present in the repo (same task was `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`). Per TESTER.md, renamed `CLOSED-` → `TESTING-`, ran verification, appended this report, then renamed to `CLOSED-`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)
- `rg -n "close_browser_session|RunEvent::Exit|ctrlc::set_handler" src-tauri/src/lib.rs` — **pass** (matches at lines 236–239, 1681–1686)
- `rg -n "pub fn close_browser_session" src-tauri/src/browser_agent/mod.rs` — **pass** (line 4266)

**Outcome:** All acceptance criteria satisfied. **Final filename:** `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`.

### Tester run — 2026-03-27 (local, Cursor agent)

**Note:** Operator requested `tasks/UNTESTED-20260321-1535-browser-use-graceful-browser-shutdown.md`; that path was not in the repo. The same task was present as `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`. Per TESTER.md, renamed `CLOSED-` → `TESTING-`, ran verification, appended this report, then renamed to `CLOSED-`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)
- `rg` spot-check on `lib.rs` and `browser_agent/mod.rs` — **pass** (`ctrlc::set_handler` and `RunEvent::Exit` call `close_browser_session`; `pub fn close_browser_session` at line 4266)

**Outcome:** All acceptance criteria satisfied. **Final filename:** `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`.

### Tester run — 2026-03-27 (local, Cursor agent; operator-requested retest)

**Date:** 2026-03-27 (local wall-clock; not UTC).

**Note:** Operator requested `tasks/UNTESTED-20260321-1535-browser-use-graceful-browser-shutdown.md` only; that path was not in the repository. The task file on disk was `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`. Per `003-tester/TESTER.md`: renamed `CLOSED-` → `TESTING-`, ran verification, append this report, then rename to `CLOSED-` (all criteria passed).

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)
- `rg -n "close_browser_session|RunEvent::Exit|ctrlc::set_handler" src-tauri/src/lib.rs` — **pass** (lines 236–239, 1681–1686)
- `rg -n "pub fn close_browser_session" src-tauri/src/browser_agent/mod.rs` — **pass** (line 4266)

**Outcome:** All acceptance criteria satisfied. **Final filename:** `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`.

### Tester run — 2026-03-27 (local, Cursor agent)

**Date:** 2026-03-27 (hora local del entorno; no UTC).

**Note:** El operador pidió probar solo `tasks/UNTESTED-20260321-1535-browser-use-graceful-browser-shutdown.md`; ese path no existe en el repo (la tarea ya estaba como `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md` antes de renombrar a `TESTING-`). Siguiendo `003-tester/TESTER.md`: `CLOSED-` → `TESTING-`, verificación, este informe, luego `CLOSED-`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la crate `mac_stats`; 1 doc-test ignored)
- `rg` spot-check en `lib.rs` y `browser_agent/mod.rs` — **pass** (mismas líneas que en la corrida anterior: 236–239, 1681–1686; `close_browser_session` en 4266)

**Outcome:** All acceptance criteria satisfied. **Final filename:** `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`.

### Tester run — 2026-03-27 (local, Cursor agent; sesión actual)

**Date:** 2026-03-27 (hora local del workspace; no UTC).

**Note:** El operador pidió `tasks/UNTESTED-20260321-1535-browser-use-graceful-browser-shutdown.md`; ese path no existía (la tarea estaba como `CLOSED-...`). Según `003-tester/TESTER.md`: `CLOSED-` → `TESTING-`, verificación, este informe, luego `CLOSED-`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en la crate `mac_stats`; 1 doc-test ignored)
- `rg -n "close_browser_session|RunEvent::Exit|ctrlc::set_handler" src-tauri/src/lib.rs` — **pass** (236–239, 1681–1686)
- `rg -n "pub fn close_browser_session" src-tauri/src/browser_agent/mod.rs` — **pass** (4266)

**Outcome:** Todos los criterios de aceptación satisfechos. **Nombre final:** `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`.

### Tester run — 2026-03-28 (local, America/Los_Angeles workspace)

**Note:** Operator requested `tasks/UNTESTED-20260321-1535-browser-use-graceful-browser-shutdown.md` only; that path was not in the repo. The task file was `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`. Per `003-tester/TESTER.md`: renamed `CLOSED-` → `TESTING-`, ran verification, append this report, then rename to `CLOSED-` (all criteria passed).

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed in `mac_stats` library crate; 1 doc-test ignored)
- `rg` spot-check on `lib.rs` (`close_browser_session`, `RunEvent::Exit`, `ctrlc::set_handler`) — **pass** (lines ~236–239, ~1681–1686)
- `rg` on `browser_agent/mod.rs` (`pub fn close_browser_session`) — **pass** (line ~4266)

**Outcome:** All acceptance criteria satisfied. **Final filename:** `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`.

### Tester run — 2026-03-28 (Cursor agent; local workspace; date not UTC)

**Note:** Requested `tasks/UNTESTED-20260321-1535-browser-use-graceful-browser-shutdown.md` only; that path was absent. The task existed as `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`. Per `003-tester/TESTER.md`: renamed `CLOSED-` → `TESTING-`, ran verification, appended this report, then renamed to `CLOSED-`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)
- `rg` on `src-tauri/src/lib.rs` (`close_browser_session`, `RunEvent::Exit`, `ctrlc::set_handler`) — **pass** (lines 236–239, 1681–1686)
- `rg` on `src-tauri/src/browser_agent/mod.rs` (`pub fn close_browser_session`) — **pass** (line 4266)

**Outcome:** All acceptance criteria satisfied. **Final filename:** `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`.

### Tester run — 2026-03-28 (UTC)

**Note:** El operador pidió probar solo `tasks/UNTESTED-20260321-1535-browser-use-graceful-browser-shutdown.md`; ese archivo no existe en el workspace (la tarea estaba como `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`). Según `003-tester/TESTER.md`: se renombró `CLOSED-` → `TESTING-`, se ejecutó la verificación, se añade este informe y se vuelve a `CLOSED-` si todo pasa.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la librería `mac_stats`; 1 doc-test ignored)
- `rg` en `src-tauri/src/lib.rs` (`close_browser_session`, `RunEvent::Exit`, `ctrlc::set_handler`) — **pass** (líneas 236–239, 1681–1686)
- `rg` en `src-tauri/src/browser_agent/mod.rs` (`pub fn close_browser_session`) — **pass** (línea 4266)

**Outcome:** Todos los criterios de aceptación satisfechos. **Nombre final:** `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`.

### Tester run — 2026-03-28 (local)

**Note:** Operator requested only `tasks/UNTESTED-20260321-1535-browser-use-graceful-browser-shutdown.md`; that path was not in the repo. The sole matching task file was `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`, renamed to `TESTING-…` per `003-tester/TESTER.md` (same task ID; no other `UNTESTED-*` file was used).

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed, 0 ignored in `mac_stats` lib tests; 1 doc-test ignored)
- `rg -n "close_browser_session|RunEvent::Exit|ctrlc::set_handler" src-tauri/src/lib.rs` — **pass** (lines 236–239, 1681–1686)
- `rg -n "pub fn close_browser_session" src-tauri/src/browser_agent/mod.rs` — **pass** (line 4266)

**Outcome:** All acceptance criteria satisfied. **Final filename:** `CLOSED-20260321-1535-browser-use-graceful-browser-shutdown.md`.
