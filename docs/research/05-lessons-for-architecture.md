# Lessons for architecture (ADR-001 input)

Research only. No architecture lock. For Walter’s ADR-001 (core loop).

**How ADR-001 is driven:** Walter drafts the ADR from evidence in this pack. **DeepSeek Harness** (`06-deepseek-harness.md`) is the **primary reference architecture** (plugin model, swappable loop, tool pipeline, session log). Comparative research (`01`–`04`) constrains weight and non-goals. **Do not call the DeepSeek LLM API (or any LLM API) to propose designs.** No API key is required.

---

## 0. Lessons from DeepSeek Harness (primary)

1. **Everything is a plugin — including the agent loop.** Default driver implements an interface; config can replace it. ADR-001 should name a small loop interface + one default implementation.
2. **Services by name (`ctx.tools`, `ctx.llm`), not concrete imports.** Dependency via `inject`; unload when a dependency vanishes.
3. **Registrations are reversible effects.** Tool/adapter/listener register returns a disposer; unload cleans up. Our `Plugin` lifecycle needs teardown from day one.
4. **Session / event log is model-visible truth.** Anything the model sees must be reconstructable from the log (or equivalent message history).
5. **Policy at the tool boundary.** `tools/pre-execute` waterfall + monotonic deny guards — aligns with deny-by-default; keep policy out of tool bodies.
6. **Compose with profiles/bundles, not source forks.** Minimal vs Standard are stacked configs.
7. **Steal ideas, not the TypeScript tree.** Cordis/dsh are large and preview-unstable; keep our core small (Python-friendly, dependency-light).

Cite: https://github.com/deepseek-ai/deepseek-harness ; https://arxiv.org/abs/2608.25512 ; docs under deepseek-harness.github.io.

---

## 1. Engineering lessons (backed by comparisons)

1. **The useful unit is the ReAct/tool loop**, not a graph DSL. Graphs help complex control flow (LangGraph) but are optional for the default agent (smolagents, mini-SWE-agent, PydanticAI `run()`).

2. **Split packages early.** OpenHands: sdk / tools / workspace / agent_server. PydanticAI-slim + extras. Prevents “install the universe.”

3. **Document “own the loop.”** OpenAI Agents docs tell advanced users to use the Responses API directly. A healthy harness allows bypass.

4. **Linear history is a feature.** mini-SWE-agent: trajectory == messages. Easier debug, fine-tune, and eval.

5. **Independent actions sandbox better than sticky shells.** mini-SWE-agent: each action like `subprocess.run`; swap to `docker exec`. SWE-agent’s persistent shell was harder.

6. **Tools are data + function.** Name, description, schema, call. MCP proves the wire shape; core should match that shape without requiring MCP.

7. **Memory beyond the context window is a product.** Letta. Do not invent a memory OS in v0 core.

8. **Multi-agent is composition of single agents.** Prefer agents-as-tools or child runs (OpenAI) over a mandatory crew runtime (CrewAI Process).

9. **Permissions belong at the tool boundary.** Claude `canUseTool` / permissionMode; OpenHands security policies. Core exposes a hook; policy packs are plugins.

10. **Token cost dominates framework CPU.** Optimize context (condensers, progressive tool discovery) before micro-optimizing dispatch.

11. **Dynamic multi-agent workflows burn tokens.** Anthropic “A harness for every task” — use for hard tasks, not default path.

12. **ACI / rich tools help some domains; bash-only can still win.** SWE-agent paper vs mini-SWE-agent FAQ: less scaffold, more model.

---

## 2. Explicit candidates for CORE

| Candidate | Rationale |
|-----------|-----------|
| `Runner.run` / async run | Universal entry |
| Step loop with `max_steps` | Prevents runaway |
| Cancel / abort propagation | Interactive + servers |
| `ModelClient` protocol | Provider-agnostic |
| `Tool` protocol + in-process registry | MCP-shaped, local default |
| Message list **or** step log + projector | Proven state shapes |
| Append tool errors as observations | Default resilience |
| `Plugin.register` + lifecycle + **teardown/disposers** | Extension without forks (dsh effect discipline) |
| Structured `RunResult` (status, messages, usage, error) | Host integration |

Keep core **dependency-light** (stdlib + typing; pydantic optional if it clarifies schemas).

---

## 3. Explicit candidates for PLUGIN (v0+)

| Plugin | Notes |
|--------|-------|
| Provider adapters | OpenAI, Anthropic, local, LiteLLM |
| MCP client | tools/resources/prompts → Tool |
| Workspace / sandbox | local, Docker, remote |
| Checkpointer | sqlite, redis, memory |
| Condenser / history trim | token control |
| Memory / Letta-like blocks | optional product |
| Observability exporter | OTel |
| Multi-agent orchestrator | handoff, crew, graph |
| Guardrail pack | sync + LLM |
| Reference tool packs | fs, shell, http |
| CLI / HTTP façade | apps |

---

## 4. Open questions / disagreements to escalate

Escalate to Thomas + Walter (and eng) before ADR-001 freeze:

1. **Messages vs Events as canonical store?**  
   - Messages: simpler, provider-native (mini-SWE-agent).  
   - Events: richer audit (OpenHands).  
   - Hybrid: events with `to_messages()`.

2. **Include pydantic in core?**  
   - Pro: schemas, validation (PydanticAI).  
   - Con: weight; TypedDict may suffice (LangGraph perf note on Pydantic state).

3. **Ship any built-in tools in the core package?**  
   - Zero tools = purest.  
   - One `echo`/`python_eval` for tests only?

4. **Async-only vs sync API?**  
   - Ecosystem is async-first (OpenAI Runner, PydanticAI, Claude SDK).  
   - Sync wrappers are cheap (`run_sync`).

5. **Native parallel tool calls in v0?**  
   - Models already return parallel calls; skipping wastes latency (LLMCompiler evidence).

6. **How MCP-compatible must the Tool schema be on day one?**  
   - Recommend: JSON Schema args + string/content result, so MCP adapter is thin.

7. **Graph API in-repo or never?**  
   - Option A: never — users compose with external LangGraph.  
   - Option B: optional `harness-graph` plugin later.  
   - Recommend: not in ADR-001 core.

8. **Default stop semantics**  
   - Final text with no tool calls?  
   - Explicit `finish` tool?  
   - Both (smolagents final answer vs provider end_turn)?

9. **License / third-party reuse**  
   - Prefer MIT/Apache patterns (OpenHands, LangGraph, smolagents, mini-SWE-agent). Avoid copying Claude Agent SDK internals (commercial terms).

10. **Name collision**  
   - “Harness” vs Anthropic “harness” marketing. Keep internal name; do not imply Claude Code compatibility.

---

## 5. Suggested ADR-001 shape (outline only)

Walter drafts this ADR from research evidence. Content not decided here:

- Context: need minimal agentic harness; plugins for the rest. **Primary reference: DeepSeek Harness** (everything-is-a-plugin, Cordis paper); comparative pack for lean bounds.
- Decision: core = loop **interface** + default driver + tool protocol + model protocol + plugin lifecycle (with teardown) + hooks.
- Non-goals: list from `04-keep-out-of-core.md` (no graph DSL, memory OS, built-in shell in core).
- Consequences: slim install; adapters for MCP/sandbox/memory later; profiles/packs compose like dsh bundles.
- Alternatives considered: LangGraph-first, Claude Agent SDK wrap, CrewAI-like roles, vendoring full dsh — rejected for weight / language / product mismatch (cite this pack + `06`).
- Explicit non-method: **no LLM API call to invent architecture.**

---

## 6. Suggested next step for Walter

1. Read `06-deepseek-harness.md` first, then `01`–`04` and `sources.md`.
2. Draft **ADR-001: Core loop** from DeepSeek Harness lessons + candidate tables above — **by hand from evidence, no API key**.
3. Spike a **≤200 LOC** prototype Runner (no MCP, no sandbox) against one model provider.
4. Measure: cold import time, deps count, one-tool ReAct latency vs raw SDK loop.
5. Bring spike + open questions list to Thomas for decision — do not expand core until spike passes.
