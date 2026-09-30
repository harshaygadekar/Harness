# Phase 3 vertical slice — Jessie gate

**Date:** 2026-09-30 (IST)  
**Verdict:** Pass for push of `8398802` + `a1020b5` + `337c8d3` (+ this fix).

## Checked
1. Deny-all default on CLI; shell still opt-in / allowlist.
2. Workspace `strip_prefix` guard holds.
3. Tool observations scrubbed + bounded (32 KiB) before record.
4. Adapter reads key from env only — not CLI flags.
5. `OpenAiCompat` Debug redacts `api_key` (fix landed with this gate).
6. No OS-sandbox claim. No beat-fx claim.

## Still open (not blocking this push)
- Live model run needs key in env (not chat).
- Replay files can still hold task text — keep `records/` local / gitignored.
- Streaming TODO in adapter; HTTP errors must not echo Authorization (ureq path OK for now).

Jessie
