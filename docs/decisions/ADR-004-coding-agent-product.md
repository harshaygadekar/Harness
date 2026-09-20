# ADR-004: Coding-agent product shape

| Field | Value |
|-------|-------|
| **Status** | **Proposed** (not Accepted) |
| **Date** | 2026-09-20 (IST) |
| **Owners** | Walter (draft), Alice (product), Thomas (evidence), Mike (impl after Accept), Jessie (security) |
| **Related** | [ADR-001](ADR-001-core-loop.md) (Accepted — tiny core loop), [ADR-002](ADR-002-model-adapter.md) (Accepted — model adapters as plugins), [ADR-003](ADR-003-protocol-freeze.md) (Accepted — protocol freeze + `simple.Agent` façade), [docs/useful-scope-phase2.md](../useful-scope-phase2.md), [docs/research/10-fx-beat-goals.md](../research/10-fx-beat-goals.md). **Await** Thomas `docs/research/11-coding-agent-product.md` before Accept (file not present yet; cite `10-fx-beat-goals.md` until then). |

---

## Problem

We need a clear **product** decision for the coding agent: what we ship, what stays out of core, and how we talk about fx honestly.

Context already locked:

- ADR-001 / 002 / 003 are **Accepted**. Core is frozen. `simple.Agent` is a façade above core. Coding tools land **only as plugins**.
- We did **not** beat fx. fx is a Zig **native coding CLI** (~**2 ms** Linux CI start budget; binary ~**6–11 MiB** depending on source/platform). Ours is a **Python library** (~**27 ms** cold import, **520** LOC core, **0** runtime deps).
- Thomas correction: **~122k/s** gated tool dispatch is **ours** (fake-model bench), not fx. fx’s engineering startup number is **2 ms** CI — not the **10 µs** marketing figure.
- DeepSeek Harness / Cordis = **design inspiration** only. Not an LLM API used to invent this ADR.
- Alice Phase 2 lock: coding agent **on** the harness via plugins; no “beat fx” claim without measured fair axes.

Without this ADR, Phase 2 risks either (a) growing core for MCP/shell/sandbox convenience, or (b) an unplanned native rewrite to chase fx process start / binary size.

---

## Constraints

1. **Core freeze (ADR-003).** No MCP, shell, fs, sandbox, or coding tools in core. Growth only via Thomas missing-primitive bar + new Accepted ADR.
2. **Plugins only for coding tools.** fs / shell / git / patch / search stay host plugins; default gate remains deny-all.
3. **Jessie security invariants hold.** Deny-all + Allow-by-name; path guard (`Path.relative_to`); no secrets in tool observations / errors; shell opt-in.
4. **Honest product framing.** Python library ≠ Zig native CLI. Cite measured numbers only (`docs/benchmarks/phase2-numbers.md`, `10-fx-beat-goals.md`).
5. **Native rewrite is out of scope for this ADR** unless Harsha explicitly wants an fx-class binary (that is Candidate B / a separate ADR).
6. **Evidence gate for Accept.** Status stays **Proposed** until Thomas ships `docs/research/11-coding-agent-product.md` (or equivalent Accept-ready evidence) and Alice product-signs.

---

## Candidates

### Candidate A (recommended): Python library product + plugin coding agent

- Keep core frozen (ADR-001/002/003).
- Ship a **PyPI package** + thin **CLI entry** that loads plugins / examples (wrapper over import + argparse + `simple.Agent` or Registry+Runner).
- Coding tools (fs / shell / git / patch / search) remain **host plugins** behind deny-all + Jessie path guard.
- Optional **PyInstaller / PyOxidizer** (or similar) packaging experiment = **SHOULD**, not MUST. Document size vs fx **honestly** (will not match 6–11 MiB native without a different stack).
- **Native rewrite = NON-GOAL** for this ADR. Separate ADR only if product goal becomes “fx-class binary.”

### Candidate B: Native rewrite of core (Rust / Zig) for µs / ms start

- Rewrite or sidecar the loop as a native binary to chase fx-class process start and binary MiB.
- Bigger bet; only justified if the product goal is explicitly “beat fx process start / binary size.”
- Leaves the Python library behind or dual-maintains two cores.

### Candidate C: Grow Python core with MCP / shell / sandbox

- Move coding I/O, MCP client, and/or sandbox into core so the coding agent “just works” without plugins.
- Rejected under ADR-003 freeze and Alice Phase 2 must-nots.

---

## Selected recommendation

**Candidate A** — Python library product + plugin coding agent.

Status remains **Proposed** until Open items below clear.

---

## Why (A)

| Reason | Detail |
|--------|--------|
| Matches Accepted ADRs | Core stays tiny; façade + plugins carry product surface. |
| Matches Alice Phase 2 | Coding agent app **on** harness; stranger install → examples → plugin coding loop. |
| Honest vs fx | Compete on library axes (import, dispatch, LOC, deps). Do not market native CLI wins we do not have. |
| Measured floor already green | ~27 ms import, ~122k/s dispatch, 520 LOC, 0 deps (Phase 2 numbers) — good library story without a rewrite. |
| Security path clear | Jessie gates stay at plugin boundary; no new core bypass. |
| Escape hatch preserved | Native rewrite can still be a **later** ADR if Harsha wants fx-class binary. |

---

## Rejected

| Candidate | Why rejected (for this ADR) |
|-----------|------------------------------|
| **B — Native rewrite** | Different product. Only if goal is fx-class start/size. Costly dual-stack; not required for Phase 2 coding-agent-via-plugins. |
| **C — Grow core (MCP/shell/sandbox)** | Violates ADR-003 freeze and `useful-scope-phase2.md` must-nots. Convenience ≠ missing primitive. |

---

## Security (Jessie)

Gates **stay**; this ADR does not weaken them:

1. **Deny-all** default; host Allow each coding tool by name.
2. **Path guard** on workspace fs plugins (`Path.relative_to` — no prefix/`startswith` bugs).
3. **Shell opt-in** — not in default pack; still gated.
4. **Secret scrub** on tool errors / observations (existing Jessie patterns).
5. **No OS sandbox / seccomp claim** in this ADR — later plugin ADR if needed.
6. Jessie reviews coding-plugin confinement before Accept → ship.

---

## Open

- [ ] Thomas: `docs/research/11-coding-agent-product.md` (Accept evidence). Until then, cite [`10-fx-beat-goals.md`](../research/10-fx-beat-goals.md); this ADR **awaits Thomas 11** for Accept.
- [ ] Alice: product sign-off on Candidate A (PyPI + CLI + plugins; packaging SHOULD; native NON-GOAL here).
- [ ] Jessie: security review of coding plugin pack + CLI entry surfaces.
- [ ] Harsha: confirm native rewrite stays out of scope (else open Candidate B ADR separately).

---

## Implementation plan (Mike — **after** Accept only)

1. PyPI package metadata / entry points for library + thin CLI that loads plugins (no core growth).
2. Keep coding tools in plugin pack (fs / shell / git / patch / search); wire examples to deny-all + Allow lists.
3. Document install → one-file Agent → coding-agent example path.
4. Optional SHOULD: PyInstaller/PyOxidizer spike; publish measured artifact size vs fx numbers with explicit “not apples-to-apples” note.
5. Hold Phase 2 MUST benches green (import ≤ 50 ms, dispatch ≥ 1000/s, LOC ≤ 800, deps `[]`).
6. No core protocol changes unless Thomas files a missing-primitive + follow-up ADR.

---

## Review checklist

| Reviewer | Status | Notes |
|----------|--------|-------|
| Alice (product) | | |
| Thomas (evidence / 11) | | |
| Jessie (security) | | |
| Mike (impl readiness) | | |
| Walter (draft) | Proposed draft | |

---

## Consequences (if Accepted later)

- Product story = **embeddable Python harness + coding plugins**, not “fx killer.”
- Marketing and docs must cite fair axes only; never claim 2 ms process start or 6 MiB binary for the Python package.
- Native / Zig / Rust rewrite requires a **new** ADR (Candidate B track).

## Review trail

- [Mike](ADR-004-mike-review.md) — Approve Candidate A; hold impl until Accept
