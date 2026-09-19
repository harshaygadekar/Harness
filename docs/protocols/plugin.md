# Plugin and registry protocol

**Module:** `agentic_harness.plugin`  
**Related:** ADR-001 (plugin + disposer); ADR-002 (model as plugin)

## Purpose

Let hosts add models and tools without forking the Runner. Registration is in-process. Teardown uses disposers so unload does not leave stale handlers.

## Required surface

### `Plugin` (Protocol)

| Member | Type | Notes |
|--------|------|-------|
| `name` | `str` | Plugin id |
| `version` | `str` | Semver-style string |
| `register` | `(registry: Registry) -> Disposer` | Must return a disposer |

`Disposer = Callable[[], None]`

### `Registry` (dataclass)

| Member | Role |
|--------|------|
| `register_tool(tool) -> Disposer` | Unique tool names; duplicate → `ValueError` |
| `register_model(model) -> Disposer` | Sets `registry.model` |
| `add_plugin(plugin) -> Disposer` | Calls `plugin.register`; tracks disposer |
| `get_tool(name) -> Tool \| None` | Lookup for Runner |
| `list_specs() -> list[ToolSpec]` | Schemas for `ModelClient.complete` |
| `dispose_all()` | Runs tracked disposers (LIFO) |
| `model` | Active model client or `None` |

## Invariants

1. Tools registered here still run **only** via Runner + gate (C2).
2. Plugins share the host process — **full trust** in v0; no isolation claim (Jessie C4).
3. No remote plugin install in core.
4. Disposer must undo registration (tool pop / clear model) when invoked.
5. Optional lifecycle hooks (`on_step`, …) are **not** required on the Protocol today; do not treat them as frozen core API until they exist in code.

## What is NOT in core

- Cordis-style service bus / HMR
- Policy packs as mandatory core plugins (policy is the `can_use_tool` hook + host code)
- Provider SDK inside Registry
- Multi-agent crew orchestration

## Replaceability test

1. Plugin A registers model + tool set.
2. Plugin B registers a different model (or tools) with the same `Plugin` contract.
3. `registry.add_plugin(B)` (after dispose of A, or on a fresh Registry).
4. `Runner(registry).run(...)` unchanged.
5. **Pass:** no edits to `runner.py`; only registration changes.
