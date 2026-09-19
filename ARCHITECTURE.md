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

Gemini lives under the gemini extra / plugin path. It is not imported by the core package by default.

## Inspiration

DeepSeek Harness / Cordis: capability seams, tool gate, no product tools in the thin core. We keep the default loop **in** core (not as a removable Cordis-style plugin) for a small Python install.

## Product lock

`docs/useful-scope.md` — what “useful” means and what we will not add to core.
