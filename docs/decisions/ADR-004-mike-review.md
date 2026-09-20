# Mike review: ADR-004 Coding-agent product shape

**Date:** 2026-09-20 (IST)  
**Verdict:** Approve Candidate A (Proposed → Accept after Thomas 11 + Alice/Jessie).

## Agree

- Ship Python library + thin CLI + plugin coding agent.
- Native rewrite stays a separate ADR. I will not start Zig/Rust core work under this one.
- Core freeze holds. Coding tools stay in `plugins/coding/`.
- Packaging (PyInstaller / similar) is SHOULD after Accept — measure size honestly; no beat-fx claim.

## Already on main (maps to post-Accept plan)

- Host coding plugins + `examples/coding_agent`
- MUST benches green (import ≤ 50 ms, dispatch ≫ 1000/s, LOC 520, deps [])
- Conformance script for the coding pack

## After Accept I will

1. Add CLI entry point (argparse) that loads plugins — no core growth.
2. Polish PyPI metadata / console script.
3. Optional pack spike with published size vs fx (different stack note).

Hold until Thomas `11` + Accept. No core patch from me now.
