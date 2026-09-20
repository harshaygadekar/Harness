# Mike review: ADR-005 Native language

**Date:** 2026-09-20 (IST)  
**Verdict:** Approve **Rust**.

## Agree

- Separate `native/` product. Python harness stays frozen.
- Rust matches what I can ship: cargo, ureq/rustls, traits for tools, later Wasmtime if needed.
- Zig stays the measured revisit if we lock “same language as fx.”
- No beat-fx claim until I publish native benches (method + SHA + machine).

## After Accept I will

1. Scaffold `native/` (cli, runtime loop, tool protocol, deny-all gate).
2. Min coding tool pack as plugins/capabilities (fs/search/read/write/patch/shell opt-in).
3. Benches: binary MiB, process start, fake-model dispatch, tool-efficiency tasks.
4. Hold OS sandbox / MCP / heavy TUI for later ADRs.

No native source until this ADR is Accepted.
