# Release 0.1.18 (2026-02-22)

## Added

- **Task file naming**: New convention `task-<date-time>-<status>.md` (e.g. `task-20260222-140215-open.md`). Topic and id are stored in-file as `## Topic:` and `## Id:` for listing and resolution.
- **Task conversation logging**: When the agent touches a task (TASK_CREATE, TASK_APPEND, TASK_STATUS, etc.), the full user question and assistant reply are appended to the task file as a `## Conversation` block. Runner turns (synthetic "Current task file content..." prompts) are skipped.
- **Having_fun ASAP**: In having_fun channels, messages that are a mention or from a human trigger an immediate response (next tick) instead of the random delay.
- **Having_fun idle timer log**: The periodic "Having fun: idle timer" log now includes time until next response and next idle thought (e.g. `next response in 45s, next idle thought in 120s`). Logged about once a minute when there are having_fun channels.
- **Perplexity Search**: Optional web search via Perplexity API. Tauri commands `perplexity_search` and `is_perplexity_configured`; API key stored in Keychain (Settings). Use from Ollama/agents for real-time web search.

## Changed

- **Task resolution**: Resolve by full task filename (with or without `.md`), by short id (from `## Id:` in file), or by topic (from `## Topic:` in file). Legacy filenames (task-topic-id-datetime-status) still supported.
- **TASK_CREATE**: Rejects when topic looks like an existing task filename; sanitizes id (strips quotes/slashes). Deduplication checks `## Topic:` and `## Id:` in existing files.
- **TASK_APPEND / TASK_CREATE parsing**: Multi-line content is preserved (all lines until the next tool line), so research and long text are stored completely in the task file.
- **Having_fun flow**: Before replying, the app fetches the latest messages from Discord (after the bot's last response) and uses those as context for better flow. Falls back to the in-memory buffer if the API fetch fails.
- **Docs and memory**: All MD files and `~/.mac-stats/agents/memory.md` updated to document the new task naming. See `docs/013_task_agent.md`, `docs/021_task_system_fix_feb2026_DONE.md`, `docs/022_feature_review_plan.md`.
