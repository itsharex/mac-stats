# Browser use: form control BROWSER_INPUT routing

## Summary

Form-aware **BROWSER_INPUT** (CDP) routes `<select>` (value or label), HTML5 compound inputs (`date`, `time`, etc.) via native value setter + events, datepicker-like text fields, `contenteditable`, and default text fields via focus + keystrokes. Spec: `docs/029_browser_automation.md` § “Form-aware BROWSER_INPUT”. Manual fixture: `docs/fixtures/browser-input-routing.html`.

## Acceptance criteria

1. `input_by_index` / in-page JS distinguishes routes and returns `ok_select`, `ok_native`, `ok_datepicker`, `ok_contenteditable`, or the default typing path; logs include `route_hint` / `path=datepicker_heuristic` where applicable (`src-tauri/src/browser_agent/mod.rs`).
2. Interactable rows expose `input_type`, `contenteditable`, and `datepicker_like` for LLM snapshots.
3. Fixture `docs/fixtures/browser-input-routing.html` exists (select, `input type="date"`, contenteditable).
4. `cargo check` and `cargo test --lib` succeed in `src-tauri/`.

## Verification (automated)

```bash
cd src-tauri && cargo check
cd src-tauri && cargo test --lib
rg -n "ok_select|ok_native|ok_datepicker|ok_contenteditable" src/browser_agent/mod.rs
```

Optional manual: with CDP Chrome, `BROWSER_NAVIGATE` to `file://…/docs/fixtures/browser-input-routing.html` and exercise **BROWSER_INPUT** on listed indices; click “Read values” to confirm.

## Test report

- **Date:** 2026-03-27 (hora local del entorno donde se ejecutaron los comandos; no UTC fijada).
- **Note:** En el árbol de trabajo no existía `UNTESTED-20260321-2015-browser-use-form-control-input-routing.md`; se creó el cuerpo de la tarea y se aplicó el paso **UNTESTED → TESTING** con `mv` para seguir `003-tester/TESTER.md`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Lib tests | `cd src-tauri && cargo test --lib` | **pass** — 854 passed, 0 failed |
| Routing symbols | `rg -n "ok_select\|ok_native\|ok_datepicker\|ok_contenteditable" src/browser_agent/mod.rs` | **pass** — matches at 8003, 8010, 8033, 8160–8161 |

- **Manual CDP / fixture:** no ejecutado en esta corrida (opcional en la tarea).
- **Outcome:** Criterios automatizados y comprobación de rutas en código cumplidos → **CLOSED**.
