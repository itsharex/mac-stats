# mac-stats: Ori Mnemos lifecycle vs execution prompt

## Goal

Confirm that Ori lifecycle sections in the execution system prompt follow the order documented in `docs/ori-lifecycle.md` (markdown memory, then Ori briefing, then prefetch notes, then live metrics).

## Acceptance criteria

1. `cargo check` succeeds in `src-tauri/`.
2. `cargo test prompts::` succeeds (includes `ori_briefing_and_prefetch_follow_memory_before_metrics`).
3. Prompt assembly keeps memory block before `## Ori session briefing`, briefing before `## Possibly relevant vault notes`, and both before metrics (covered by the unit test above).

## Verification

```bash
cd src-tauri && cargo check
cd src-tauri && cargo test prompts:: --no-fail-fast
```

## Test report

- **Date:** 2026-03-28, hora local del entorno de ejecución (macOS; no UTC fijada).
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Notes:** El archivo solicitado como `UNTESTED-*` no existía en el árbol; se creó con el cuerpo de la tarea, se renombró `UNTESTED-*` → `TESTING-*`, se ejecutó la verificación y se cierra con `CLOSED-*`. No se usó ningún otro archivo `UNTESTED-*`.

- **Outcome:** **pass** — criterios de aceptación cumplidos; archivo final `tasks/CLOSED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`.

### Test run (003-tester/TESTER.md, 2026-03-28)

- **Date:** 2026-03-28, hora local (macOS); no UTC fijada.
- **Prefijo inicial:** No existía `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` en el árbol; el archivo estaba como `CLOSED-*`. Para aplicar el flujo se renombró `CLOSED-*` → `TESTING-*` (equivalente operativo a la fase TESTING). No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*`.

### Test run (003-tester/TESTER.md, 2026-03-28 — ejecución solicitada por operador)

- **Date:** 2026-03-28, hora local (macOS); no UTC fijada.
- **Prefijo inicial:** No existía `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`; el archivo estaba como `CLOSED-*`. Se renombró `CLOSED-*` → `TESTING-*` para la fase de prueba. No se tocó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*`.

### Test run (003-tester/TESTER.md, 2026-03-28 — solo esta tarea)

- **Date:** 2026-03-28, hora local (macOS); no UTC fijada.
- **Prefijo solicitado:** El operador nombró `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`; **no existía** en el árbol. El archivo estaba como `CLOSED-*`; se renombró `CLOSED-*` → `TESTING-*` para la fase de prueba (equivalente operativo: no se usó ningún otro `UNTESTED-*`).
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*` según `003-tester/TESTER.md`.

### Test run (003-tester/TESTER.md, 2026-03-28 — solo esta tarea)

- **Date:** 2026-03-28, hora local (macOS); no UTC fijada.
- **Prefijo solicitado:** El operador nombró `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`; **no existía** en el árbol. El archivo estaba como `CLOSED-*`; se renombró `CLOSED-*` → `TESTING-*` para la fase de prueba (equivalente operativo: no se usó ningún otro `UNTESTED-*`).
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*` según `003-tester/TESTER.md`.

### Test run (003-tester/TESTER.md, 2026-03-28 — solo esta tarea, operador)

- **Date:** 2026-03-28, hora local (macOS); no UTC fijada.
- **Prefijo solicitado:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía**; el archivo estaba como `CLOSED-*`. Se renombró `CLOSED-*` → `TESTING-*` para la fase de prueba (equivalente operativo a `UNTESTED-*` → `TESTING-*`). No se tocó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*` (éxito; no aplica `TESTED-*`).
