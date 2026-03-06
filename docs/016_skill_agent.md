## Global Context

### Overview

mac-stats is a local AI agent for macOS, providing a range of features including Ollama chat, Discord bot, task runner, scheduler, and MCP. The app is built with Rust and Tauri.

### Installation

#### DMG (Recommended)

Download the latest release and drag the app to Applications.

#### Build from Source

```bash
git clone https://github.com/raro42/mac-stats.git && cd mac-stats && ./run
```

Or one-liner:
```bash
curl -fsSL https://raw.githubusercontent.com/raro42/mac-stats/refs/heads/main/run -o run && chmod +x run && ./run
```

#### Gatekeeper Configuration

If macOS blocks the app, Gatekeeper may show "damaged" or block the unsigned app. Right-click the DMG → **Open**, then confirm. Or after install:
```bash
xattr -rd com.apple.quarantine /Applications/mac-stats.app
```

## At a Glance

*   **Menu Bar**: Displays CPU, GPU, RAM, and disk usage at a glance. Click to open the details window.
*   **AI Chat**: Ollama in the app or via Discord. Supports FETCH_URL, BRAVE_SEARCH, PERPLEXITY_SEARCH, RUN_CMD, code execution, and MCP.
*   **Discord Bot**: Integrates with Discord, allowing users to interact with Ollama and access various tools.

## Tool Agents

Whenever Ollama is asked to decide which agent to use, the app sends the complete list of active agents. Ollama invokes tools by replying with exactly one line in the form `TOOL_NAME: <argument>`.

### Available Tool Agents

| Agent | Invocation | Purpose | Implementation |
|-------|------------|---------|----------------|
| **FETCH_URL** | `FETCH_URL: <full URL>` | Fetch a web page’s body as text (server-side, no CORS). | `commands/browser.rs` → `fetch_page_content()` (reqwest blocking client, 15s timeout). Used by Discord pipeline and by CPU-window chat (`ollama_chat_with_execution`). |
| **BRAVE_SEARCH** | `BRAVE_SEARCH: <search query>` | Web search via Brave Search API; results (titles, URLs, snippets) are injected back for Ollama to summarize. | `commands/brave.rs` → `brave_web_search()`. Requires `BRAVE_API_KEY` (env or `.config.env`). Used by Discord and (when wired) CPU-window agent flow. |
| **RUN_JS** | `RUN_JS: <JavaScript code>` | Execute JavaScript (e.g. in CPU window). | In **CPU window**: executed in

## Skill Agent

The SKILL agent lets Ollama run a specialized skill (a Markdown system-prompt overlay) in a separate Ollama session with no main conversation history. The skill’s reply is injected back into the main conversation so the model can use it to answer the user.

### Overview

*   **Agent name**: SKILL
*   **Invocation**: Ollama replies with one line: `SKILL: <number or topic> [optional task]`
*   **Selector**: Either a **number** (e.g. `2`) or a **topic** slug (e.g. `summarize`, `code`)
*   **Task**: Optional. If present (text after the first space), it is the user message for the skill session. If omitted, the **current user question** is used as the user message.
*   **Execution**: The app loads the skill content from `~/.mac-stats/skills/`, runs one Ollama request (system = skill content, user = task or question), and injects the result as `Skill "<number>-<topic>" result:\n\n<result>` into the main conversation.

## Skill Files

*   **Directory**: `~/.mac-stats/skills/` (see `Config::skills_dir()` in `config/mod.rs`)
*   **Naming**: `skill-<number>-<topic>.md`, e.g. `skill-1-summarize.md`, `skill-2-code.md`
*   **Default skills**: When the skills directory is empty, the app creates two default skills: **1-summarize** (summarization) and **2-code** (code help). You can edit or remove them.
*   **Content**: Markdown (or plain text) used as the **system prompt** for the skill session. The file is read and trimmed; empty files are skipped.

## Open tasks:

*   Investigate why the app sometimes fails to load skills.
*   Improve the user interface for managing skills.
*   Add support for more advanced skill features, such as conditional logic and user-defined variables.