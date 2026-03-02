# Tool-first fix: implementation plan (baby steps)

## Status: core fix done, small follow-ups optional

The fix for “orchestrator recommends AGENT instead of tool” is **implemented** for the minimal scope (proposal [031](031_orchestrator_tool_first_proposal.md)).

---

## ✅ Already implemented

| Step | What | Where |
|------|------|--------|
| 1 | **Screenshot + URL pre-route** | `src-tauri/src/commands/ollama.rs`: `extract_url_from_question()`, `extract_screenshot_recommendation()`, and pre-routing runs screenshot first, then RUN_CMD, then REDMINE. Requests like “Use Chrome, goto https://example.com and create a screenshot” skip the planner and go straight to BROWSER_SCREENSHOT. |
| 2 | **Tool-first rule in default planning prompt** | `src-tauri/defaults/prompts/planning_prompt.md`: paragraph “**Tool-first rule:** If the user request can be fulfilled by exactly one base tool … recommend that tool directly … Prefer the tool over AGENT …” |
| 3 | **Proposal + OpenClaw references** | `docs/031_orchestrator_tool_first_proposal.md`: problem, OpenClaw comparison, codebase references, and mac-stats proposal. |

**User copy of planning prompt:** New installs get the default (with tool-first rule) via `write_default_if_missing`. Existing users who already have `~/.mac-stats/prompts/planning_prompt.md` keep their file as-is; they don’t auto-get the new paragraph.

---

## Remaining baby steps (in order)

### Step 4 — Changelog (recommended)

- [x] In `CHANGELOG.md` under `[Unreleased]` add:
  - **Tool-first routing:** Pre-route “screenshot + URL” requests to BROWSER_SCREENSHOT (skip planner). Default planning prompt now includes a tool-first rule: when one base tool fits the request, recommend that tool instead of AGENT. See `docs/031_orchestrator_tool_first_proposal.md`.

### Step 5 — Document custom planning prompt (optional)

- [ ] In the proposal doc or in a short “merge defaults” note: if you use a custom `~/.mac-stats/prompts/planning_prompt.md`, add the tool-first paragraph from `defaults/prompts/planning_prompt.md` so the planner prefers tools over AGENT when the request clearly matches one tool.

### Step 6 — Append tool-first rule when loading (optional)

- [ ] In `Config::load_planning_prompt()`: if the loaded content does not contain `"Tool-first"` (or a chosen marker), append the default tool-first paragraph so custom prompts still get the rule without manual edit. Decide whether to append always or only when file is user-provided (e.g. path exists and content differs from default).

### Step 7 — Reduce planning context (optional, proposal §3)

- [ ] For the planning step only, either send no conversation history or only the last user message (or a short summary of recent turns) so prior “no tool used” replies don’t bias the planner toward AGENT. Requires changing what `answer_with_ollama_and_fetch` passes into the planning messages.

### Step 8 — Pre-route FETCH_URL + URL (optional)

- [ ] Add `extract_fetch_url_recommendation(question)` (or similar): if the question clearly asks to “fetch/get/open this page” and contains a URL, return `FETCH_URL: <url>` and skip planning. Wire it into the same pre-route chain (e.g. after screenshot, before RUN_CMD).

---

## Summary

- **Must-do for “fix done”:** Step 4 (CHANGELOG) — done.
- **Nice-to-have:** Steps 5–6 (document or auto-append tool-first for custom prompts), 7 (planning context), 8 (FETCH_URL pre-route).

No further code is required for the minimal fix; the rest is documentation and optional improvements.
