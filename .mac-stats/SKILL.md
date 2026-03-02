# SKILL (orchestrator / agent prompt overlays)

For **implementation tasks** (code changes, refactors, fixes, new features): recommend **CURSOR_AGENT** so the orchestrator uses `CURSOR_AGENT: <prompt>` with the task description, then appends the result and sets the task status to finished. Prefer CURSOR_AGENT over RUN_CMD for coding tasks when cursor-agent is available.

See also: orchestrator skill (`~/.mac-stats/agents/agent-000/skill.md`), `docs/012_cursor_agent_tasks.md`.
