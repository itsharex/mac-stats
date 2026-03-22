# Feature coder — FEAT backlog

Agent-facing backlog for mac-stats. Pick an open row, implement, run `cargo check` / `cargo test` / `cargo clippy`, then mark done and note in `CHANGELOG.md` under `[Unreleased]` when behaviour changes.

## Recently closed

| ID | Item | Notes |
|----|------|--------|
| FEAT-D5 | SSRF redirect DNS / empty resolve | `check_redirect_target_ssrf()`: redirect hop must resolve like the initial URL; DNS failure or zero addresses → `PermissionDenied` (no silent `follow`). Extracted from `ssrf_redirect_policy`; 4 unit tests (private IP, allowlist, public, `.invalid` DNS fail). Closes 022 §9 nit (redirect DNS-fail previously followed). |
| FEAT-D4 | FETCH_URL truncation marker | Oversized bodies: `truncate_fetch_body_if_needed()` ellipses with budget `MAX_BODY_CHARS - len(suffix)` and appends ` [content truncated]`; unit tests for short/long. Resolves review doc D3 (explicit LLM hint). |
| FEAT-D3 | SSRF: IPv4-mapped broadcast | `is_blocked_ip` IPv6 branch: `to_ipv4_mapped()` closure now includes `is_broadcast()` (parity with IPv4); test for `::ffff:255.255.255.255`. |
| FEAT-D2 | Clippy `items_after_test_module` | `logging/subsystem.rs`: `mod tests` moved after exported `mac_stats_*` macros. |
| FEAT-D1 | Session file resume: legacy filename layout | `load_messages_from_latest_session_file` matches new `session-memory-{id}-{ts}-{topic}.md` and legacy `session-memory-{topic}-{id}-{ts}.md` via `session_filename_matches_id` + tests. |

## Open / deferred (no owner)

Large integrations and vague items stay in **docs/006_roadmap_ai_tasks.md** and per-doc “Future/backlog” sections (Mail, WhatsApp, Google Docs, parallel tool loop, etc.) until scoped.

## When empty

Triage **docs/022_feature_review_plan.md** unchecked checklist items, **docs/035_memory_and_topic_handling.md**, or run `cargo clippy` for actionable warnings.
