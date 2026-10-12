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

- **OpenAI-compatible** — `OpenAiCompat` in the Rust crate `harness-model` (`native/crates/harness-model`). It reads `HARNESS_API_KEY` (or `OPENAI_API_KEY`), `HARNESS_MODEL` (required) and `HARNESS_BASE_URL` (optional) from env. Gemini runs through its OpenAI-compatible endpoint. Retries with backoff on 429/5xx live in the adapter, not in core. The key is redacted in debug output, and HTTP error bodies are scrubbed.
- **Scripted** — `ScriptedModel` in `harness-model` plays a fixed list of responses for CI and replay.
- **Tests/CI** — use the scripted model. No network or key needed.
- Adapters do not ship a `finish` tool or shell.

See [ADR-002](decisions/ADR-002-model-adapter.md) (Accepted).

## Plugins

MCP, sandboxes, memory, observability, tool packs — out of core. V0 plugins are in-process and fully trusted (`SECURITY.md`).

## Product lock

`docs/useful-scope.md`.
