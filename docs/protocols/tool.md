# Tool protocol

**Module:** `agentic_harness.tool`  
**Related:** [ADR-001](../decisions/ADR-001-core-loop.md) (C1–C2), Jessie gate rules

## Purpose

Describe an in-process capability the model may request. The Runner is the only path that calls `tool.call` after the host gate Allows the name.

## Required surface

### `Tool` (Protocol)

| Member | Type | Notes |
|--------|------|-------|
| `name` | `str` | Stable id; gate and registry key |
| `description` | `str` | Shown to the model via `ToolSpec` |
| `parameters` | `dict[str, Any]` | JSON Schema object |
| `call` | `(args: dict[str, Any], ctx: Any) -> ToolResult` | Side effects only after Allow |

### `ToolSpec` (frozen dataclass)

| Field | Type | Notes |
|-------|------|-------|
| `name` | `str` | |
| `description` | `str` | |
| `parameters` | `dict[str, Any]` | JSON Schema object |

Passed to `ModelClient.complete` as the tool list.

### `ToolResult` (frozen dataclass)

| Field | Type | Default | Notes |
|-------|------|---------|-------|
| `content` | `str` | | Observation text (sanitized by Runner) |
| `ok` | `bool` | `True` | If false, Runner prefixes `TOOL_FAILED:` |

### `FunctionTool` (dataclass helper)

Host-supplied in-process tool: `name`, `description`, `parameters`, `handler(args, ctx) -> str`, plus `call` / `spec()`. Not a built-in product tool.

## Invariants

1. **C2:** Only `Runner` reaches `tool.call`. No public bypass execute helper.
2. **C1:** Default gate is deny-all. Host must Allow by name (or allow list).
3. Unknown tool name → tool-role message with safe `TOOL_FAILED` text; no raise out of the loop.
4. Exceptions in `call` become sanitized observations (`safe_tool_error`); no secret leak.
5. v0 runs multiple tool calls in one turn **sequentially** (ADR-010 for parallel later).
6. No shell, `python_eval`, MCP client, or network product tool in the core package.

## What is NOT in core

- Shell / editor / browser / web-search packs
- MCP transport or full MCP tool object (icons, outputSchema, …)
- Auto-classify “pure” vs “side-effect” (documentation label only; gate still deny-all)
- Parallel scheduling of tool calls

## Replaceability test

1. Register tool A (e.g. echo) on `Registry`.
2. Register tool B with the same `name` / schema / `call` contract (different handler).
3. Host Allow list includes that name.
4. Run the same prompt through `Runner`.
5. **Pass:** loop and gate unchanged; only the registered `Tool` implementation changes.
