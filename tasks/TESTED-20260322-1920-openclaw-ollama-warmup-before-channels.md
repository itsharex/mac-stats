# OpenClaw — Ollama startup warmup before Discord / scheduler / heartbeat

## Goal

`ensure_ollama_agent_ready_at_startup` must run to completion on the Tauri async runtime **before** spawning the Discord gateway, scheduler, heartbeat, task review, and other channel-style automation, so the first inbound Discord message or due scheduled job does not race default Ollama config, `GET /api/tags`, or `ModelCatalog` population.

## Acceptance criteria

- `lib.rs` startup path calls `ensure_ollama_agent_ready_at_startup().await` inside `tauri::async_runtime::block_on` **before** any `discord::spawn_discord_if_configured`, `scheduler::spawn_scheduler_thread`, or `scheduler::heartbeat::spawn_heartbeat_thread`.
- A structured log line documents the gate opening (`mac_stats_startup` target: `Ollama startup warmup finished (gate open); spawning Discord…`).
- Warmup failures are non-fatal (warnings, automation still starts); behaviour matches `docs/015_ollama_api.md` startup ordering note.
- `cd src-tauri && cargo check` succeeds.

## Verification commands

```bash
rg -n 'ensure_ollama_agent_ready_at_startup|Ollama startup warmup finished' src-tauri/src/lib.rs
rg -n 'spawn_discord_if_configured|spawn_scheduler_thread|spawn_heartbeat_thread' src-tauri/src/lib.rs
cd src-tauri && cargo check
```

## Test report

### Run: 2026-03-28 (closing reviewer + task verification)

**Preflight:** En el árbol **no existía** `tasks/TESTING-20260322-1920-openclaw-ollama-warmup-before-channels.md`; se creó el cuerpo de la tarea a partir del orden real en `src-tauri/src/lib.rs` y `docs/015_ollama_api.md`. No se eligió otro archivo de tarea.

**Commands run**

- `rg -n 'ensure_ollama_agent_ready_at_startup|Ollama startup warmup finished' src-tauri/src/lib.rs` — **pass** (`block_on` + `ensure_ollama_agent_ready_at_startup().await` en L461; log `mac_stats_startup` en L463–465).
- `rg -n 'spawn_discord_if_configured|spawn_scheduler_thread|spawn_heartbeat_thread' src-tauri/src/lib.rs` — **pass** (Discord L471, scheduler L475, heartbeat L478; todas **después** del warmup).
- `cd src-tauri && cargo check` — **pass**.
- `cd src-tauri && cargo test` — **pass** (**870** tests en crate `mac_stats`; **1** doc-test ignorado).
- `cd src-tauri && cargo build --release` — **pass** (**v0.1.68** en `Cargo.toml`).
- `cd src-tauri && cargo clippy --all-targets -- -D warnings` — **fail** (decenas de lints preexistentes en `browser_agent/`, `commands/`, etc.; **no** atribuibles al gate de warmup de Ollama).

**Docs**

- `docs/015_ollama_api.md` — **pass** (párrafo «Startup ordering» describe el mismo contrato: warmup antes de Discord/scheduler/heartbeat, log `mac_stats_startup`, fallos no fatales).

**Outcome:** Criterios de aceptación de la tarea verificados. La barra completa de **`004-closing-reviewer/CLOSING-REVIEWER-PROMPT.md`** (`clippy -D warnings`) **no** se cumple en el workspace actual → prefijo de archivo **`TESTED-`** (no **`CLOSED-`**). Sin **`pkill -f mac_stats`** (regla **AGENTS.md**).
