# ADR-004 review — Jessie (security)

**Date:** 2026-09-20 (IST)  
**Verdict:** Approve Candidate A (Python library + CLI + plugin coding agent).

## Agree

- Coding power stays in plugins. Core freeze holds (ADR-003).
- Native rewrite = separate ADR. Do not smuggle shell / MCP / sandbox into core for "product completeness."
- Packaging spike (PyInstaller / similar) is SHOULD after Accept. It does not change trust boundaries.
- Security section in ADR-004 matches current gates: deny-all, path guard, shell opt-in, scrub.

## Conditions (must hold through Accept → ship)

1. **Deny-all stays default.** CLI must not silently Allow all coding tools on start. Host or user config sets the Allow list.
2. **Shell stays opt-in.** Default coding pack has no `run_shell`. Shell still needs Allow by name.
3. **Path guard stays `Path.relative_to`.** No `startswith` regressions on workspace roots.
4. **Scrub stays on tool errors and model-adapter errors.** CLI logs / crash dumps must not print keys.
5. **In-process plugins = full trust.** Document that a malicious plugin has host power. Process isolation is a later plugin ADR, not this one.
6. **No OS sandbox claim** for Candidate A. Do not market "sandboxed like fx" without a sandbox plugin + tests.

## CLI surface (new attack face)

Thin CLI is a new entry. Before ship I will add or extend tests for:

- Allow-list wiring (default deny)
- Workspace root binding (cannot escape via cwd tricks)
- No secrets in `--help` / env echo / error paths

## Nit

ADR Open item "Jessie reviews coding-plugin confinement before Accept → ship" — confinement review for *existing* packs is already green (`tests/security/`). CLI entry is the remaining gate before ship, not a block on Accept of Candidate A.

Jessie — 2026-09-20
