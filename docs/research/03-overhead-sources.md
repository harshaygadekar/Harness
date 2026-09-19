# Overhead sources

Where latency, memory, dependency complexity, and token cost come from in agent stacks. Cite sources when possible. Prefer mechanisms over blame.

---

## 1. Dominant cost: model tokens and serial tool loops

**Token growth**

- Each step resends (or re-summarizes) history. smolagents writes full memory to messages every step (HF ReAct guide).
- Tool observations (file dumps, stack traces, HTML) inflate context.
- Multi-agent handoffs duplicate history unless filtered (OpenAI Agents nested handoff / history options).

**Serial LLM rounds**

- ReAct is sequential Thought → Act → Observe (Yao et al., 2022). Independent tools still often run one model turn at a time unless the runtime batches parallel tool calls.
- LLMCompiler work (Kim et al., arXiv:2312.04511) reports large latency gaps vs parallel planning on some tasks — evidence that **loop structure**, not only model size, drives wall clock.

**Harness implication:** Core should allow parallel tool execution when the model returns multiple calls. Context trimming belongs in a plugin (condenser / history processor).

---

## 2. Startup and import weight

| Source | Effect | Evidence |
|--------|--------|----------|
| Large SDK bundles | Slow import, large venv | Claude Agent SDK optional native binary (Anthropic docs); OpenAI Agents pulls openai + mcp + many extras |
| Monolithic wheels | Install friction | CrewAI + tools extras; full LangChain stacks |
| Eager provider SDKs | Import all cloud clients | Contrast: PydanticAI-slim + extras; OpenHands package split |

**Harness implication:** Core package should depend on little (typing, maybe pydantic). Providers and MCP are extras.

---

## 3. Tool dispatch overhead

| Layer | Cost |
|-------|------|
| Schema conversion | JSON Schema ↔ provider tool format each turn |
| Validation | Pydantic/JSON Schema validate args (CPU; usually small vs LLM) |
| Permission / guardrail checks | Extra model calls if LLM-based guardrails (OpenAI guardrails pattern) |
| MCP round-trip | Subprocess stdio or HTTP JSON-RPC (MCP architecture docs) |
| Sandbox exec | Container start, file sync, remote API (OpenHands Docker mode; SWE-ReX; OpenAI SandboxAgent) |
| Shell session vs one-shot | Stateful shells add session complexity; mini-SWE-agent argues independent `subprocess.run`-style actions simplify sandboxing |

**Harness implication:** Local in-process tools = cheapest. MCP and sandboxes are opt-in plugins. Do not start containers in core.

---

## 4. State, serialization, persistence

| Source | Effect | Evidence |
|--------|--------|----------|
| Checkpoint every superstep | Disk/DB I/O, serde | LangGraph checkpointers |
| Pydantic-heavy state | Validation CPU | LangGraph docs: Pydantic state less performant than TypedDict/dataclass |
| Event log + projection | Memory for events + messages | OpenHands events; smolagents steps |
| DB-backed agent state | Service latency | Letta persists messages/blocks in DB |
| Trajectory recording | Disk growth | SWE-agent / mini trajectory browsers |

**Harness implication:** Default = in-memory list. Persistence plugin behind a narrow `SaveState`/`LoadState` interface.

---

## 5. Framework layers (call stack tax)

Extra frames common in heavy frameworks:

1. App → Crew/Graph/Runner facade
2. Agent wrapper
3. Middleware / hooks / callbacks
4. Provider SDK
5. HTTP to model

Usually **small vs LLM time**, but hurts:

- Debuggability
- Streaming time-to-first-token if buffering
- CPU-bound local models

OpenAI docs: skip Runner and call Responses API to own the loop — explicit acknowledgment of framework tax.

---

## 6. Memory and process footprint

| Contributor | Notes |
|-------------|-------|
| Embedding models / vector stores | RAG stacks |
| Browser / desktop tooling | OpenHands desktop endpoints; browser agents |
| Native binaries | Claude Code binary beside Agent SDK |
| Multiple agents in one process | Crews, graphs, Hub runtimes |
| Java/TS polyglot bridges | Extra runtimes |

---

## 7. Token overhead beyond raw dialogue

| Practice | Token effect |
|----------|--------------|
| Long system prompts / skills / CLAUDE.md / tool catalogs | Fixed cost every turn |
| Listing hundreds of MCP tools | Large tool schema tokens |
| Verbose tool JSON | Larger tool messages |
| Re-planning steps | Extra model calls (smolagents planning interval) |
| Adversarial multi-agent review | More agents × more turns (Anthropic dynamic workflows blog; higher token use warned) |

MCP progressive discovery and OpenHands condensers exist because **tool/history bloat is expected**.

---

## 8. Summary table for Walter

| Overhead class | In core? | Mitigation |
|----------------|----------|------------|
| Model RTT + tokens | Unavoidable | Streaming; parallel tools; trim plugin |
| Fat dependencies | Avoid | Slim core + extras |
| MCP transport | Plugin | Lazy connect |
| Sandbox / Docker | Plugin | Local tools default |
| Checkpoints | Plugin | Opt-in |
| Multi-agent | Plugin | Single loop default |
| Tracing export | Plugin | Hooks only in core |
| Guardrail LLM calls | Plugin | Sync validators optional |

---

## 9. Sources for this note

- Yao et al., ReAct (2022): https://arxiv.org/abs/2210.03629
- Kim et al., LLMCompiler (2023): https://arxiv.org/abs/2312.04511
- HF smolagents ReAct guide
- OpenAI Agents “Running agents” / Agents docs (own loop via Responses API)
- LangGraph Graph API (reducers, Pydantic perf note, checkpointers)
- MCP architecture (transports, primitives)
- OpenHands SDK architecture (local vs Docker)
- mini-SWE-agent docs (linear history, independent actions)
- Anthropic “A harness for every task” (workflows use more tokens; best for complex tasks)
