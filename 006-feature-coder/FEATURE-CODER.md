# Feature coder — FEAT backlog

Agent-facing backlog for mac-stats. Pick an open row, implement, run `cargo check` / `cargo test` / `cargo clippy`, then mark done and note in `CHANGELOG.md` under `[Unreleased]` when behaviour changes.

## Recently closed

| ID | Item | Notes |
|----|------|--------|
| FEAT-D8 | Conversation history cap helper + F1 ordering test | `cap_tail_chronological` in `commands/session_history.rs` (same semantics as `rev().take(N).rev()`); four unit tests. Used by `answer_with_ollama_and_fetch` and Discord having_fun (20 / 10 caps). `get_messages_before_add_user_excludes_current_turn` in `session_memory.rs` documents F1: fetch prior before `add_message` for the current user turn. Closes 022 §3 F1/F2 checklist automation. |
| FEAT-D7 | Session file parse: headings in body | `load_messages_from_latest_session_file` / `parse_session_markdown`: only full-line `## User` / `## Assistant` start a block (line-oriented parser). Lines like `## Notes` inside a message stay in the body; old `split("\n## ")` dropped those turns. Four unit tests. Closes 022 F1 checklist edge case (`## ` in content). |
| FEAT-D6 | Monitor check interval minimum | `check_interval_secs` clamped to ≥1 on add, load, and in `run_due_monitor_checks` so `0` cannot make a monitor due every pass (`elapsed >= 0`). Three unit tests. Closes review doc F10 edge case (022 §3). |
| FEAT-D5 | SSRF redirect DNS / empty resolve | `check_redirect_target_ssrf()`: redirect hop must resolve like the initial URL; DNS failure or zero addresses → `PermissionDenied` (no silent `follow`). Extracted from `ssrf_redirect_policy`; 4 unit tests (private IP, allowlist, public, `.invalid` DNS fail). Closes 022 §9 nit (redirect DNS-fail previously followed). |
| FEAT-D4 | FETCH_URL truncation marker | Oversized bodies: `truncate_fetch_body_if_needed()` ellipses with budget `MAX_BODY_CHARS - len(suffix)` and appends ` [content truncated]`; unit tests for short/long. Resolves review doc D3 (explicit LLM hint). |
| FEAT-D3 | SSRF: IPv4-mapped broadcast | `is_blocked_ip` IPv6 branch: `to_ipv4_mapped()` closure now includes `is_broadcast()` (parity with IPv4); test for `::ffff:255.255.255.255`. |
| FEAT-D2 | Clippy `items_after_test_module` | `logging/subsystem.rs`: `mod tests` moved after exported `mac_stats_*` macros. |
| FEAT-D1 | Session file resume: legacy filename layout | `load_messages_from_latest_session_file` matches new `session-memory-{id}-{ts}-{topic}.md` and legacy `session-memory-{topic}-{id}-{ts}.md` via `session_filename_matches_id` + tests. |

## Open / deferred (no owner)

Large integrations and vague items stay in **docs/006_roadmap_ai_tasks.md** and per-doc “Future/backlog” sections (Mail, WhatsApp, Google Docs, parallel tool loop, etc.) until scoped.

## When empty

Triage **docs/022_feature_review_plan.md** unchecked checklist items, **docs/035_memory_and_topic_handling.md**, or run `cargo clippy` for actionable warnings.
