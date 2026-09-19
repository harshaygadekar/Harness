# Phase 2 — Security track (Jessie)

## Status: complete for night push

### Ownership
Permission tests, secret scrub, injection boundaries, coding-plugin confinement. No core growth.

### Delivered
1. `tests/security/test_attacks.py` — deny-all, allowlist, scrub, injection args, no Registry execute.
2. `tests/security/test_coding_security.py` — workspace path escape (relative_to), shell not in default pack, shell soft deny.
3. `plugins/coding/workspace.py` — path guard uses `Path.relative_to` (fixes startswith prefix bug).
4. Freeze: shell stays opt-in; gate still required for Allow.

### Not claiming
OS sandbox, seccomp, or network jail. Those need a later plugin ADR.
