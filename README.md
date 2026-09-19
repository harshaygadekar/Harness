# agentic-harness

Minimal **agentic harness**: a small core loop, tool protocol, deny-all gate, and plugins.

Everything else is a plugin. DeepSeek Harness inspired the seams — we did not vendor Cordis/`dsh`.

## Status

ADR-001 spike (Proposed). Stdlib only in core.

## Install (dev)

```bash
pip install -e ".[dev]"
pytest
python examples/echo_demo.py
PYTHONPATH=src python benchmarks/cold_import.py
```

## First demo

One model adapter + one user tool + `Runner.run`. No shell, MCP, or multi-agent in core.

Host **must allowlist tools**. Default policy denies every tool call.

## Docs

- `docs/useful-scope.md` — product lock
- `docs/decisions/ADR-001-core-loop.md` — core loop
- `docs/research/` — research pack
