# WIP — OpenClaw / Ollama request queue (2026-03-21)

## Goal

*(Especificación original ausente.)* El operador solicitó probar únicamente `tasks/UNTESTED-20260321-2100-openclaw-ollama-request-queue.md` según `003-tester/TESTER.md`, pero ese archivo **no existía** en el repositorio en el momento de la ejecución, por lo que no fue posible el paso de renombrado `UNTESTED-` → `TESTING-`.

## References (relacionadas en código, no parte del cuerpo de tarea original)

- `src-tauri/src/ollama_queue.rs` — cola HTTP Ollama por clave, `with_ollama_http_queue`, prueba `ollama_http_queue_serializes_and_fires_wait_hook`
- `src-tauri/src/commands/ollama.rs` — `skip_ollama_queue`, `ollama_queue_key`, `ollama_queue_wait_hook`

## Test report

**Fecha:** 2026-03-27 (local, entorno de ejecución del agente; no se garantiza UTC).

**Bloqueo:** No se encontró `tasks/UNTESTED-20260321-2100-openclaw-ollama-request-queue.md` (listado de `tasks/` solo contenía archivos `CLOSED-*`). No se aplicó otro `UNTESTED-*` en esta corrida.

**Comandos ejecutados** (verificación parcial sin criterios de aceptación del archivo de tarea):

- `cd src-tauri && cargo check` — **pass**
- `cd src-tauri && cargo test ollama_http_queue --lib` — **pass** (1 test: `ollama_queue::tests::ollama_http_queue_serializes_and_fires_wait_hook`)

**Resultado:** **WIP** — tarea bloqueada: falta el archivo `UNTESTED-…` con objetivo, criterios de aceptación y comandos de verificación. Restaurar o crear ese contenido y volver a ejecutar el flujo de `TESTER.md` (renombrar a `TESTING-`, verificar, informe, `CLOSED-` o `WIP-`).
