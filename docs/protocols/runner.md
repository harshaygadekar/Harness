# Runner, gate, and result

**Modules:** `agentic_harness.runner`, `agentic_harness.gate`, `agentic_harness.result`  
**Related:** [ADR-001](../decisions/ADR-001-core-loop.md)

## Purpose

Drive the ReAct/tool loop: model step → optional tool calls (gated) → append observations → stop on final text, `max_steps`, cancel, or hard setup failure. This is the core driver. It is not a removable Cordis-style plugin.

## Required surface

### `Runner`

| Member | Shape | Notes |
|--------|-------|-------|
| `__init__` | `(registry: Registry)` | |
| `run` | `(user_input: str, *, config: RunConfig \| None = None, messages: list[Message] \| None = None) -> RunResult` | Sync loop |

### `RunConfig` (dataclass)

| Field | Type | Default | Notes |
|-------|------|---------|-------|
| `max_steps` | `int` | `8` | Bound runaway |
| `can_use_tool` | `CanUseTool` | `deny_all` | Host policy |
| `cancel` | `CancelCheck \| None` | `None` | `() -> bool` |
| `system` | `str \| None` | `None` | Inserted once if no system message yet |

### `RunContext` (dataclass)

Opaque host bag passed into tools: `data: dict`, `step: int`. Not a credential store.

### Gate (`gate.py`)

| Name | Role |
|------|------|
| `Allow` | Frozen allow decision |
| `Deny(reason=...)` | Frozen deny decision |
| `Decision` | `Allow \| Deny` |
| `CanUseTool` | `(tool_name, args, ctx) -> Decision` |
| `deny_all` | Default: Deny every tool |

### `RunResult` / `RunStatus` / `SafeError` (`result.py`)

| Piece | Fields / values |
|-------|-----------------|
| `RunStatus` | `succeeded`, `failed`, `cancelled`, `max_steps` |
| `SafeError` | `code: str`, `message: str` (safe text only) |
| `RunResult` | `status`, `messages`, `usage`, `error`, `steps` |

## Invariants

1. No model registered → `failed` + `SafeError(code="NO_MODEL", ...)`.
2. Assistant with no `tool_calls` → `succeeded`.
3. Gate Deny (or non-Allow) → tool-role observation `DENIED: …`; do not call `tool.call`.
4. Cancel before/during steps → `cancelled`; do not start a **new** tool call after cancel (C5).
5. `max_steps` exhausted → `max_steps` + `SafeError(code="MAX_STEPS", ...)`.
6. Tool exceptions → sanitized observation; secrets scrubbed.
7. Tool calls in one turn run **sequentially** (v0).

## What is NOT in core

- Graph DSL / multi-agent orchestrator
- Async `run_async` as shipped API (open brief; sync `run` only today)
- Event emitter / public hook bus on Runner
- Built-in product tools
- Skipping the gate for “trusted” tools

## Replaceability test

The Runner **is** the loop. You do not replace Runner to swap model/tools.

1. Keep the same `Runner` class.
2. Swap `ModelClient` A→B and/or `Tool` A→B via Registry.
3. Swap gate function A→B via `RunConfig.can_use_tool`.
4. **Pass:** identical `Runner.run` call site; only config/registry implementations change.
5. **Fail:** need to subclass Runner or patch `_run_one_tool` for a new primitive → missing-primitive process (ADR-003), not a silent core edit.
