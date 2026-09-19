# Protocol contracts (current freeze)

**Audience:** Phase 2 stress-app authors, Mike (impl), Thomas (primitives)  
**Date:** 2026-09-19 (IST)  
**Status:** Documentation of the **current** public surface. See [ADR-003](../decisions/ADR-003-protocol-freeze.md) (Proposed).  
**Rule:** Do not grow core. Swap implementations at these seams. Do not edit the Runner loop to add a new primitive.

These pages describe types that exist in `src/agentic_harness/` today. They are contracts for replaceability tests, not a feature roadmap.

## Index

| Surface | Module | Doc |
|---------|--------|-----|
| Model | `model.py` — `ModelClient`, `ModelResponse` | [model.md](model.md) |
| Tool | `tool.py` — `Tool`, `ToolSpec`, `ToolResult`, `FunctionTool` | [tool.md](tool.md) |
| Message | `messages.py` — `Message`, `ToolCall`, `Usage`, `Role` | [message.md](message.md) |
| Plugin | `plugin.py` — `Plugin`, `Registry` | [plugin.md](plugin.md) |
| Runner | `runner.py` + `gate.py` + `result.py` | [runner.md](runner.md) |
| Agent (façade) | `simple.py` — `Agent` | [agent.md](agent.md) |

## Related core pieces (covered under Runner)

| Piece | Module | Note |
|-------|--------|------|
| Gate | `gate.py` — `Allow`, `Deny`, `CanUseTool`, `deny_all` | Host policy hook; default deny-all |
| Result | `result.py` — `RunResult`, `RunStatus`, `SafeError` | Return value of `Runner.run` |
| Sanitize | `sanitize.py` | Helper for safe tool/error text; not a protocol |

## Not in core today

- **Events / typed step log** — not a public core type. History is a message list. See research brief 1 and 7 in `docs/research/07-open-adr-briefs.md`.
- **Streaming** on `ModelClient` — open in ADR-002; not required yet.
- **Async `run`** — sketch in ADR-001; sync `Runner.run` is what ships.
- **MCP, shell, sandboxes, memory, OTel** — plugins / packs only.
- **`simple.Agent`** — DX façade above Registry + Runner. Not a new core primitive (Mike).

## Replaceability rule

For each surface: implement A, then implement B with the same contract. Register B instead of A. The Runner loop must not change. If you must edit `runner.py` to swap A→B, the contract failed — file a missing-primitive with a failing example under `examples/` (ADR-003).

## Decisions

- [ADR-001](../decisions/ADR-001-core-loop.md) — core loop (Accepted)
- [ADR-002](../decisions/ADR-002-model-adapter.md) — model adapters as plugins (Accepted)
- [ADR-003](../decisions/ADR-003-protocol-freeze.md) — freeze this set for Phase 2 (Proposed)
