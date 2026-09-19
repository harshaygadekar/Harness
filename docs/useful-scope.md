# Useful scope (product lock)

**Owner:** Alice  
**Date:** 2026-09-19  
**Source:** `docs/research/` pack — primary design ref `06-deepseek-harness.md` (DeepSeek Harness + Cordis paper); comparative pack `02`/`04`/`05`  
**Status:** Locked for ADR-001. Not an ADR. Not code.  
**Correction:** DeepSeek = harness architecture inspiration. Not a DeepSeek API key. Gemini key later for tests only.

---

## What “useful” means here

A stranger can:

1. Install a small core.
2. Call one model.
3. Run one tool loop.
4. Stop safely (`max_steps` or cancel).
5. Add one plugin without forking the core.

If they need a graph, a crew, MCP, Docker, RAG, or a memory OS on day one, they add a plugin. They do not get a fat core.

---

## First useful demo (must work)

One script:

- One model provider adapter (plugin, not core).
- One user-supplied tool (function in-process).
- `Runner.run` until final text or `max_steps`.
- Print `RunResult` (status, messages, usage, error).

No shell. No `python_eval`. No MCP. No server. No multi-agent.

---

## In scope for v0 (core + one thin adapter)

| In | Why |
|----|-----|
| Runner loop | Every useful agent needs it |
| ModelClient protocol | Provider-agnostic call |
| Tool protocol + registry | Name, schema, call |
| Message or step history | Next model call |
| `max_steps` + cancel | Stop runaway |
| Hooks / `Plugin.register` | Extend without forks |
| Structured `RunResult` | Host can integrate |

---

## Out of scope (do not put in core)

From research `04-keep-out-of-core.md`. Do not reopen without evidence:

- Memory OS / Letta-like stores
- MCP runtime
- Observability backends
- Checkpointers / durable runs
- Multi-agent crews / graphs
- RAG / vector DB
- Sandbox / Docker / remote workspace
- Built-in shell, editor, browser, web search
- Guardrail LLM packs
- Planning / reflection workflows
- HTTP / UI / CLI as core
- Skills / markdown prompt loaders as core

**From DeepSeek Harness (take ideas, do not vendor Cordis/`dsh`):**

- Capability seams: swap model and tools by registration.
- Tool pipeline: pre-execute → execute → post-execute; deny-by-default at the gate.
- Plugin teardown (disposers) so unload does not leak handlers.
- Keep a tiny default loop **in core** (we reject “loop only as a removable plugin” for v0 install size).

**Hard kills for core (Jessie + product):**

- No `python_eval` in core.
- No shell in core.
- No side-effect tool without a deny-by-default gate at the tool boundary (hook in core; policy pack is a plugin).
- Do not paste API keys in chat. Gemini key only when Mike has a runner to test.

---

## Success metrics (useful, not vanity)

| Metric | Target for first cut |
|--------|----------------------|
| Cold import of core | Measured; publish number (Mike) |
| Core dependency count | Stdlib + typing; pydantic only if ADR proves need |
| First demo lines outside adapters | Small; prefer &lt;50 lines of user code |
| Time to first tool observation | Measured vs raw SDK loop |
| “Can I add a plugin without editing core?” | Yes / no test in CI |

Stars, follower count, and feature parity with LangGraph or CrewAI are not success.

---

## Public docs order (Alice)

After ADR-001 is selected:

1. `README.md` — install, one demo, non-goals.
2. `ARCHITECTURE.md` — core vs plugins (from research + ADR).
3. `ROADMAP.md` — plugins only; no core bloat.
4. `CONTRIBUTING.md` / `SECURITY.md` / `BENCHMARKS.md` with Mike and Jessie.

Do not publish a feature list that research put in plugins.

---

## Kill / reopen rules

- If ADR-001 adds a graph DSL, memory OS, or built-in shell to core → Alice veto. Escalate to Chief.
- If the first demo needs more than loop + model + one tool → scope failed. Cut until it does not.
- Walter must record tradeoffs in the ADR (evidence from research, DeepSeek Harness as design reference). We do not freeze on vibe. No LLM API for ADR decisions.
