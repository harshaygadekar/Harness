# research_agent

**Status:** Works without core change.

## Goal

Multi-step tool use: search stub → observe → search again → final answer.

## How

1. Host `function_tool("search", ...)` over an in-memory dict (no network).
2. Fake `ModelClient` returns two `tool_calls`, then text.
3. `Agent` allowlists only `search`. `Runner` runs the ReAct loop.

## Missing seam?

None for multi-step research with host tools. Parallel tool calls in one turn
stay sequential (ADR-001 / ADR-010) — acceptable for this stub.

No event bus needed: message list + tool roles are enough.
