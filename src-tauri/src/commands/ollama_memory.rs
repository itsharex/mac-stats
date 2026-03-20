//! Memory and soul loading helpers.
//!
//! Extracted from `ollama.rs`: soul content, global/channel/main-session
//! memory blocks, keyword-based memory search.

use crate::config::Config;

pub(crate) fn load_soul_content() -> String {
    Config::load_soul_content()
}

/// Load global memory (~/.mac-stats/agents/memory.md) for inclusion in system prompt.
pub(crate) fn load_global_memory_block() -> String {
    let path = Config::memory_file_path();
    let content = match std::fs::read_to_string(&path) {
        Ok(s) => s.trim().to_string(),
        Err(_) => return String::new(),
    };
    if content.is_empty() {
        return String::new();
    }
    format!(
        "\n\n## Memory (lessons learned — follow these)\n\n{}\n\n",
        content
    )
}

/// Load per-channel Discord memory (~/.mac-stats/agents/memory-discord-{id}.md). Returns empty if missing.
pub(crate) fn load_channel_memory_block(channel_id: u64) -> String {
    let path = Config::memory_file_path_for_discord_channel(channel_id);
    let content = match std::fs::read_to_string(&path) {
        Ok(s) => s.trim().to_string(),
        Err(_) => return String::new(),
    };
    if content.is_empty() {
        return String::new();
    }
    format!(
        "\n\n## Memory (this channel — follow these)\n\n{}\n\n",
        content
    )
}

/// Load main-session (in-app) memory (~/.mac-stats/agents/memory-main.md). Returns empty if missing.
/// Used when the request is from the CPU window (no Discord channel) so the main session has per-context memory.
pub(crate) fn load_main_session_memory_block() -> String {
    let path = Config::memory_file_path_for_main_session();
    let content = match std::fs::read_to_string(&path) {
        Ok(s) => s.trim().to_string(),
        Err(_) => return String::new(),
    };
    if content.is_empty() {
        return String::new();
    }
    format!(
        "\n\n## Memory (main session — follow these)\n\n{}\n\n",
        content
    )
}

/// Load memory for the current request.
/// Global memory (personal/long-term) is only loaded in main session (in-app, or Discord DM).
/// In Discord guild channels and having_fun, only per-channel memory is loaded to avoid leaking personal context.
/// When there is no Discord channel (in-app), main-session memory (memory-main.md) is also loaded.
pub(crate) fn load_memory_block_for_request(
    discord_channel_id: Option<u64>,
    load_global_memory: bool,
) -> String {
    let global = if load_global_memory {
        load_global_memory_block()
    } else {
        String::new()
    };
    let channel = discord_channel_id
        .map(load_channel_memory_block)
        .unwrap_or_else(|| {
            if load_global_memory {
                load_main_session_memory_block()
            } else {
                String::new()
            }
        });
    if channel.is_empty() {
        global
    } else {
        format!("{}{}", global, channel)
    }
}

/// Extract words (alphanumeric, lowercase) for simple keyword matching.
pub(crate) fn words_for_search(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| s.len() >= 2)
        .map(String::from)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect()
}

/// Search memory (global + optional Discord channel or main-session) for lines relevant to the request.
/// Returns at most 5 matching lines, or None if no matches. When discord_channel_id is Some,
/// channel memory is merged with global; when None (in-app), main-session memory is included.
pub(crate) fn search_memory_for_request(
    question: &str,
    reason: Option<&str>,
    discord_channel_id: Option<u64>,
) -> Option<String> {
    let global = std::fs::read_to_string(Config::memory_file_path())
        .ok()
        .unwrap_or_default();
    let channel = discord_channel_id
        .and_then(|id| {
            std::fs::read_to_string(Config::memory_file_path_for_discord_channel(id)).ok()
        })
        .unwrap_or_else(|| {
            std::fs::read_to_string(Config::memory_file_path_for_main_session())
                .ok()
                .unwrap_or_default()
        });
    let content = format!("{}\n{}", global.trim(), channel.trim())
        .trim()
        .to_string();
    if content.is_empty() {
        return None;
    }
    let mut query_words: Vec<String> = words_for_search(question);
    if let Some(r) = reason {
        query_words.extend(words_for_search(r));
    }
    query_words.sort();
    query_words.dedup();
    if query_words.is_empty() {
        return None;
    }
    const MIN_MEMORY_MATCH_WORDS: usize = 2;
    let mut scored: Vec<(usize, String)> = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let line_lower = line.to_lowercase();
            let score = query_words
                .iter()
                .filter(|w| line_lower.contains(w.as_str()))
                .count();
            (score, line.to_string())
        })
        .filter(|(score, _)| *score >= MIN_MEMORY_MATCH_WORDS)
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    let top: Vec<String> = scored.into_iter().take(5).map(|(_, line)| line).collect();
    if top.is_empty() {
        None
    } else {
        Some(top.join("\n"))
    }
}
