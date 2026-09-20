# ADR-005 — Alice product review

**Date:** 2026-09-20 (IST)  
**Verdict:** Approve **Rust**.

## Checks

| Lock | Pass |
|------|------|
| Separate product under `native/` | Yes |
| Python Harness frozen | Yes |
| No beat-fx without Mike benches | Yes |
| Zig remains measured revisit | Yes |
| Jessie conditions bind | Yes |

## Product framing

We sell a **tiny native coding CLI** (Rust). We keep the Python library as the reference harness.

Success = real-repo coding loop + tool-efficiency benches, not feature parity with fx.

Scope: `docs/useful-scope-native.md` → Locked.

No Python core growth. Approve Rust.
