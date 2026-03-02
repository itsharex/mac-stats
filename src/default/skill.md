# Default skill guidance (orchestrator)

For **implementation tasks** (code changes, refactors, fixes, new features): recommend **CURSOR_AGENT** so the orchestrator uses `CURSOR_AGENT: <prompt>` with the task description, then appends the result and sets the task status to finished. Prefer CURSOR_AGENT over RUN_CMD for coding tasks when cursor-agent is available.

This guidance is included in the default orchestrator skill (`src-tauri/defaults/agents/agent-000/skill.md`). See `docs/012_cursor_agent_tasks.md`.
