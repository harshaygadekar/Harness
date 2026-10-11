# Bench task T1 v2 — broken_calc_v2

**Category:** bug fix (inspect required).  
**Fixture tree:** `native/fixtures/broken_calc_v2/` (copy before each run).  
**Audience for this file:** the coding agent under test only.

## Prompt (give this text to the agent; nothing else)

```text
This directory is a small Rust library crate with failing unit tests.

Your job:
1. Inspect the repository (read code and tests as needed).
2. Fix the production bugs so `cargo test` passes.
3. Prefer the smallest correct edit.
4. Do not change correct modules only to silence tests.
5. Some files may hold stale or wrong notes. Trust the tests.

Stop when `cargo test` exits 0.
```

## Rules for operators

- Do **not** paste `ORACLE.md`, this section’s layout table, or known patch diffs into the agent prompt.
- Do **not** paste the Phase 3 fixture README “Requirements” list (it leaks formulas).
- Success is only the command in `SUCCESS.md`.
