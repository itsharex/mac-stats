# WIP — OpenClaw-style Discord draft stream / throttled reply (2026-03-22)

**Preflight:** `tasks/UNTESTED-20260322-0105-openclaw-draft-stream-throttled-reply.md` was **not found** on disk in this workspace (`tasks/` listing on 2026-03-27). The `003-tester/TESTER.md` step **UNTESTED → TESTING** could not be applied. No other `UNTESTED-*` file was used.

**Context (repo):** Throttled in-place Discord draft edits live in `src-tauri/src/commands/discord_draft_stream.rs`, wired from `src-tauri/src/discord/mod.rs` (`spawn_discord_draft_editor`, `discord_draft_throttle_ms`). Operator docs: `docs/007_discord_agent.md`.

Restore the original task body under `UNTESTED-…` (or merge Goal / Acceptance criteria here) before closing.

## Test report

**Date:** 2026-03-27, local time (America/Mexico_City); stated explicitly for audit.

**Workflow**

- Expected source: `tasks/UNTESTED-20260322-0105-openclaw-draft-stream-throttled-reply.md` — **missing** (glob and `ls tasks/`).
- **UNTESTED → TESTING:** skipped (no source file).
- **Outcome file:** created directly as `WIP-20260322-0105-openclaw-draft-stream-throttled-reply.md` because the formal task acceptance criteria were unavailable.

**Commands run**

```bash
ls -la tasks/
cd src-tauri && cargo check
cd src-tauri && cargo test discord_draft_stream::
```

**Static spot-check (manual)**

- `spawn_discord_draft_editor` referenced from `src-tauri/src/discord/mod.rs`.
- `DiscordDraftHandle` used from `commands/tool_loop.rs`, `commands/turn_lifecycle.rs`, `commands/ollama.rs`.

**Results**

| Check | Result |
|--------|--------|
| `cargo check` (src-tauri) | **Pass** |
| `cargo test discord_draft_stream::` (2 tests: `clamp_*`) | **Pass** |
| Task-file-driven acceptance criteria | **Not executed** (no task body) |
| Live Discord / router manual test | **Not run** |

**Conclusion:** **WIP** — blocked at tester preflight (missing `UNTESTED-*` artifact). Partial automated checks for the related `discord_draft_stream` module passed. Re-run tester workflow after the UNTESTED file exists with explicit Goal and Verification commands.
