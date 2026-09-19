# Core primitives

Question: What does every useful agent actually need?

Method: Compare loops and interfaces across systems in `01-ecosystem-map.md`. Prefer official docs over opinion.

---

## 1. Smallest useful agent loop

Observed pattern (ReAct / tool loop), documented in smolagents and vendor SDKs:

1. Build messages (system + user + prior steps).
2. Call the model (optionally with tool schemas).
3. If the model returns a final answer → stop.
4. If the model returns tool call(s) → execute → append observation(s).
5. Repeat until stop, max steps, cancel, or error.

Evidence:

- smolagents MultiStepAgent: memory → messages → model → parse action → execute → ActionStep → callbacks; stop on final answer or max_steps (HF docs, ReAct guide).
- OpenAI Agents SDK Runner: manages turns, tools, handoffs, sessions; docs say own the loop via Responses API if you do not want Runner.
- Anthropic Agent SDK: same loop/tools/context as Claude Code; `query()` streams the loop.
- PydanticAI graph: UserPromptNode → ModelRequestNode → CallToolsNode → End.
- mini-SWE-agent: linear history; step = query model + execute action; ~100 lines claimed in project docs.
- OpenHands SDK: Agent reason–act; Conversation holds state; Tool Action → Observation.

**Harness implication:** Core = this loop. Graphs, crews, and memory OS are wrappers.

---

## 2. Tool interface (minimum)

Common fields across SDKs and MCP:

| Field | Role |
|-------|------|
| `name` | Stable id for the model |
| `description` | When to use |
| `parameters` / JSON Schema | Arguments |
| `call(args) → result` | Side effect + observation text/structured payload |

Evidence:

- OpenAI function tools / `@function_tool`.
- PydanticAI `@agent.tool` + schema from types.
- smolagents `Tool` class.
- OpenHands Action / Observation / Executor triad.
- MCP `tools/list` + `tools/call` with `inputSchema`.

**Optional on tools (not core):** permission gates, risk scores, sandbox binding, MCP transport, retries, caching.

**Harness implication:** Define a tiny Tool protocol in core. Adapters (MCP, HTTP, local fn, bash) are plugins.

---

## 3. Plugin interface (minimum)

What “plugin” means in studied systems:

| System | Extension surface |
|--------|-------------------|
| OpenAI Agents | tools, handoffs, hooks, MCP servers, sandbox capabilities |
| Claude Agent SDK | MCP, hooks, subagents, plugins (skills/agents/hooks/MCP paths) |
| LangGraph | nodes, checkpointers, store, subgraphs |
| PydanticAI | toolsets, capabilities, deps, model providers |
| smolagents | tools, managed_agents, callbacks, executors |
| OpenHands | tools package, workspace backends, skills, condensers |
| MCP | servers exposing tools/resources/prompts |

**Minimal plugin contract for our harness (research proposal, not ADR):**

- `name` + `version`
- `register(registry)` — attach tools and/or lifecycle hooks
- Optional: `on_start` / `on_step` / `on_end` / `on_error`
- No required network, DB, or UI

---

## 4. Message / state shape

Two proven shapes:

**A. Linear message list** (chat / Responses style)

- Roles: system, user, assistant, tool (or equivalent).
- Tool calls carry `id` + `name` + `arguments`.
- Tool results reference `tool_call_id`.
- Used by: mini-SWE-agent, OpenAI/Anthropic raw loops, PydanticAI message history between runs.

**B. Step / event log** (then projected to messages)

- smolagents: typed steps in AgentMemory, then `write_memory_to_messages()`.
- OpenHands: typed Events on Conversation.
- LangGraph: state channels + reducers (e.g. `add_messages`).

**Harness implication:**

- Store a **canonical step/event list** *or* a **message list**.
- If you store steps, provide a pure function `steps → messages` for the model.
- Do not require LangChain message classes or provider-specific types in core.
- Keep state serializable (JSON-friendly) so persistence can be a plugin.

Minimum run state fields:

- `messages` or `steps`
- `status`: running | succeeded | failed | cancelled
- `step_count` / `max_steps`
- `usage` (optional counters)
- `error` (optional)

---

## 5. Failure and cancellation

Observed needs:

| Concern | Examples in ecosystem |
|---------|----------------------|
| Max steps / recursion limit | smolagents `max_steps`; LangGraph `recursion_limit`; CrewAI `max_iter` |
| Tool errors | Return error string as observation (common); or raise and stop |
| Model errors | Retry policies (LangGraph, PydanticAI); surface to caller |
| Cancel / abort | PydanticAI `CancellationToken` / `RunCancelled`; Runner stop; Claude interrupt |
| Permissions deny | Claude `canUseTool` / permissionMode; OpenHands security policies |
| Partial stream | Stream APIs leave partial assistant text; must not corrupt history |

**Harness implication (core):**

- Propagate cancellation (async cancel / abort flag).
- Bound steps (`max_steps`).
- Always append tool failures as observations unless policy says hard-stop.
- Do not bake retry/backoff strategy into core — hook or plugin.

---

## 6. What every useful agent needs (checklist)

Must have:

1. Model call boundary (provider-agnostic callable).
2. Tool registry + invoke.
3. Loop with stop conditions (final output, max steps, cancel).
4. Message/step history for the next model call.
5. Error/cancel surface to the host app.

Should not require in core:

- Graph DSL
- Multi-agent routing
- Vector DB / RAG
- MCP wire protocol
- Durable checkpointer
- Sandbox / Docker
- Observability backend
- Memory OS / archival store
- UI / HTTP server

---

## 7. What typical frameworks can remove

| Feature often in frameworks | Why removable from core |
|----------------------------|-------------------------|
| Role / backstory / crew process | Prompt + optional multi-agent plugin (CrewAI) |
| StateGraph / Pregel | Only needed for complex branching; loop covers ReAct (LangGraph is powerful but not minimal) |
| Built-in filesystem/shell tools | Domain tools; OpenHands splits `tools` package |
| MCP host | Protocol adapter (MCP docs: does not dictate agent use) |
| Memory blocks / archival | Letta specialty; plugin |
| Guardrails / planning / knowledge | Product features; hooks/plugins |
| Handoffs / hierarchies | Multi-agent plugin |
| Sandbox manifests | Execution plugin (OpenAI SandboxAgent, OpenHands workspace) |
| Tracing exporters | Observability plugin |

---

## 8. Strawman primitive set (research only)

Not an ADR. Names illustrative.

```
ModelClient.complete(messages, tools) -> ModelResponse
ToolSpec { name, description, schema }
ToolHandler.call(name, args, ctx) -> ToolResult
RunState { messages|steps, status, step_count }
Runner.run(agent, input, *, max_steps, cancel) -> RunResult
Plugin.register(registry)
Hooks: on_step, on_tool, on_error, on_end
```

Everything else attaches via Plugin or Hooks.
