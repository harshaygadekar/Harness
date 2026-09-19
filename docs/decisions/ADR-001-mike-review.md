# Mike review: ADR-001 Core loop

**Date:** 2026-09-19 (IST)  
**Verdict:** Approve Candidate A for spike. Hold code until Status = Proposed → Accepted after Jessie C1–C5 land in tests.

## Implementability

- Interfaces are enough to code a ≤200 LOC `Runner` + registry + gate.
- No provider SDK in core. Gemini adapter is a plugin when Harsha gives a key.
- Sequential tools in v0 matches my benchmark plan (simple timing, no race harness).

## Agree

- Thomas: Status → Proposed until Jessie + Mike sign; define side-effect / gate rule; no `finish` tool in spike.
- Jessie C1–C5: default deny **all** tools; no bypass path; concrete scrub patterns; plugins = full trust in-process; cancel stops new tool calls.
- Alice first demo: one model adapter + one user tool + loop.

## Spike plan (after ADR Accepted)

1. `src/core/`: protocols, registry, Runner, sanitizer, gate (stdlib only).
2. Tests: deny-all default, scrub secret, no shell import in core, cancel, plugin disposer.
3. `examples/`: echo-tool demo with a fake ModelClient (no network).
4. Benchmarks: cold import, dep count, one-tool loop vs raw fake client.
5. Ask Harsha in this group for Gemini key only when the fake-client path is green.

## Ask (group)

@Walter please flip Status to Proposed and add Jessie C1 (deny all tools by default, host allow-list) into the ADR body. Then I spike.

Mike
