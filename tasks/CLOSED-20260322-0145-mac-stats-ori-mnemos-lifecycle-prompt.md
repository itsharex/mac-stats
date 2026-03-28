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

- **Date:** 2026-03-27, hora local del entorno donde se ejecutaron los comandos (no UTC fijada).
- **Note:** En el árbol de trabajo **no existía** `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`; se creó el cuerpo de la tarea y se aplicó **UNTESTED → TESTING** según `003-tester/TESTER.md`, sin usar otro archivo `UNTESTED-*`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Prompt tests | `cd src-tauri && cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Outcome:** Criterios de aceptación cumplidos → **CLOSED**.

## Test report

- **Date:** 2026-03-27, hora local del entorno de ejecución (no UTC fijada).
- **Note:** El operador solicitó `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`; en el árbol **no existía** ese prefijo (la tarea estaba como `CLOSED-*`). Se aplicó el flujo de `003-tester/TESTER.md` renombrando `CLOSED-*` → `TESTING-*` para la corrida, sin elegir otro archivo `UNTESTED-*`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Prompt tests | `cd src-tauri && cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Outcome:** Criterios de aceptación cumplidos → **CLOSED**.

## Test report

- **Date:** 2026-03-27, hora local del entorno de ejecución (no UTC fijada).
- **Note:** El operador nombró `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`; ese archivo **no existe** en el árbol. Se siguió `003-tester/TESTER.md` sobre la misma tarea renombrando `CLOSED-*` → `TESTING-*` para esta corrida, sin abrir otro `UNTESTED-*`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Prompt tests | `cd src-tauri && cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Outcome:** Criterios de aceptación cumplidos → **CLOSED**.

## Test report

- **Date:** 2026-03-27, hora local del entorno de ejecución (macOS; no UTC fijada).
- **Note:** El operador pidió `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`; en el árbol **no existía** con prefijo `UNTESTED-`. Se aplicó `003-tester/TESTER.md` sobre la misma tarea: `CLOSED-*` → `TESTING-*` durante la corrida, verificación, informe y `TESTING-*` → `CLOSED-*` al cerrar. No se usó otro archivo `UNTESTED-*`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Prompt tests | `cd src-tauri && cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Outcome:** Criterios de aceptación cumplidos → **CLOSED**.

## Test report

- **Date:** 2026-03-27, hora local del entorno de ejecución (macOS; no UTC fijada).
- **Note:** El operador nombró explícitamente `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`; ese archivo **no existía** (la tarea ya estaba `CLOSED-*`). Se ejecutó el flujo de `003-tester/TESTER.md` solo sobre esta tarea: `CLOSED-*` → `TESTING-*`, comandos de verificación, este informe, luego `TESTING-*` → `CLOSED-*`. No se eligió otro `UNTESTED-*`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Prompt tests | `cd src-tauri && cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Outcome:** Criterios de aceptación cumplidos → **CLOSED**.

## Test report

- **Date:** 2026-03-27, hora local del entorno de ejecución (macOS; no UTC fijada).
- **Note:** Nueva corrida del flujo `003-tester/TESTER.md`: el archivo nombrado como `UNTESTED-*` no estaba en el árbol; se usó la misma tarea (`CLOSED-*` → `TESTING-*` → verificación → informe → `CLOSED-*`). No se abrió ningún otro `UNTESTED-*`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Prompt tests | `cd src-tauri && cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Outcome:** Criterios de aceptación cumplidos → **CLOSED**.

## Test report

- **Date:** 2026-03-27, hora local del entorno de ejecución (macOS; no UTC fijada).
- **Note:** El operador pidió probar solo `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`; ese path **no existía** (la tarea ya era `CLOSED-*`). Se siguió `003-tester/TESTER.md` sobre el mismo basename: `CLOSED-*` → `TESTING-*`, comandos de verificación, este informe, luego `TESTING-*` → `CLOSED-*`. No se eligió otro `UNTESTED-*`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Prompt tests | `cd src-tauri && cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Outcome:** Criterios de aceptación cumplidos → **CLOSED**.

## Test report

- **Date:** 2026-03-28, hora local del entorno de ejecución (macOS; no UTC fijada).
- **Note:** El operador nombró `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`; ese prefijo **no existía** en el árbol (la tarea estaba como `CLOSED-*`). Se aplicó `003-tester/TESTER.md` solo sobre esta tarea: `CLOSED-*` → `TESTING-*`, verificación, este informe, luego `TESTING-*` → `CLOSED-*`. No se usó ningún otro archivo `UNTESTED-*`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Prompt tests | `cd src-tauri && cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Outcome:** Criterios de aceptación cumplidos → **CLOSED**.

## Test report

- **Date:** 2026-03-28, hora local del entorno de ejecución (macOS; no UTC fijada).
- **Note:** El operador indicó `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` siguiendo `003-tester/TESTER.md`; ese path **no existía** (la tarea estaba como `CLOSED-*`). Se ejecutó el flujo solo sobre este basename: `CLOSED-*` → `TESTING-*`, comandos de verificación, este informe, luego `TESTING-*` → `CLOSED-*`. No se eligió otro `UNTESTED-*`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Prompt tests | `cd src-tauri && cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Outcome:** Criterios de aceptación cumplidos → **CLOSED**.

## Test report

- **Date:** 2026-03-28, hora local del entorno de ejecución (macOS; no UTC fijada).
- **Note:** El operador solicitó `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`; ese archivo **no existía** (la tarea estaba como `CLOSED-*`). Se aplicó `003-tester/TESTER.md` sobre el mismo basename: `CLOSED-*` → `TESTING-*`, verificación, este informe, luego `TESTING-*` → `CLOSED-*`. No se eligió otro `UNTESTED-*`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Prompt tests | `cd src-tauri && cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Outcome:** Criterios de aceptación cumplidos → **CLOSED**.

## Test report

- **Date:** 2026-03-28, hora local del entorno de ejecución (macOS; no UTC fijada).
- **Note:** Corrida según `003-tester/TESTER.md` para la tarea nombrada como `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`; ese prefijo **no estaba** en el árbol (solo existía `CLOSED-*` / se usó `CLOSED-*` → `TESTING-*` para esta corrida). No se abrió ningún otro archivo `UNTESTED-*`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Prompt tests | `cd src-tauri && cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Outcome:** Criterios de aceptación cumplidos → **CLOSED**.

## Test report

- **Date:** 2026-03-28, hora local del entorno de ejecución (macOS; no UTC fijada).
- **Note:** El path `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` **no existe** en el árbol; la tarea solo estaba como `CLOSED-*`. Flujo `003-tester/TESTER.md` sobre el mismo basename: `CLOSED-*` → `TESTING-*` (esta corrida), verificación, este informe, `TESTING-*` → `CLOSED-*`. No se usó ningún otro archivo `UNTESTED-*`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Prompt tests | `cd src-tauri && cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Outcome:** Criterios de aceptación cumplidos → **CLOSED**.

## Test report

- **Date:** 2026-03-28, hora local del entorno de ejecución (macOS; no UTC fijada).
- **Note:** Corrida `003-tester/TESTER.md` para la tarea indicada como `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md`; ese prefijo **no existe** en el repo (solo `CLOSED-*` / en esta corrida `CLOSED-*` → `TESTING-*` → verificación → informe → `CLOSED-*`). No se abrió ningún otro `UNTESTED-*`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Prompt tests | `cd src-tauri && cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Outcome:** Criterios de aceptación cumplidos → **CLOSED**.

## Test report

- **Date:** 2026-03-28, hora local del entorno de ejecución (macOS; no UTC fijada).
- **Note:** Corrida según `003-tester/TESTER.md` para `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` (path `UNTESTED-*` inexistente; trabajo sobre `CLOSED-*` → `TESTING-*` → verificación → informe → `CLOSED-*`). No se usó ningún otro archivo `UNTESTED-*`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Prompt tests | `cd src-tauri && cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Outcome:** Criterios de aceptación cumplidos → **CLOSED**.

## Test report

- **Date:** 2026-03-28, hora local del entorno de ejecución (macOS; no UTC fijada).
- **Note:** Corrida `003-tester/TESTER.md` para la tarea nombrada `tasks/UNTESTED-20260322-0145-mac-stats-ori-mnemos-lifecycle-prompt.md` (no existe en el árbol). Misma tarea: `CLOSED-*` → `TESTING-*`, `cargo check` + `cargo test prompts:: --no-fail-fast`, este informe, `TESTING-*` → `CLOSED-*`. Tras un `replace_all` erróneo se revirtió el cuerpo del archivo a `git HEAD` y se volvió a aplicar el flujo con un solo apéndice. No se usó ningún otro archivo `UNTESTED-*`.

| Step | Command | Result |
|------|---------|--------|
| Check | `cd src-tauri && cargo check` | **pass** |
| Prompt tests | `cd src-tauri && cargo test prompts:: --no-fail-fast` | **pass** — 5 passed (incl. `ori_briefing_and_prefetch_follow_memory_before_metrics`), 0 failed |

- **Outcome:** Criterios de aceptación cumplidos → **CLOSED**.
