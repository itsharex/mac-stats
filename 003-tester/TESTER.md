# Tester agent — task file workflow

Use this folder as the tester role anchor. When testing a single task under `tasks/`:

1. **Pick one file** — Only the task the operator named (do not switch to another `UNTESTED-*` in the same run).
2. **Rename** `UNTESTED-…` → `TESTING-…` (same basename after the prefix).
3. **Run verification** from the task body: commands, greps, or manual steps listed there. Prefer `cargo check` / `cargo test` in `src-tauri/` when the task touches Rust.
4. **Append** a `## Test report` section to the same task file: date (UTC or local, state which), commands run, pass/fail, notes.
5. **Rename outcome**
   - `CLOSED-…` if all acceptance criteria pass.
   - `WIP-…` if blocked, failed, or needs follow-up work (say why in the report).

Do not delete history inside the file; append reports.
