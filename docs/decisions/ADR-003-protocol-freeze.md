# ADR-003: Protocol freeze for Phase 2

| Field | Value |
|-------|-------|
| **Status** | Proposed |
| **Date** | 2026-09-19 (IST) |
| **Owners** | Walter (draft), Thomas (approve missing-primitives), Mike (façade / stress apps), Alice (no core growth), Jessie (gate invariants hold) |
| **Related** | [ADR-001](ADR-001-core-loop.md) (Accepted — tiny core loop), [ADR-002](ADR-002-model-adapter.md) (Accepted — model adapters as plugins), [docs/protocols/](../protocols/README.md) |

---

## Problem

Phase 2 stress apps need a **stable** set of seams: model, tool, message, plugin/registry, runner (+ gate/result), and the optional `simple.Agent` façade.

If every stress-app gap becomes a core edit, the loop grows and ADR-001’s “tiny protocol core” fails. If we freeze forever with no escape hatch, real missing primitives block hosts.

## Decision

**Freeze the current protocol set for Phase 2 stress apps.**

1. Document the live contracts under `docs/protocols/` (this ADR’s companion docs). Documentation only — **do not** change core loop code to “match” the docs; docs follow code.
2. **Growth only when Thomas files a missing-primitive** with:
   - a short problem statement, and
   - a **failing example** under `examples/` that cannot be solved by swapping ModelClient / Tool / Plugin / gate at the existing seams.
3. Do **not** expand core for convenience APIs, provider quirks, MCP, graphs, events, or streaming until that bar is met (or a later ADR Accepted).
4. **Mike’s `simple.Agent`** is a **façade above core**, not a new core primitive. Stress apps may use it or call `Registry` + `Runner` directly. Façade bugs are fixed in `simple.py`, not by growing `Runner`.

### Frozen core surfaces (see protocol docs)

| Surface | Primary types |
|---------|----------------|
| Model | `ModelClient`, `ModelResponse` |
| Tool | `Tool`, `ToolSpec`, `ToolResult` (+ helper `FunctionTool`) |
| Message | `Message`, `ToolCall`, `Usage`, `Role` |
| Plugin | `Plugin`, `Registry` |
| Runner | `Runner`, `RunConfig`, `RunContext` |
| Gate | `Allow`, `Deny`, `CanUseTool`, `deny_all` |
| Result | `RunResult`, `RunStatus`, `SafeError` |

**Not frozen as core:** typed events, streaming model API, async Runner, MCP-in-core, shell-in-core.

**Façade (non-core):** `agentic_harness.simple.Agent`.

## Consequences

- Phase 2 authors implement against `docs/protocols/` and ADR-001/002.
- Replaceability tests (swap A→B without editing the loop) are the acceptance check for “still frozen.”
- Missing-primitive proposals cite a failing `examples/` path; Walter opens a follow-up ADR only after Thomas accepts the gap.
- Alice product lock (`docs/useful-scope.md`) still applies: no shell/MCP/multi-agent in core.

## Alternatives rejected

| Alternative | Why rejected |
|-------------|--------------|
| **Grow core per stress-app request** | Violates ADR-001 minimal core; dep and import cost rise. |
| **Hard freeze with no process** | Blocks real gaps; invites unofficial forks of `runner.py`. |
| **Promote `Agent` to core primitive** | Duplicates Registry+Runner; confuses façade with protocol. |

## Open

- Status remains **Proposed** until Thomas (+ Alice scope, Jessie gate) accept.
- Streaming / async / events stay on the research brief track (`07-open-adr-briefs.md`), not silent core adds.

## Review trail

- [Mike](ADR-003-mike-review.md) — Approve; façade stays non-core
- Thomas — Approve (in chat)
