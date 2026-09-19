# Agent façade (`simple.Agent`)

**Module:** `agentic_harness.simple`  
**Related:** ADR-001 (core unchanged); ADR-003 (façade note)

## Purpose

Short DX API for demos and one-file scripts: construct `Agent(model, tools=...)`, call `run(prompt)`. Internally builds a `Registry`, registers model/tools, sets an allow-list gate (or deny-all if no tools), and delegates to `Runner`.

**Mike’s note:** This is a **façade above core**, not a new core primitive. It is not exported from `agentic_harness.__init__` today; import `agentic_harness.simple`.

## Required surface

### `Agent` (dataclass)

| Field | Type | Default | Notes |
|-------|------|---------|-------|
| `model` | `ModelClient` | (required) | |
| `tools` | `Sequence[Tool \| FunctionTool]` | `()` | |
| `system` | `str \| None` | `None` | Passed to `RunConfig` |
| `max_steps` | `int` | `8` | |

| Method | Shape | Notes |
|--------|-------|-------|
| `run` | `(prompt: str) -> RunResult` | Builds Registry + Runner each call |

### Helpers

| Name | Role |
|------|------|
| `function_tool(...)` | Builds a `FunctionTool` for short examples |
| `allow_listed(names)` | Gate that Allows only names in the set |

## Invariants

1. Does not add protocols to core. Same `ModelClient` / `Tool` / `RunResult` contracts.
2. With tools → allow only those tool names. With no tools → default deny-all from `RunConfig`.
3. Hosts that need cancel, seed messages, or custom gates use `Registry` + `Runner` directly.
4. Must not become a second loop implementation. If façade logic diverges from Runner semantics, fix the façade — do not fork the loop.

## What is NOT in core

- `Agent` is **not** part of the frozen **core** primitive set (ADR-003).
- No multi-agent, memory, or session APIs on this class.
- Not a replacement for Plugin registration of provider adapters.

## Replaceability test

1. Path A: `simple.Agent(model, tools=[t]).run(prompt)`.
2. Path B: manual `Registry` + `register_model` / `register_tool` + `RunConfig(can_use_tool=allow_listed(...))` + `Runner(reg).run(prompt)`.
3. Same model and tool implementations.
4. **Pass:** same `RunResult` status shape and tool observations without changing `runner.py`.
5. Hosts may drop the façade entirely; core stays usable.
