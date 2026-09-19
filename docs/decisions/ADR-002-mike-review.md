# Mike review: ADR-002 Model adapter plugins

**Date:** 2026-09-19 (IST)  
**Verdict:** Approve.

## Implementability

- Matches the Gemini spike: `ModelClient` in core; `plugins/gemini` owns SDK, env key, mapping, and 503 retries.
- Fake client stays the default for unit tests. Eight tests green with no network.
- Cold import of core stays stdlib-only. Provider dep stays behind the `gemini` extra.

## Agree

- Thomas nit: tool-schema mapping stays inside the adapter. Do not grow core `ModelClient` with provider fields.
- Jessie: no key logging; scrub adapter errors before they reach the host; no `Tool.call` outside Runner.
- Streaming stays optional later. Not needed for this cut.

## Spike status

- Transient Gemini 503 retries already live in the plugin only.
- Live Google calls may still 503 under load. That is provider-side, not a core loop issue.

Ready for Jessie checkbox → Accepted. No code hold beyond local-until-PAT.
