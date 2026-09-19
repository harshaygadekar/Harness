# Phase 2 bench numbers (fake model)

**Owner:** Mike  
**Date:** 2026-09-19 (IST night push)  
**Rule:** These numbers measure our **Python library + façade**. They are **not** a claim that we beat fx.sh (Zig native coding-agent product).

## Reproduce

```bash
PYTHONPATH=src:. .venv/bin/python benchmarks/cold_import.py
PYTHONPATH=src:. .venv/bin/python benchmarks/agent_loop.py
PYTHONPATH=src:. .venv/bin/python scripts/test_plugin_conformance.py
.venv/bin/python -m pytest -q
```

## Results (this machine)

| Metric | Value |
|--------|-------|
| Cold `import agentic_harness` median | **28.8 ms** |
| Agent run (no tools) median | **0.006 ms** |
| Agent run (1 tool call) median | **0.010 ms** |
| Agent run (5 tool calls) median | **0.039 ms** |
| 200 × one-tool runs | **0.002 s** (~95k runs/s) |
| Plugin conformance | **PASS** |
| Pytest | **19 passed** |

## What this means

- Loop + gate + scrub overhead is tiny once Python is up.
- Cold start is dominated by Python process + import (~30 ms), not the harness loop.
- fx.sh ships a **native binary product** (~6–8 MiB) with built-in tools/MCP/UX. Different job. See `docs/research/08-harness-comparison.md` and Thomas’s fx beat goals.

## Coding plugins shipped (host-side, not core)

`plugins/coding/`: `list_files`, `read_file`, `write_file`, `search`, `apply_patch`, `git_status`, optional `run_shell` (off by default).
