# Native Phase 3 vertical slice

**Status:** scripted loop measured locally. No beat-fx claim.

**Date:** 2026-09-21 IST  
**Owner:** Mike  
**Rule:** ASD-STE100 plain notes. No invented fx numbers.

## What this slice proves

One real agent loop on a tiny broken Rust lib:

USER TASK → MODEL → TOOLS → EDIT → TEST → FIX → SUCCESS

The model is **ScriptedModel** (fixed tool-call sequence). No live HTTP. This checks tools, gate, events, and the fixture path — not model quality.

## Fixture

`native/fixtures/broken_add/`

- `src/lib.rs` — `add(2,2)` returns 5 (bug: `a + b + 1`)
- Unit test expects 4
- Noise: `src/noise.rs`, `big_noise.txt`
- `Cargo.toml` has empty `[workspace]` so it does not join `native/` workspace


## How to run

```bash
cd native
cargo test -p harness-model scripted_broken_add_vertical_slice -- --nocapture
cargo build --release -p harness-cli

# Copy fixture so the tree stay broken in git, then ask:
FIX=$(mktemp -d)
cp -a fixtures/broken_add/. "$FIX/"
./target/release/harness ask \
  --model scripted \
  --allow coding,shell \
  --workspace "$FIX" \
  --prompt "fix the failing tests" \
  --record /tmp/harness-run

./target/release/harness replay /tmp/harness-run/run.jsonl
cargo test --manifest-path "$FIX/Cargo.toml" -q
```

Direct ask on `fixtures/broken_add` also works but **mutates** the fixture. Prefer a copy for repeat runs.

## What was measured (local)

| Item | Result | Notes |
|------|--------|-------|
| Scripted slice cargo test | pass | search → read → patch → run_tests |
| CLI `ask --model scripted` on fixture | pass | exit 0; 5 steps; run_tests ok |
| Events JSONL | written | `--record` or workspace `.harness/runs/` |
| Release binary | **3.335 MiB** (3414.5 KiB) | ureq+rustls OpenAI adapter linked |
| Live OpenAI | not required | adapter present; env-gated |
| Beat-fx | **not claimed** | method only |

Re-measure after change:

```bash
cd native && cargo test && cargo build --release -p harness-cli
./benches/size.sh
```

## Security (Jessie)

- Deny-all default
- `--allow coding` / `coding,shell`
- Workspace bind with `strip_prefix`
- Shell opt-in only
- Observation bound after scrub
- Not an OS sandbox

## Models

| `--model` | Use |
|-----------|-----|
| `fake` | default; CI smoke |
| `scripted` | Phase 3 fixture sequence |
| `openai` | `HARNESS_API_KEY` or `OPENAI_API_KEY`, `HARNESS_MODEL`, optional `HARNESS_BASE_URL` |
