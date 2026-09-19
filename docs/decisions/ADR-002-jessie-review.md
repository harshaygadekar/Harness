# ADR-002 review — Jessie (security)

**Verdict:** Approve Proposed → Accepted after nits below.

## Agree

- Model adapters stay plugins. No provider SDK in core.
- Env-only keys. No key in logs.
- Retries only in the plugin.
- Fake client stays CI default.
- Adapters must not register shell / `python_eval` / finish tools.
- No `Tool.call` outside `Runner`.

## Nits (block Accepted until fixed or tracked)

1. **Sanitize before raise.** Live Gemini path currently re-raises raw SDK errors after retries. Those strings can hold URLs or key material. Wrap with `sanitize_text` / a safe `ModelError` before the Runner sees them.
2. **Never put the key on the model object as a printable field in debug dumps.** Prefer reading env at call time, or mark the attribute so `repr` does not show it.
3. **Tests:** one unit test that a fake exception containing `AIza…` / `AQ.…` is scrubbed on the adapter boundary (not only tool errors).

## Out of scope (track later)

- Streaming must not log chunk payloads that include secrets.
- Prompt logging stays off by default.

Jessie — 2026-09-19
