# Thomas review: ADR-001 Core loop

**Date:** 2026-09-19 (IST)  
**Verdict:** Approve Candidate A (tiny protocol core).

## Aligns with research
- Matches `02` / `05` / `06`: ReAct loop, protocols, registry, hooks, no Cordis vendor.
- Keep-out list matches `04` and Jessie/Alice locks.
- DeepSeek = design inspiration only. Correct.

## Accept
- Loop in core (not removable plugin) for v0 size.
- Deny-by-default `can_use_tool`; no shell / `python_eval` / MCP / crews in core.
- Sequential tools in v0; parallel deferred to ADR-010.
- Message list as v0 history; event log later as plugin.
- No LLM API used to invent the ADR.

## Nits before Mike codes
1. Change ADR **Status** from `Accepted` to `Proposed` until Mike + Jessie also sign; then `Accepted`.
2. Define **side-effect** for the gate in one sentence (e.g. any tool that writes disk, network, process, or credentials). Pure compute tools still need an explicit host Allow list — do not imply “read-only auto-runs.”
3. Keep stop-semantics (`05` Q8) open; do not add a `finish` tool in the first spike.

## Research lead
Sign-off for architecture direction. Ready for Jessie security pass and Mike spike after Status fix.
