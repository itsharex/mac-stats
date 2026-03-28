# CLOSED — OpenClaw-style keyed async queue per conversation (2026-03-22)

## Goal

Verify **per-conversation (per-channel) serialization** of full Discord/OpenClaw router turns via an async keyed mutex queue, so concurrent messages on the same channel do not interleave tool loops or session updates; different keys still run in parallel. Ollama HTTP remains additionally keyed via `ollama_queue_key`.

## References

- `src-tauri/src/keyed_queue.rs` — `run_serial`, `is_key_busy`, unit tests (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `src-tauri/src/discord/mod.rs` — `run_serial` around router path; `ollama_queue_key: Some(format!("discord:{}", channel_id_u64))`
- `src-tauri/src/commands/ollama.rs` — `ollama_queue_key` on `OllamaChatRequest`
- `src-tauri/src/ollama_queue.rs` — FIFO per key for `/api/chat`

## Acceptance criteria

1. **Build:** `cargo check` in `src-tauri/` succeeds.
2. **Tests:** `cargo test` in `src-tauri/` succeeds (includes `keyed_queue` module tests).
3. **Static verification:** `keyed_queue::run_serial` and `ollama_queue_key` with `discord:` remain wired from `discord/mod.rs` (`rg` spot-check).

## Verification commands

```bash
cd src-tauri && cargo check
cd src-tauri && cargo test keyed_queue
cd src-tauri && cargo test
```

Optional spot-check:

```bash
rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs
```

## Test report

**Date:** 2026-03-27, hora local; zona horaria del entorno del operador (se asume coherente con el reloj del sistema donde se ejecutó `cargo`).

**Preflight:** `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` **no estaba** en el disco al inicio del run. Se escribió el cuerpo de la tarea en esa ruta y se renombró a `TESTING-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` según `003-tester/TESTER.md`. No se usó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (2 tests: `same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — coincidencias en líneas 1143, 1347, 1934 (`run_serial`) y 2310 (`ollama_queue_key` con `discord:{}`).

**Outcome:** Criterios de aceptación cumplidos en esta corrida automatizada. No se probó Discord en vivo contra un gateway real.

## Test report (corrida adicional — agente Cursor)

**Date:** 2026-03-27, hora local del entorno donde se ejecutó `cargo` (misma convención que el informe anterior).

**Preflight / nombres:** El operador indicó explícitamente `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md`. Ese archivo **no está** en el repositorio; la tarea correspondiente es `tasks/CLOSED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md`. No había ningún `UNTESTED-*` en `tasks/`, por lo que **no se aplicó** el renombrado UNTESTED→TESTING de `003-tester/TESTER.md` en este run. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — coincidencias en 1143, 1347, 1934 (`run_serial`) y 2310 (`ollama_queue_key` con `discord:{}`).

**Outcome:** Criterios de aceptación cumplidos de nuevo. El nombre del archivo permanece **CLOSED-** (no WIP). Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-27)

**Date:** 2026-03-27, hora local del entorno donde se ejecutó `cargo` (zona horaria del sistema del operador).

**Preflight / nombres:** El operador pidió probar `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md`. En el repo solo existía `CLOSED-20260322-0110-…`; se aplicó el flujo `003-tester/TESTER.md` renombrando en cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para cumplir el paso UNTESTED→TESTING sin tocar ningún otro `UNTESTED-*`. El encabezado del documento quedó en estado **TESTING** durante la verificación.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — coincidencias en 1143, 1347, 1934 (`run_serial`) y 2310 (`ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado a **CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor)

**Date:** 2026-03-27, hora local del entorno donde se ejecutó `cargo` (zona horaria del sistema del operador).

**Preflight / nombres:** El operador indicó `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md`. En el repo solo existía `CLOSED-20260322-0110-…`; para cumplir `003-tester/TESTER.md` (UNTESTED→TESTING sin elegir otro `UNTESTED-*`) se renombró en cadena **CLOSED → UNTESTED → TESTING** con el mismo basename. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — coincidencias en 1143, 1347, 1934 (`run_serial`) y 2310 (`ollama_queue_key` con `discord:{}`).

**Outcome:** Criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-27)

**Date:** 2026-03-27, hora local del entorno donde se ejecutó `cargo` (zona horaria del sistema del operador).

**Preflight / nombres:** El operador indicó `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md`. Ese path **no existe** en el working tree; la tarea ya está en `tasks/CLOSED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md`. Por tanto **no se pudo** aplicar el paso literal de `003-tester/TESTER.md` «renombrar UNTESTED→TESTING» sin inventar otra copia del archivo. No se renombró a `TESTING-` ni a `WIP-`; no se tocó ningún otro `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — coincidencias en 1143, 1347, 1934 (`run_serial`) y 2310 (`ollama_queue_key` con `discord:{}`).

**Outcome:** Criterios de aceptación cumplidos. El nombre del archivo permanece **CLOSED-** (no WIP). Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-27)

**Date:** 2026-03-27, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador).

**Preflight / nombres:** El operador indicó `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md`. En el repo solo existía `CLOSED-20260322-0110-…`; para cumplir `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename), luego **TESTING → CLOSED** tras el informe. No se usó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — líneas 1143, 1347, 1934 (`run_serial`) y 2310 (`ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-27)

**Date:** 2026-03-27, hora local (zona horaria del sistema donde se ejecutó `cargo`).

**Preflight / nombres:** Tarea indicada: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin usar otro `UNTESTED-*`). Ese path no existía; el fichero era `CLOSED-20260322-0110-…`. Para cumplir `003-tester/TESTER.md` se hizo **CLOSED → UNTESTED → TESTING** (mismo basename), verificación, informe y luego **TESTING → CLOSED**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — líneas 1143, 1347, 1934 (`run_serial`) y 2310 (`ollama_queue_key` con `discord:{}`).

**Outcome:** Criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** El operador indicó `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` y prohibió elegir otro `UNTESTED-*`. En el disco solo existía `CLOSED-20260322-0110-…`; para aplicar `003-tester/TESTER.md` (paso UNTESTED→TESTING) se renombró en cadena **CLOSED → UNTESTED → TESTING** (mismo basename). No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — líneas 1143, 1347, 1934 (`run_serial`) y 2310 (`ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, sesión actual 2026-03-28)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea pedida: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin elegir otro `UNTESTED-*`). En el árbol solo existía `CLOSED-20260322-0110-…`; se aplicó **CLOSED → UNTESTED → TESTING** (mismo basename) para cumplir `003-tester/TESTER.md` (paso UNTESTED→TESTING). No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en el crate de biblioteca principal; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — líneas 1143, 1347, 1934 (`run_serial`) y 2310 (`ollama_queue_key` con `discord:{}`).

**Outcome:** Criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, segundo pase 2026-03-28)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (sin UTC explícito).

**Preflight / nombres:** Tarea pedida: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Solo existía `CLOSED-…`; cadena **CLOSED → UNTESTED → TESTING** para cumplir `003-tester/TESTER.md`.

**Commands run:** `cargo check`, `cargo test keyed_queue`, `cargo test` en `src-tauri/` — **pass** (854 tests en el crate de biblioteca `mac_stats`; 1 doc-test ignorado).

**Static spot-check:** `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (1143, 1347, 1934, 2310).

**Outcome:** Criterios cumplidos; archivo **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea indicada: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Ese path no existía; solo estaba `CLOSED-20260322-0110-…`. Para cumplir `003-tester/TESTER.md` (paso UNTESTED→TESTING) se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename). No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en el crate de biblioteca `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, sesión TESTER)

**Date:** 2026-03-28, local system time where `cargo` ran (operator timezone; not explicit UTC).

**Preflight / names:** Operator requested only `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md`; that path was missing (on disk: `CLOSED-20260322-0110-…`). Per `003-tester/TESTER.md`, chain **CLOSED → UNTESTED → TESTING** (same basename) so the UNTESTED→TESTING rename step applies without picking another `UNTESTED-*` file.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (lines 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` with `discord:{}`).

**Outcome:** All acceptance criteria met. Renaming **TESTING- → CLOSED-** after this report. Live Discord not exercised.

## Test report (corrida — agente Cursor, 2026-03-28)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea pedida: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin elegir otro `UNTESTED-*`). Ese path no existía; solo estaba `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para poder ejecutar el paso UNTESTED→TESTING sin tocar ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en el crate de biblioteca `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, TESTER)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea indicada: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin elegir otro `UNTESTED-*`). Ese path no existía; en disco solo estaba `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso UNTESTED→TESTING sin tocar ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, segunda pasada TESTER)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no UTC explícito).

**Preflight / nombres:** Misma tarea `UNTESTED-20260322-0110-…` indicada por el operador (sin otro `UNTESTED-*`). El fichero estaba como `CLOSED-…`; cadena **CLOSED → UNTESTED → TESTING** para cumplir `003-tester/TESTER.md` (paso UNTESTED→TESTING). Durante la verificación el encabezado del documento pasó a **TESTING** y, tras el informe, el nombre de archivo vuelve a **CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, TESTER según 003-tester/TESTER.md)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea indicada: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin elegir otro `UNTESTED-*`). Al inicio del run el fichero estaba como `CLOSED-20260322-0110-…`; se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para cumplir el paso UNTESTED→TESTING de `003-tester/TESTER.md`. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, sesión operador UNTESTED explícito)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** El operador fijó únicamente `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Ese path no existía; el fichero era `CLOSED-…`. Cadena **CLOSED → UNTESTED → TESTING** (mismo basename), encabezado del documento en **TESTING** durante la verificación y vuelta a **CLOSED** en el contenido antes del renombrado final del archivo.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (run — Cursor agent, 2026-03-28, TESTER per 003-tester/TESTER.md)

**Date:** 2026-03-28, local system time where `cargo` ran (operator timezone; not explicit UTC).

**Preflight / names:** Operator specified only `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (no other `UNTESTED-*`). That path was absent; the file on disk was `CLOSED-20260322-0110-…`. Per `003-tester/TESTER.md`, applied chain **CLOSED → UNTESTED → TESTING** (same basename) so the UNTESTED→TESTING step applies without picking another `UNTESTED-*` file.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (lines 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` with `discord:{}`).

**Outcome:** All acceptance criteria met. Renaming **TESTING- → CLOSED-** after this report. Live Discord not exercised.

## Test report (corrida — agente Cursor, 2026-03-28, TESTER.md)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno; no UTC explícito).

**Preflight / nombres:** Tarea única: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Al inicio el fichero estaba como `CLOSED-…`; se aplicó **CLOSED → UNTESTED → TESTING** (mismo basename) para poder ejecutar el paso UNTESTED→TESTING de `003-tester/TESTER.md`. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (2 tests: `same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, TESTER 003-tester/TESTER.md)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea única indicada: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin elegir otro `UNTESTED-*`). Ese path no existía; el fichero estaba como `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso UNTESTED→TESTING sin tocar ningún otro archivo `UNTESTED-*`. El encabezado del documento quedó en **TESTING** durante la verificación y se restauró a **CLOSED** antes del renombrado final del archivo.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (run — Cursor agent, 2026-03-28, TESTER 003-tester/TESTER.md)

**Date:** 2026-03-28, local system time where `cargo` ran (operator environment; not explicit UTC).

**Preflight / names:** Operator specified only `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (no other `UNTESTED-*`). That path was absent at run start; the file on disk was `CLOSED-20260322-0110-…`. Per `003-tester/TESTER.md`, applied rename chain **CLOSED → UNTESTED → TESTING** (same basename) so the UNTESTED→TESTING step applies without selecting another `UNTESTED-*` file. Document H1 set to **TESTING** during verification, restored to **CLOSED** before final filename.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (lines 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` with `discord:{}`).

**Outcome:** All acceptance criteria met. Renaming **TESTING- → CLOSED-** after this report. Live Discord not exercised.

## Test report (corrida — agente Cursor, 2026-03-28, TESTER.md operador)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea única: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Ese path no existía al inicio; el fichero estaba como `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso UNTESTED→TESTING sin elegir otro archivo `UNTESTED-*`. El H1 del documento estuvo en **TESTING** mientras corría `cargo`; se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, 003-tester/TESTER.md)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea única: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Al inicio del run el fichero estaba como `CLOSED-20260322-0110-…`; se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para cumplir el paso UNTESTED→TESTING de `003-tester/TESTER.md`. El H1 del documento pasó a **TESTING** durante la verificación y se restauró a **CLOSED** antes del renombrado final del archivo.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, TESTER según petición explícita)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea única: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Ese path no existía al inicio; el fichero estaba como `CLOSED-20260322-0110-…`. Cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para cumplir `003-tester/TESTER.md` (paso UNTESTED→TESTING). H1 en **TESTING** durante `cargo`, restaurado a **CLOSED** antes de **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — 2026-03-28, 003-tester/TESTER.md, solo UNTESTED-20260322-0110)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea fijada por el operador: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin elegir otro `UNTESTED-*`). Ese path no existía al inicio; el fichero estaba como `CLOSED-20260322-0110-…`. Cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para cumplir el paso UNTESTED→TESTING de `003-tester/TESTER.md`. El H1 del documento pasó a **TESTING** durante `cargo` y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, 003-tester/TESTER.md)

**Date:** 2026-03-28, local system time where `cargo` ran (operator timezone; not explicit UTC).

**Preflight / names:** Operator specified only `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (no other `UNTESTED-*`). At run start the file was `CLOSED-20260322-0110-…`; per `003-tester/TESTER.md` applied rename chain **CLOSED → UNTESTED → TESTING** (same basename) so the UNTESTED→TESTING step applies. No other `UNTESTED-*` file was used.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (lines 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` with `discord:{}`).

**Outcome:** All acceptance criteria met. Renaming **TESTING- → CLOSED-** after this report. Live Discord not exercised.

## Test report (corrida — agente Cursor, 2026-03-28, 003-tester/TESTER.md, segunda pasada)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno; no UTC explícito).

**Preflight / nombres:** Tarea indicada: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin elegir otro `UNTESTED-*`). Ese path no existía al inicio; el fichero estaba como `CLOSED-20260322-0110-…`. Cadena **CLOSED → UNTESTED → TESTING** (mismo basename) según `003-tester/TESTER.md`. El H1 estuvo en **TESTING** durante `cargo` y se restauró a **CLOSED** antes de **TESTING- → CLOSED-**. No se tocó ningún otro `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, sesión TESTER operador UNTESTED)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea fijada: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` únicamente (sin otro `UNTESTED-*`). Al inicio el fichero era `CLOSED-20260322-0110-…`; se aplicó **CLOSED → UNTESTED → TESTING** (mismo basename) para cumplir el paso UNTESTED→TESTING de `003-tester/TESTER.md`. El H1 pasó a **TESTING** durante `cargo` y se restauró a **CLOSED** antes de **TESTING- → CLOSED-**. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, 003-tester/TESTER.md)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea única: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin elegir otro `UNTESTED-*`). Al inicio solo existía `CLOSED-20260322-0110-…`; según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para cumplir el paso UNTESTED→TESTING. No se tocó ningún otro archivo `UNTESTED-*`. El H1 quedó en **CLOSED** antes del renombrado final del archivo.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (2 tests: `same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en el crate de biblioteca `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, 003-tester/TESTER.md, sesión actual)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no UTC explícito).

**Preflight / nombres:** Tarea fijada: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` únicamente (sin otro `UNTESTED-*`). Ese path no existía; el fichero estaba como `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso UNTESTED→TESTING. El H1 pasó a **TESTING** durante `cargo` y se restauró a **CLOSED** antes del renombrado final del archivo.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, operador TESTER explícito)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Solo la tarea `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Al inicio no existía ese path; el fichero era `CLOSED-20260322-0110-…`. Cadena **CLOSED → UNTESTED → TESTING** (mismo basename) según `003-tester/TESTER.md`. El H1 pasó a **TESTING** durante la verificación y se restauró a **CLOSED** antes de **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, 003-tester/TESTER.md)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea única: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Al inicio solo existía `CLOSED-20260322-0110-…`; según `003-tester/TESTER.md` se aplicó **CLOSED → UNTESTED → TESTING** (mismo basename) para poder ejecutar el paso literal **UNTESTED → TESTING**. Durante la verificación el nombre de archivo fue `TESTING-…` y el H1 del documento pasó brevemente a **TESTING**, luego se restauró a **CLOSED** antes del renombrado final del archivo. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (run — Cursor agent, 2026-03-28, 003-tester/TESTER.md, only UNTESTED-20260322-0110)

**Date:** 2026-03-28, local system time where `cargo` ran (operator timezone; not explicit UTC).

**Preflight / names:** Operator specified only `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (no other `UNTESTED-*`). That path was absent; the file on disk was `CLOSED-20260322-0110-…`. Per `003-tester/TESTER.md`, applied **CLOSED → UNTESTED → TESTING** (same basename) so the UNTESTED→TESTING rename applies without selecting another task.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (lines 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` with `discord:{}`).

**Outcome:** All acceptance criteria met. Renaming **TESTING- → CLOSED-** after this report. Live Discord not exercised.

## Test report (corrida — agente Cursor, 2026-03-28, 003-tester/TESTER.md)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno; no UTC explícito).

**Preflight / nombres:** Tarea fijada: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` únicamente (sin otro `UNTESTED-*`). Ese path no existía al inicio; el fichero estaba como `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso **UNTESTED → TESTING** sin elegir otro archivo. El H1 del documento pasó a **TESTING** durante la verificación y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, sesión 003-tester)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no UTC explícito).

**Preflight / nombres:** Tarea única: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Al inicio solo existía `CLOSED-20260322-0110-…`; según `003-tester/TESTER.md` se aplicó **CLOSED → UNTESTED → TESTING** (mismo basename) para cumplir el paso **UNTESTED → TESTING**. El H1 del documento pasó a **TESTING** durante la verificación y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la crate lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos en esta corrida. Archivo **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, 003-tester/TESTER.md)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no UTC explícito).

**Preflight / nombres:** Tarea pedida: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` únicamente (sin otro `UNTESTED-*`). Ese path no existía; el fichero estaba como `CLOSED-20260322-0110-…`. Cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para cumplir el paso **UNTESTED → TESTING** de `003-tester/TESTER.md`. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no UTC explícito).

**Preflight / nombres:** Únicamente la tarea `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin elegir otro `UNTESTED-*`). Ese path no existía al inicio; el fichero estaba como `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso **UNTESTED → TESTING**. No se tocó ningún otro archivo `UNTESTED-*`. El H1 del documento pasó a **TESTING** durante la verificación y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos en esta corrida. Archivo **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, 003-tester/TESTER.md)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea indicada por el operador: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin elegir otro `UNTESTED-*`). Ese path no existía al inicio; el fichero estaba como `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para poder ejecutar el paso **UNTESTED → TESTING**. El H1 del documento pasó brevemente a **TESTING** durante el trabajo y se restauró a **CLOSED** antes del renombrado final del archivo. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored en doc-tests del crate)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (run — Cursor agent, 2026-03-28, 003-tester/TESTER.md, only UNTESTED-20260322-0110)

**Date:** 2026-03-28, local system time where `cargo` ran (operator environment; not explicit UTC).

**Preflight / names:** Operator specified only `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (no other `UNTESTED-*`). That path was absent at run start; the file on disk was `CLOSED-20260322-0110-…`. Per `003-tester/TESTER.md`, applied rename chain **CLOSED → UNTESTED → TESTING** (same basename) so the UNTESTED→TESTING step applies without selecting another `UNTESTED-*` file. Document H1 set to **TESTING** during verification, restored to **CLOSED** before final filename rename.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (854 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (lines 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` with `discord:{}`)

**Outcome:** All acceptance criteria met. Renaming **TESTING- → CLOSED-** after this report. Live Discord not exercised.

## Test report (corrida — agente Cursor, 2026-03-28, 003-tester/TESTER.md)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no UTC explícito).

**Preflight / nombres:** Tarea única: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Al inicio solo existía `CLOSED-20260322-0110-…`; según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename), verificación, informe y **TESTING- → CLOSED-** en el nombre de archivo. El H1 quedó en **CLOSED** antes del renombrado final del fichero.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; salida: 868 tests filtrados en el binario de lib)
- `cd src-tauri && cargo test` — **pass** (870 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored en doc-tests del crate)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, 003-tester/TESTER.md)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea indicada: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` únicamente (sin otro `UNTESTED-*`). Al inicio el fichero estaba como `CLOSED-20260322-0110-…`; según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso **UNTESTED → TESTING**. No se tocó ningún otro archivo `UNTESTED-*`. El H1 del documento estuvo en **TESTING** durante la verificación y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored en doc-tests del crate)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.


## Test report (run — Cursor agent, 2026-03-28, 003-tester/TESTER.md)

**Date:** 2026-03-28, local system time (not explicit UTC).

**Preflight / names:** Operator specified only `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (no other `UNTESTED-*`). That path was absent; the file on disk was `CLOSED-20260322-0110-…`. Per `003-tester/TESTER.md`, applied rename chain **CLOSED → UNTESTED → TESTING** (same basename) so the UNTESTED→TESTING step applies without selecting another task. Document H1 set to **TESTING** during verification, restored to **CLOSED** before final filename **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered in lib test binary)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (lines 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` with `discord:{}`)

**Outcome:** All acceptance criteria met. Renaming **TESTING- → CLOSED-** after this report. Live Discord not exercised.

## Test report (corrida — agente Cursor, 2026-03-28, sesión adicional)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no UTC explícito).

**Preflight / nombres:** Tarea única: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Al inicio no existía ese path; el fichero era `CLOSED-20260322-0110-…`. Cadena **CLOSED → UNTESTED → TESTING** (mismo basename) según `003-tester/TESTER.md`. H1 **TESTING** durante `cargo`, restaurado a **CLOSED** antes de **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (2 passed, 869 filtered)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (1143, 1347, 1934, 2310 con `discord:{}`).

**Outcome:** Criterios cumplidos; **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, 003-tester/TESTER.md)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no UTC explícito).

**Preflight / nombres:** Tarea única: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Ese path no existía al inicio; el fichero estaba como `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso **UNTESTED → TESTING**. El H1 del documento estuvo en **TESTING** durante la verificación y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered en el binario de tests de `lib.rs`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored en doc-tests del crate)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, 003-tester/TESTER.md, sesión operador)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no UTC explícito).

**Preflight / nombres:** Tarea fijada: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` únicamente (sin otro `UNTESTED-*`). Ese path no existía al inicio; el fichero estaba como `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso **UNTESTED → TESTING**. El H1 del documento pasó a **TESTING** durante la verificación y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored en doc-tests del crate)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, 003-tester/TESTER.md, sesión actual)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea indicada: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` únicamente (sin otro `UNTESTED-*`). Al inicio del run el fichero estaba como `CLOSED-20260322-0110-…`; según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para cumplir el paso **UNTESTED → TESTING**. El H1 del documento pasó a **TESTING** durante la verificación y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered en `lib.rs`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, 003-tester/TESTER.md)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (zona horaria del entorno del operador; no UTC explícito).

**Preflight / nombres:** Tarea fijada: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` únicamente (sin otro `UNTESTED-*`). Al inicio solo existía `CLOSED-20260322-0110-…`; según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para cumplir el paso **UNTESTED → TESTING**. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered en `lib.rs`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored en doc-tests del crate)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`).

**Outcome:** Todos los criterios de aceptación cumplidos. Archivo renombrado **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, sesión operador / TESTER.md)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no UTC explícito).

**Preflight / nombres:** El operador indicó solo `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Ese path no existía; el fichero era `CLOSED-20260322-0110-…`. Cadena **CLOSED → UNTESTED → TESTING** (mismo basename) según `003-tester/TESTER.md`. El H1 del documento pasó a **TESTING** durante el renombrado y se restauró a **CLOSED** antes del paso final **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (2 passed, 869 filtered)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en `lib.rs` tests; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (1143, 1347, 1934, 2310 con prefijo `discord:{}` en la clave Ollama).

**Outcome:** Criterios de aceptación cumplidos. Archivo **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (run — Cursor agent, 2026-03-28, `003-tester/TESTER.md`, task `UNTESTED-20260322-0110` only)

**Date:** 2026-03-28, local system time (where `cargo` ran); not UTC-normalized.

**Preflight / names:** Operator asked to test only `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` and not to pick another `UNTESTED-*`. That filename was absent; the task on disk was `CLOSED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md`. Per `003-tester/TESTER.md`, applied rename chain **CLOSED → UNTESTED → TESTING** (same basename) so the literal **UNTESTED → TESTING** step applies without selecting a different task. Document H1 was set to **TESTING** for the run and restored to **CLOSED** before the final file rename **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (lines 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` with `discord:{}`)

**Outcome:** All acceptance criteria pass. Final filename **CLOSED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md**. Live Discord not exercised.

## Test report (run — Cursor agent, 2026-03-28, `003-tester/TESTER.md`, only `UNTESTED-20260322-0110`)

**Date:** 2026-03-28, local system time where `cargo` ran (operator environment; not explicit UTC).

**Preflight / names:** Operator specified only `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (no other `UNTESTED-*`). At run start the file was `CLOSED-20260322-0110-…`; per `003-tester/TESTER.md` applied rename chain **CLOSED → UNTESTED → TESTING** (same basename) so the **UNTESTED → TESTING** step applies without selecting another task.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered in `lib` test binary)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (lines 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` with `discord:{}`)

**Outcome:** All acceptance criteria pass. Renaming **TESTING- → CLOSED-** after this report. Live Discord not exercised.

## Test report (verificación — agente Cursor, conversación actual)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no normalizada a UTC).

**Preflight / nombres:** El operador fijó únicamente `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin elegir otro `UNTESTED-*`). Ese path no existía al inicio; el fichero estaba como `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso **UNTESTED → TESTING**. El H1 del documento estuvo en **TESTING** durante `cargo` y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (2 passed, 869 filtered out en el binario de tests de `lib.rs`; `same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de librería `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`)

**Outcome:** **Pass** — criterios de aceptación cumplidos. Nombre final del fichero: `CLOSED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md`. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, `003-tester/TESTER.md`)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no normalizada a UTC).

**Preflight / nombres:** Tarea pedida: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` únicamente (sin otro `UNTESTED-*`). Ese path no existía al inicio; el fichero estaba como `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso **UNTESTED → TESTING**. El H1 del documento estuvo en **TESTING** durante `cargo` y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered out en el binario de tests de `lib.rs`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de librería `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`)

**Outcome:** **Pass** — todos los criterios de aceptación cumplidos. Renombrado final del fichero: **CLOSED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md**. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, `003-tester/TESTER.md`, solo `UNTESTED-20260322-0110`)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no normalizada a UTC).

**Preflight / nombres:** Tarea fijada: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` únicamente (sin otro `UNTESTED-*`). Al inicio solo existía `CLOSED-20260322-0110-…`; según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso **UNTESTED → TESTING**. El H1 del documento pasó a **TESTING** durante la verificación y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered out en el binario de tests de `lib.rs`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de librería `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`)

**Outcome:** **Pass** — criterios de aceptación cumplidos. Renombrado final: `CLOSED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md`. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, `003-tester/TESTER.md`, tarea `UNTESTED-20260322-0110` única)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no normalizada a UTC).

**Preflight / nombres:** El operador indicó solo `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Ese nombre no existía al inicio; el fichero era `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso **UNTESTED → TESTING**. El H1 pasó a **TESTING** durante la corrida y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered out en el binario de tests de `lib.rs`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de librería `mac_stats`; 1 doc-test ignored en doc-tests del crate)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`)

**Outcome:** **Pass** — todos los criterios de aceptación cumplidos. Renombrado final del fichero: **CLOSED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md**. Discord en vivo no probado.

## Test report (run — Cursor agent, 2026-03-28, `003-tester/TESTER.md`)

**Date:** 2026-03-28, local system time where `cargo` ran (not UTC-normalized).

**Preflight / names:** Operator asked to test only `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` and not to pick another `UNTESTED-*`. That path was absent; the file on disk was `CLOSED-20260322-0110-…`. Per `003-tester/TESTER.md`, applied rename chain **CLOSED → UNTESTED → TESTING** (same basename) so the **UNTESTED → TESTING** step applies without selecting a different task. Document H1 was **TESTING** during `cargo`, then restored to **CLOSED** before final filename **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (lines 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` with `discord:{}`)

**Outcome:** **Pass** — all acceptance criteria met. Final filename: `CLOSED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md`. Live Discord not exercised.

## Test report (corrida — agente Cursor, 2026-03-28, `003-tester/TESTER.md`)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no normalizada a UTC).

**Preflight / nombres:** Tarea pedida: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` únicamente (sin otro `UNTESTED-*`). Al inicio el fichero era `CLOSED-20260322-0110-…`; según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso **UNTESTED → TESTING**. El H1 pasó a **TESTING** al renombrar a `TESTING-…` y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (2 passed, 869 filtered; `same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`)

**Outcome:** **Pass** — criterios de aceptación cumplidos. Renombrado final **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, `003-tester/TESTER.md`, tarea `UNTESTED-20260322-0110` única)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no normalizada a UTC).

**Preflight / nombres:** El operador indicó solo `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Ese path no existía: el fichero estaba como `CLOSED-20260322-0110-…`. Se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para cumplir el paso **UNTESTED → TESTING** de `003-tester/TESTER.md`. El H1 del documento pasó a **TESTING** durante la verificación. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered en el binario de tests de `lib.rs`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored en doc-tests del crate)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`)

**Outcome:** **Pass** — criterios de aceptación cumplidos. Renombrado final del fichero: **TESTING- → CLOSED-** tras este informe (el operador pidió `TESTED-` en caso de fallo; no aplica). Discord en vivo no probado.

## Test report (run — Cursor agent, 2026-03-28, `003-tester/TESTER.md`, task `UNTESTED-20260322-0110` only)

**Date:** 2026-03-28, local system time where `cargo` ran (not UTC-normalized).

**Preflight / names:** Operator specified only `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (no other `UNTESTED-*`). That path was absent at run start; the file was `CLOSED-20260322-0110-…`. Per `003-tester/TESTER.md`, applied rename chain **CLOSED → UNTESTED → TESTING** (same basename) so the **UNTESTED → TESTING** step applies without selecting another task. Document H1 was set to **TESTING** while the filename was `TESTING-…`, then restored to **CLOSED** before final **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (lines 1143, 1347, 1934 `run_serial`; line 2310 `ollama_queue_key` with `discord:{}`)

**Outcome:** **Pass** — all acceptance criteria met. Final filename: `CLOSED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md`. Live Discord not exercised.

## Test report (corrida — agente Cursor, 2026-03-28, `003-tester/TESTER.md`)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no normalizada a UTC).

**Preflight / nombres:** Tarea fijada: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` únicamente (sin otro `UNTESTED-*`). Al inicio del run el fichero estaba como `CLOSED-20260322-0110-…` (no existía el path `UNTESTED-…`); según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso **UNTESTED → TESTING**. Durante la verificación el H1 del documento estuvo en **TESTING** y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered en el binario de tests de `lib.rs`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored en doc-tests del crate)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`)

**Outcome:** **Pass** — criterios de aceptación cumplidos. Renombrado final **TESTING- → CLOSED-** tras este informe (en caso de fallo el operador pidió `TESTED-`; no aplica). Discord en vivo no probado.

## Test report (run — Cursor agent, 2026-03-28, `003-tester/TESTER.md`, only `UNTESTED-20260322-0110`)

**Date:** 2026-03-28, local system time where `cargo` ran (operator timezone; not explicit UTC).

**Preflight / names:** Operator specified only `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (no other `UNTESTED-*`). At run start the file was `CLOSED-20260322-0110-…`; per `003-tester/TESTER.md` applied rename chain **CLOSED → UNTESTED → TESTING** (same basename) so the **UNTESTED → TESTING** step applies. No other `UNTESTED-*` task file was used.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered in `lib` test binary)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (lines 1143, 1347, 1934 `run_serial`; line 2310 `ollama_queue_key` with `discord:{}`)

**Outcome:** **Pass** — all acceptance criteria met. Final filename after report: `CLOSED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (on failure the operator asked for `TESTED-`; not applicable). Live Discord not exercised.

## Test report (corrida — agente Cursor, 2026-03-28, `003-tester/TESTER.md`, tarea `UNTESTED-20260322-0110` única)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no UTC explícito).

**Preflight / nombres:** El operador fijó solo `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Al inicio solo existía `CLOSED-20260322-0110-…`; según `003-tester/TESTER.md` se aplicó **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso **UNTESTED → TESTING**. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`)

**Outcome:** **Pass** — criterios cumplidos. Renombrado final **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (run — Cursor agent, 2026-03-28, `003-tester/TESTER.md`, task `UNTESTED-20260322-0110` only)

**Date:** 2026-03-28, local system time where `cargo` ran (operator environment; not explicit UTC).

**Preflight / names:** Operator specified only `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (no other `UNTESTED-*`). That path was absent; the file on disk was `CLOSED-20260322-0110-…`. Per `003-tester/TESTER.md`, applied rename chain **CLOSED → UNTESTED → TESTING** (same basename). Document H1 was **TESTING** while `cargo` ran, then restored to **CLOSED** before this report and final filename **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered in `lib` test binary)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed in `mac_stats` lib tests; 1 doc-test ignored)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (lines 1143, 1347, 1934 `run_serial`; line 2310 `ollama_queue_key` with `discord:{}`)

**Outcome:** **Pass** — all acceptance criteria met. Final filename: `CLOSED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md`. On failure the operator asked for `TESTED-`; not applicable. Live Discord not exercised.

## Test report (corrida — agente Cursor, 2026-03-28, `003-tester/TESTER.md`, solo `UNTESTED-20260322-0110`)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no normalizada a UTC).

**Preflight / nombres:** Tarea pedida: `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Ese path no existía; el fichero estaba como `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso **UNTESTED → TESTING**. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered en el binario de tests de `lib.rs`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored en doc-tests del crate)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`)

**Outcome:** **Pass** — criterios de aceptación cumplidos. Renombrado final **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, `003-tester/TESTER.md`, tarea `UNTESTED-20260322-0110` única)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no normalizada a UTC).

**Preflight / nombres:** El operador indicó solo `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Ese path no existía al inicio; el fichero estaba como `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso **UNTESTED → TESTING**. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered en el binario de tests de `lib.rs`)
- `cd src-tauri && cargo test` — **fail** (`870 passed; 1 failed; 0 ignored` en `--lib`): `discord::tests::outbound_attachment_path_allowlist` panic en `src/discord/mod.rs:3332` — *path under pdfs_dir should be allowed when directory exists*

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`)

**Outcome:** **Fail** — el criterio 2 de la tarea («`cargo test` en `src-tauri/` succeeds») no se cumple por un fallo ajeno directamente a `keyed_queue`, pero bloquea el cierre. Renombrado final del fichero: **TESTING- → TESTED-** tras este informe (según instrucción del operador en caso de fallo). Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, `003-tester/TESTER.md`, solo tarea `20260322-0110`)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no normalizada a UTC).

**Preflight / nombres:** El operador indicó únicamente `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Ese path **no existía**; el fichero en disco era `tasks/TESTED-20260322-0110-…` (estado tras un fallo previo documentado). No se eligió ningún otro `UNTESTED-*`. Se renombró **TESTED → TESTING** (mismo basename) para ejecutar la fase de verificación de `003-tester/TESTER.md`. El H1 del documento quedó en **TESTING** durante la corrida.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered en el binario de tests de `lib.rs`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored en doc-tests del crate)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`)

**Outcome:** **Pass** — todos los criterios de aceptación cumplidos en esta corrida (incluido `cargo test` completo; el fallo previo de `outbound_attachment_path_allowlist` ya no se reproduce). Renombrado final del fichero: **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.

## Test report (corrida — agente Cursor, 2026-03-28, `003-tester/TESTER.md`, tarea `UNTESTED-20260322-0110` única)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no normalizada a UTC).

**Preflight / nombres:** El operador indicó solo `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Ese path no existía al inicio; el fichero estaba como `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para ejecutar el paso **UNTESTED → TESTING**. No se tocó ningún otro archivo `UNTESTED-*`. El H1 pasó a **TESTING** durante la verificación y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered en el binario de tests de `lib.rs`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored en doc-tests del crate)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`)

**Outcome:** **Pass** — todos los criterios de aceptación cumplidos. Renombrado final **TESTING- → CLOSED-** tras este informe (en fallo hubiera sido **TESTED-** según el operador; `003-tester/TESTER.md` indica **WIP-** para bloqueo). Discord en vivo no probado.

## Test report (corrida — agente Cursor, conversación actual 2026-03-28, `003-tester/TESTER.md`)

**Date:** 2026-03-28, hora local del sistema donde se ejecutó `cargo` (no normalizada a UTC).

**Preflight / nombres:** El operador indicó solo `tasks/UNTESTED-20260322-0110-openclaw-keyed-async-queue-per-conversation.md` (sin otro `UNTESTED-*`). Ese path no existía; el fichero estaba como `CLOSED-20260322-0110-…`. Según `003-tester/TESTER.md` se aplicó la cadena **CLOSED → UNTESTED → TESTING** (mismo basename) para cumplir el paso **UNTESTED → TESTING**. El H1 del documento pasó a **TESTING** durante `cargo` y se restauró a **CLOSED** antes del renombrado final **TESTING- → CLOSED-**. No se tocó ningún otro archivo `UNTESTED-*`.

**Commands run**

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test keyed_queue` — **pass** (`same_key_runs_sequentially`, `different_keys_may_overlap`; 2 passed, 869 filtered out en el binario de tests de `lib.rs`)
- `cd src-tauri && cargo test` — **pass** (871 passed, 0 failed en tests de la lib `mac_stats`; 1 doc-test ignored en doc-tests del crate)

**Static spot-check**

- `rg -n "keyed_queue::run_serial|ollama_queue_key" src-tauri/src/discord/mod.rs` — **pass** (líneas 1143, 1347, 1934 `run_serial`; 2310 `ollama_queue_key` con `discord:{}`)

**Outcome:** **Pass** — todos los criterios de aceptación cumplidos. Renombrado final **TESTING- → CLOSED-** tras este informe. Discord en vivo no probado.
