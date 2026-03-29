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

### Test run (003-tester/TESTER.md, 2026-03-28 — solo UNTESTED-20260322-0145… nombrado por operador)

- **Date:** 2026-03-28, hora local (macOS); no UTC fijada.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía** en el árbol; el archivo estaba como `CLOSED-*`. Se renombró `CLOSED-*` → `TESTING-*` (fase de prueba; no se tocó ningún otro `UNTESTED-*`).
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*` según `003-tester/TESTER.md`.

### Test run (003-tester/TESTER.md, 2026-03-28 — ejecución única, UNTESTED nombrado inexistente)

- **Date:** 2026-03-28, hora local (macOS); no UTC fijada.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía**; el archivo estaba como `CLOSED-*`. Se aplicó `CLOSED-*` → `TESTING-*` para la fase de prueba (equivalente a `UNTESTED-*` → `TESTING-*`). No se tocó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*` (éxito; no aplica `TESTED-*`).

### Test run (003-tester/TESTER.md, 2026-03-28 — solo UNTESTED-20260322-0145…, agente Cursor)

- **Date:** 2026-03-28, hora local (macOS); no UTC fijada.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía**; el archivo estaba como `CLOSED-*`. Se aplicó `CLOSED-*` → `TESTING-*` (equivalente operativo a `UNTESTED-*` → `TESTING-*`). No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*`.

### Test run (003-tester/TESTER.md, 2026-03-28 — UNTESTED nombrado, solo esta tarea)

- **Date:** 2026-03-28, hora local (macOS); no UTC fijada.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía**; el archivo estaba como `CLOSED-*`. Se aplicó `CLOSED-*` → `TESTING-*` (equivalente operativo a `UNTESTED-*` → `TESTING-*`). No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Notes:** Criterios de aceptación 1–3 cumplidos. Petición del operador: en fallo renombrar a `TESTED-*`; `003-tester/TESTER.md` prescribe `WIP-*` si bloqueo o fallo.
- **Outcome:** **pass** — `TESTING-*` → `CLOSED-*`.

### Test run (003-tester/TESTER.md, 2026-03-29 — solo UNTESTED-20260322-0145… nombrado)

- **Date:** 2026-03-29, hora local (macOS); no UTC fijada.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía** en el árbol; el archivo estaba como `CLOSED-*` al inicio de esta ejecución. Se aplicó `CLOSED-*` → `TESTING-*` (equivalente operativo a `UNTESTED-*` → `TESTING-*`). No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*` (éxito; no aplica `TESTED-*`).

### Test run (003-tester/TESTER.md, 2026-03-29 — ejecución operador; UNTESTED nombrado inexistente)

- **Date:** 2026-03-29, hora local (macOS); no UTC fijada.
- **Prefijo:** El operador nombró `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`; **no existía**. El archivo estaba como `CLOSED-*`; se aplicó `CLOSED-*` → `TESTING-*` para la fase de prueba (equivalente a `UNTESTED-*` → `TESTING-*`). No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*` (éxito; no aplica `TESTED-*`).

### Test run (003-tester/TESTER.md, 2026-03-29 — Cursor; UNTESTED nombrado inexistente)

- **Date:** 2026-03-29, hora local (macOS); no UTC fijada.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía**; el archivo estaba como `CLOSED-*` al inicio. Se aplicó `CLOSED-*` → `TESTING-*` (equivalente a `UNTESTED-*` → `TESTING-*`). No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*` (éxito; no aplica `TESTED-*`).

### Test run (003-tester/TESTER.md, 2026-03-29 — solo UNTESTED-20260322-0145… nombrado; Cursor)

- **Date:** 2026-03-29, hora local (macOS); no UTC fijada.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía** en el árbol al inicio; el archivo estaba como `CLOSED-*`. Se aplicó `CLOSED-*` → `TESTING-*` para la fase de prueba (equivalente operativo a `UNTESTED-*` → `TESTING-*`). No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*` según resultado (éxito; no aplica `TESTED-*`).

### Test run (003-tester/TESTER.md, 2026-03-29 — UNTESTED nombrado inexistente; solo esta tarea)

- **Date:** 2026-03-29, hora local (macOS); no UTC fijada.
- **Prefijo:** El operador nombró `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`; **no existía** en el árbol. El archivo estaba como `CLOSED-*`; se aplicó `CLOSED-*` → `TESTING-*` para la fase de prueba (equivalente operativo a `UNTESTED-*` → `TESTING-*`). No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*` (éxito; no aplica `TESTED-*`).

### Test run (003-tester/TESTER.md, 2026-03-29 — UNTESTED nombrado inexistente; ejecución agente)

- **Date:** 2026-03-29, hora local (macOS); no UTC fijada.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía** al inicio; el archivo estaba como `CLOSED-*`. Se aplicó `CLOSED-*` → `TESTING-*` (equivalente operativo a `UNTESTED-*` → `TESTING-*`). No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*` según `003-tester/TESTER.md`.

### Test run (003-tester/TESTER.md, 2026-03-29 — verificación con cargo ejecutada en esta sesión)

- **Date:** 2026-03-29, hora local (macOS); no UTC fijada.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía**; el archivo estaba como `CLOSED-*` y se renombró a `TESTING-*` antes de ejecutar los comandos. No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*` (éxito; no aplica `TESTED-*`).

### Test run (003-tester/TESTER.md, 2026-03-29 — solo tarea UNTESTED-20260322-0145… nombrada)

- **Date:** 2026-03-29, hora local (macOS); no UTC fijada.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía** en el árbol al inicio de esta ejecución. El archivo estaba como `CLOSED-*`; se aplicó `CLOSED-*` → `TESTING-*` (equivalente operativo a `UNTESTED-*` → `TESTING-*`). No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*`.

### Test run (003-tester/TESTER.md, 2026-03-29 — solo UNTESTED-20260322-0145… nombrado; Cursor, orden estricto)

- **Date:** 2026-03-29, hora local (macOS); no UTC fijada.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía**; el archivo estaba como `CLOSED-*`. Se aplicó `CLOSED-*` → `TESTING-*`, se ejecutaron comandos, se añadió este bloque y se renombró `TESTING-*` → `CLOSED-*`. No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Archivo final `tasks/CLOSED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`.

### Test run (003-tester/TESTER.md, 2026-03-29 — solo UNTESTED-20260322-0145… nombrado por operador)

- **Date:** 2026-03-29, hora local (macOS); no UTC fijada.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía** al inicio; el archivo estaba como `CLOSED-*`. Se aplicó `CLOSED-*` → `TESTING-*` (equivalente operativo a `UNTESTED-*` → `TESTING-*`). No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*`.

### Test run (003-tester/TESTER.md, 2026-03-29 — UNTESTED-20260322-0145… nombrado; solo esta tarea)

- **Date:** 2026-03-29, local time (macOS); not fixed to UTC.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` was **not present** in the tree; the task file was `CLOSED-*`. Applied `CLOSED-*` → `TESTING-*` for the testing phase (operational equivalent to `UNTESTED-*` → `TESTING-*`). No other `UNTESTED-*` file was used.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — acceptance criteria 1–3 satisfied. Renamed `TESTING-*` → `CLOSED-*` (pass).

### Test run (003-tester/TESTER.md, 2026-03-29 — UNTESTED-20260322-0145… nombrado; ejecución Cursor)

- **Date:** 2026-03-29, hora local (macOS); no fijada a UTC.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía**; el archivo estaba como `CLOSED-*`. Se aplicó `CLOSED-*` → `TESTING-*` (equivalente operativo a `UNTESTED-*` → `TESTING-*`). No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*` (éxito; no aplica `TESTED-*`).

### Test run (003-tester/TESTER.md, 2026-03-29 — solo UNTESTED-20260322-0145… nombrado; ejecución agente)

- **Date:** 2026-03-29, hora local (macOS); no fijada a UTC.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía** al inicio; el archivo estaba como `CLOSED-*`. Se aplicó `CLOSED-*` → `TESTING-*` (equivalente operativo a `UNTESTED-*` → `TESTING-*`). No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*`.

### Test run (003-tester/TESTER.md, 2026-03-29 — flujo TESTER; UNTESTED nombrado ausente)

- **Date:** 2026-03-29, hora local (macOS); no fijada a UTC.
- **Prefijo:** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existía**; el archivo estaba como `CLOSED-*` y se renombró a `TESTING-*` antes de ejecutar comandos (equivalente a `UNTESTED-*` → `TESTING-*`). No se usó ningún otro `UNTESTED-*`.
- **Commands run:** `cd src-tauri && cargo check`; `cd src-tauri && cargo test prompts:: --no-fail-fast`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cargo check` | **pass** |
| Prompt tests | `cargo test prompts:: --no-fail-fast` | **pass** — 5 passed; 0 failed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`) |

- **Outcome:** **pass** — criterios de aceptación 1–3 cumplidos. Renombrado `TESTING-*` → `CLOSED-*` (éxito; no aplica `TESTED-*`).
