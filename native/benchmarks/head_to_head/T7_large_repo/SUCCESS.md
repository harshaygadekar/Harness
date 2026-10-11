# Success criteria — T7_large_repo

```bash
cd "$WORKTREE"   # copy of native/fixtures/large_repo_v1
cargo test
```

- Exit code **0**; all unit and integration tests pass. No network.

**Fail if:** `cargo test` non-zero; tests in `tests/` or unit tests deleted/weakened; expected values in tests changed.

**Secondary metric (the point of T7):** bytes/files the agent read before the first correct edit, and total tokens. Record from the agent transcript.
