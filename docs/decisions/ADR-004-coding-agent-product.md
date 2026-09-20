# ADR-004: Coding-agent product shape

| Field | Value |
|-------|-------|
| **Status** | **Accepted** |
| **Date** | 2026-09-20 (IST) |
| **Owners** | Walter (draft), Alice (product), Thomas (evidence), Mike (impl after Accept), Jessie (security) |
| **Related** | [ADR-001](ADR-001-core-loop.md), [ADR-002](ADR-002-model-adapter.md), [ADR-003](ADR-003-protocol-freeze.md), [docs/useful-scope-phase2.md](../useful-scope-phase2.md), [docs/research/10-fx-beat-goals.md](../research/10-fx-beat-goals.md), [docs/research/11-coding-agent-product.md](../research/11-coding-agent-product.md) (Thomas Accept evidence). |

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
6. **Evidence gate.** Thomas `11-coding-agent-product.md` landed. Jessie security conditions bind through ship. Alice product sign-off tracked in review trail.

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

**Accepted** — Candidate A. Open items below are ship follow-ups, not reopen of the product shape.

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

1. **Deny-all** default; host Allow each coding tool by name. **CLI must not silently Allow all coding tools on start.**
2. **Path guard** on workspace fs plugins (`Path.relative_to` — no prefix/`startswith` bugs).
3. **Shell opt-in** — not in default pack; still gated by name.
4. **Secret scrub** on tool errors / model-adapter errors; CLI logs / crash dumps must not print keys.
5. **In-process plugins = full trust.** Document that. Process isolation = later ADR.
6. **No OS sandbox claim** for Candidate A. Do not market “sandboxed like fx” without a sandbox plugin + tests.
7. **CLI ship gate (Jessie):** Allow-list wiring, workspace root binding, no secrets in `--help` / env echo / error paths — before release, not a block on this Accept.

---

## Open (ship follow-ups)

- [x] Thomas: `docs/research/11-coding-agent-product.md` (Accept evidence).
- [ ] Alice: product sign-off note (lock already matches Candidate A).
- [x] Jessie: Candidate A approve + conditions (CLI ship tests remaining).
- [ ] Mike: CLI + PyPI after Accept; packaging SHOULD spike.
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
| Alice (product) | pending note | Phase 2 lock already = Candidate A |
| Thomas (evidence / 11) | approve | `11-coding-agent-product.md` |
| Jessie (security) | approve | conditions above; CLI ship gate |
| Mike (impl readiness) | approve | hold CLI/pack until Accept — now clear |
| Walter (draft) | Accepted | Candidate A |

---

## Consequences

- Product story = **embeddable Python harness + coding plugins**, not “fx killer.”
- Marketing and docs must cite fair axes only; never claim 2 ms process start or 6 MiB binary for the Python package.
- Native / Zig / Rust rewrite requires a **new** ADR (Candidate B track).

## Review trail

- [Mike](ADR-004-mike-review.md) — Approve Candidate A; hold impl until Accept
- [Thomas](ADR-004-thomas-review.md) — Approve Candidate A; `11` evidence
- [Jessie](ADR-004-jessie-review.md) — Approve Candidate A; CLI ship conditions

- [Alice](ADR-004-alice-review.md) — Approve Candidate A; product lock Locked
