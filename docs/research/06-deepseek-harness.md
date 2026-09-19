# DeepSeek Harness — design inspiration (short note)

**Audience:** Thomas (may expand), Walter (ADR-001)  
**Date:** 2026-09-19 (IST)  
**Status:** Summary note. Not an ADR. Not a vendor plan.

---

## Correction

**“DeepSeek” here means DeepSeek Harness (`dsh`) and the Cordis paper** — architecture ideas only.

It does **not** mean calling the DeepSeek LLM API to invent designs. **No API key is required.** We do **not** vendor Cordis or `dsh`.

| Item | URL |
|------|-----|
| Harness repo | https://github.com/deepseek-ai/deepseek-harness |
| Docs | https://deepseek-harness.github.io/deepseek-harness/en/reference/ |
| Cordis paper | https://arxiv.org/abs/2608.25512 |

---

## What it is (one paragraph)

`dsh` is a TypeScript agent harness in developer preview. Almost everything is a **Cordis plugin**: models, tools, sessions, sandboxes, storage, UI, and **the agent loop itself**. Cordis gives named context services, typed events, inject/dependencies, and **reversible effects** (register → disposer on unload). Profiles and bundles compose modes without forking source.

---

## Ideas we take for our Python-minimal core

1. **Capability seams** — swap model and tools by registration, not by editing the loop.
2. **Policy at the tool boundary** — pre-execute / waterfall-style gate; monotonic deny; keep policy out of `execute` bodies (aligns with Jessie deny-by-default).
3. **Tool pipeline shape** — conceptual `pre-execute → execute → post-execute` (hooks, not Cordis event names).
4. **Session / log as model-visible truth** — anything the model sees should be reconstructable from history (we start with a message list; append-only step log can be a later plugin).
5. **Plugin teardown** — registrations unwind (disposers) so unload does not leak handlers.
6. **No built-in product tools in the thin core** — shell, MCP, sandbox stay in packs.
7. **Compose with packs/profiles later** — stack adapters instead of forking core.

---

## Ideas we reject

| Reject | Why |
|--------|-----|
| Vendor Cordis / port full `dsh` | Wrong language and weight; preview churn |
| Agent-loop only as a removable plugin | ADR-001 keeps a tiny default loop **in core** so the install stays small |
| Full typed-event bus / fiber HMR | Hooks are enough for v0 |
| Built-in shell or `python_eval` | Jessie hard kills; product tools are plugins |

Steal ideas, not the tree. Comparative lean bounds: `02`, `04`, `05`, mini-SWE-agent, OpenHands package split.

---

## How this feeds ADR-001

- Primary **design reference** for seams, tool gate, and “everything optional except a thin driver.”
- Comparative pack (`01`–`05`) caps weight and lists non-goals.
- Walter drafts from evidence by hand — **no LLM API for architecture.**

Thomas: expand with deeper Cordis/`dsh` subsystem notes when useful; keep the “inspiration not vendor” rule at the top.
