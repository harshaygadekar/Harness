# Useful scope — Phase 2 (coding agent)

**Owner:** Alice  
**Date:** 2026-09-19  
**Status:** Locked for Phase 2 apps. Core freeze still holds (ADR-001/002).

---

## Product

We are building an **AI coding agent app on top of the harness**, not a bigger core.

Success = a stranger can install the harness, run a one-file agent, then run a coding-agent example that edits a repo **via plugins**, without core growth.

We do **not** claim to beat fx.sh unless Mike’s numbers say so.

---

## Must ship (Phase 2)

1. One-file `Agent` façade (Mike) — already started.
2. Examples that use current API only: simple, tool, then coding_agent.
3. Coding agent capabilities **as plugins**: filesystem, shell, git, patch, search, tests — each behind deny-all Allow list.
4. Honest benches vs fake model; optional live Gemini.
5. Docs: how to write a plugin; non-goals; link to `08-harness-comparison.md`.

---

## Must not ship in core

Same as `useful-scope.md`. Plus:

- Do not move shell/fs/git into core to “make coding easier.”
- Do not add MCP/sandbox/memory OS to core for the coding agent.
- Do not add a graph/crew runtime for “better coding.”

If an example needs a new primitive, @Thomas documents the wall first. Then Walter ADR. Then Mike may patch core.

---

## Coding agent “sells” this (scope)

| In | Out |
|----|-----|
| Read/write files in a workspace (plugin) | Full IDE / LSP product |
| Run shell/tests with host Allow (plugin) | Unrestricted RCE by default |
| Patch / search (plugin) | Oh-my-pi-scale tool soup |
| Replay with fake model (reliability track) | Live-model-only demos as the only proof |
| Clear install + examples | Feature parity marketing vs fx |

---

## Kill / reopen

- If coding_agent imports provider SDKs or shell into `agentic_harness` core → Alice veto.
- If we claim “faster than fx” without benches → Alice veto.
- fx-size Zig rewrite = **new product decision**, not Phase 2 scope creep.
