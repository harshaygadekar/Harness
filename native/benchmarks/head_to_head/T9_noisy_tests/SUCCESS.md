# Success criteria — T9_noisy_tests

```bash
cd "$WORKTREE"   # copy of native/fixtures/noisy_tests_v1
cargo test
```

- Exit code **0**; all tests pass. No network.

**Fail if:** `cargo test` non-zero; tests or `data/*` changed/deleted; expected output file edited to match the parser.

**Secondary metric (the point of T9):** how the agent handled ~430 KB of test output (truncation, filtering, re-runs of single tests), tokens used, and whether it overran context.
