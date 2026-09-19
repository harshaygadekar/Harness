# Benchmarks

**Owner (impl):** Mike  
**Goals:** [`docs/research/10-fx-beat-goals.md`](../docs/research/10-fx-beat-goals.md)  
**Rule:** Fake model only. No “we beat fx” unless a DoD row in that doc passes.

| Script | Goal | Status |
|--------|------|--------|
| `cold_import.py` | P2-4 MUST import ≤ 50 ms | **Done** |
| `tool_dispatch.py` | P2-5 MUST ≥ 1000 gated dispatch/s | **Done** |
| `plugin_churn.py` | P2-6 SHOULD ≤ 5 ms for 100 plugins | **Done** |
| `fake_coding_loop.py` | P2-7 SHOULD coding via plugins | **Done** |
| `size_gate.py` | P2-2/P2-3 MUST deps=0, core LOC ≤ 800 | **Done** |
| `agent_loop.py` | Extra façade timings | Done |

```bash
PYTHONPATH=src:. python benchmarks/cold_import.py
PYTHONPATH=src:. python benchmarks/tool_dispatch.py --steps 1000 --tools 8
PYTHONPATH=src:. python benchmarks/plugin_churn.py --n 100
PYTHONPATH=src:. python benchmarks/fake_coding_loop.py --max-steps 20
PYTHONPATH=src:. python benchmarks/size_gate.py
```
