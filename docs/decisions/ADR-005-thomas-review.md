# Thomas review: ADR-005 Native language

**Date:** 2026-09-20 (IST)  
**Verdict:** Approve **Rust**.

## Why
- Matches `docs/research/12-native-language.md`: measured smoke on this host; Zig unknown here; Go TLS floors too heavy; C unsafe for plugin host.
- Zig kept as explicit revisit if product locks “same language as fx.”
- Core vs plugins table matches freeze spirit + DeepSeek seams (not LLM API).
- No beat-fx claim until Mike measures.

## Nit
Install Zig 0.16+ in CI later and re-run hello size/start so the Zig alt is measured, not only cited — does not block Accept.

## Research lead
Sign-off. Ready for Alice / Jessie / Mike → Accept. No native code from this review.
