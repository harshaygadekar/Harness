# Phase 2 numbers vs MUST goals

**Owner:** Mike  
**Date:** 2026-09-19 IST  
**Commit:** (fill on push)  
**Rule:** Meet our budgets. Do **not** claim we beat fx.

| Goal | Target | Measured | Pass |
|------|--------|----------|------|
| P2-2 deps | `[]` | `[]` | YES |
| P2-3 core LOC | ≤ 800 | **520** | YES |
| P2-4 cold import | ≤ 50 ms | **~27 ms** median | YES |
| P2-5 dispatch/s | ≥ 1000 | **~122k** /s (1000 steps, 8 no-op tools) | YES |
| P2-6 plugin churn | ≤ 5 ms / 100 | **~0.06 ms** | YES |
| P2-7 fake coding loop | completed via plugins | **completed**, 4 steps | YES |

Reproduce: see `benchmarks/README.md`.
