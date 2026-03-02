# Proposal: Tool-first routing (avoid wrong AGENT delegation)

## General problem

When the user asks for something that **exactly matches a base tool** (e.g. “go to this URL and take a screenshot”), the **orchestrator** (planning step) can recommend **AGENT: some-agent** instead of the tool (e.g. **BROWSER_SCREENSHOT: &lt;url&gt;**). The sub-agent then often replies with text and never invokes the tool. Result: the user’s request is not fulfilled.

**Root cause:** The planner sees (1) the current question and (2) **conversation history**. If a previous turn in the same thread was a similar request that got a **text-only** reply (no tool), the planner may infer “handle this via an agent” instead of “use the tool this time.” Long-term **global memory** can also push toward “delegate” when it contains many “Chrome offline” / “unverified” / “no API” lessons.

So the failure is **wrong RECOMMEND** at planning time: tool should win over AGENT when the task clearly matches one base tool.

---

## How OpenClaw solves this

OpenClaw does **not** have a separate “planning” step that asks the model to “RECOMMEND: your plan.”

- **Single-phase model:** The active agent receives (1) **structured tool schema** (function definitions sent to the model API) and (2) **system prompt text** (TOOLS.md / human-readable list). The model **directly** chooses tool calls from that schema. There is no intermediate “orchestrator says use AGENT vs BROWSER_SCREENSHOT.”
- **Routing is binding-based:** Which agent handles a request is determined by **bindings** (e.g. channel + account → agent), not by the model “recommending” an agent. So “wrong delegation” in our sense does not exist: the routed agent simply has a set of tools and calls them.
- **Tool presentation:** Tools are first-class: allow/deny, profiles (full, messaging, coding, minimal), and **loop-detection** to block repetitive no-progress tool-call patterns. The agent sees both schema and prose; it doesn’t first “plan” then “execute.”
- **Session pruning:** Context is managed by pruning old tool results (and compaction), not by a separate planner that might be biased by history.

So OpenClaw avoids the class of bug we hit by **not having a planner that can choose “AGENT” instead of “TOOL.”** The tradeoff: mac-stats’ two-phase flow allows “which agent?” to be decided by the model; OpenClaw decides that via config bindings only.

### OpenClaw codebase references (from `openclaw` repo)

- **Routing (bindings, no LLM):** `src/routing/resolve-route.ts` — `ResolveAgentRouteInput` (channel, accountId, peer, guildId, teamId) → `ResolvedAgentRoute` with `agentId` and `matchedBy` (`binding.peer`, `binding.account`, `binding.channel`, `default`). Bindings are evaluated in a fixed order (peer → parent peer → guild+roles → guild → team → account → channel → default). No model is asked to "recommend" an agent.
- **Channel routing doc:** `docs/channels/channel-routing.md` — "Routing picks **one agent** for each inbound message" via exact peer, parent peer, guild, team, account, channel, then default agent.
- **Multi-agent doc:** `docs/concepts/multi-agent.md` — Defines agent as workspace + agentDir + session store; bindings map (channel, accountId, peer) → agentId; "Multiple agents = multiple people, multiple personalities."
- **Agent loop (single run, no planning step):** `docs/concepts/agent-loop.md` — Entry: `agent` / `agent.wait` RPC → `agentCommand` → `runEmbeddedPiAgent` → pi-agent-core; "intake → context assembly → model inference → tool execution → streaming replies → persistence." No separate "plan then execute" phase.
- **System prompt (tools in prompt):** `docs/concepts/system-prompt.md` — System prompt includes "**Tooling**: current tool list + short descriptions"; workspace files (AGENTS.md, SOUL.md, TOOLS.md, …) are injected. TOOLS.md is "user-maintained tool notes" and does not control which tools exist (that's config/skills). `src/agents/pi-embedded-runner/system-prompt.ts` — `buildEmbeddedSystemPrompt` takes `tools: AgentTool[]` and passes `toolNames` and `toolSummaries` (from `buildToolSummaryMap`) into the system prompt builder.
- **Agent runtime doc:** `docs/concepts/agent.md` — "Core tools (read/exec/edit/write and related) are always available, subject to tool policy"; skills from bundled / managed / workspace; bootstrap files (AGENTS.md, SOUL.md, TOOLS.md, …) injected on first turn.
- **Loop detection:** `docs/tools/loop-detection.md` — Optional guardrails: `repeatedFailure`, `knownPollLoop`, `repeatingNoProgress` to block repetitive no-progress tool-call patterns (disabled by default).
- **TOOLS.md template:** `docs/reference/templates/TOOLS.md` — "Skills define _how_ tools work. This file is for _your_ specifics" (camera names, SSH hosts, TTS voices, etc.); i.e. prose guidance, not tool registration.

---

## Proposal (mac-stats): general fix

Keep the two-phase flow but make **base tools win** when the request unambiguously matches one.

### 1. Tool-first rule in the planning prompt

Add an explicit instruction so the orchestrator prefers the matching **base tool** over **AGENT** when the user request clearly fits a single tool:

- **Screenshot + URL** → RECOMMEND: **BROWSER_SCREENSHOT: &lt;url&gt;** (not AGENT).
- **Fetch page text** → RECOMMEND: **FETCH_URL: &lt;url&gt;** (not AGENT).
- **Web search** → RECOMMEND: **BRAVE_SEARCH** or **PERPLEXITY_SEARCH** (not AGENT) when it’s a search query.

Wording for the planning prompt:

- *“If the user request can be fulfilled by exactly one base tool (e.g. BROWSER_SCREENSHOT for ‘screenshot this URL’ or ‘go to URL and take a screenshot’, FETCH_URL for ‘fetch/get this page’), recommend that tool directly. Prefer the tool over AGENT unless the task clearly requires multi-step or specialist capability.”*

This is a **general** rule: it applies to any “clear single-tool” case, not only screenshots.

### 2. Pre-route for unambiguous “screenshot + URL”

Like the existing **RUN_CMD** and **REDMINE_API** pre-routes: when the question clearly asks for a screenshot of a URL and we can extract the URL, **skip planning** and set recommendation to **BROWSER_SCREENSHOT: &lt;url&gt;**:

- Detected intent: e.g. “screenshot”, “take a screenshot”, “create a screenshot”, “capture the page” plus “browser”/“chrome”/“goto”/“visit” and a URL.
- URL extraction: `https?://...` or `www....` from the question (strip trailing punctuation).

This guarantees that “Use the Chrome browser, goto https://www.amvara.de and create a screenshot for me” never goes to the planner and always runs BROWSER_SCREENSHOT.

### 3. (Optional) Reduce planning context

To avoid conversation history (e.g. a previous “screenshot” turn that got a text-only reply) biasing the planner:

- **Option A:** For the planning step only, send **no** conversation history, or only the last user message. Then the planner always decides from the current question + tools/agents list.
- **Option B:** Send a **short summary** of recent turns instead of full assistant text (e.g. “User asked for screenshot; assistant replied with text only”) so the planner isn’t led to “try an agent again.”

Optional and can be done later; 1 and 2 already fix the reported case and generalize.

### 4. (Optional) OpenClaw-style single-phase

Longer-term, one could move to a **single-phase** model: no “RECOMMEND” planning step; the main agent gets tools (and optional sub-agents as tools). Routing (which “agent”/workspace) would be by channel/binding only. That would remove this class of error entirely but is a larger refactor.

---

## Summary

| Approach | OpenClaw | mac-stats proposal |
|----------|----------|--------------------|
| Planning step | None; model calls tools directly | Keep; add tool-first rule + pre-route |
| “Wrong RECOMMEND” | N/A (no planner) | Mitigate with rule + deterministic pre-route for screenshot+URL |
| Routing | Config bindings (channel/account → agent) | Model recommends plan; we bias plan toward tools when obvious |

**Implementation (minimal):**

1. **Planning prompt:** Add the tool-first sentence to `defaults/prompts/planning_prompt.md` (and user copy if present).
2. **Pre-route:** In `answer_with_ollama_and_fetch`, before the planning step: if the question indicates “screenshot” + a URL, extract URL and set `recommendation = format!("BROWSER_SCREENSHOT: {}", url)` and skip LLM planning (same pattern as RUN_CMD / REDMINE_API).

This is a general solution: the same pattern (pre-route when unambiguous, plus tool-first rule) can be extended to other tools (e.g. FETCH_URL + URL) if desired.
