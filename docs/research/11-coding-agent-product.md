# 11 — Coding-agent product (evidence for ADR-004)

**Audience:** Thomas (Research Lead), Walter (ADR-004), Alice (product), Harsha  
**Date:** 2026-09-20 (IST)  
**Status:** Accept evidence for [ADR-004](../decisions/ADR-004-coding-agent-product.md). Core freeze holds. No core code in this note.  
**Rule:** Cite measured numbers only. Prefer [`10-fx-beat-goals.md`](10-fx-beat-goals.md) and [`docs/benchmarks/phase2-numbers.md`](../benchmarks/phase2-numbers.md). Do **not** invent fx figures.

---

## 1. Brief myth corrections

| Myth | Fact | Cite |
|------|------|------|
| fx “10 µs cold start” is the engineering SLA | **No.** First-party Linux CI startup budget is **2 ms** mean (`AGENTS.md` / `LINUX_BUDGETS`). “10 µs” is marketing / third-party blog. Prefer **2 ms**. | `10` §1 secondary table |
| ~**122k/s** gated tool dispatch is an fx number | **No.** That rate is **ours** (fake-model bench, 1000 steps, 8 no-op tools). fx does **not** publish an absolute Zig tool-dispatch µs SLA. | `phase2-numbers.md`; `10` §1 “not found” |
| DeepSeek Harness / Cordis drove this product ADR via LLM API | **No.** DeepSeek Harness = **design inspiration** only (`06-deepseek-harness.md`). No API key. No Cordis vendor. | Harsha correction in `00-index.md`; `06` |
| We beat fx | **No.** Different products: Zig native coding CLI vs Python library + plugins. | `10` product honesty; Alice Phase 2 |

---

## 2. Fair vs unfair beat axes

Full table: [`10-fx-beat-goals.md`](10-fx-beat-goals.md) §2–§3. Summary for product talk:

### Fair (compete as a library)

| Axis | Why fair | Our measured / budget |
|------|----------|------------------------|
| Cold `import agentic_harness` | Pay before first useful call | ~**27 ms** median (P2-4 ≤ 50 ms) — `phase2-numbers.md` |
| Fake-model gated tool dispatch | Loop + gate without LLM noise | ~**122k/s** (P2-5 ≥ 1000/s) — **ours**, not fx |
| Core LOC (excl. plugins) | Maintained core surface | **520** (P2-3 ≤ 800) |
| Runtime deps | Empty core deps | **0** (`dependencies = []`) |
| Plugin register / dispose | Extension without core growth | ~**0.06 ms** / 100 (P2-6 SHOULD) |
| Deny-all gate correctness | Security default | Jessie suites — not a latency race with fx |

### Unfair / different product (do not claim as wins)

| Axis | fx reality (from `10`) | Our stance |
|------|------------------------|------------|
| Native binary MiB | Homepage **6.17 MiB**; AGENTS ceiling **7.800 MiB**; measured linux-x86_64 **~11.27 MiB** | NON-GOAL for Python core |
| Process cold start | Linux CI **2 ms** (not 10 µs) | NON-GOAL to match CPython process start |
| Full coding CLI / TUI / ACP / WASM terminal | Shipped fx product | NON-GOAL for core; product via plugins |
| Built-in tool count (~17 + MCP) | fx product feature | Tools stay plugins |
| “1.6× / 2.5× faster turns” | fx vs prior fx | Do not cite as our win |

**DoD rule (`10` §6):** “Beat fx on X” is valid only for a fair axis, with method + SHA + machine class. Never imply CLI parity.

---

## 3. Product shape options

Walter ADR-004 Candidate A maps to **Option A** below. Packaging and native tracks are separate decisions.

| ID | Shape | Startup | Artifact size | DX | Tradeoff |
|----|-------|---------|---------------|-----|----------|
| **A** | **Python package + thin CLI + plugins** (PyPI; `simple.Agent` / Registry+Runner) | Library cold import ~**27 ms**; thin CLI wrapper budget SHOULD ≤ **200 ms** (`10` P3-2) — **not** fx 2 ms | Source / wheel; needs CPython | Best for embedders and plugin authors; matches Accepted ADR-001/002/003 | Cannot match native process start or 6–11 MiB binary without a different stack |
| **B** | **PyInstaller / PyOxidizer** (or similar) single binary wrap of A | Better than “install Python + pip” for some users; still **not** fx-class µs/ms without measuring | Likely **tens of MiB**; will not match fx **6–11 MiB** without honest caveats | One-file distribute; worse debug/plugin hot-reload | Packaging spike only — **SHOULD after ADR-004 Accept**, not a core change |
| **C** | **Rust / Zig core rewrite** (or native sidecar) | Can chase fx Linux **2 ms** help / start budgets (`10` P3-4) | Can chase ≤ **8 MiB** stripped if product demands | Two cores or abandon Python library story | **Separate ADR.** Only if product goal is explicitly fx-class process start / binary size |
| **D** | **WASM** embed (browser / WASI host) | Host-dependent; different floor from CLI | Small module possible; glue and WASI I/O still needed | Good for web embeds; poor for local shell/fs/git without host bridges | Research / stretch. Not Phase 2 ship path. Coding I/O still plugins on the host |

### Recommendation

1. **Ship Option A now** — Python library product + plugin coding agent (Walter ADR-004 Candidate A).  
2. **After Accept:** Option **B** as a **packaging spike** (measure size and start; publish with “not apples-to-apples” vs fx).  
3. **Option C** only if a future product ADR demands fx-class process start / MiB — not this ADR.  
4. **Option D** stays research; do not block Phase 2.

---

## 4. What MUST stay plugins (never core)

Aligns with [`04-keep-out-of-core.md`](04-keep-out-of-core.md), ADR-003 freeze, Jessie gates, Alice coding scope.

| Capability | Why never core |
|------------|----------------|
| **shell** | Side effects; opt-in Allow; Jessie hard kill in core |
| **fs** (list/read/write/patch) | Path guard lives in host plugin (`Path.relative_to`) |
| **git** | Domain VCS; not required for bare loop |
| **MCP** | Context protocol / transports — adapter implements `Tool` |
| **sandbox** | Docker/seccomp/remote = ops product; `Environment` plugin |
| **memory** (long-term / OS) | Message list is enough for v0; MemoryPlugin later |
| **TUI** | Presentation; not loop primitive |

Also out of core (same rule): provider SDKs, observability backends, RAG/vector stores, multi-agent crews, built-in web browser tools.

Core keeps: run loop, tool protocol, message/state, deny-all gate hooks, plugin registry + teardown, model adapter **seam** (adapters are plugins per ADR-002).

---

## 5. Evidence for Walter ADR-004 Candidate A

| Claim | Evidence |
|-------|----------|
| Keep freeze | [`09-disprove-core.md`](09-disprove-core.md): Phase 2 apps (simple, tool, structured, research, coding stub) work at existing seams. **No missing primitive** for coding-agent-via-plugins. Streaming is a UX wall, not a Phase 2 product block. |
| No need to grow core for MCP/shell/sandbox | Alice `useful-scope-coding-product.md` + ADR-003: convenience ≠ missing primitive. |
| Library numbers already green | `phase2-numbers.md`: import ~27 ms, dispatch ~122k/s, 520 LOC, 0 deps — fair-axis story without rewrite. |
| Security path clear | Deny-all + Allow-by-name; path guard; shell opt-in; secret scrub — stay at plugin boundary (Jessie). |
| Escape hatch | Native rewrite = **new** ADR (Option C), not silent scope creep inside ADR-004. |

**Verdict for Accept:** Candidate A is the only option that matches Accepted ADR-001/002/003, Alice Phase 2, and measured fair axes. Recommend **Approve Candidate A**.

---

## 6. Metrics we will publish vs claims we will NOT make

### Publish (ours — CI / docs)

| Metric | How | Notes |
|--------|-----|-------|
| Cold import median ms | `benchmarks/cold_import.py` | Budget ≤ 50 ms (SHOULD stretch ≤ 20 ms) |
| Fake-model dispatch / s | `benchmarks/tool_dispatch.py` | Ours only; ≥ 1000/s MUST |
| Core LOC; deps = 0 | `size_gate` / `wc` + `pyproject.toml` | ≤ 800 LOC; `dependencies = []` |
| Plugin churn ms | `plugin_churn.py` | SHOULD |
| Fake coding loop | `fake_coding_loop.py` via plugins | completed + step count |
| Optional B artifact size + start | After packaging spike | Label platform; say “not fx binary” |

### Do NOT claim

| Claim | Why banned |
|-------|------------|
| “We beat fx” / “fx killer” | Unfair product mix; no §6 DoD |
| fx process start **10 µs** as their SLA | Marketing; engineering SLA is **2 ms** |
| Our **~122k/s** as an fx number | Ours only |
| Python package has **2 ms** process start or **6 MiB** binary | We do not ship that artifact on Option A |
| “Safer than fx” without Jessie evidence scope | Security is deny-all + path guard — not a cross-product slogan |
| DeepSeek LLM designed the ADR | Design inspiration only |

---

## 7. Open for Accept (handoffs)

- [x] Thomas: this file (`11-coding-agent-product.md`)  
- [ ] Alice: product sign-off on Candidate A  
- [ ] Jessie: coding plugin pack + CLI surface review  
- [ ] Harsha: confirm Option C (native) stays out of ADR-004 scope  

Mike implements **only after** Accept: PyPI + thin CLI + plugin pack; optional B spike; no core protocol change without a new missing-primitive ADR.

---

## Sources

- [`10-fx-beat-goals.md`](10-fx-beat-goals.md)  
- [`09-disprove-core.md`](09-disprove-core.md)  
- [`06-deepseek-harness.md`](06-deepseek-harness.md)  
- [`04-keep-out-of-core.md`](04-keep-out-of-core.md)  
- [`docs/benchmarks/phase2-numbers.md`](../benchmarks/phase2-numbers.md)  
- [`docs/decisions/ADR-004-coding-agent-product.md`](../decisions/ADR-004-coding-agent-product.md)  
- [`docs/useful-scope-coding-product.md`](../useful-scope-coding-product.md)  
