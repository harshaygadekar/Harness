# 09 — Disprove the tiny core (Phase 2 apps)

**Owner:** Thomas  
**Date:** 2026-09-19 (IST)  
**Rule:** Do not edit Runner / gate / model / tool / plugin core files. Use
`agentic_harness.simple.Agent` and existing APIs only. Add under `examples/`
and `docs/research/` only.

**Bar for “missing primitive”:** The app cannot work by swapping ModelClient,
Tool, Plugin, or gate alone. Convenience gaps are not missing primitives.

---

## Apps tried

| App | Path | Result |
|-----|------|--------|
| simple | `examples/simple_agent/` | Works |
| tool | `examples/tool_agent/` | Works |
| structured | `examples/structured_agent/` | Works (host JSON parse) |
| streaming | `examples/streaming_agent/` | **Wall** — see below |
| research | `examples/research_agent/` | Works (multi-step search stub) |
| coding stub | `examples/coding_agent/` | Works (host list/read tools) |

---

## What worked without core change

1. **simple_agent** — Fake model, no tools, final assistant text.
2. **tool_agent** — One `FunctionTool`, allowlist gate via `Agent`, one tool
   round then answer.
3. **structured_agent** — Prompt asks for JSON; fake model returns fenced JSON
   in `Message.content`; host parses with `json` + a small helper. Optional
   tool that returns a JSON string also works. No pydantic / schema validator
   in core required.
4. **research_agent** — Two sequential search tool calls + final summary.
   In-memory stub index. Message list is enough history (no event bus).
5. **coding_agent** — Host `list_files` / `read_file` over
   `workspace_sample/`. Path escape check in the tool. No shell in core. Fake
   model drives list → read → answer.

All of the above use only: `ModelClient.complete`, `FunctionTool`,
`Agent` / Runner ReAct loop, deny-all + allowlist, `RunResult.messages`.

---

## What hit a wall

### Streaming tokens (host-visible mid-call)

- **Example:** `examples/streaming_agent/`
- **Status:** NOT-POSSIBLE through `Agent.run` / `Runner.run` today.
- **Cause:** `ModelClient` exposes only sync `complete → ModelResponse`.
  Runner waits for the full assistant message each step. No
  `complete_stream`, no token iterator, no Runner progress hook / event for
  text deltas (`docs/protocols/model.md`, ADR-003 “not frozen as core”).
- **Workaround that is not enough:** Stream inside the adapter, then return
  one buffered `Message`. Host still sees nothing until `complete` returns.
- **Not a Phase 2 product block:** Coding/research stubs do not need TTY
  token drip.

### Async Runner

- Not required by these apps. Sync `run` works for scripts.
- Open brief in `07-open-adr-briefs.md` §3. No failing product path filed here.

### Events / step log

- Research and coding stubs use the message list. Enough for replay of tool
  rounds in these examples.
- Typed event bus stays out of core (brief §1 / §7).

### Structured parse in core

- Host parse works. Core schema validation would be convenience / weight, not
  a missing loop primitive (brief §2).

### Parallel tool calls

- Research stub uses sequential calls. ADR-010 remains deferred. Not filed as
  a missing primitive from these apps.

---

## Missing primitive?

**None for Phase 2.**

Streaming is a real UX wall for `streaming_agent`, but it does **not** meet
the ADR-003 bar to unfreeze core yet:

- Product apps (structured / research / coding stub) work at existing seams.
- A `complete_stream` (or Runner delta hook) would be a **new** Model/Runner
  surface, not a ModelClient/Tool/Plugin/gate swap — so it *could* become a
  precise missing primitive later.
- File it only with a product need (e.g. live coding UX) and keep
  `examples/streaming_agent/` as the failing path.

---

## Recommendation

**Keep the protocol freeze (ADR-003).**

Do not grow core for streaming, async, events, or pydantic based on this
pass. Grow only if a later product app cannot ship without a new seam; cite
a failing `examples/` path and open a Walter ADR.

---

## Paths added

- `examples/structured_agent/main.py` + `README.md`
- `examples/streaming_agent/main.py` + `README.md` (NOT-POSSIBLE note)
- `examples/research_agent/main.py` + `README.md`
- `examples/coding_agent/main.py` + `README.md` + `workspace_sample/`
- `examples/simple_agent/README.md`, `examples/tool_agent/README.md`
- `docs/research/09-disprove-core.md` (this file)
