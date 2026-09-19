# Architecture

## Core

- `Runner` — ReAct/tool loop (`max_steps`, cancel)
- `ModelClient` — provider-agnostic model call
- `Tool` + registry — name, schema, `call`
- Message list history
- `Plugin.register` + disposer
- `can_use_tool` — **deny all** until host Allows by name
- `RunResult` — status, messages, usage, safe error

See `docs/decisions/ADR-001-core-loop.md`.

## Plugins

Provider adapters (e.g. Gemini), MCP, sandboxes, memory, observability, tool packs — **out of core**.

Gemini lives under the gemini extra / plugin path. It is not imported by the core package by default. Details: ADR-002.


## Model adapters

`ModelClient` is a **protocol in core**; each provider is a **plugin** that implements it. Core must not import `google-genai` or other provider SDKs.

- Gemini plugin: env `GEMINI_API_KEY` / `GOOGLE_API_KEY`; default `gemini-flash-latest`; 503 retries only in the plugin; never log the key; sanitize errors (Jessie `AIza` / `AQ.`).
- Fake `ModelClient` is the default for tests/CI.
- No `finish` tool or shell in adapters.

See `docs/decisions/ADR-002-model-adapter.md` (Proposed).

## Inspiration

DeepSeek Harness / Cordis: capability seams, tool gate, no product tools in the thin core. We keep the default loop **in** core (not as a removable Cordis-style plugin) for a small Python install.

## Product lock

`docs/useful-scope.md` — what “useful” means and what we will not add to core.
