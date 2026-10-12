# Architecture

Brief note; full core decision is [ADR-001](decisions/ADR-001-core-loop.md). Repo-root `ARCHITECTURE.md` mirrors the same shape.

## Core

- `Runner` — ReAct/tool loop (`max_steps`, cancel)
- `ModelClient` — provider-agnostic protocol in core
- `Tool` + registry — name, schema, `call` (only via Runner)
- Deny-all `can_use_tool` until host Allows by name
- `Plugin.register` + disposer

## Model adapters

Provider SDKs stay **out of core**. Each adapter is a plugin that implements the `ModelClient` protocol (`complete(messages, tools) → ModelResponse`).

- **Gemini** — plugin under the gemini extra; reads `GEMINI_API_KEY` / `GOOGLE_API_KEY` from env; default model `gemini-flash-latest`; 503 retries/backoff only in the plugin; never log the key; sanitize errors (Jessie `AIza` / `AQ.` patterns).
- **Tests/CI** — fake `ModelClient` is the default; no provider SDK required.
- Adapters do not ship a `finish` tool or shell.

See [ADR-002](decisions/ADR-002-model-adapter.md) (Accepted).

## Plugins

MCP, sandboxes, memory, observability, tool packs — out of core. V0 plugins are in-process and fully trusted (`SECURITY.md`).

## Product lock

`docs/useful-scope.md`.
