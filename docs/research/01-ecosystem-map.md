# Ecosystem map

Evidence from official docs and project pages. License and weight noted only when verified. “Weight” means install/runtime surface, not marketing claims.

**Primary design reference:** DeepSeek Harness (`dsh`) — see §0 and `06-deepseek-harness.md`. Design inspiration only; not an LLM API call.

---

## 0. DeepSeek Harness (`dsh`) — PRIMARY REFERENCE

| Field | Evidence |
|-------|----------|
| **Purpose** | Open-source agent harness: every capability (models, tools, sessions, sandboxes, storage, **loops**, scheduling, UI) is a Cordis plugin. Developer preview. |
| **Language** | TypeScript / Node (`dsh` CLI); Python SDK launches the same runtime. |
| **License** | MIT (repo). |
| **Core abstractions** | Cordis: plugin + `inject` + `apply(ctx)`; services on `ctx` (`tools`, `llm`, `sessions`, `agents`, `agentLoop`); typed events; reversible effects; profiles / bundles / `cordis.patch.yml`; Fiber lifecycle + HMR. |
| **Dependency weight** | **Heavy product tree if you take full profiles; architecture idea is light.** `sdk-minimal` / Minimal mode shrink tools. Preview: breaking changes expected. |
| **Plugin / extensibility** | Everything is a plugin. Register tools/adapters via `ctx`; policy via `tools/pre-execute` + guards; swap loop via `core/agent-loop`; compose modes without forking source. |
| **Looks “core” (still plugins)** | session log, system-prompt assembly, tools registry, agent interface, default agent-loop, llm adapter seam. |
| **Looks optional / product** | Web UI, sandboxes, webhook, credentials, telemetry, skills, subagents, Code/Creator modes, desktop Electron. |

**Paper:** *A Programming Paradigm for Spatiotemporal Composability* (Cordis) — https://arxiv.org/abs/2608.25512

Sources: https://github.com/deepseek-ai/deepseek-harness ; https://deepseek.com/harness/en/ ; https://deepseek-harness.github.io/deepseek-harness/en/reference/ ; https://deepseek-harness.github.io/deepseek-harness/en/reference/cordis-primer

## 1. OpenAI Agents SDK

| Field | Evidence |
|-------|----------|
| **Purpose** | Multi-agent workflows: Agent + Runner manage turns, tools, guardrails, handoffs, sessions. |
| **Language** | Python (`openai-agents`). Also JS/TS variants in ecosystem. |
| **License** | MIT (PyPI / GitHub pyproject). |
| **Core abstractions** | `Agent` (name, instructions, tools, handoffs, guardrails, output_type, hooks, mcp_servers); `Runner` (`run` / `run_sync` / `run_streamed`); `RunConfig`; context DI; `SandboxAgent` + `Manifest` + Capabilities for workspace compute. |
| **Dependency weight** | **Medium–heavy.** Core pulls `openai`, `pydantic`, `mcp`, websockets, etc. Many optional extras (voice, redis, sqlalchemy, litellm, docker, s3…). Docs explicitly say: if you want to own the loop, use the Responses API directly. |
| **Plugin / extensibility** | Tools (decorators / function tools); handoffs; agents-as-tools; MCP servers; hooks (`RunHooks` / `AgentHooks`); sandbox capabilities; sessions. |
| **Looks core** | Agent definition, Runner loop, tool call handling, streaming result types. |
| **Looks optional** | Sandboxes, memory strategies, guardrails, MCP, durable execution, nested handoff history compaction. |

Sources: https://openai.github.io/openai-agents-python/agents/ ; https://openai.github.io/openai-agents-python/running_agents/ ; https://github.com/openai/openai-agents-python

---

## 2. Anthropic Agent SDK / Claude Agent tooling

| Field | Evidence |
|-------|----------|
| **Purpose** | Same tools, agent loop, and context management as Claude Code, as a Python/TypeScript library. |
| **Language** | Python (`claude-agent-sdk`), TypeScript (`@anthropic-ai/claude-agent-sdk`). Other languages: CLI `-p` + JSON. |
| **License** | Anthropic Commercial Terms (product); components may vary per LICENSE file. |
| **Core abstractions** | `query()` / `ClaudeSDKClient`; `ClaudeAgentOptions`; built-in tools (Read, Edit, Glob, Grep, Bash, …); permissions (`permissionMode`, `canUseTool`); sessions; hooks; subagents; MCP (`create_sdk_mcp_server`); plugins / skills / memory loaded from `.claude/`. |
| **Dependency weight** | **Heavy for a “SDK”.** TS SDK bundles a native Claude Code binary as optional dependency. Python package requires 3.10+. This is a full coding harness, not a thin loop. |
| **Plugin / extensibility** | MCP servers; hooks; subagents; plugins (skills, agents, hooks, MCP by path); permission callbacks. |
| **Looks core (to them)** | Agent loop + built-in tools + permissions + sessions. |
| **Looks optional / product-shaped** | Skills, memory files, plugins, managed agents (separate product), dynamic workflow JS harness (Claude Code blog). |

Sources: https://code.claude.com/docs/en/agent-sdk/overview ; https://code.claude.com/docs/en/agent-sdk/quickstart ; https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code ; Anthropic engineering: effective harnesses / harness design for long-running agents.

---

## 3. LangGraph

| Field | Evidence |
|-------|----------|
| **Purpose** | Stateful agent workflows as graphs (or functional entrypoints) with durable execution. |
| **Language** | Python and JS/TS. |
| **License** | MIT (langchain-ai/langgraph). |
| **Core abstractions** | `State` (+ reducers); `Nodes`; `Edges` / conditional edges; `StateGraph.compile()`; Pregel runtime (super-steps, channels); `Command`, `Send`; checkpointers; interrupts; `MessagesState` / `add_messages`. |
| **Dependency weight** | **Medium–heavy.** Graph runtime + LangChain message types often pulled in practice; checkpointers, cache, and LangSmith are adjacent. Docs note Pydantic state is less performant than TypedDict/dataclass. |
| **Plugin / extensibility** | Custom nodes/edges; checkpointer backends; store; cache policies; subgraphs; Functional API `@entrypoint`. |
| **Looks core** | State channels + reducers + node execution + routing + compile. |
| **Looks optional** | Persistence, HITL interrupts, LangSmith tracing, map-reduce `Send`, multi-agent subgraphs. |

Sources: https://docs.langchain.com/oss/python/langgraph/graph-api ; https://docs.langchain.com/oss/python/langgraph/pregel

---

## 4. PydanticAI

| Field | Evidence |
|-------|----------|
| **Purpose** | Type-safe agent interface: instructions, tools/toolsets, deps, structured output. |
| **Language** | Python. |
| **License** | MIT (ecosystem packaging includes `pydantic-ai-slim`). |
| **Core abstractions** | `Agent[DepsT, OutputT]`; tools / toolsets / capabilities; `RunContext`; run APIs (`run`, `run_sync`, stream, `iter`); underlying graph nodes (UserPrompt → ModelRequest → CallTools → End) via pydantic-graph; `CancellationToken` / `RunCancelled`. |
| **Dependency weight** | **Light–medium if slim.** `pydantic-ai-slim` keeps core deps small; providers are extras. Full `pydantic-ai` adds more. |
| **Plugin / extensibility** | Tools, toolsets, capabilities, deps injection, model wrappers, agent specs (YAML/JSON), hooks/events. |
| **Looks core** | Agent container, typed deps/output, tool calling loop, message history handoff between runs. |
| **Looks optional** | Graphs for multi-agent, durable execution capabilities, Logfire/OTel, MCP integrations, RAG-like knowledge elsewhere. |

Sources: https://pydantic.dev/docs/ai/core-concepts/ ; https://pydantic.dev/docs/ai/graph/graph/

---

## 5. smolagents (Hugging Face)

| Field | Evidence |
|-------|----------|
| **Purpose** | Small library for multi-step agents (ReAct), especially CodeAgent. |
| **Language** | Python. |
| **License** | Apache-2.0. |
| **Core abstractions** | `MultiStepAgent` (ReAct while-loop); `CodeAgent` / `ToolCallingAgent`; `Tool`; `Model`; `AgentMemory` steps (`SystemPromptStep`, `TaskStep`, `ActionStep`, `PlanningStep`); `managed_agents`; step callbacks; optional planning interval. |
| **Dependency weight** | **Light core; heavier with extras.** Docs push few-line agents; sandboxes (E2B, Docker, Modal…) and transformers/litellm are optional. |
| **Plugin / extensibility** | Tools from Hub/MCP/LangChain; managed agents as tools; callbacks; executor backends. |
| **Looks core** | Multi-step loop, memory→messages, parse action, execute, stop on final answer / max_steps. |
| **Looks optional** | Planning steps, Hub sharing, Gradio, sandbox executors. |

Sources: https://huggingface.co/docs/smolagents/en/conceptual_guides/react ; https://huggingface.co/docs/smolagents/en/reference/agents ; ReAct paper Yao et al. 2022.

---

## 6. OpenHands (Software Agent SDK)

| Field | Evidence |
|-------|----------|
| **Purpose** | Composable SDK for software agents; applications stay separate. |
| **Language** | Python packages. |
| **License** | MIT (software-agent-sdk LICENSE). |
| **Core abstractions** | Four packages: `openhands.sdk` (Agent, Conversation, LLM, Tool Action/Observation/Executor, Events, Workspace bases, Skill, Condenser, Security); `openhands.tools`; `openhands.workspace`; `openhands.agent_server`. Local vs Docker/remote modes. |
| **Dependency weight** | **Modular: light local, heavy production.** Docs: local = sdk + tools; production adds workspace + agent_server. Explicit split is the lesson. |
| **Plugin / extensibility** | Tools as Action/Observation/Executor; workspace swap; MCP in tool system; skills; security policies. |
| **Looks core** | Reasoning-action loop, Conversation state, LLM interface, typed events/tools, base Workspace. |
| **Looks optional** | Prebuilt tools, Docker/remote workspaces, FastAPI agent server, condensers, desktop/VSCode endpoints. |

Sources: https://docs.openhands.dev/sdk/arch/overview ; arXiv:2407.16741 (OpenHands platform paper).

---

## 7. SWE-agent / mini-SWE-agent

| Field | Evidence |
|-------|----------|
| **Purpose** | Coding agents for SWE tasks; SWE-agent pioneered ACI; mini-SWE-agent is the lean successor. |
| **Language** | Python. |
| **License** | MIT (SWE-agent). |
| **Core abstractions (SWE-agent)** | CLI → `Agent` + `SWEEnv`; SWE-ReX deployment (Docker/cloud); shell session + ACI tools; `forward()`; `HistoryProcessor`; action parser. **Maintenance-only;** docs push mini-SWE-agent. |
| **Core abstractions (mini-SWE-agent)** | ~100-line agent + env + model; linear message history; bash-centric actions; `subprocess.run`-style independent actions; model/env duck typing; optional native tool calling in v2. |
| **Dependency weight** | SWE-agent: **heavy** (Docker/SWE-ReX). mini: **very light**. |
| **Plugin / extensibility** | YAML config / tool bundles (SWE-agent); swap Model/Environment (mini). |
| **Looks core** | Prompt → action → env execute → append observation → repeat. |
| **Looks optional** | Rich ACI tool UIs, history processors, multi-cloud deployments. |

Sources: https://swe-agent.com/latest/background/architecture/ ; https://mini-swe-agent.com/latest/ ; Yang et al. SWE-agent (NeurIPS 2024).

---

## 8. AutoGen / AG2

| Field | Evidence |
|-------|----------|
| **Purpose** | Multi-agent conversation / network runtime. Microsoft Agent Framework is positioned as AutoGen’s successor in MS docs; AG2 is the community continuation (`ag2ai/ag2`). |
| **Language** | Python. |
| **License** | Check current AG2 repo (historically open; verify before shipping). |
| **Core abstractions (classic AutoGen)** | Layered stack: AgentChat over Core; async message passing; standalone or distributed runtime (Microsoft AutoGen docs). |
| **Core abstractions (AG2 direction)** | `Agent` / `Agent.ask`; `ag2.network` Hub (store, registry, audit, WAL); channels (consulting/conversation/discussion/workflow); adapters. AG2 Classic (`ConversableAgent`, GroupChat) in maintenance mode per community materials. |
| **Dependency weight** | **Medium–heavy** once network/Hub/persistence enter. |
| **Plugin / extensibility** | Channel adapters; Hub links (LocalLink / WsLink); tools on agents. |
| **Looks core** | Message-passing agent + runtime. |
| **Looks optional** | Group chat patterns, distributed Hub, A2A, observability. |

**Blocker note:** Several AG2 doc URLs returned 404 during this pack; treat AG2 details as provisional pending a live docs pass.

Sources: https://microsoft.github.io/autogen/stable/user-guide/core-user-guide/core-concepts/application-stack.html ; https://learn.microsoft.com/en-us/agent-framework/overview/ ; https://github.com/ag2ai/ag2

---

## 9. CrewAI

| Field | Evidence |
|-------|----------|
| **Purpose** | Role-based multi-agent crews and higher-level Flows. |
| **Language** | Python. |
| **License** | MIT (PyPI / GitHub). |
| **Core abstractions** | `Agent` (role, goal, backstory, tools, llm, max_iter, memory, delegation…); `Task`; `Crew` + `Process` (sequential / hierarchical); Flows for app-level orchestration; `kickoff()` LiteAgent path for single-agent. |
| **Dependency weight** | **Medium+.** Many agent knobs (RAG knowledge sources, embedders, planning, multimodal) live on Agent; tools often from `crewai_tools`. |
| **Plugin / extensibility** | Tools, knowledge sources, JSONC/YAML agent specs, step callbacks, Flows composing Crews. |
| **Looks core (product)** | Agent + Task + Crew process. |
| **Looks optional / should stay out of a minimal harness** | Memory, knowledge/RAG, hierarchical managers, reasoning pre-plans, multimodal, AMP visual builder. |

Sources: https://docs.crewai.com/en/concepts/agents ; Crews / Tasks docs on docs.crewai.com

---

## 10. Letta (MemGPT)

| Field | Evidence |
|-------|----------|
| **Purpose** | Stateful agents with OS-like memory management across context limits. |
| **Language** | Python / service API. |
| **License** | Apache-2.0 (PyPI `letta` reported). |
| **Core abstractions** | Agent = system prompt + memory blocks + messages + tools; core memory blocks (in-context, editable via tools); archival memory; conversation search / recall; runs & steps; conversations (threads); persistence in DB; optional MemFS. |
| **Dependency weight** | **Heavy** if you take the full server + DB model. Memory *is* the product. |
| **Plugin / extensibility** | Tools (server / MCP / client-side schema); attach/detach shared blocks; archival backends. |
| **Looks core (to Letta)** | Memory hierarchy + persistence + agent step loop. |
| **For our harness** | Treat entire memory OS as **plugin**, not core. Paper: MemGPT (Packer et al., arXiv:2310.08560). |

Sources: https://docs.letta.com/guides/core-concepts/stateful-agents/ (fetched via docs.letta.com); https://www.letta.com/blog/agent-memory/

---

## 11. MCP (Model Context Protocol)

| Field | Evidence |
|-------|----------|
| **Purpose** | Protocol for context exchange between hosts and servers. Not an agent framework. |
| **Language** | Spec + multi-language SDKs. |
| **License** | Spec/project licensing per modelcontextprotocol.io / GitHub org (verify per artifact). |
| **Core abstractions** | Host / Client / Server; data layer JSON-RPC 2.0; transports: stdio, Streamable HTTP; server primitives: **tools, resources, prompts**; client elicitation; discovery (`server/discover`); optional Tasks extension; notifications via subscriptions. |
| **Dependency weight** | **Protocol: light conceptually; host integration varies.** Official note: MCP does not dictate how apps use LLMs or manage context. |
| **Plugin / extensibility** | Servers are the extension surface. Clients consume tools/resources/prompts. |
| **For our harness** | MCP client/server adapter = **plugin**. Core tool interface should be MCP-compatible in shape (name, schema, call) but not require MCP runtime. |

Sources: https://modelcontextprotocol.io/docs/learn/architecture ; https://modelcontextprotocol.io/specification/2026-07-28

---

## 12. Minimal / lean runtimes (sampled)

### mini-SWE-agent
Already covered. Strongest evidence for “~100 lines + bash + linear history” still useful on SWE-bench.

### Bare Responses / Messages API loop
OpenAI Agents docs: own the loop with Responses API. Anthropic Client SDK: you implement the tool loop. Pattern: messages list + `tools` param + execute `tool_calls` + append results + repeat until stop.

### PydanticAI / pydantic-graph thin path
`agent.run()` hides graph; `agent.iter()` exposes nodes. Slim package shows typed agents without full stack.

### OpenHands local mode
`openhands-sdk` + `LocalWorkspace` only — same Agent code as production without server.

### Other lean examples found (secondary)
Community “tiny/bare/minimal agent” repos exist (e.g. bare-agent, tinyagent-style projects). Treat as inspiration only unless READMEs are audited; prefer mini-SWE-agent and vendor “own the loop” docs as primary evidence.

---

## Cross-cutting comparison (short)

| System | Loop style | Multi-agent | Memory | Tools | Weight |
|--------|------------|-------------|--------|-------|--------|
| OpenAI Agents SDK | Runner while tools | Handoffs / as_tool | Sessions optional | Functions + MCP | Med–heavy |
| Claude Agent SDK | Claude Code loop | Subagents | Product memory/skills | Built-ins + MCP | Heavy |
| LangGraph | Graph / Pregel | Subgraphs | Checkpointer/store | In nodes | Med–heavy |
| PydanticAI | Agent graph nodes | Graphs / handoffs | History by caller | Tools/toolsets | Light–med (slim) |
| smolagents | ReAct MultiStep | Managed agents | Step memory | Tools / code | Light+ |
| OpenHands SDK | Reason–act | App-level | Condenser optional | Action/Observation | Modular |
| SWE-agent | forward() | Single | HistoryProcessor | ACI / shell | Heavy |
| mini-SWE-agent | Linear step | Single | Linear messages | Bash / tools | Very light |
| CrewAI | Crew process | First-class | Agent memory flag | Tools / knowledge | Med+ |
| Letta | Steps in run | Conversations | Core product | Tools + memory tools | Heavy |
| DeepSeek Harness | Turn/step agent-loop (plugin) | Subagents / teams (opt) | Session log | ctx.tools + guards | Heavy tree / swappable |
| MCP | N/A (protocol) | N/A | N/A | tools/resources/prompts | Protocol |

---

## Takeaway for a minimal harness

1. Every useful coding/chat agent reduces to a **step loop + tool interface + message list** (or session log that projects messages).
2. **DeepSeek Harness** is the primary “everything is a plugin” reference — including a swappable loop — but do not vendor its full TypeScript tree; take seams and lifecycle ideas.
3. Systems that stay maintainable **split packages** (OpenHands) or **document “own the loop”** (OpenAI) or **compose profiles** (dsh).
4. Systems that bake memory, MCP, multi-agent, sandboxes, and observability into one package become heavy fast (Claude Agent SDK, full CrewAI Agent surface, Letta server, full dsh Standard profile).
5. Best external references for minimalism: **DeepSeek Harness Minimal mode / sdk-minimal ideas**, **mini-SWE-agent**, **smolagents MultiStepAgent**, **PydanticAI-slim**, **OpenHands sdk-only path**, **MCP as plugin not core**.
