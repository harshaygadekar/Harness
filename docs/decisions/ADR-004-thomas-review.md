# Thomas review: ADR-004 Coding-agent product shape

**Date:** 2026-09-20 (IST)  
**Verdict:** Approve Candidate A (Python library + plugin coding agent).

## Why
- Matches Accepted ADR-001/002/003 freeze. Coding power stays in plugins.
- Evidence in `docs/research/11-coding-agent-product.md`: fair axes only; myths corrected (fx **2 ms** CI, not 10 µs; **~122k/s** is ours; DeepSeek = design inspiration).
- `09-disprove-core.md`: no missing primitive for Phase 2 coding stub. Keep freeze.
- Option B (PyInstaller/Oxidizer) = packaging spike **after** Accept — not core growth.
- Option C (Rust/Zig rewrite) = separate ADR only if product demands fx-class process start.

## Nits
1. In ADR-004 body, label packaging (PyInstaller/Oxidizer) as **Option B / SHOULD spike**, and native rewrite as **Option C / separate ADR**, so names match research `11` (Walter’s draft Candidate B was “native”; research splits packaging vs rewrite).
2. Before Accept → ship: Alice product sign-off + Jessie plugin/CLI security pass (already in Open list).
3. Marketing copy must cite `10` / `phase2-numbers.md` only — never homepage “beat fx” language.

## Research lead
Sign-off for Candidate A. Ready for Alice + Jessie; Mike after Accept only. No core code from this review.
