# mac-stats: Local AI Agent for macOS

## Overview
A local AI agent for macOS with:
- **Menu bar monitoring**: CPU, GPU, RAM, disk usage
- **AI capabilities**: Ollama chat, Discord bot, task runner, scheduler
- **No cloud/telemetry**: All operations run locally
- **Themes**: Data Poster (default), customizable

[![GitHub release](https://img.shields.io/github/v/release/raro42/mac-stats?include_prereleases&style=flat-square)](https://github.com/raro42/mac-stats/releases/latest)

---

## Installation
**Recommended:**  
[Download latest release](https://github.com/raro42/mac-stats/releases/latest) → Drag to Applications

**From source:**
```bash
git clone https://github.com/raro42/mac-stats.git && cd mac-stats && ./run
# Or one-liner: curl -fsSL https://raw.githubusercontent.com/raro42/mac-stats/refs/heads/main/run -o run && chmod +x run && ./run
```

**If blocked by Gatekeeper:**  
Right-click DMG → **Open**, or run:  
`xattr -rd com.apple.quarantine /Applications/mac-stats.app`

---

## Core Features
- **Menu bar**: Click to open detailed metrics window
- **AI chat**: Ollama integration (via app or Discord)
- **Discord bot**: Message parsing, agent routing
- **Task automation**: Scheduler, command execution
- **Real-time monitoring**: CPU/GPU/RAM/disk metrics

---

## Agent System

### Tool Agents (Ollama Invokable)
| Agent         | Command              | Purpose                          |
|---------------|----------------------|----------------------------------|
| FETCH_URL     | `FETCH_URL: <URL>`   | Fetch web page content           |
| BRAVE_SEARCH  | `BRAVE_SEARCH: <q>`  | Web search via Brave API         |
| RUN_JS        | `RUN_JS: <code>`     | Execute JavaScript in CPU window |

**Implementation**:  
- `commands/browser.rs` for FETCH_URL  
- `commands/brave.rs` for BRAVE_SEARCH  
- JavaScript execution in CPU window

---

### LLM Agents (Directory-Based)
**Location**: `~/.mac-stats/agents/`  
Each agent has:
- `agent.json`: Model, role, orchestrator status
- `skill.md`: System prompt (task-specific)
- `mood.md`: Tone/context
- `soul.md`: Identity/principles
- `testing.md`: Test prompts

**Model Roles** (resolved at startup):
| Role       | Description                     |
|------------|---------------------------------|
| `code`     | Code-oriented model (coder, etc)|
| `general`  | General-purpose model           |
| `small`    | Smallest/local model            |
| `vision`   | Multimodal model (LLaVA, etc)   |
| `thinking` | Reasoning model (DeepSeek, etc) |
| `expensive`| Largest/local model             |

**Orchestrator Agents**:
- Delegate tasks to specialized agents via `AGENT: <id>`  
- Must include "Router API Commands" in `skill.md`

---

## Default Agents
Pre-installed agents (editable by user):
| ID         | Name               | Role                     |
|------------|--------------------|--------------------------|
| `000`      | Orchestrator       | Routes to specialists    |
| `001`      | General Assistant  | General Q&A              |
| `002`      | Coder              | Code generation          |
| `003`      | Generalist         | Fast replies             |
| `004`      | Discord Expert     | Discord API specialist   |
| `005`      | Task Runner        | Task file execution      |

---

## Agent Testing
```bash
mac_stats agent test <selector> [path]
```
- Resolves agent by ID/name/slug
- Uses `testing.md` for prompts
- Simulates `AGENT:` tool invocation

---

## SKILL vs LLM Agents
| Feature         | SKILL                     | LLM Agent                     |
|-----------------|---------------------------|-------------------------------|
| Model           | Shared default model      | Per-agent model               |
| Prompt          | Simple overlay            | Combined soul/mood/skill prompt |
| Use Case        | Lightweight tasks         | Complex workflows, delegation |

---

## Open tasks:
- Clarify `model_role` resolution logic
- Add documentation for `AGENT: <selector> [task]` syntax
- Implement missing `orchestrator` routing examples
- Define fallback behavior for cloud model roles
- Add CLI command for agent reset/defaults
- Document `testing.md` format requirements