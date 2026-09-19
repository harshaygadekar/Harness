# ADR-001: Core loop

| Field | Value |
|-------|-------|
| **Status** | Accepted |
| **Date** | 2026-09-19 (IST) |
| **Owners** | Walter (draft), Thomas (approve), Alice (product lock), Jessie (C1–C5 in spike), Mike (spike green) |
| **Related** | `docs/useful-scope.md`, `docs/research/02`, `04`, `05`, `06`; [ADR-002](ADR-002-model-adapter.md) (model adapter plugins — Proposed) |

---

## Problem

We need the smallest useful agent core for an OSS Python-first harness.

A host app must be able to:

1. Call one model through a thin adapter.
2. Run one or more tools in a loop.
3. Stop on final text, `max_steps`, cancel, or hard error.
4. Extend behavior with plugins without forking the core.

If the core is too big (graphs, MCP, shell, crews), cold import and deps rise. If the core is too thin (no tool gate, no cancel), hosts cannot ship safe demos.

Alice’s first demo lock (`docs/useful-scope.md`): one model adapter + one user tool + loop. No shell, MCP, or multi-agent in core.

---

## Constraints

From research and product lock:

- **Minimal core.** Loop + protocols + registry + hooks. Rest is plugins (`04-keep-out-of-core.md`).
- **Python-first**, dependency-light (stdlib + typing; pydantic only if a later ADR proves need).
- **Deny-all tool gate** at the tool boundary (Jessie C1). Host must Allow each tool by name (or explicit allow list). Unset gate = Deny for every tool call. Core does not guess “pure” or “side-effect.” Policy packs are plugins; the hook lives in core.
- **No `python_eval` / shell in core.** No built-in product tools that write disk, network, process, or credentials.
- **Tool errors must not leak secrets** (env, keys, tokens) in observations or `RunResult.error`.
- **DeepSeek Harness + Cordis paper are design inspiration only.** We do **not** vendor Cordis or `dsh`. We do **not** call the DeepSeek LLM API (or any LLM API) to invent this ADR.
- Cite evidence: `docs/research/` (esp. `02`, `04`, `05`, `06`), `docs/useful-scope.md`, Jessie red flags.

---

## Candidate designs (at least 3)

### A. Tiny protocol core (selected)

- Core owns: `Runner` loop, `ModelClient` protocol, `Tool` protocol + registry, message-list history, `max_steps` + cancel, `Plugin.register` + lifecycle hooks, `RunResult`, deny-all `can_use_tool` hook (host Allow list).
- Loop **is** the core (not a Cordis-style plugin that can unload the driver).
- Capability seams: swap model/tools via register; no built-in tools.
- Inspired by dsh seams and tool gate; smaller than Cordis.

### B. Cordis / DeepSeek Harness port

- Vendor or reimplement Cordis: named services on `ctx`, inject, reversible effects, typed events, agent-loop as a plugin, session event log, profiles/bundles.
- Full spatiotemporal composability from the Cordis paper (arXiv:2608.25512).

### C. Graph-first core (LangGraph-style)

- Core = state channels + nodes + edges; ReAct is one compiled graph.
- Checkpointers and multi-node flows sit beside the default path.

### D. SDK wrap (OpenAI Agents / Claude Agent SDK façade)

- Thin wrapper over a vendor Runner; “our” API maps to theirs.
- Plugins mostly mean vendor hooks/MCP.

---

## DeepSeek Harness analysis (inspiration — what we take / what we reject)

Source note: `docs/research/06-deepseek-harness.md`. Upstream: [deepseek-ai/deepseek-harness](https://github.com/deepseek-ai/deepseek-harness), Cordis paper [arXiv:2608.25512](https://arxiv.org/abs/2608.25512).

### What we take (ideas only)

| Idea | How we use it |
|------|----------------|
| Capability seams | Register model adapters and tools; host swaps them without editing the loop |
| Policy at tool boundary | Waterfall-style `can_use_tool` / pre-execute gate; monotonic deny; policy not inside `execute` |
| No built-in product tools in core | Tool packs are plugins (shell, MCP, sandbox later) |
| Session / step log as model-visible truth | v0: message list is canonical; append-only step log may arrive later as a plugin |
| Plugin teardown | `Plugin.register` returns or registers disposers so unload does not leak handlers |
| Profiles / bundles as composition | Future packs stack adapters — not source forks |
| Tool pipeline shape | Conceptual pre-execute → execute → post-execute; we expose hooks, not Cordis event names |

### What we reject

| Idea | Why |
|------|-----|
| Vendoring Cordis or `dsh` | TypeScript, large, preview-unstable; wrong weight for Python-minimal |
| Agent-loop as a removable Cordis plugin | We keep one small default loop **in core** so the install stays tiny |
| Full typed-event bus / fiber HMR | Overkill for v0; hooks suffice |
| Built-in shell / filesystem / UI | Product tools; Jessie hard kill for shell in core |
| DeepSeek (or any) LLM API for architecture | Human draft from evidence only |

We stay **smaller than Cordis**: inspiration, not a port.

---

## Selected design

**Tiny protocol core (Candidate A).**

### Core surface

| Piece | Role |
|-------|------|
| `Runner.run` / `run_async` | Drive the ReAct/tool loop |
| `ModelClient` protocol | `complete(messages, tools) → ModelResponse` (provider-agnostic) |
| `Tool` protocol + registry | `name`, `description`, `parameters` (JSON Schema), `call(args, ctx) → ToolResult` |
| Message list history | Linear chat-style messages; tool calls + results with ids |
| `max_steps` + cancel | Bound runaway; propagate abort |
| `Plugin` | `name`, `version`, `register(registry) → disposer`, optional lifecycle hooks |
| `RunResult` | `status`, `messages`, `usage`, `error` |
| `can_use_tool` hook | Deny all tools unless the host Allows by name (or allow list); unset = Deny |

### Loop (conceptual)

```
messages = [system?, user, ...]
while step_count < max_steps and not cancelled:
  response = model.complete(messages, tool_schemas)
  append assistant message
  if no tool_calls → succeed and stop
  for each tool_call (v0: sequential):
    decision = can_use_tool(...)   # default Deny for every tool; host must Allow
    if deny → append denial observation; continue
    try: result = tool.call(...)
    except: result = safe_error_observation(...)  # no secret leak
    append tool result message
  step_count += 1
if max_steps → status=failed or truncated per RunResult rules
```

### Parallel tool calls

- **v0 default: sequential** execution even if the model returns multiple calls.
- If a host enables parallel later: tools **must not** share mutable state unless the host provides a lock. Until **ADR-010**, prefer sequential.
- Document this rule in `SECURITY.md` / tool authoring notes.

### Alice first demo

One script: one model adapter plugin + one user-supplied in-process tool + `Runner.run` → print `RunResult`. No shell, MCP, crew, or server in core.

---

## Why selected

1. Matches `02-core-primitives.md`: every useful agent needs model call, tool invoke, loop, history, cancel/error surface.
2. Matches `useful-scope.md` success path and hard kills.
3. Keeps dsh’s best seams (swap model/tools, tool gate, no built-in tools) without Cordis weight (`05`, `06`).
4. Loop in core is easier to reason about and benchmark than “loop is just another plugin” for a minimal OSS first cut.
5. Rejects graph/SDK-wrap paths that pull heavy deps or vendor lock-in (`04`).

---

## Rejected alternatives

| Candidate | Reject reason |
|-----------|----------------|
| **B. Cordis / dsh port** | Vendoring Cordis/`dsh` violates size and language goals; preview API churn; we do not need full spatiotemporal runtime in v0 |
| **C. Graph-first** | Graphs help complex branching (LangGraph) but are not required for the default ReAct agent; Alice veto if graph DSL lands in core |
| **D. Vendor SDK wrap** | Pins provider SDKs and product tools; “own the loop” becomes hard; license/product mismatch risk |
| **Built-in shell / `python_eval`** | Jessie red flags; secret and RCE risk; domain tools belong in plugins |
| **MCP in core** | Protocol adapter, not the loop (`04`); ship as plugin later |
| **Multi-agent / crews in core** | Composition of single agents; orchestrator plugin later |

---

## Core vs plugin boundary (table)

| In core | Out of core (plugin / pack) |
|---------|------------------------------|
| `Runner` loop | Provider SDKs (`openai`, `anthropic`, …) |
| `ModelClient` protocol | MCP client / transport |
| `Tool` protocol + in-process registry | Shell, editor, browser, web search tool packs |
| Message list history | Append-only session/event log (optional later) |
| `max_steps` + cancel | Checkpointers / durable runs |
| `Plugin.register` + lifecycle + disposer | Memory OS / Letta-like stores |
| `RunResult` | Observability exporters (OTel, SaaS) |
| Deny-all `can_use_tool` hook (host Allow list) | Policy / guardrail packs (LLM judges) |
| Safe tool-error sanitization rules | Sandbox / Docker / remote workspace |
| | Multi-agent / graph / crew orchestrators |
| | RAG / vector DB |
| | HTTP / UI / CLI façades |
| | Skills / markdown prompt loaders |

If core imports a feature’s heavy dependency by default, the boundary failed (`04`).

---

## Implementation sketch (interfaces only, no full code)

Names illustrative; exact module layout is eng’s choice.

```text
ModelClient
  complete(messages: list[Message], tools: list[ToolSpec] | None) -> ModelResponse

Tool / ToolSpec
  name: str
  description: str
  parameters: JSONSchema
  call(args: dict, ctx: RunContext) -> ToolResult

ToolRegistry
  register(tool) -> Disposer
  get(name) -> Tool
  list_specs() -> list[ToolSpec]

Plugin
  name: str
  version: str
  register(registry: Registry) -> Disposer   # teardown required
  # optional hooks: on_start, on_step, on_tool, on_error, on_end

CanUseTool
  (tool_name, args, ctx) -> Allow | Deny(reason)   # default: Deny for ALL tools until host Allows

Runner
  run(agent_or_config, input, *, max_steps, cancel=None) -> RunResult
  run_async(...) -> RunResult

RunResult
  status: succeeded | failed | cancelled | max_steps
  messages: list[Message]
  usage: Usage | None
  error: SafeError | None
```

Hooks around tools may mirror the dsh pipeline **conceptually** (`pre_execute` / `execute` / `post_execute`) without Cordis event names.

No full implementations in this ADR.

---

## Security rules (Jessie)

**Side-effect (definition, Thomas):** any tool that can write disk, network, process, or credentials. This label is documentation only — it does **not** drive auto-allow. Per C1, core denies **all** tools until the host Allows.

1. **C1 — Deny all tools by default.** Host must allow each tool by name (or an explicit allow list). Core does not guess “pure” or “side-effect.” Unset `can_use_tool` / unset gate = **Deny** for every tool call. Pure/local tools still need an explicit host Allow.
2. **No `python_eval` in core.** No shell, subprocess, or OS command tool in the core package.
3. **C3 — Concrete secret scrub.** Observations and `RunResult.error` must scrub at least: `sk-`, `Bearer `, `api_key`, `API_KEY`, `password=`, AWS-like keys, and home-directory path prefixes. Prefer `TOOL_FAILED` (or similar stable code) plus a short safe string — never raw exception text with secrets.
4. **Parallel tools and shared state.** v0 runs tool calls sequentially. If parallel is enabled later, tools must not share mutable state unless the host supplies a lock; otherwise keep sequential until ADR-010.
5. **C2 — Gate cannot be skipped.** `ToolRegistry` / `Runner` is the only path to `tool.call`. No public helper that lets plugins invoke tools outside the Runner pipeline. Bypass attempts must fail or not be exported.
6. **No network or credential stores in core.** Credentials stay in host or credential plugins.
7. **C4 — v0 plugins = full trust in-process.** Plugins share the host process; do **not** claim isolation. Only load plugins the host chose. No remote plugin install in v0. Message-mutating hooks (`on_step`, etc.) can rewrite history — treat as trusted code.
8. **C5 — Cancel stops new tool calls.** After cancel, do not start a new tool call. In-flight calls should check cancel where possible; document if they cannot. Mid-run cancel → status `cancelled`; no further tool side effects after the flag.

---

## Open questions for later ADRs

1. **Messages vs events as canonical store** — hybrid (events + `to_messages`) deferred; v0 = messages (`05` Q1).
2. **Pydantic in core?** — deferred; TypedDict / dataclasses first (`05` Q2).
3. **Async-only vs sync wrappers** — prefer async-first + `run_sync` helper (`05` Q4).
4. **Native parallel tool calls** — ADR-010 (`05` Q5).
5. **MCP-shaped Tool schema day-one fidelity** — recommend JSON Schema args + string/content result (`05` Q6).
6. **Graph API** — never in core; optional `harness-graph` plugin only with evidence (`05` Q7).
7. **Stop semantics** — final text with no tool calls vs explicit `finish` tool vs both (`05` Q8). **Keep open;** do **not** add a `finish` tool in the first spike (Thomas).
8. **Append-only step log plugin** — when message list is not enough for resume/fork.
9. **Package / PyPI name** — avoid implying Claude Code or DeepSeek product compatibility (`05` Q10).
10. **Provider adapters** — see [ADR-002](ADR-002-model-adapter.md) (`ModelClient` protocol in core; Gemini/other SDKs as plugins).

---

## Benchmark plan (Mike)

Measure before claiming “minimal” or “fast”:

| Metric | Method | Target (first cut) |
|--------|--------|--------------------|
| Cold import of core | `python -c "import …"` timed | Publish number; keep deps = stdlib + typing (+ approved extras) |
| Core dependency count | lockfile / `import` graph | No provider SDK in core |
| First demo user code size | LOC outside adapters | Prefer &lt;50 lines |
| Time to first tool observation | One-tool ReAct vs raw SDK loop | Publish delta; no vanity “faster than LangGraph” claim |
| Plugin without editing core | CI test: register plugin, run loop | Must pass |
| Tool gate (C1) | CI: unset gate → every tool denied; host Allow required | Must pass |
| Error sanitization (C3) | CI: patterns (`sk-`, Bearer, api_key, …) scrubbed; prefer TOOL_FAILED | Must pass |

Stars and feature parity with LangGraph/CrewAI are not success (`useful-scope.md`).

---

## Post-implementation review (empty checklist)

After the spike / first cut ships, fill in:

- [x] Cold import time recorded (Mike) — ~26ms; see ADR-001-spike-notes.md
- [x] Dep count matches ADR claim — stdlib only
- [x] Alice demo works — examples/echo_demo.py (fake model)
- [x] Jessie C1: unset gate denies ALL tools; host Allow-by-name (or allow list) covered by tests
- [ ] Jessie C2: gate cannot be skipped — only Runner/registry path to `call`; no bypass export
- [ ] Jessie C3: scrub patterns (`sk-`, `Bearer `, `api_key`, `API_KEY`, `password=`, AWS-like, home paths); prefer TOOL_FAILED + short safe string — tests green
- [ ] Jessie C4: docs state v0 plugins = full trust in-process; no isolation claim; no remote plugin install
- [ ] Jessie C5: cancel stops new tool calls; mid-run cancel → `cancelled`, no further tool side effects
- [x] Jessie: no shell / `python_eval` in core package
- [x] Parallel = sequential default documented
- [x] No Cordis/`dsh` vendored code in tree
- [ ] No `finish` tool in first spike (stop semantics still open)
- [ ] Open questions filed as ADR stubs or issues
- [ ] Thomas sign-off on architecture note in `ARCHITECTURE.md`
- [ ] Alice sign-off that scope did not grow
- [x] Status flipped to Accepted once C1–C5 tests land

---

## Review trail

- [Thomas review](ADR-001-thomas-review.md) — Approve Candidate A; Status → Proposed; side-effect one-liner; no `finish` in spike
- [Mike review](ADR-001-mike-review.md) — Approve for spike; hold code until Proposed → Accepted after Jessie C1–C5 in tests
- [Jessie security review](../security/ADR-001-security-review.md) — Conditionally accepted; C1–C5 required in spike

- [Alice review](ADR-001-alice-review.md) — Approve Candidate A; scope lock holds; README after spike
