# Harness

Harness is a native coding agent in Rust. The core is tiny. The tool gate is deny-all by default. A tool runs only when you allow it.

Repo: https://github.com/harshaygadekar/Harness

## Crate layout

The Rust workspace is in `native/`.

| Crate | Purpose |
|---|---|
| `harness-core` | Core loop, events, tool protocol, allow-list gate. No HTTP. |
| `harness-tools` | Coding tools (workspace, files, processes). |
| `harness-model` | Model adapters (OpenAI-compatible, scripted model for CI). |
| `harness-cli` | The `harness` binary. |

## Build and test

```bash
cd native
cargo build --release      # binary: target/release/harness
cargo test --workspace
```

Model keys come from the environment. Never commit `.env` or keys.

## Status

- Benchmarks against fx are in progress. We make no win claims yet.
- Size metrics: [docs/benchmarks/size-metrics.md](docs/benchmarks/size-metrics.md).
- The Python spike (`src/`, `tests/`) is frozen (ADR-004). New work goes into the Rust workspace.

## Docs

- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- [docs/decisions/](docs/decisions/) (ADRs)
- [SECURITY.md](SECURITY.md)
