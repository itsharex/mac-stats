## Global Context
### README.md snippets
#### # mac-stats
#### [![GitHub release](https://img.shields.io/github/v/release/raro42/mac-stats?include_prereleases&style=flat-square)](https://github.com/raro42/mac-stats/releases/latest)

#### A local AI agent for macOS: Ollama chat, Discord bot, task runner, scheduler, and MCP—all on your Mac. No cloud, no telemetry. Lives in your menu bar—CPU, GPU, RAM, disk at a glance. Real-time, minimal, there when you look. Built with Rust and Tauri.

#### <img src="screens/data-poster.png" alt="mac-stats Data Poster theme" width="500">

#### 📋 [Changelog](CHANGELOG.md) · 📸 [Screenshots & themes](screens/README.md)

---

## Install
### DMG (recommended): [Download latest release](https://github.com/raro42/mac-stats/releases/latest) → drag to Applications.

### Build from source:
```bash
git clone https://github.com/raro42/mac-stats.git && cd mac-stats && ./run
```
Or one-liner: `curl -fsSL https://raw.githubusercontent.com/raro42/mac-stats/refs/heads/main/run -o run && chmod +x run && ./run`

### If macOS blocks the app: Gatekeeper may show "damaged" or block the unsigned app—the file is fine. Right-click the DMG → **Open**, then confirm. Or after install: `xattr -rd com.apple.quarantine /Applications/mac-stats.app`

---

## At a Glance
- **Menu bar** — CPU, GPU, RAM, disk at a glance; click to open the details window.
- **AI chat** — Ollama in the app or via Discord; FETCH_URL, BRAVE_SEARCH, PERPLEXITY_SEARCH, RUN_CMD, code execution, MCP.
- **Discord**

---

## 1. Tool Agents (what Ollama can invoke)
Whenever Ollama is asked to decide which agent to use (planning step in Discord and scheduler flow), the app sends the **complete list of active agents**: the invocable tools below plus the **SCHEDULER** (informational; Ollama can recommend it for recurring or delayed tasks but cannot invoke it with a tool line). Ollama invokes tools by replying with exactly one line in the form `TOOL_NAME: <argument>`.

| Agent | Invocation | Purpose | Implementation |
|-------|------------|---------|----------------|
| **FETCH_URL** | `FETCH_URL: <full URL>` | Fetch a web page’s body as text (server-side, no CORS). | `commands/browser.rs` → `fetch_page_content()` (reqwest blocking client, 15s timeout). Used by Discord pipeline and by CPU-window chat (`ollama_chat_with_execution`). |
| **BRAVE_SEARCH** | `BRAVE_SEARCH: <search query>` | Web search via Brave Search API; results (titles, URLs, snippets) are injected back for Ollama to summarize. | `commands/brave.rs` → `brave_web_search()`. Requires `BRAVE_API_KEY` (env or `.config.env`). Used by Discord and (when wired) CPU-window agent flow. |
| **RUN_JS** | `RUN_JS: <JavaScript code>` | Execute JavaScript (e.g. in CPU window). | In **CPU window**: executed in

---

## 2. Expectation Check at End of Tool-Chain (Design)
### Goal
At **start**: extract 1–3 concrete success criteria from the user’s request. At **end**: before returning, verify we met them (one Ollama call). If not, append a disclaimer or retry. Reduces early stops (e.g. model returns text instead of taking a screenshot) and unverified deliverables.

### How Others Do It (Short)
| System | Loop End | Upfront Criteria? | End Verification? | Heuristic Guard? | Escalation? |
|--------|----------|-------------------|--------------------|------------------|-------------|
| **browser-use** | Model calls `done` | No | code_use only: `validate_task_completion` → can force continue | No | No |
| **OpenClaw** | No tool calls = stop | No | No (open-prose post-run only for .prose) | No | No |
| **Hermes** | No tool calls = done | No | No (completed = structural) | No | No |
| **mac-stats** | No tool calls = final answer | **Yes: 1–3 criteria at start** | **Yes: always-on** (if NO → disclaimer) | **Yes** (screenshot asked, no attachment) | **Yes** (stronger run + higher tool budget) |

### Design Choices (mac-stats)
- **Criteria at start.** One short Ollama call: “From this request, list 1–3 concrete success criteria (e.g. ‘screenshot of X attached’).” Feed them into end verification. Not optional.
- **Always-on verification.** One Ollama call at end: “Did we fully satisfy the request?” (+ criteria if present). If NO → append disclaimer (A1); optional retry (A2) later.
- **Heuristic guard.** If question/plan mentions screenshot but `attachment_paths` is empty → append note.
- **Escalation.** User says “think harder” / “get it done” etc. → prepend “user is not satisfied, you MUST complete the task”, bump `max_tool_iterations` by 10.

---

## 3. User Escalation
### Detection
Phrases like “think harder”, “get it done”, “try again”, “no”, “nope” after a short or disclaimed reply.  
### Action
Stronger system framing (“you MUST use tools to fulfill the request”), optional retry once if verification says NO, increase tool budget (+10 iterations).  
### Implementation
Patterns from **~/.mac-stats/escalation_patterns.md** (user-editable, one phrase per line). `Config::load_escalation_patterns()`; `is_escalation_message()` in Discord handler; `escalation: bool` on `answer_with_ollama_and_fetch`; when true, prepend framing and add 10 to `max_tool_iterations`.

### How to Steer the Behaviour (Without Being Rude to the Code)
When the bot gives you a shrug or a half-baked answer, you don’t have to stick to “think harder”. Edit **~/.mac-stats/escalation_patterns.md**: one phrase per line, anything that means “I’m not satisfied, do better.” The next time your message *contains* one of those phrases (case doesn’t matter), mac-stats flips into *completion mode*: it tells the model the user is not happy and bumps the tool budget so it can actually finish the job.

### Examples You Can Add
`I don't like your answer` · `You are stupid` · `That was useless` · `Try again properly` · `Nope, do it for real` · `I said do it` · or your favourite polite variant. No restart required — we re-read the file on every message. So go ahead: make your displeasure *actionable*.

### Auto-Add
When we detect escalation (the message matched an existing pattern), we append the user’s phrase to the file if it isn’t already there. So the list grows with the way *you* complain — next time the same wording will trigger escalation without you having to edit the file.

---

## 4. Implementation Plan
| Item | Status | Location |
|------|--------|----------|
| Criteria at start (extract 1–3, feed into verification) | **Done** | `extract_success_criteria()` in `ollama.rs`; passed to `verify_completion()` |
| Always-on end verification (one Ollama call, disclaimer on NO) | **Done** | `verify_completion()` in `ollama.rs`; called before `Ok(OllamaReply)` |
| Heuristic: screenshot requested but no attachment | **Done** | `ollama.rs` before `verify_completion`: append note if (screenshot in question/plan && `attachment_paths.is_empty()`) |
| Escalation detection and framing | **Done** | `discord/mod.rs`: `is_escalation_message()`, `default_verbose_for_dm`; `ollama.rs`: `escalation` param, prepend text, `max_tool_iterations += 10` |
| Retry once on verification NO (A2) | **Done** | When verification says NO we re-run once with "complete the remaining steps"; retry run uses `retry_on_verification_no: false`. Discord, scheduler, task runner pass `true`. |
| Vision verification (screenshot tasks) | **Done** | When we have image attachment(s) and a local vision model is available, we send the first screenshot as base64 and ask "Does this image satisfy the user's request?" (2) Vision-in-the-loop for browser tasks remains optional/future.

---

## 5. Browser / SPA Behaviour
- **Verification:** For find X and screenshot on SPAs, FETCH_URL only returns the initial HTML shell; the requested text is often in JS-rendered content. We pass the last BROWSER_EXTRACT result (when present) into completion verification so the verifier can answer using the real visible text. Reduces false NO when the screenshot was correct but verification had only static HTML.
- **Navigation:** Hash or in-app navigations (e.g. #/about, #/about-night) often do not fire the CDP navigated event. We treat wait_until_navigated failure as non-fatal: warn, short delay, then get state/screenshot anyway so BROWSER_NAVIGATE succeeds on hash-routed sites.

---

## 6. Deferred vs Best-of-Breed
We promised **“retries or warns”** in the stand-out line. To satisfy that and the user:

- **A2 (retry once on verification NO)** is now **implemented**: when the verifier says we didn’t complete, we run one more pass with a “complete the remaining steps” prompt and return that result (or append the disclaimer if the retry still doesn’t satisfy). So we truly **retry or warn**, not only warn.
- **“Done” tool** is implemented: the model can reply with **DONE: success** or **DONE: no**; we honour it by exiting the tool loop (no further tool runs), stripping the DONE line from the final reply, then running completion verification as usual. Planning prompt instructs the executor to end with DONE when the task is complete or cannot be completed.

---

## 7. Vision Model (Optional, Future)
If a **local vision model** is available (e.g. Ollama with a vision-capable model), it could make sense to use it in two places—**only when we have an image to show**.

1. **Verification (screenshot tasks)**
   Right now the verifier is text-only: “Original request: … What we did: … Attachments: yes/no. Did we fully satisfy the request?” For screenshot requests we only check “attachment present,” not “attachment content matches the request.” A vision call could take the **screenshot image** plus the user’s request (e.g. “find Ralf Röber and create a screenshot”) and answer: “Does this image show a page that contains that name / meets the request?” That would make verification **content-aware** for screenshots and reduce false YES when we attached the wrong page.

2. **Optional: vision-in-the-loop for browser tasks**
   After each `BROWSER_SCREENSHOT`, we could send the image to a vision model: “User asked for X. Does this page show X? If not, what should we do next (e.g. click ‘Team’, go to /contact)?” The answer could drive another NAVIGATE/CLICK/SCREENSHOT step. That would help “navigate all pages to find X” without relying on the text-only model to infer page content from BROWSER_EXTRACT/BROWSER_SEARCH_PAGE alone. Bigger design change (agent loop with image input and possibly a separate vision-only model for this step).

### When it’s Worth it:
Screenshot-heavy flows (e.g. “screenshot of page containing Y”) benefit most; pure text/FETCH_URL tasks don’t need vision. **Fallback:** If no vision model is configured or the call fails, keep current behaviour: text-only verification and no vision-in-the-loop. **Cost:** Vision inference is heavier; use only when we actually have an attachment (or explicitly opt in for browser-in-the-loop). **(1) Verification with vision is implemented:** when the reply has image attachments and a local vision model is available, we send the first screenshot as base64 and ask "Does this image satisfy the user's request?" (2) Vision-in-the-loop for browser tasks remains optional/future.

---

## 8. Related Work (Papers)
- **Plan Verification for LLM-Based Embodied Task Completion Agents** — [arXiv:2509.02761](https://arxiv.org/abs/2509.02761). Judge + Planner iterative verification.
- **Auto-Eval Judge** — [arXiv:2508.05508](https://arxiv.org/abs/2508.05508). Decompose tasks, validate steps; Judge aligns with human task success.
- **LLM evaluation of constraint-satisfaction in agent responses** — [arXiv:2409.14371](https://arxiv.org/abs/2409.14371). LLM as verifier for constraints.
- **Beyond Task Completion** (Akshathala et al.) — [arXiv:2512.12791](https://arxiv.org/abs/2512.12791). Four pillars: LLM, Memory, Tools, Environment; beyond binary completion.
- **Task Completion Agents are Not Ideal Collaborators** (Shen et al.) — [OpenReview](https://openreview.net/forum?id=JMblCtvaDH). Collaborative agents; effort scaling.

---

## 9. References (Code Pointers)
- **browser-use:** `code_use/namespace.py` → `validate_task_completion()`; `tools/service.py` → `DoneAction`; `agent/judge.py` (post-hoc).
- **OpenClaw:** Main agent ends on `finish_reason: "stop"`; open-prose inspector for .prose runs only.
- **Hermes:** `run_agent.py` (completed = final_response + under max_iterations); `agent_loop.py` (“No tool calls — model is done”).

---

## Open tasks:
- Questionable logic:
  - Is the heuristic guard necessary?
  - Is the vision model necessary for all screenshot tasks?
- Missing features:
  - Implement vision-in-the-loop for browser tasks
  - Add more escalation patterns
- Pending items:
  - Review and refine the expectation check at the end of the tool-chain
  - Review and refine the user escalation system