# Oracle — T1 v2 broken_calc_v2 (OPERATORS ONLY — never give to the agent)

**Why v2:** v1 (`native/fixtures/broken_calc`) had fix hints inside the agent's tree: `BUG:` comments, a test comment with the overflow-safe formula, a README with the requirements, notes that named the real bug files, and "correct, do not change" headers. Every v1 row is labeled "T1 v1 (bugs labeled in source)".

**v2 changes:** same code bugs and same test asserts. All fix hints removed. No README in the tree. Decoys still point to `tax.rs` and `arithmetic.rs` but no file says they are wrong. Prompt line 5 now says "Trust the tests" instead of naming decoy folders.

| Field | Value |
|-------|-------|
| Path | `native/fixtures/broken_calc_v2/` |
| fixture_content_sha256 | `3180ddf315384f5e66015878ff8a1b6e4e5ebfabb94c6203eff80a5eb014f7b6` |
| Baseline | `cargo test`: 5 passed, 5 failed (same names as v1) |
| Intended bugs | `percent` (zero whole, truncating division); `midpoint` (overflowing add) |

Hash command (repo root, exclude target/):

```bash
find native/fixtures/broken_calc_v2 -type f ! -path '*/target/*' | sort | xargs sha256sum | sha256sum
```

Useful / unnecessary labels: same as v1 `ORACLE.md`. An edit outside `percent.rs` and `midpoint.rs` is an extra edit. A comment-only extra edit is cosmetic, not a quality fail.
