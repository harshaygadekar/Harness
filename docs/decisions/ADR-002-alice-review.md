# ADR-002 — Alice product review

**Date:** 2026-09-19 (IST)  
**Verdict:** Approve.

## Checks

| Lock | Pass |
|------|------|
| Providers stay plugins (no Gemini in core) | Yes |
| Fake model remains CI default | Yes |
| Adapter does not register shell / finish / product tools | Yes |
| Matches first demo (swap model without growing core) | Yes |

## Nits (non-blocking)

1. README already says Gemini is optional — keep that as the only live-provider story until push.
2. Do not add a second provider to core “for convenience.” New providers = new plugins.
3. Streaming stays open; do not block the first useful demo on it.

Scope lock holds. No core growth.
