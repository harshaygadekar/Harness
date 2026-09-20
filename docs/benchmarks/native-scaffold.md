# Native benches (ADR-005)

**Status:** measured smoke on scaffold+min pack. No beat-fx claim.

**Machine:** Grok Bot Linux box (shared). **Method:** `cargo build --release -p harness-cli`, `./benches/size.sh`, `./benches/startup.sh 30`.

## Numbers (local, 2026-09-20 IST)

| Metric | Value | Notes |
|--------|-------|-------|
| Release binary | **1.924 MiB** (1,970.5 KiB) | After linking `regex` for search; was ~635 KiB scaffold-only |
| Startup median | **1.029 ms** | `harness version`, n=30 |
| Startup p95 | **1.190 ms** | same |
| Core+tools tests | 5 passed | deny-all, allowlist echo, escape, read/search/patch |
| Python harness | 27 passed | frozen |

## Commands

```bash
cd native
cargo test
cargo build --release -p harness-cli
./benches/size.sh
./benches/startup.sh 50
./benches/tool_smoke.sh
```

## Tool-efficiency

Method: [`docs/research/13-tool-efficiency.md`](../research/13-tool-efficiency.md). Fake-model scripted tasks not landed yet. Mark measured vs scripted when they land.

## Security note

Not an OS sandbox. Deny-all CLI default. Workspace path bind. `--shell` alone does not open the gate; need `--allow …` including `shell` / `run_shell`.
