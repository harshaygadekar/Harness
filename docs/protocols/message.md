# Message types

**Module:** `agentic_harness.messages`  
**Related:** ADR-001 (message list = canonical history); research brief 1

## Purpose

Hold the linear chat history the model and Runner share. v0 has **no** typed event bus in core. The message list is the store.

## Required surface

### `Role`

`Literal["system", "user", "assistant", "tool"]`

### `ToolCall` (frozen dataclass)

| Field | Type | Notes |
|-------|------|-------|
| `id` | `str` | Correlates with tool-role `tool_call_id` |
| `name` | `str` | Tool registry name |
| `arguments` | `dict[str, Any]` | Parsed args for `Tool.call` |

### `Message` (dataclass)

| Field | Type | Default | Notes |
|-------|------|---------|-------|
| `role` | `Role` | | |
| `content` | `str \| None` | `None` | Text body |
| `tool_calls` | `list[ToolCall]` | `[]` | Assistant turn may list calls |
| `tool_call_id` | `str \| None` | `None` | Tool-role reply id |
| `name` | `str \| None` | `None` | Tool name on tool-role messages |

### `Usage` (frozen dataclass)

| Field | Type | Default |
|-------|------|---------|
| `input_tokens` | `int` | `0` |
| `output_tokens` | `int` | `0` |

## Invariants

1. History is a `list[Message]` owned by the Runner for the run (plus optional host-supplied seed messages).
2. Stop when an assistant message has **no** `tool_calls` → `RunStatus.succeeded` (no built-in `finish` tool).
3. Each executed (or denied/failed) tool call appends a `role="tool"` message with matching `tool_call_id`.
4. Content that leaves tools is sanitized before append.
5. No separate core `Event` type. Event/step log stays a future plugin topic.

## What is NOT in core

- Typed event log / fiber bus
- Multi-part content blocks (images, etc.) as first-class message parts
- Pydantic models (dataclasses / dicts only)
- Durable session store or checkpointer

## Replaceability test

Message is a **data** contract, not a swappable service.

1. Build history with the same fields using two host serializers (e.g. JSON round-trip A vs B).
2. Feed either list into `Runner.run(..., messages=...)`.
3. **Pass:** Runner and `ModelClient` accept the list without loop changes.
4. **Fail (do not “fix” in core):** need for a new role or event type without a failing `examples/` case and Thomas approval (ADR-003).
