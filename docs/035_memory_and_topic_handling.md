# mac-stats

## Overview  
**Local AI agent for macOS**  
- Menu bar: CPU, GPU, RAM, disk metrics  
- AI chat: Ollama, Discord, task runner, scheduler, MCP  
- No cloud, no telemetry  
- Built with Rust and Tauri  

---

## Installation  
**Recommended:**  
- Download DMG from [GitHub releases](https://github.com/raro42/mac-stats/releases/latest) → drag to Applications  

**From source:**  
```bash
git clone https://github.com/raro42/mac-stats.git && cd mac-stats && ./run  
# Or one-liner: curl -fsSL https://raw.githubusercontent.com/raro42/mac-stats/refs/heads/main/run -o run && chmod +x run && ./run  
```  

**Gatekeeper issues:**  
- Right-click DMG → **Open**  
- Or run: `xattr -rd com.apple.quarantine /Applications/mac-stats.app`  

---

## Features  
- **Menu bar metrics** (click for details)  
- **AI chat** (Ollama in-app or via Discord)  
  - Tools: `FETCH_URL`, `BRAVE_SEARCH`, `RUN_CMD`, code execution, MCP  
- **Discord bot**  
- **Task scheduler**  
- **Real-time system monitoring**  

---

## Agents Overview  
### Tool agents (Ollama can invoke)  
| Agent | Invocation | Purpose |  
|-------|-----------|--------|  
| `FETCH_URL` | `FETCH_URL: <URL>` | Fetch web page content (server-side, no CORS) |  
| `BRAVE_SEARCH` | `BRAVE_SEARCH: <query>` | Web search via Brave API (requires `BRAVE_API_KEY`) |  
| `RUN_JS` | `RUN_JS: <code>` | Execute JavaScript in CPU window |  

**Other agents:**  
- **SCHEDULER** (info-only; Ollama can recommend but cannot invoke)  
- **Discord bot** (entry-point for chat)  
- **CPU window** (chat interface with execution)  

---

## Memory & Topic Handling  
### How mac-stats keeps context focused  
**Key strategies:**  
1. **Topic-aware compaction**  
   - Summarize context relevant to the current question  
   - If new topic: "Previous context not needed" → no prior context used  

2. **User-initiated reset**  
   - Phrases: "clear session", "new topic", "reset" (multi-language)  
   - Clears session for the channel  

3. **Memory injection**  
   - **Global memory:** `~/.mac-stats/agents/memory.md`  
   - **Discord per-channel:** `memory-discord-{channel_id}.md`  
   - Injected into system prompt for every run  

4. **New-topic detection (local only)**  
   - If 2+ prior messages: LLM check for "NEW_TOPIC" → no prior context  

5. **No tool-result pruning**  
   - Tool outputs are embedded in messages; compaction handles context control  

6. **Per-channel memory (Discord)**  
   - No cross-channel leakage  
   - DMs and channels have separate memory files  

---

## Quick Reference  
| Need | Action |  
|------|--------|  
| Focus on current question | Topic-aware compaction |  
| Fresh start | "Reset" phrases (multi-language) |  
| Use past lessons | Global + channel-specific memory |  
| Help on failed request | Search memory + append "From past sessions" |  
| Detect new topic | Local LLM check (no user input) |  
| Keep channels separate | Session/memory keyed by channel ID |  

---

## Open tasks:  
- [ ] Implement per-channel memory in non-Discord contexts  
- [ ] Add explicit "new topic" detection for non-local models  
- [ ] Clarify memory pruning logic in documentation  
- [ ] Expand multi-language support for reset phrases  
- [ ] Optimize compaction performance for large histories  

*Sibling repos: OpenClaw = `../openclaw`, Hermes = `../hermes-agent`*