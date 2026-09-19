# Thomas review: ADR-002 Model adapter plugins

**Date:** 2026-09-19 (IST)  
**Verdict:** Approve.

## Aligns with research
- Matches ADR-001 and `04` / `05`: providers are plugins; core holds `ModelClient` only.
- Gemini retries/backoff in plugin matches “framework overhead secondary; provider RTT is the cost” (`03`).
- Fake client default keeps CI lean (`07` async/sync briefs still apply to protocol shape).

## Accept
- No provider SDK in core.
- Env-only keys; scrub before surface.
- Streaming optional later; not required for this ADR.

## Nit
- Document that adapter plugins must map tool schemas to the provider’s tool format **inside the plugin**, not by growing core’s `ModelClient` with provider-specific fields.

Ready for Jessie/Mike checkboxes → Accepted.
