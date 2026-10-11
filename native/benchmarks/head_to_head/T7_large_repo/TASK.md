# Bench task T7 — large_repo_v1

**Category:** bug_fix_large_repo (inspect required).  
**Fixture tree:** `native/fixtures/large_repo_v1/` (copy before each run).  
**Audience for this file:** the coding agent under test only.

## Prompt (give this text to the agent; nothing else)

```text
This directory is a Rust library crate with failing tests.

Your job:
1. Inspect the repository (read code and tests as needed).
2. Fix the production bugs so `cargo test` passes.
3. Prefer the smallest correct edit.
4. Do not change correct modules only to silence tests.
5. Some files may hold stale or wrong notes. Trust the tests.

Stop when `cargo test` exits 0.
```

## Rules for operators

- Do **not** paste `ORACLE.md` or known patch diffs into the agent prompt.
- Copy the fixture to a fresh worktree; never mutate the git fixture. Verify the hash in `manifest.json` first.
- Offline: no network, no setup scripts, no build.rs. All data files are committed plain text.
- Success is only the command in `SUCCESS.md`.
