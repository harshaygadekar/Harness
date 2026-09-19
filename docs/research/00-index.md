# Research pack: minimal agentic harness

**Audience:** Thomas (AI/ML Research Lead), Walter (ADR-001 core loop)  
**Project goal:** A minimal, useful, fast, extensible agentic harness. Core stays small. Everything else is a plugin.  
**Date:** 2026-09-19 (IST)  
**Status:** Research pack updated. DeepSeek Harness is primary design reference. ADR-001 draft at `docs/decisions/ADR-001-core-loop.md` (awaiting review).

## Correction (Harsha)

**“DeepSeek” means the DeepSeek agent harness** (everything-is-a-plugin architecture) and its Cordis paper — take design inspiration from that release.  
It does **not** mean calling the DeepSeek LLM API for architecture decisions. **No API key is required.**  
See [06-deepseek-harness.md](06-deepseek-harness.md). Walter drafts ADR-001 from evidence in this pack.

## Documents

| File | Purpose |
|------|---------|
| [01-ecosystem-map.md](01-ecosystem-map.md) | Systems studied: purpose, language, license, abstractions, weight, plugins, core vs optional |
| [02-core-primitives.md](02-core-primitives.md) | What every useful agent needs; smallest loop; tool/plugin/message/failure surfaces |
| [03-overhead-sources.md](03-overhead-sources.md) | Latency, memory, dependency, and token overhead sources |
| [04-keep-out-of-core.md](04-keep-out-of-core.md) | Features that must stay out of core; plugin boundaries |
| [05-lessons-for-architecture.md](05-lessons-for-architecture.md) | Engineering lessons for ADR-001; candidates and open questions |
| [06-deepseek-harness.md](06-deepseek-harness.md) | **Primary reference:** DeepSeek Harness + Cordis paper — abstractions, plugins, loop, tools, lessons |
| [sources.md](sources.md) | URLs and papers used |

## One-paragraph status

Across OpenAI Agents SDK, Anthropic Claude Agent SDK, LangGraph, PydanticAI, smolagents, OpenHands, SWE-agent / mini-SWE-agent, AutoGen/AG2, CrewAI, Letta, MCP, and **DeepSeek Harness (`dsh`)**, the recurring useful agent is a small loop: model call → parse tool calls → execute tools → append observations → stop or continue. DeepSeek Harness makes that explicit: models, tools, sessions, sandboxes, storage, **and the agent loop itself** are Cordis plugins composed by profile/bundle/patch — backed by the Cordis paper on spatiotemporal composability (arXiv:2608.25512). Heavy frameworks add graphs, multi-agent crews, memory OS layers, sandboxes, MCP hosting, observability, and persistence around the loop. Lean systems (smolagents MultiStepAgent, mini-SWE-agent ~100-line loop, OpenAI’s “own the loop,” OpenHands’ package split, dsh Minimal mode) show the core can stay thin if tool registries, transports, memory, and orchestration are plugins. This pack recommends a tiny core (run loop interface + default driver, tool protocol, message/state or session log, cancel/error hooks, plugin registry with teardown) and lists what to keep out. **Walter drafted ADR-001 from `05` + `06` (DeepSeek Harness as primary reference) without an API key. Review pending before code.**

## Scope notes

- No application source under `src/`.
- ADR-001 draft lives under `docs/decisions/` (not in this research folder).
- Some AG2 / Letta deep pages returned 404 or empty GitHub blobs; findings use pages that fetched cleanly plus secondary confirmed docs.
- MCP architecture page fetched successfully (tools, resources, prompts; JSON-RPC; stdio / Streamable HTTP).
- DeepSeek Harness docs and GitHub fetched 2026-09-19 (IST); Cordis paper cited via arXiv abs.
