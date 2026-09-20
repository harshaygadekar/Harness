# Native Harness (ADR-005)

Separate **Rust** coding-agent product. Python harness under `src/agentic_harness/` stays frozen.

## Build

```bash
cd native
cargo build --release -p harness-cli
./target/release/harness version
./target/release/harness run --prompt "hi"                 # deny-all
./target/release/harness run --prompt "hi" --allow echo   # allowlisted echo
cargo test -p harness-core
```

## Security (Jessie)

- Deny-all tools by default
- Explicit `--allow` list
- Not an OS sandbox; in-process tools = host trust
- Scrub light secrets in tool observations

## Status

Scaffold only. Min coding tool pack + real-repo loop come next. No beat-fx claim until benches land.
