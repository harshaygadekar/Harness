# Jessie review: ADR-005 Native language

**Date:** 2026-09-20 (IST)  
**Verdict:** Approve **Rust**.

## Agree

- Separate `native/` product. Python harness stays frozen.
- Memory safety helps. It does **not** replace deny-all, capability classes, workspace bind, scrub, or approval for exec/net/secrets.
- ADR-005 Security section matches policy port (not a Python code copy).
- OS sandbox = later measured ADR. Do not claim Landlock/seccomp until that ADR + tests exist.
- In-process / dylib plugins = full trust until WASM or process isolation. Document that in CLI help.

## Conditions (bind through Accept → first ship)

1. Deny-all default. CLI must not silently Allow the full coding pack on start.
2. Capability classes explicit: read / write / exec / net / git / secrets.
3. Workspace path resolve with canonical bounds (no prefix bugs).
4. Secret scrub on tool errors, model errors, logs, crash dumps.
5. Approval seam for exec / net / secrets-class before they run.
6. No “sandboxed like fx” marketing under this ADR.

## Nit

CI should fail a test that starts the CLI with zero Allow flags and still sees a tool run. Same shape as Python CLI gate.

Jessie — 2026-09-20
