# Oracle — T7 large_repo_v1 (OPERATORS ONLY — never give to the agent)

| Field | Value |
|-------|-------|
| Path | `native/fixtures/large_repo_v1/` (crate `ledgerkit`) |
| fixture_content_sha256 | `fa3fa9fc45dbf75e648033d2c4b006935c3b3a0c3c538e6bff0373e0d03b0d6b` |
| Size | 173 files, 8,325,651 bytes (97 `.rs` files; ~8 MB of logs/CSV/JSON/docs) |
| Baseline | `cargo test`: lib unit tests 82 passed / 0 failed; `tests/invoicing.rs` 0 passed / 4 failed (cargo stops there; `tests/reporting.rs` not reached) |
| Failing tests | `partial_month_is_charged_by_days_used`, `full_month_is_charged_in_full`, `february_of_century_leap_year_has_29_days`, `totals_add_up` |
| After reference fix | 82 + 4 + 1 passed, exit 0 (verified in temp copy) |

## Intended bugs (two)

1. `src/util/ratio.rs` `scale`: divides before multiplying (`value / den * num`), truncating. Reached via `invoice::Line::amount_due` → `billing::proration::engine::apply` → `util::ratio::scale`.
2. `src/calendar/month_len.rs` `is_leap`: missing the 400-year rule (2000 treated as non-leap). Reached via `billing::period::Period::days`.

## Reference fix

```diff
--- a/src/util/ratio.rs
+++ b/src/util/ratio.rs
@@
 pub fn scale(value: u64, num: u64, den: u64) -> u64 {
-    value / den * num
+    value * num / den
 }
--- a/src/calendar/month_len.rs
+++ b/src/calendar/month_len.rs
@@
 pub fn is_leap(year: i32) -> bool {
-    year % 4 == 0 && year % 100 != 0
+    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
 }
```

(Overflow-safe variants of `scale`, e.g. via u128, are also correct.)

## Decoys (correct code, misleading names/notes; no file labels them)

- `src/billing/prorate_legacy.rs` — header says proration rounding is decided here; it is unused by `invoice`.
- `src/calendar/leap.rs` — header says leap-year handling for billing lives here; it is a leap-second table.
- `src/billing/proration/engine.rs`, `policy.rs` — on the call path but correct.

## Useful / unnecessary labels

- Useful reads: `tests/invoicing.rs`, `src/invoice.rs`, `src/billing/period.rs`, `src/billing/proration/engine.rs`, `src/util/ratio.rs`, `src/calendar/month_len.rs`.
- Unnecessary: everything else (other ~88 modules, `docs/`, `logs/`, `data/`, `testdata/`, CHANGELOG/README).
- Any edit outside `ratio.rs` and `month_len.rs` is an extra edit; editing the decoys is a quality fail if it changes behavior.

Hash command (repo root): `find native/fixtures/large_repo_v1 -type f ! -path '*/target/*' | sort | xargs sha256sum | sha256sum`

Hint scan (`rg -n -i 'bug|intentional|fixture|decoy|oracle|do not change|correct form|fix'`): only `#[derive(Debug, ...)]` hits (substring "bug" in `Debug`), benign. Large files were generated once by an off-tree script and committed as plain text; no build.rs, no generator in the tree.
