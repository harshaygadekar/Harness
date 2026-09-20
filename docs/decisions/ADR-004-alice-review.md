# ADR-004 — Alice product review

**Date:** 2026-09-20 (IST)  
**Verdict:** Approve Candidate A.

## Checks

| Lock | Pass |
|------|------|
| Coding agent on harness via plugins | Yes |
| Core freeze (no MCP/shell in core) | Yes |
| No beat-fx claim without numbers | Yes |
| Native rewrite = later ADR, not this cut | Yes |
| Jessie CLI ship gates bind | Yes |

## Product framing

We sell: **Python library + thin CLI + coding plugins**.

We do not sell: Zig-class native binary, feature parity with fx, or “safer than fx” without Jessie’s threat model.

Scope file: `docs/useful-scope-coding-product.md` → Status Locked.

## Ship nits (non-blocking on Accept)

1. CLI must not silently Allow all coding tools (Jessie).
2. README first screen: install → coding example → non-goals vs fx.
3. Packaging spike (PyInstaller etc.) is SHOULD — document size honestly.

No core growth. Approve A.
