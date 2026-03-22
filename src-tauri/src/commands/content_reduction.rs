//! Content reduction (truncation/summarization) and skill/JS execution helpers.
//!
//! Extracted from `ollama.rs` to keep the orchestrator focused.

use std::io::Write;
use std::process::Command;

use crate::commands::ollama_chat::send_ollama_chat_messages;

/// Heuristic: chars to tokens (conservative).
pub(crate) const CHARS_PER_TOKEN: usize = 4;

/// Reserve tokens for model reply and wrapper text.
const RESERVE_TOKENS: u32 = 512;

/// When over limit by at most 1/this fraction, truncate only (no summarization) to avoid extra Ollama call.
const TRUNCATE_ONLY_THRESHOLD_DENOM: u32 = 4;

/// Truncate at last newline or space before max_chars so we don't cut mid-word. O(max_chars).
pub(crate) fn truncate_at_boundary(body: &str, max_chars: usize) -> String {
    let mut last_break = max_chars;
    let mut broke_early = false;
    for (i, c) in body.chars().enumerate() {
        if i >= max_chars {
            broke_early = true;
            break;
        }
        if c == '\n' || c == ' ' {
            last_break = i + 1;
        }
    }
    if !broke_early {
        return body.to_string();
    }
    body.chars().take(last_break).collect()
}

/// Reduce fetched page content to fit the model context: summarize via Ollama if needed, else truncate.
/// Uses byte-length heuristic for fast path and "slightly over" path to avoid full char count; only
/// when summarization is needed do we count chars for logging.
pub(crate) async fn reduce_fetched_content_to_fit(
    body: &str,
    context_size_tokens: u32,
    estimated_used_tokens: u32,
    model_override: Option<String>,
    options_override: Option<crate::ollama::ChatOptions>,
) -> Result<String, String> {
    use tracing::info;

    let max_tokens_for_body = context_size_tokens
        .saturating_sub(RESERVE_TOKENS)
        .saturating_sub(estimated_used_tokens);
    let max_chars = (max_tokens_for_body as usize).saturating_mul(CHARS_PER_TOKEN);

    // Fast path: cheap byte heuristic (len/4 >= char_count/4 for UTF-8). Avoids char count when body fits.
    let body_tokens_upper = body.len() / CHARS_PER_TOKEN;
    if body_tokens_upper <= max_tokens_for_body as usize {
        return Ok(body.to_string());
    }

    // Slightly over: within 25% of limit → truncate only, no summarization (saves one Ollama round-trip).
    let threshold = max_tokens_for_body + (max_tokens_for_body / TRUNCATE_ONLY_THRESHOLD_DENOM);
    if body_tokens_upper <= threshold as usize {
        let truncated = truncate_at_boundary(body, max_chars);
        return Ok(format!(
            "{} (content truncated due to context limit)",
            truncated.trim_end()
        ));
    }

    // Way over: summarization path. Compute exact token estimate only for logging.
    let body_tokens_est = body.chars().count() / CHARS_PER_TOKEN;
    info!(
        "Agent router: page content too large (est. {} tokens), max {} tokens; reducing",
        body_tokens_est, max_tokens_for_body
    );

    let body_truncated_for_request = truncate_at_boundary(body, max_chars);
    let summary_tokens = (max_tokens_for_body / 2).max(256);
    let summarization_messages = vec![
        crate::ollama::ChatMessage {
            role: "system".to_string(),
            content: format!(
                "Summarize the following web page content in under {} tokens, keeping the most relevant information for answering questions. Output only the summary, no preamble.",
                summary_tokens
            ),
            images: None,
        },
        crate::ollama::ChatMessage {
            role: "user".to_string(),
            content: body_truncated_for_request,
            images: None,
        },
    ];

    match send_ollama_chat_messages(summarization_messages, model_override, options_override).await
    {
        Ok(resp) => {
            let summary = resp.message.content.trim().to_string();
            if summary.is_empty() {
                let fallback = truncate_at_boundary(body, max_chars);
                Ok(format!(
                    "{} (content truncated due to context limit)",
                    fallback.trim_end()
                ))
            } else {
                Ok(summary)
            }
        }
        Err(e) => {
            info!("Agent router: summarization failed ({}), truncating", e);
            let fallback = truncate_at_boundary(body, max_chars);
            Ok(format!(
                "{} (content truncated due to context limit)",
                fallback.trim_end()
            ))
        }
    }
}

/// Check whether an Ollama error string indicates a context-window overflow.
pub(crate) fn is_context_overflow_error(err: &str) -> bool {
    let lower = err.to_lowercase();
    lower.contains("context overflow")
        || lower.contains("prompt too long")
        || lower.contains("context length exceeded")
        || lower.contains("maximum context length")
        || lower.contains("exceeds the model's context window")
        || (lower.contains("too long") && lower.contains("context"))
}

/// Check whether an Ollama error indicates message role/ordering conflict.
fn is_role_ordering_error(lower: &str) -> bool {
    (lower.contains("role") && (lower.contains("alternate") || lower.contains("ordering")))
        || lower.contains("incorrect role")
        || lower.contains("roles must alternate")
        || lower.contains("expected role")
        || (lower.contains("invalid") && lower.contains("role"))
}

/// Check whether an Ollama error indicates corrupted session / missing tool input.
fn is_corrupted_session_error(lower: &str) -> bool {
    (lower.contains("tool") && lower.contains("missing"))
        || lower.contains("invalid message")
        || lower.contains("malformed")
        || (lower.contains("tool_calls") && lower.contains("expected"))
}

/// Rewrite a raw Ollama/pipeline error into a short, user-friendly message.
///
/// Maps known error categories to actionable text that suggests starting a
/// new topic (matching the wording users already know from session reset).
/// Returns `None` when the error does not match any known pattern, so callers
/// can fall back to their existing formatting.
pub(crate) fn sanitize_ollama_error_for_user(raw: &str) -> Option<String> {
    let lower = raw.to_lowercase();

    let friendly = if is_context_overflow_error(raw) {
        Some(
            "The conversation got too long for the model's context window. \
             Try starting a new topic or using a model with a larger context."
                .to_string(),
        )
    } else if is_role_ordering_error(&lower) {
        Some(
            "Message ordering conflict — please try again. \
             If this keeps happening, start a new topic to reset the conversation."
                .to_string(),
        )
    } else if is_corrupted_session_error(&lower) {
        Some(
            "The conversation history looks corrupted. \
             Start a new topic to begin a fresh session."
                .to_string(),
        )
    } else {
        None
    };

    if friendly.is_some() {
        tracing::debug!("Sanitized Ollama error for user — raw: {}", raw);
    }

    friendly
}

/// Truncate oversized tool-result messages in the conversation to `max_chars_per_result`.
///
/// Only truncates assistant/user/system messages whose content exceeds `max_chars_per_result`
/// and that look like tool results (heuristic: not the very first system prompt, and not
/// messages that are the user's original question).
///
/// Returns the number of messages that were truncated.
pub(crate) fn truncate_oversized_tool_results(
    messages: &mut [crate::ollama::ChatMessage],
    max_chars_per_result: usize,
) -> usize {
    let mut truncated_count = 0usize;
    for (i, msg) in messages.iter_mut().enumerate() {
        // Skip the first message (system prompt) — it contains the agent instructions.
        if i == 0 && msg.role == "system" {
            continue;
        }
        let char_count = msg.content.chars().count();
        if char_count <= max_chars_per_result {
            continue;
        }
        let truncated_body = truncate_at_boundary(&msg.content, max_chars_per_result);
        msg.content = format!(
            "{}\n\n[truncated from {} to {} chars due to context limit]",
            truncated_body.trim_end(),
            char_count,
            max_chars_per_result
        );
        truncated_count += 1;
    }
    truncated_count
}

/// Run a single Ollama request in a new session (no conversation history). Used for SKILL agent.
/// System message = skill content, user message = task. Returns the assistant reply or error string.
pub(crate) async fn run_skill_ollama_session(
    skill_content: &str,
    user_message: &str,
    model_override: Option<String>,
    options_override: Option<crate::ollama::ChatOptions>,
) -> Result<String, String> {
    use tracing::info;
    let messages = vec![
        crate::ollama::ChatMessage {
            role: "system".to_string(),
            content: skill_content.to_string(),
            images: None,
        },
        crate::ollama::ChatMessage {
            role: "user".to_string(),
            content: user_message.to_string(),
            images: None,
        },
    ];
    info!(
        "Agent router: SKILL session request (user message {} chars)",
        user_message.chars().count()
    );
    let response = send_ollama_chat_messages(messages, model_override, options_override).await?;
    Ok(response.message.content.trim().to_string())
}

/// Run JavaScript via Node.js (if available). Used for RUN_JS in Discord/agent context.
/// Writes code to a temp file and runs `node -e "..."` to eval and print the result.
///
/// **Security:** RUN_JS is agent-triggered and runs with process privileges. Agent or prompt
/// compromise can lead to arbitrary code execution. Treat agent output as untrusted code.
pub(crate) fn run_js_via_node(code: &str) -> Result<String, String> {
    let tmp_dir = crate::config::Config::tmp_js_dir();
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let _ = std::fs::create_dir_all(&tmp_dir);
    let path = tmp_dir.join(format!("mac_stats_js_{}_{}.js", std::process::id(), stamp));
    let path_str = path
        .to_str()
        .ok_or_else(|| "Invalid temp path".to_string())?;

    let mut f = std::fs::File::create(&path).map_err(|e| format!("Create temp file: {}", e))?;
    f.write_all(code.as_bytes())
        .map_err(|e| format!("Write temp file: {}", e))?;
    f.flush().map_err(|e| format!("Flush: {}", e))?;
    drop(f);

    // Node -e script: read file, eval code, print result (no user code in -e, so no escaping).
    let eval_script = r#"const fs=require('fs');const p=process.argv[1];const c=fs.readFileSync(p,'utf8');try{const r=eval(c);console.log(r!==undefined?String(r):'undefined');}catch(e){console.error(e.message);process.exit(1);}"#;
    let out = Command::new("node")
        .arg("-e")
        .arg(eval_script)
        .arg(path_str)
        .output()
        .map_err(|e| format!("Node not available or failed: {}", e))?;

    let _ = std::fs::remove_file(&path);

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(stderr.trim().to_string());
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    Ok(stdout.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_at_boundary_returns_full_string_when_short() {
        let body = "hello";
        assert_eq!(truncate_at_boundary(body, 100), "hello");
    }

    #[test]
    fn truncate_at_boundary_exact_length_returns_full_string() {
        let body = "hello";
        assert_eq!(truncate_at_boundary(body, 5), "hello");
    }

    #[test]
    fn truncate_at_boundary_truncates_at_last_word_boundary() {
        let body = "hello world this is a test";
        let result = truncate_at_boundary(body, 11);
        // Last space within first 11 chars is at index 5 → takes "hello " (6 chars)
        assert_eq!(result, "hello ");
    }

    #[test]
    fn truncate_at_boundary_breaks_at_last_space_before_limit() {
        let body = "abcde fghij klmno";
        let result = truncate_at_boundary(body, 10);
        // Last space within first 10 chars is at index 5 → takes "abcde " (6 chars)
        assert_eq!(result, "abcde ");
    }

    #[test]
    fn truncate_at_boundary_uses_later_boundary_when_available() {
        let body = "ab cd ef gh ij kl mn";
        let result = truncate_at_boundary(body, 10);
        // Last space within first 10 chars is at index 8 (before 'g') → takes "ab cd ef " (9 chars)
        assert_eq!(result, "ab cd ef ");
    }

    #[test]
    fn truncate_at_boundary_no_break_point_uses_max() {
        let body = "abcdefghijklmno";
        let result = truncate_at_boundary(body, 5);
        assert_eq!(result, "abcde");
    }

    #[test]
    fn detects_context_overflow_errors() {
        assert!(is_context_overflow_error("Ollama error: context overflow"));
        assert!(is_context_overflow_error(
            "Ollama error: prompt too long for context"
        ));
        assert!(is_context_overflow_error("context length exceeded"));
        assert!(is_context_overflow_error(
            "maximum context length is 4096 tokens"
        ));
        assert!(is_context_overflow_error(
            "exceeds the model's context window"
        ));
        assert!(is_context_overflow_error("request too long for context"));
    }

    #[test]
    fn does_not_match_unrelated_errors() {
        assert!(!is_context_overflow_error(
            "Ollama HTTP 503: service unavailable"
        ));
        assert!(!is_context_overflow_error("connection refused"));
        assert!(!is_context_overflow_error("rate limit exceeded"));
        assert!(!is_context_overflow_error("timeout"));
    }

    fn make_msg(role: &str, content: &str) -> crate::ollama::ChatMessage {
        crate::ollama::ChatMessage {
            role: role.to_string(),
            content: content.to_string(),
            images: None,
        }
    }

    #[test]
    fn truncate_tool_results_skips_system_prompt() {
        let big = "x".repeat(10_000);
        let mut msgs = vec![make_msg("system", &big), make_msg("user", "hello")];
        let n = truncate_oversized_tool_results(&mut msgs, 500);
        assert_eq!(n, 0, "system prompt at index 0 should not be truncated");
        assert_eq!(msgs[0].content.len(), 10_000);
    }

    #[test]
    fn truncate_tool_results_truncates_large_messages() {
        let big_result = "word ".repeat(2000);
        let mut msgs = vec![
            make_msg("system", "You are an AI."),
            make_msg("user", "fetch https://example.com"),
            make_msg("assistant", "FETCH_URL: https://example.com"),
            make_msg("user", &big_result),
        ];
        let n = truncate_oversized_tool_results(&mut msgs, 500);
        assert_eq!(n, 1);
        assert!(
            msgs[3].content.chars().count() < 600,
            "expected truncated msg under 600 chars, got {}",
            msgs[3].content.chars().count()
        );
        assert!(msgs[3].content.contains("[truncated from"));
    }

    #[test]
    fn truncate_tool_results_leaves_small_messages() {
        let mut msgs = vec![
            make_msg("system", "You are an AI."),
            make_msg("user", "hello"),
            make_msg("assistant", "hi there"),
        ];
        let n = truncate_oversized_tool_results(&mut msgs, 500);
        assert_eq!(n, 0);
    }

    #[test]
    fn truncate_tool_results_handles_multiple() {
        let big1 = "a".repeat(5000);
        let big2 = "b".repeat(8000);
        let mut msgs = vec![
            make_msg("system", "prompt"),
            make_msg("user", &big1),
            make_msg("assistant", "ok"),
            make_msg("user", &big2),
        ];
        let n = truncate_oversized_tool_results(&mut msgs, 1000);
        assert_eq!(n, 2);
        assert!(msgs[1].content.contains("[truncated from 5000 to 1000"));
        assert!(msgs[3].content.contains("[truncated from 8000 to 1000"));
    }

    #[test]
    fn sanitize_context_overflow_suggests_new_topic() {
        let msg = sanitize_ollama_error_for_user("Ollama error: context overflow");
        assert!(msg.is_some());
        let msg = msg.unwrap();
        assert!(msg.contains("new topic"));
        assert!(msg.contains("context window"));
    }

    #[test]
    fn sanitize_prompt_too_long_suggests_new_topic() {
        let msg = sanitize_ollama_error_for_user("Ollama error: prompt too long for context");
        assert!(msg.is_some());
        assert!(msg.unwrap().contains("new topic"));
    }

    #[test]
    fn sanitize_role_ordering_error() {
        let msg =
            sanitize_ollama_error_for_user("Ollama error: roles must alternate user/assistant");
        assert!(msg.is_some());
        let msg = msg.unwrap();
        assert!(msg.contains("ordering"));
        assert!(msg.contains("new topic"));
    }

    #[test]
    fn sanitize_incorrect_role_error() {
        let msg = sanitize_ollama_error_for_user("incorrect role information in message");
        assert!(msg.is_some());
        assert!(msg.unwrap().contains("ordering"));
    }

    #[test]
    fn sanitize_corrupted_session_tool_missing() {
        let msg = sanitize_ollama_error_for_user("tool call input missing from history");
        assert!(msg.is_some());
        let msg = msg.unwrap();
        assert!(msg.contains("corrupted"));
        assert!(msg.contains("new topic"));
    }

    #[test]
    fn sanitize_returns_none_for_unknown_errors() {
        assert!(sanitize_ollama_error_for_user("connection refused").is_none());
        assert!(sanitize_ollama_error_for_user("timeout").is_none());
        assert!(sanitize_ollama_error_for_user("Ollama HTTP 503: service unavailable").is_none());
        assert!(sanitize_ollama_error_for_user("Failed to send chat request").is_none());
    }
}
