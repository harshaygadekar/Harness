# Open ADR briefs (deferred from ADR-001)

**Audience:** Walter (open later ADRs), Thomas (decide)  
**Date:** 2026-09-19 (IST)  
**Status:** Evidence briefs only. Not full ADRs. ADR-001 Status stays **Accepted**.  
**Source of deferred list:** `docs/decisions/ADR-001-core-loop.md` § “Open questions for later ADRs” (items 1–5, 7–8; Graph API and package name omitted here).

Each brief: one problem sentence → options → evidence → lean v0.5 default. ASD-STE100 style.

---

## 1. Messages vs events as canonical store

**Problem:** The core must store one history shape that the model and host both trust.

**Options:**

1. **Messages only** — linear chat list (system / user / assistant / tool) is the store.
2. **Events only** — typed step/event log is the store; project to messages for the model.
3. **Hybrid** — append-only events plus `to_messages()`; messages are a view.

**Evidence:**

- ADR-001 Selected design and core table: v0 = message list; append-only session/event log is out of core (`02` §4, `06` “session / log as model-visible truth”).
- mini-SWE-agent: trajectory == messages; linear append; good for debug and fine-tune (`05` Q1; mini-swe-agent.com).
- OpenHands / smolagents: typed Events or ActionSteps, then project to messages (`02` §4).
- DeepSeek Harness: session/event log as model-visible truth; we rejected full typed-event bus for v0 (`06`, ADR-001 reject table).

**Lean default for v0.5:** **Messages only** as canonical store. Keep event/step log for a later plugin (see brief 7). Do not add hybrid until resume/fork needs more than messages.

**Later ADR stub:** ADR-00x Messages vs events.

---

## 2. Pydantic in core?

**Problem:** Core types need clear schemas without a heavy default dependency.

**Options:**

1. **TypedDict / dataclasses / stdlib** — no pydantic in core install.
2. **Pydantic required in core** — models validate every message and tool args.
3. **Optional extra** — core stays stdlib; `harness[pydantic]` or adapters use pydantic.

**Evidence:**

- ADR-001 Constraints: “stdlib + typing; pydantic only if a later ADR proves need.” Spike notes: stdlib only; cold import ~26 ms.
- `05` Q2: pro = schemas (PydanticAI); con = weight; LangGraph notes cost of Pydantic state.
- `03` / lean goal: cold import and dep count are first-cut metrics (ADR-001 Benchmark plan).
- MCP and provider APIs already speak JSON Schema dicts — no pydantic required on the wire.

**Lean default for v0.5:** **No pydantic in core.** Use TypedDict / dataclasses / plain dicts. Allow pydantic only in optional adapter extras after measured need.

**Later ADR stub:** ADR-00x Core validation / pydantic.

---

## 3. Async-only vs sync wrappers

**Problem:** Hosts need both script-friendly sync entry and server-friendly async I/O.

**Options:**

1. **Async-only** — `run_async` only; hosts wrap themselves.
2. **Sync-only** — blocking `run`; async is out of core.
3. **Async-first + sync helper** — `run_async` is primary; `run` / `run_sync` bridges for scripts.

**Evidence:**

- ADR-001 sketch already names `run` and `run_async`.
- `05` Q4: ecosystem is async-first (OpenAI Runner, PydanticAI, Claude SDK); sync wrappers are cheap.
- Provider SDKs and network tools are async; a sync-only core forces awkward bridges.
- Alice demo is a short script — sync helper keeps demo LOC low (`useful-scope.md` / ADR-001 Alice path).

**Lean default for v0.5:** **Async-first + thin `run_sync` (or sync `run`) helper.** Document that the helper must not nest event loops carelessly (e.g. prefer one loop per process).

**Later ADR stub:** ADR-00x Runner sync/async API.

---

## 4. Native parallel tool calls (ADR-010)

**Problem:** Models often return several tool calls in one turn; sequential run wastes wall time and risks races if parallelized without rules.

**Options:**

1. **Always sequential** — ignore parallel hints; run calls one by one (current ADR-001 v0).
2. **Host opt-in parallel** — when enabled, run independent calls concurrently; require no shared mutable state unless host supplies a lock.
3. **Always parallel when model emits N calls** — maximize latency win; host must make all tools race-safe.

**Evidence:**

- ADR-001: “v0 default: sequential”; parallel deferred to **ADR-010**; Jessie rule on shared mutable state.
- `05` Q5: models already return parallel calls; LLMCompiler-style evidence that skipping wastes latency.
- Security: Jessie C1 gate still applies per call; C5 cancel must stop **new** calls; parallel makes cancel and shared state harder.
- DeepSeek / Claude-style pipelines still gate each tool at the boundary (`06`, ADR-001 take table).

**Lean default for v0.5:** **Sequential only.** Document ADR-010 as host opt-in parallel with: deny-all gate per call, no shared mutable tool state without host lock, cancel stops scheduling new calls. Do not enable parallel by default in v0.5.

**Later ADR:** **ADR-010** Native parallel tool calls.

---

## 5. MCP-shaped Tool schema fidelity

**Problem:** Core `Tool` must map cleanly to MCP later without shipping MCP in core.

**Options:**

1. **MCP-identical wire types** — full MCP tool object (icons, outputSchema, annotations, content parts) in core.
2. **MCP-shaped subset** — `name`, `description`, JSON Schema `parameters` / `inputSchema`, result as string or simple content; adapter fills the rest.
3. **Custom tool shape** — invent our own fields; MCP adapter translates heavily.

**Evidence:**

- MCP tools: `name`, `description`, `inputSchema` (JSON Schema); `tools/call` returns `content[]` (text/image/…) and optional structured fields (`modelcontextprotocol.io` tools docs; `04` §2).
- ADR-001 Tool protocol: `name`, `description`, `parameters` (JSON Schema), `call → ToolResult`.
- `05` Q6: recommend JSON Schema args + string/content result so MCP adapter stays thin.
- `04`: MCP stays a plugin; protocol does not dictate the agent loop.

**Lean default for v0.5:** **MCP-shaped subset** — JSON Schema args + string (or single text content) result. Do not require icons, outputSchema, or multi-part content in core. MCP plugin maps subset ↔ full MCP.

**Later ADR stub:** ADR-00x Tool schema / MCP fidelity.

---

## 6. Stop semantics (final text vs finish tool)

**Problem:** The loop must know when the run succeeded without ambiguous stops.

**Options:**

1. **Final text / no tool calls** — assistant message with content and empty tool_calls → `succeeded`.
2. **Explicit `finish` (or submit) tool** — only that tool ends the run (mini-SWE-style submit sentinel; some coding agents).
3. **Both** — no tool calls **or** finish tool; host chooses policy.

**Evidence:**

- ADR-001 loop sketch: “if no tool_calls → succeed and stop.” Thomas: **do not** add a `finish` tool in the first spike; keep open (`05` Q8; Thomas review).
- smolagents: stop on final answer or max_steps (`02` §1).
- mini-SWE-agent: environment detects submit / COMPLETE sentinel; exit message — domain-specific, not a core universal (`05`; mini-swe-agent control flow).
- Provider APIs: `end_turn` / stop when the model returns text without tools (OpenAI/Anthropic-style loops).

**Lean default for v0.5:** **Final text with no tool calls** ends the run as `succeeded`. Do **not** ship a built-in `finish` tool in core. Hosts that need submit semantics add a finish/submit tool via plugin and teach it in the system prompt. Revisit “both” only with product evidence.

**Later ADR stub:** ADR-00x Stop / finish semantics.

---

## 7. Append-only step log plugin

**Problem:** Message list alone may not support resume, fork, or rich audit when hosts need more than chat history.

**Options:**

1. **Never** — messages only forever; hosts build their own logs.
2. **Core step log** — move append-only events into core beside messages (hybrid in core).
3. **Plugin** — optional append-only step/event log that records steps and can project or mirror messages; checkpointer may consume it.

**Evidence:**

- ADR-001 core vs plugin table: “Append-only session/event log (optional later)” is **out of core**; open question 8.
- `06` / ADR-001 take: session log as model-visible truth — we start with messages; step log may arrive later as a plugin.
- OpenHands Events + Conversation; smolagents AgentMemory steps (`02` §4).
- `04` §4: persistence / checkpointers stay plugins; durable resume needs storage.

**Lean default for v0.5:** **No step log in core.** Ship messages + hooks (`on_step` / `on_tool` / `on_end`) so a host or plugin can append an audit log. Design an **optional** append-only step-log plugin when resume/fork or audit is a real product need — do not block Alice demo.

**Later ADR stub:** ADR-00x Append-only step log plugin.

---

## How Walter should use this pack

1. Keep ADR-001 **Accepted**; do not change its Status for these topics.
2. Open one ADR (or GitHub issue) per brief when the spike needs a lock.
3. Prefer the **lean v0.5 default** unless spike or Jessie/Thomas evidence overturns it.
4. Cite this file + `02` / `05` / `06` / ADR-001 open-questions section in the full ADR.

---

## Cross-ref

| Brief | ADR-001 open Q | Primary research |
|-------|----------------|------------------|
| 1 Messages vs events | Q1 | `02` §4, `05` Q1, `06` |
| 2 Pydantic | Q2 | `05` Q2, ADR-001 constraints |
| 3 Async vs sync | Q4 | `05` Q4, ADR-001 sketch |
| 4 Parallel tools | Q5 → ADR-010 | ADR-001 parallel §, Jessie |
| 5 MCP Tool schema | Q6 | `04` §2, `05` Q6, MCP tools docs |
| 6 Stop semantics | Q7 in ADR list / `05` Q8 | Thomas review, `02` §1 |
| 7 Step log plugin | Q8 | ADR-001 table, `06`, `04` §4 |
