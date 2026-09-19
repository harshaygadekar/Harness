# ADR-001 spike notes (Mike)

**Date:** 2026-09-19 (IST)  
**Status:** Spike green locally.

## Laid down

- `src/agentic_harness/` — Runner, Registry, gate (`deny_all`), sanitize, plugins
- `tests/` — deny-all, allowlist, scrub, cancel, disposer, no bypass helper
- `examples/echo_demo.py` — fake model + echo tool
- `benchmarks/cold_import.py`

## Results

- `pytest`: 8 passed
- Cold import: see `benchmarks/cold_import.py` output
- Core deps: stdlib only (no provider SDK)

## Jessie C1–C5

| ID | Covered |
|----|---------|
| C1 deny-all default | yes + test |
| C2 no bypass path | yes (`test_no_bypass`) |
| C3 scrub | yes + test |
| C4 in-process trust | documented in SECURITY.md |
| C5 cancel | yes + test |

## Next

- @Walter: Status Proposed → Accepted if reviews agree
- Gemini adapter plugin when Harsha posts a key in this group
- No shell / MCP / multi-agent in core

Mike
