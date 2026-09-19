# Model protocol

**Module:** `agentic_harness.model`  
**Related:** [ADR-002](../decisions/ADR-002-model-adapter.md)

## Purpose

Give the Runner one provider-agnostic step: send messages (and optional tool schemas), get one assistant turn back. Providers live in plugins. Core does not import provider SDKs.

## Required surface

### `ModelClient` (Protocol)

| Member | Shape | Notes |
|--------|-------|-------|
| `complete` | `(messages: list[Message], tools: list[ToolSpec] \| None = None) -> ModelResponse` | One model step. May put `tool_calls` on the assistant `Message`. |

Runtime-checkable. Sync only in the current cut.

### `ModelResponse` (dataclass)

| Field | Type | Notes |
|-------|------|-------|
| `message` | `Message` | Usually `role="assistant"` |
| `usage` | `Usage \| None` | Optional token counts; Runner sums across steps |

## Invariants

1. Core never imports `google-genai`, `openai`, or other provider SDKs.
2. Tool JSON Schema → provider tool format is mapped **inside the adapter**, not on `ModelClient`.
3. Adapter errors that reach the host or Runner must be sanitized (no raw keys in text). See ADR-002 / Jessie.
4. Registry holds at most one active model for a run (`Registry.model`).
5. Fake `ModelClient` is the default for unit tests and CI (no network).

## What is NOT in core

- Provider SDKs, retries, rate-limit backoff (plugin only)
- Streaming / `complete_stream` (open question; optional later)
- Auth or credential stores
- A built-in `finish` tool or any product tool

## Replaceability test

1. Implement fake client A that returns fixed assistant text (or tool calls).
2. Implement client B (e.g. Gemini plugin) with the same `complete` signature.
3. `registry.register_model(B)` instead of A.
4. Call `Runner(registry).run(...)` with the same tools and gate.
5. **Pass:** loop code unchanged; only the registered model changes.
