# Native Harness (ADR-005)

Separate **Rust** coding-agent product. Python harness under `src/agentic_harness/` stays frozen.

## Build

```bash
cd native
cargo test
cargo build --release -p harness-cli
./target/release/harness version
./target/release/harness run --prompt "hi"                 # deny-all
./target/release/harness run --prompt "hi" --allow echo
./target/release/harness run --prompt "hi" --allow coding --workspace .
./target/release/harness run --prompt "hi" --allow coding,shell --workspace .
```

## Phase 3 — ask / replay / scripted fixture

```bash
# Scripted vertical slice (copies recommended; see docs/benchmarks/native-phase3-slice.md)
FIX=$(mktemp -d)
cp -a fixtures/broken_add/. "$FIX/"
./target/release/harness ask \
  --model scripted \
  --allow coding,shell \
  --workspace "$FIX" \
  --prompt "fix the failing tests" \
  --record /tmp/harness-run

./target/release/harness replay /tmp/harness-run/run.jsonl

# Models: fake (default) | scripted | openai
# openai needs HARNESS_API_KEY|OPENAI_API_KEY, HARNESS_MODEL, optional HARNESS_BASE_URL
```

## Crates

| Crate | Role |
|-------|------|
| `harness-core` | Loop, messages, ToolSpec, RunEvent, deny-all gate, fake model |
| `harness-tools` | Coding pack: list/search/read/write/patch/run_tests/git_*; `run_shell` opt-in |
| `harness-model` | OpenAI-compat adapter + ScriptedModel (HTTP outside core) |
| `harness-cli` | `harness` binary: version / run / ask / replay |

## Security (Jessie)

- Deny-all tools by default
- Explicit `--allow` list (`coding` expands the min pack)
- Workspace path bind (no `..` escape; `strip_prefix`)
- Shell only when `run_shell` / `shell` is allowed — not an OS sandbox
- Light secret scrub + bounded tool observations

## Benches

```bash
./benches/size.sh
./benches/startup.sh 50
./benches/tool_smoke.sh
```

Phase 3 notes: `docs/benchmarks/native-phase3-slice.md`. No beat-fx claim.

## Status

Phase 3 vertical slice: scripted MODEL→TOOLS→EDIT→TEST on `fixtures/broken_add`.
