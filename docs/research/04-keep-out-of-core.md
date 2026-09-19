# Keep out of core

Rule: If a feature is not required for a single agent to call a model and tools in a loop, it stays out of core.

For each item: why exclude; plugin boundary; evidence from studied systems.

---

## 1. Long-term / OS memory

**Why out:** Letta/MemGPT show memory hierarchy (core blocks, recall, archival) is a full product. Most agents only need the current message/step list.

**Plugin boundary:** `MemoryPlugin` with `load(ctx) -> messages|snippets` and `save(step)`. Core only passes history it already has.

**Evidence:** Letta stateful agents / memory blocks docs; MemGPT paper (Packer et al., arXiv:2310.08560).

---

## 2. MCP runtime

**Why out:** MCP is a **context protocol**, not an agent loop. Official docs: MCP does not dictate LLM use or context management. Shipping MCP in core forces transports, discovery, and server lifecycle on every app.

**Plugin boundary:** `McpToolAdapter` implements core `Tool` by calling `tools/list` / `tools/call`. Host may open stdio or Streamable HTTP outside core.

**Evidence:** https://modelcontextprotocol.io/docs/learn/architecture ; OpenAI/Anthropic/smolagents treat MCP as optional integration.

---

## 3. Observability / tracing backends

**Why out:** Exporters (LangSmith, Logfire, OTel collectors) pull network and vendor deps. Core only needs hook points.

**Plugin boundary:** Hooks `on_step` / `on_tool` / `on_model` / `on_end`. Plugins export spans. No hard dependency on a SaaS.

**Evidence:** LangGraph points to LangSmith for observability; PydanticAI documents Logfire as optional instrumentation; OpenAI Agents has tracing hooks.

---

## 4. Persistence / checkpointers / durable execution

**Why out:** Resume and time-travel need storage backends and serde. Default runs are ephemeral.

**Plugin boundary:** `Checkpointer` protocol: `save(run_id, state)`, `load(run_id)`. LangGraph-style interrupt/resume stays in plugin.

**Evidence:** LangGraph compile-time checkpointer; OpenAI SandboxAgent snapshots; Letta DB persistence. All optional relative to a bare loop.

---

## 5. Multi-agent orchestration

**Why out:** Crews, handoffs, hubs, and dynamic workflow graphs are **composition**. A minimal harness serves one agent. Multi-agent builds on the same loop.

**Plugin boundary:** Orchestrator plugin that creates child `Runner` instances or exposes agents as tools. Core API stays `run(agent, input)`.

**Evidence:** OpenAI handoffs / agents-as-tools; CrewAI Crew/Process; AG2 Hub/channels; Anthropic dynamic workflows (explicitly higher token use).

---

## 6. RAG / vector stores / knowledge

**Why out:** Retrieval stacks (embedders, indexes, chunking) are app-specific and heavy.

**Plugin boundary:** Tools that search corpora, or a `before_model` hook that injects snippets. No vector DB client in core.

**Evidence:** CrewAI knowledge_sources / RagTool patterns; not required by mini-SWE-agent or smolagents core loop.

---

## 7. Sandboxes / remote workspaces / ACI tool UIs

**Why out:** Docker, remote agents, and rich IDE tools dominate ops complexity. mini-SWE-agent shows bash + independent actions suffice for strong coding agents.

**Plugin boundary:** `Environment.execute(action) -> observation`. Local subprocess default; Docker/remote implement same interface (OpenHands Local vs Docker workspace).

**Evidence:** OpenHands four-package split; SWE-agent → mini-SWE-agent simplification; OpenAI SandboxAgent as additive layer.

---

## 8. Built-in domain tools (shell, editor, browser, web search)

**Why out:** Tool sets define the product (coding vs research vs ops). Core should ship **zero or few** optional reference tools, not a kitchen sink.

**Plugin boundary:** Tool packages register into the registry. Apps depend on `harness-tools-coding` etc. as separate dists.

**Evidence:** OpenHands `openhands.tools` optional; smolagents tools optional; Claude Agent SDK bundles coding tools because it *is* Claude Code — different product goal than a minimal harness.

---

## 9. Guardrails / policy LLM judges

**Why out:** Extra model calls and policy packs. Useful, not universal.

**Plugin boundary:** `before_tool` / `before_output` hooks returning allow|deny|transform. Sync validators stay cheap; LLM judges are plugins.

**Evidence:** OpenAI Agents input/output guardrails; Claude permissionMode / canUseTool.

---

## 10. Planning / reflection / tournament workflows

**Why out:** Extra model turns. smolagents planning is optional (`planning_interval`). Anthropic workflows warn of higher token use.

**Plugin boundary:** Step hooks or a planner plugin that inserts plan messages. Default loop has no planner.

---

## 11. Provider-specific SDKs and model routers

**Why out:** Pinning `openai` / `anthropic` / LiteLLM in core blocks lean installs.

**Plugin boundary:** `ModelClient` protocol. Packages: `harness-model-openai`, etc. PydanticAI-slim pattern.

---

## 12. HTTP servers / UIs / CLIs

**Why out:** Serving is an application concern.

**Plugin boundary:** Apps wrap `Runner`. OpenHands keeps `agent_server` separate.

---

## 13. Skills / markdown memory files / prompt asset loaders

**Why out:** Claude Code skills/plugins and CLAUDE.md loading are product conventions.

**Plugin boundary:** `PromptLoader` plugin prepends instructions at run start.

---

## Clean boundary summary

```
CORE                          PLUGINS
-------------------------     ---------------------------------
Runner loop                   Memory, RAG, condensers
Tool protocol + registry      MCP adapter, tool packs
ModelClient protocol          Provider SDKs
Message/step state            Checkpointer / DB
Hooks / Plugin.register       OTel, LangSmith, Logfire
max_steps + cancel            Multi-agent orchestrators
                              Sandbox / workspace
                              Guardrail packs
                              HTTP/UI
```

If core imports a feature’s heavy dependency by default, the boundary failed.
