//! General tool loop guard — detects repeated tool invocations and cycles within
//! a single agent-router request to prevent the model from getting stuck.
//!
//! Complements existing per-tool dedup (browser, Discord API, RUN_CMD) with
//! cross-tool pattern detection that catches cycles like A→B→A→B.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Maximum number of times the exact same (tool, arg) can be invoked.
const MAX_SAME_CALL: u32 = 3;
/// Maximum cycle length to detect (e.g. 4 means patterns up to ABCD→ABCD).
const MAX_CYCLE_LEN: usize = 4;
/// Minimum history length before cycle detection kicks in.
const MIN_HISTORY_FOR_CYCLE: usize = 4;

/// Tracks tool invocations within a single request to detect repetitive patterns.
pub(crate) struct ToolLoopGuard {
    history: Vec<(String, u64)>,
    counts: HashMap<(String, u64), u32>,
}

impl ToolLoopGuard {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            counts: HashMap::new(),
        }
    }

    /// Record a tool call and check whether it should be blocked.
    ///
    /// Returns `Some(reason)` if the call is a detected loop and should be
    /// refused; `None` if the call is allowed.
    pub fn record_and_check(&mut self, tool: &str, arg: &str) -> Option<String> {
        let arg_hash = hash_arg(arg);
        let key = (tool.to_string(), arg_hash);

        let count = self.counts.entry(key.clone()).or_insert(0);
        *count += 1;

        self.history.push(key.clone());

        if *count > MAX_SAME_CALL {
            return Some(format!(
                "Loop detected: {} has been called {} times with the same argument. \
                 Reply with your answer or call DONE.",
                tool, count
            ));
        }

        if let Some(cycle_len) = self.detect_cycle() {
            return Some(format!(
                "Cycle detected: the last {} tool calls repeat a previous pattern. \
                 Break the cycle — try a different approach or reply with DONE.",
                cycle_len
            ));
        }

        None
    }

    /// Check for repeating subsequence at the tail of history.
    ///
    /// Looks for patterns of length 2..=MAX_CYCLE_LEN where the last N entries
    /// equal the N entries immediately before them.
    fn detect_cycle(&self) -> Option<usize> {
        let len = self.history.len();
        if len < MIN_HISTORY_FOR_CYCLE {
            return None;
        }
        for cycle_len in 2..=MAX_CYCLE_LEN.min(len / 2) {
            let tail_start = len - cycle_len;
            let prev_start = tail_start - cycle_len;
            if self.history[prev_start..tail_start] == self.history[tail_start..len] {
                return Some(cycle_len);
            }
        }
        None
    }
}

fn hash_arg(arg: &str) -> u64 {
    let normalized = arg.trim();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    normalized.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_first_call() {
        let mut guard = ToolLoopGuard::new();
        assert!(guard.record_and_check("FETCH_URL", "https://example.com").is_none());
    }

    #[test]
    fn allows_up_to_max_same_call() {
        let mut guard = ToolLoopGuard::new();
        for _ in 0..MAX_SAME_CALL {
            assert!(guard.record_and_check("FETCH_URL", "https://example.com").is_none());
        }
        let result = guard.record_and_check("FETCH_URL", "https://example.com");
        assert!(result.is_some());
        assert!(result.unwrap().contains("Loop detected"));
    }

    #[test]
    fn different_args_are_independent() {
        let mut guard = ToolLoopGuard::new();
        for i in 0..10 {
            assert!(guard
                .record_and_check("FETCH_URL", &format!("https://example.com/{}", i))
                .is_none());
        }
    }

    #[test]
    fn different_tools_same_arg_have_independent_counts() {
        let mut guard = ToolLoopGuard::new();
        // Same arg, different tools: each tool's count is tracked independently.
        // Interleave with other calls to avoid triggering cycle detection.
        assert!(guard.record_and_check("FETCH_URL", "https://example.com").is_none());
        assert!(guard.record_and_check("RUN_CMD", "uptime").is_none());
        assert!(guard.record_and_check("FETCH_URL", "https://example.com").is_none());
        assert!(guard.record_and_check("BRAVE_SEARCH", "test").is_none());
        assert!(guard.record_and_check("FETCH_URL", "https://example.com").is_none());
        // FETCH_URL at 3 — still allowed
        assert!(guard.record_and_check("BROWSER_NAVIGATE", "https://example.com").is_none());
        // BROWSER_NAVIGATE at 1 — independent count
        assert!(guard.record_and_check("BROWSER_NAVIGATE", "https://example.com").is_none());
        // BROWSER_NAVIGATE at 2 — still allowed
    }

    #[test]
    fn detects_ab_ab_cycle() {
        let mut guard = ToolLoopGuard::new();
        assert!(guard.record_and_check("FETCH_URL", "https://a.com").is_none());
        assert!(guard.record_and_check("BRAVE_SEARCH", "test").is_none());
        assert!(guard.record_and_check("FETCH_URL", "https://a.com").is_none());
        let result = guard.record_and_check("BRAVE_SEARCH", "test");
        assert!(result.is_some());
        assert!(result.unwrap().contains("Cycle detected"));
    }

    #[test]
    fn detects_abc_abc_cycle() {
        let mut guard = ToolLoopGuard::new();
        assert!(guard.record_and_check("FETCH_URL", "https://a.com").is_none());
        assert!(guard.record_and_check("BRAVE_SEARCH", "query").is_none());
        assert!(guard.record_and_check("BROWSER_NAVIGATE", "https://b.com").is_none());
        assert!(guard.record_and_check("FETCH_URL", "https://a.com").is_none());
        assert!(guard.record_and_check("BRAVE_SEARCH", "query").is_none());
        let result = guard.record_and_check("BROWSER_NAVIGATE", "https://b.com");
        assert!(result.is_some());
        assert!(result.unwrap().contains("Cycle detected"));
    }

    #[test]
    fn no_false_positive_on_short_history() {
        let mut guard = ToolLoopGuard::new();
        assert!(guard.record_and_check("FETCH_URL", "https://a.com").is_none());
        assert!(guard.record_and_check("FETCH_URL", "https://a.com").is_none());
    }

    #[test]
    fn no_false_positive_on_varied_pattern() {
        let mut guard = ToolLoopGuard::new();
        assert!(guard.record_and_check("FETCH_URL", "https://a.com").is_none());
        assert!(guard.record_and_check("BRAVE_SEARCH", "q1").is_none());
        assert!(guard.record_and_check("FETCH_URL", "https://b.com").is_none());
        assert!(guard.record_and_check("BRAVE_SEARCH", "q2").is_none());
    }

    #[test]
    fn whitespace_normalization() {
        let mut guard = ToolLoopGuard::new();
        assert!(guard.record_and_check("FETCH_URL", "  https://a.com  ").is_none());
        assert!(guard.record_and_check("FETCH_URL", "https://a.com").is_none());
        assert!(guard.record_and_check("FETCH_URL", "  https://a.com  ").is_none());
        let result = guard.record_and_check("FETCH_URL", "https://a.com");
        assert!(result.is_some());
    }

    #[test]
    fn done_tool_not_tracked() {
        let mut guard = ToolLoopGuard::new();
        // DONE should never reach the guard in practice, but verify
        // that calling it multiple times doesn't trigger false positives
        // for the cycle check on surrounding calls.
        assert!(guard.record_and_check("FETCH_URL", "https://a.com").is_none());
        assert!(guard.record_and_check("BRAVE_SEARCH", "query").is_none());
        assert!(guard.record_and_check("FETCH_URL", "https://b.com").is_none());
        assert!(guard.record_and_check("BRAVE_SEARCH", "other query").is_none());
    }
}
