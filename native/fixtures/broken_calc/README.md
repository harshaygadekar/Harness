# Fixture: broken_calc (Phase 3 / T1)

**Category:** T1 bug fix (tool-efficiency).  
**Owner:** Thomas (fixtures + metric schema).  
**Rule:** No secrets. Copy the tree before an agent run so git stays broken.

## Task prompt (give this to the agent)

```text
Fix the failing unit tests in this Rust crate.

Requirements:
1. `percent(part, whole)` must return None when whole is 0.
2. `percent` must round to the nearest integer (half away from zero).
   Example: percent(2, 3) == Some(67); percent(-2, 3) == Some(-67).
3. `midpoint(lo, hi)` must not overflow near i32::MIN / i32::MAX.
   Use an overflow-safe form such as lo + (hi - lo) / 2 (for lo <= hi).

Do not change correct modules only to silence tests. Prefer the smallest
edit that makes `cargo test` pass. Ignore decoy docs under docs/ and notes/.
```

## Layout (search matters)

| Path | Role |
|------|------|
| `src/percent.rs` | **Buggy** percent (zero-whole + truncating division) |
| `src/midpoint.rs` | **Buggy** midpoint (overflowing add) |
| `src/arithmetic.rs` | Correct basic ops |
| `src/tax.rs` | Correct decoy (misleading NOTES point here) |
| `src/noise.rs` | Misleading strings |
| `docs/NOTES.md`, `notes/OLD_BUGS.md` | Stale / wrong advice |
| `data/*` | Irrelevant ledger noise |

## Verify locally

```bash
cd native/fixtures/broken_calc
cargo test
# Expect failures in percent_* and midpoint_no_overflow_*
```

After a correct fix, the same command must exit 0.

## Harness tip

Copy before `harness ask` so the canonical fixture stays red:

```bash
FIX=$(mktemp -d)
cp -a native/fixtures/broken_calc/. "$FIX/"
# then ask with --workspace "$FIX"
```

No beat-fx claim. Fake-model / scripted runs preferred for CI metrics.
