# Oracle — T1_broken_calc (OPERATORS ONLY — never give to the agent)

## Fixture identity

| Field | Value |
|-------|--------|
| Path | `native/fixtures/broken_calc/` |
| Category | T1 bug fix |
| Intended bugs | `percent` (zero-whole + truncating div); `midpoint` (overflowing add) |
| Decoys | `tax.rs`, `noise.rs`, `docs/`, `notes/`, `data/` |

## Fixture content hash (fill at freeze)

```bash
# Run from repo root after freezing the fixture tree (exclude target/)
find native/fixtures/broken_calc -type f ! -path '*/target/*' \
  | sort | xargs sha256sum | sha256sum
# Record as fixture_content_sha256 below when freezing a bench row.
```

`fixture_content_sha256`: `e72e0c456c320fe3b1032db6dc9f812c31e70c57a0af52a97b228f9b81bcbc19`

## Expected production fixes (grader knowledge)

1. `percent(part, whole)` → `None` when `whole == 0`.
2. `percent` rounds half away from zero to nearest `i32`.
3. `midpoint(lo, hi)` overflow-safe when `lo <= hi` (e.g. `lo + (hi - lo) / 2`).

## Useful-action labels (for tool-intelligence scoring)

Label **useful** if any of:

- First successful read of `src/percent.rs` or `src/midpoint.rs`.
- Successful write/patch to those files that later appear in the green diff.
- `run_tests` that first shows the failing test names, or that flips fail→pass.

Label **unnecessary** if:

- Redundant read/search of the same path/query with unchanged result.
- Edits only under `docs/`, `notes/`, `data/`, or `tax.rs`/`noise.rs` that are not required for green tests.
- Identical `run_tests` with identical fail output as an earlier call.

## Scripted / fake-model path (Benchmark A — runtime)

Use a fixed tool sequence that: search/list → read buggy files → patch → `run_tests` pass.  
No live model. Compare Harness vs fx **only** if fx can run an equivalent scripted/fake path; otherwise report Harness-only runtime overhead and mark fx as **N/A (no fake-model API)**.
