//! Per-channel debounce of rapid Discord messages before a single Ollama router call.

use super::{run_discord_ollama_router, set_discord_user_name, ChannelMode};
use serenity::client::Context;
use serenity::model::channel::Message;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use tracing::{info, warn};

struct DebouncedPart {
    content: String,
    author_id: u64,
    display_name: String,
    message: Message,
}

struct ChannelPending {
    generation: u64,
    parts: Vec<DebouncedPart>,
}

static PENDING: OnceLock<Mutex<HashMap<u64, ChannelPending>>> = OnceLock::new();

fn pending_map() -> &'static Mutex<HashMap<u64, ChannelPending>> {
    PENDING.get_or_init(|| Mutex::new(HashMap::new()))
}

fn merge_debounced_contents(parts: &[DebouncedPart]) -> String {
    if parts.is_empty() {
        return String::new();
    }
    if parts.len() == 1 {
        return parts[0].content.clone();
    }
    let same_author = parts
        .windows(2)
        .all(|w| w[0].author_id == w[1].author_id);
    if same_author {
        parts
            .iter()
            .map(|p| p.content.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        parts
            .iter()
            .map(|p| format!("{}: {}", p.display_name, p.content))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub(super) fn discord_message_bypasses_debounce(
    msg: &Message,
    content: &str,
    debounce_ms: u64,
) -> bool {
    if debounce_ms == 0 {
        return true;
    }
    if !msg.attachments.is_empty() {
        return true;
    }
    let t = content.trim_start();
    if t.starts_with('/') {
        return true;
    }
    let lower = content.trim().to_lowercase();
    if lower.starts_with("new session:") || lower.starts_with("new session ") {
        return true;
    }
    if crate::session_memory::user_wants_session_reset(content) {
        return true;
    }
    false
}

/// Drop queued debounce batches (e.g. app exit). Does not run Ollama.
pub(super) fn discard_pending_batches_on_shutdown() {
    let Ok(mut map) = pending_map().lock() else {
        return;
    };
    let n = map.len();
    map.clear();
    if n > 0 {
        warn!(
            "Discord debounce: discarded {} pending channel batch(es) on shutdown (not flushed)",
            n
        );
    }
}

pub(super) async fn enqueue_or_run_router(
    ctx: Context,
    new_message: Message,
    content: String,
    attachment_images_base64: Vec<String>,
    mode: ChannelMode,
    debounce_ms: u64,
) {
    if discord_message_bypasses_debounce(&new_message, &content, debounce_ms)
        || !attachment_images_base64.is_empty()
    {
        run_discord_ollama_router(
            ctx,
            new_message,
            content,
            attachment_images_base64,
            mode,
        )
        .await;
        return;
    }

    let channel_id = new_message.channel_id.get();
    let display_name = new_message
        .author
        .global_name
        .as_deref()
        .unwrap_or(&new_message.author.name)
        .to_string();
    let part = DebouncedPart {
        content,
        author_id: new_message.author.id.get(),
        display_name,
        message: new_message,
    };

    let gen = {
        let mut map = pending_map()
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let entry = map.entry(channel_id).or_insert(ChannelPending {
            generation: 0,
            parts: Vec::new(),
        });
        entry.generation = entry.generation.saturating_add(1);
        let gen = entry.generation;
        entry.parts.push(part);
        let n_parts = entry.parts.len();
        info!(
            "Discord debounce: queued message for channel {} (generation {}, {} part(s), {} ms until flush)",
            channel_id, gen, n_parts, debounce_ms
        );
        gen
    };

    let ctx2 = ctx.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(debounce_ms)).await;
        let pending = {
            let mut map = pending_map()
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            let Some(p) = map.get(&channel_id) else {
                return;
            };
            if p.generation != gen {
                return;
            }
            map.remove(&channel_id)
        };
        let Some(pending) = pending else {
            return;
        };
        let last_msg = pending
            .parts
            .last()
            .expect("debounce parts non-empty")
            .message
            .clone();
        let merged = merge_debounced_contents(&pending.parts);
        info!(
            "Discord debounce: flushing channel {} ({} part(s), {} chars merged)",
            channel_id,
            pending.parts.len(),
            merged.chars().count()
        );
        for p in &pending.parts {
            set_discord_user_name(p.author_id, p.display_name.clone());
            crate::user_info::maybe_update_display_name_from_discord(p.author_id, &p.display_name);
        }
        run_discord_ollama_router(ctx2, last_msg, merged, Vec::new(), mode).await;
    });
}
