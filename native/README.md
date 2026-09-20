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

## Crates

| Crate | Role |
|-------|------|
| `harness-core` | Loop, messages, tools trait, deny-all gate, fake model |
| `harness-tools` | Min coding pack: list/search/read/write/patch; `run_shell` opt-in |
| `harness-cli` | `harness` binary |

## Security (Jessie)

- Deny-all tools by default
- Explicit `--allow` list (`coding` expands the min pack)
- Workspace path bind (no `..` escape)
- Shell only when `run_shell` / `shell` is allowed — not an OS sandbox
- Light secret scrub in tool observations

## Benches

```bash
./benches/size.sh
./benches/startup.sh 50
./benches/tool_smoke.sh
```

Tool-efficiency method: `docs/research/13-tool-efficiency.md` (Thomas). No beat-fx claim until SHA + machine + scripts publish.

## Status

Min coding pack + size/start smoke. Fake-model tool-efficiency tasks next.
