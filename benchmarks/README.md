# Benchmarks (planned + existing)

**Owner (impl):** Mike  
**Goals doc:** [`docs/research/10-fx-beat-goals.md`](../docs/research/10-fx-beat-goals.md)  
**Rule:** Fake model only for CI budgets. No network. No “we beat fx” unless a DoD row in the goals doc passes.

These benches measure **our Python harness** on fair axes (import, dispatch, LOC, deps, plugin churn). They do **not** compete with fx’s native binary MiB or Linux **2 ms** process budget.

## Existing

| Script | What it measures | Status |
|--------|------------------|--------|
| `cold_import.py` | Median cold `import agentic_harness` (ms); bans provider SDKs on import | **Exists** — keep; wire to CI budget ≤ 50 ms (MUST) |

```bash
python benchmarks/cold_import.py
```

## Planned (Mike)

| Script | What it measures | Maps to goal | Suggested CLI |
|--------|------------------|--------------|---------------|
| `tool_dispatch.py` | Fake-model gated tool dispatch throughput | P2-5 MUST | `python benchmarks/tool_dispatch.py --steps 1000 --tools 8` |
| `plugin_churn.py` | Register + dispose N empty plugins | P2-6 SHOULD | `python benchmarks/plugin_churn.py --n 100` |
| `fake_coding_loop.py` | End-to-end fake coding loop (plugins supply tools) | P2-7 SHOULD | `python benchmarks/fake_coding_loop.py --max-steps 20` |
| `size_gate.py` | Core LOC ≤ 800; `dependencies == []` | P2-2 / P2-3 MUST | `python benchmarks/size_gate.py` |

## CI expectations (proposal)

- Run on Linux x86_64, CPython 3.11 and 3.12.
- Emit JSON lines or a small `benchmarks/results/` artifact.
- Fail the job on MUST budget misses after baselines are locked.
- Do **not** download or time the fx binary in the default PR check (different product). Optional nightly comparison job may record fx `FX_BENCH=1` help latency as **context only**.

## Out of scope here

- Live LLM latency or quality.
- Native binary size / Zig rewrite (Phase 3 STRETCH / product ADR).
- Claiming parity with fx’s full coding CLI tool set.
