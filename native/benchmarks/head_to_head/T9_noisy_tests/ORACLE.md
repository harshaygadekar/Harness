# Oracle — T9 noisy_tests_v1 (OPERATORS ONLY — never give to the agent)

| Field | Value |
|-------|-------|
| Path | `native/fixtures/noisy_tests_v1/` (crate `tabular`) |
| fixture_content_sha256 | `1af0af8a77913f599095320f0bd87766c77de53d91c4043a4dbb88a6a082a078` |
| Size | 11 files, 365,629 bytes |
| Baseline | `cargo test` exit 101: `tests/generated.rs` 540 passed / 0 failed; `tests/orders.rs` 1 passed / 2 failed |
| Failing tests | `orders_file_parses_completely` (3000-row Vec<Vec<String>> assert_eq diff + 3000 println lines), `quoted_speech_is_preserved` |
| Baseline output size | **431,640 bytes** (`cargo test > out 2>&1`, warm build; 431,906 with `--no-fail-fast`) |
| After reference fix | 540 + 3 passed, exit 0 (verified in temp copy) |

## Intended bug (one)

`src/reader.rs` `parse_record`: inside a quoted field, an escaped `""` is consumed but no `"` is pushed, so `say "x" now` parses as `say x now`.

## Reference fix

```diff
--- a/src/reader.rs
+++ b/src/reader.rs
@@ -15,6 +15,7 @@
             if c == '"' {
                 if chars.peek() == Some(&'"') {
                     chars.next();
+                    cur.push('"');
                 } else {
                     in_quotes = false;
                 }
```

## Decoys / labels

- No labeled decoys. `src/writer.rs` and `src/stats.rs` are correct; the 540 generated tests are noise (none contain quotes).
- Useful: `tests/orders.rs`, `src/reader.rs`, a few lines of `data/orders.csv`. Unnecessary: `tests/generated.rs`, full data files, `writer.rs`, `stats.rs`.
- Editing `data/orders_expected.txt` or tests = fail.

Hash command (repo root): `find native/fixtures/noisy_tests_v1 -type f ! -path '*/target/*' | sort | xargs sha256sum | sha256sum`

Hint scan: zero hits. Data/tests generated once by an off-tree script, committed as plain text; no build.rs.
