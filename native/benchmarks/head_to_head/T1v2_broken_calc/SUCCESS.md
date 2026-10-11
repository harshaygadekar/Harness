# Success criteria — T1v2_broken_calc

**Deterministic check (oracle):**

```bash
cd "$WORKTREE"   # copy of native/fixtures/broken_calc_v2
cargo test
```

- Exit code must be **0**.
- All library unit tests must pass.
- No network required for the check.

**Fail if:**

- `cargo test` non-zero.
- Agent deletes or empties tests to force green.
- Agent edits only decoy modules and leaves `percent` / `midpoint` bugs.

**Optional integrity (operator):**

```bash
# After the run, confirm tests still assert the intended behaviors
# (operators may open src/*_tests inside percent.rs / midpoint.rs).
# Agents must not receive this file.
rg -n 'percent_zero_whole|percent_rounds|midpoint_no_overflow' "$WORKTREE/src"
```
