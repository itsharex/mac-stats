## mac-stats

### Overview

mac-stats is a local AI agent for macOS that provides a range of features, including:

* Ollama chat and Discord bot
* Task runner and scheduler
* MCP (Mac Performance Counter) monitoring
* CPU, GPU, RAM, and disk at a glance

### Installation

#### DMG (Recommended)

1. Download the latest release from [GitHub](https://github.com/raro42/mac-stats/releases/latest)
2. Drag the app to the Applications folder

#### Build from Source

1. Clone the repository: `git clone https://github.com/raro42/mac-stats.git && cd mac-stats && ./run`
2. Or use the one-liner: `curl -fsSL https://raw.githubusercontent.com/raro42/mac-stats/refs/heads/main/run -o run && chmod +x run && ./run`

#### Gatekeeper Configuration

If macOS blocks the app, follow these steps:

1. Right-click the DMG and select **Open**
2. Confirm the installation

Or, after installation:

1. Run `xattr -rd com.apple.quarantine /Applications/mac-stats.app`

## At a Glance

### Menu Bar

* Displays CPU, GPU, RAM, and disk usage at a glance
* Click to open the details window

### AI Chat

* Ollama chat in the app or via Discord
* Supports FETCH_URL, BRAVE_SEARCH, PERPLEXITY_SEARCH, RUN_CMD, code execution, and MCP

### Discord Bot

* Integrates with Discord to provide a chat interface for Ollama

## Tool Agents

Whenever Ollama is asked to decide which agent to use, the app sends the complete list of active agents. Ollama invokes tools by replying with exactly one line in the form `TOOL_NAME: <argument>`.

| Agent | Invocation | Purpose | Implementation |
|-------|------------|---------|----------------|
| **FETCH_URL** | `FETCH_URL: <full URL>` | Fetch a web page’s body as text (server-side, no CORS) | `commands/browser.rs` → `fetch_page_content()` (reqwest blocking client, 15s timeout). Used by Discord pipeline and by CPU-window chat (`ollama_chat_with_execution`). |
| **BRAVE_SEARCH** | `BRAVE_SEARCH: <search query>` | Web search via Brave Search API; results (titles, URLs, snippets) are injected back for Ollama to summarize. | `commands/brave.rs` → `brave_web_search()`. Requires `BRAVE_API_KEY` (env or `.config.env`). Used by Discord and (when wired) CPU-window agent flow. |
| **RUN_JS** | `RUN_JS: <JavaScript code>` | Execute JavaScript (e.g. in CPU window). | In **CPU window**: executed in

## Implementation Status

### Completed Backend Modules

1. **Security Module** (`src/security/mod.rs`)
	* Keychain integration for secure credential storage
	* Store, retrieve, delete credentials
	* List credentials (partial - needs proper Keychain API for account extraction)
	* Uses `security-framework` crate with proper error handling
2. **Monitors Module** (`src/monitors/`)
	* Website monitoring (HTTP/HTTPS checks, response times, SSL verification)
	* Social monitoring (Mastodon mentions, Twitter placeholder)
	* Monitor status tracking and history
3. **Alerts Module** (`src/alerts/`)
	* Rule-based alert system
	* Alert rules: SiteDown, NewMentions, BatteryLow, TemperatureHigh, CpuHigh, Custom
	* Alert channels: Telegram, Slack, Mastodon, Signal (placeholder)
	* Cooldown mechanism to prevent spam
4. **Plugins Module** (`src/plugins/mod.rs`)
	* Script-based plugin system (bash/python)
	* JSON output contract
	* Scheduling and execution
	* Plugin validation
5. **Ollama Module** (`src/ollama/mod.rs`)
	* Local LLM integration
	* Chat interface
	* Model listing
	* Connection checking

### Known Issues / TODOs

1. **Security Module**
	* `list_credentials()` function needs proper Keychain API implementation to extract account names
	* Currently returns empty vector - needs implementation using Keychain Services API
2. **Plugin System**
	* Timeout handling not fully implemented (std::process::Command doesn't have timeout)
	* Should use tokio or crossbeam for proper timeout handling
	* Plugin script execution could be improved with better error messages
3. **Monitor System**
	* Monitor state persistence not implemented (in-memory only)
	* Should persist to disk (JSON/TOML config file)
	* Monitor scheduling/background checking not implemented yet
4. **Alert System**
	* Alert channel registration not exposed via Tauri commands
	* Need to add commands for registering Telegram/Slack/Mastodon channels
	* Alert evaluation needs to be called periodically (background task)
5. **Ollama Integration**
	* Stream support not implemented (currently only non-streaming chat)
	* Could add streaming for better UX
6. **UI Implementation**
	* Frontend UI not yet updated to show new dashboard
	* Need to create:
		+ 3 core gauges (Temperature, CPU Usage, CPU Frequency)
		+ Battery & Power status strip
		+ Collapsible External/Monitors section
		+ Settings UI for adding monitors, configuring alerts
		+ Ollama chat interface

## Open tasks:

### Security Module
* Implement `list_credentials()` function using Keychain Services API

### Plugin System
* Implement proper timeout handling using tokio or crossbeam

### Monitor System
* Implement monitor state persistence using JSON/TOML config file

### Alert System
* Add commands for registering Telegram/Slack/Mastodon channels

### Ollama Integration
* Implement stream support for better UX

### UI Implementation
* Update frontend UI to show new dashboard
* Implement 3 core gauges with SVG
* Add battery/power status strip
* Create collapsible sections for external monitors
* Implement settings UI for adding monitors, configuring alerts
* Implement Ollama chat interface