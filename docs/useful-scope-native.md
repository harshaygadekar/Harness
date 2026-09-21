# Useful scope — native coding agent

**Owner:** Alice  
**Date:** 2026-09-20  
**Status:** Locked. ADR-005 Accepted (Rust).  
**Rule:** Python Harness stays frozen. This is a **separate product**.

---

## Product

Ship a **tiny native AI coding CLI** that works on a real repo.

Benchmark story:

> Useful coding-agent work per byte, per ms, per tool call, and per model token.

We do **not** copy fx feature-for-feature. We do **not** claim to beat fx until Mike’s native benches say so.

---

## Keep

| Keep | Notes |
|------|--------|
| Python Harness on `main` | Reference library. No chase-fx core growth. |
| Deny-all + scrub + workspace bounds | Port the *policy*, not Python code, into native. |
| Plugins / replaceable tools | Shell, fs, git, MCP stay out of the smallest runtime. |
| Honest numbers | Size, start, tool efficiency, tokens — measured. |

---

## Sell (native)

| In | Out (v1) |
|----|----------|
| CLI: ask / run / help on a workspace | Full IDE / browser / heavy TUI |
| Min tool pack: list, search, read, write/patch, git status/diff, opt-in shell, tests | Dozens of redundant tools |
| Streaming that shows real latency | Fake UI to hide wait |
| Capability gate: read / write / exec / net / git / secrets | Auto-allow everything |
| MCP / skills as extensions | MCP forced into the smallest runtime |
| Subagents only if benches show benefit | Multi-agent by default |

---

## Success for “ready”

1. Native binary runs without Python.
2. Fixes a real failing test in a real sample repo (not a toy echo).
3. Tool-efficiency bench published (wasteful reads/searches/tests counted).
4. Jessie native security gates pass.
5. No beat-fx marketing without green measured axes.

---

## Kill / reopen

- Growing Python core to chase fx → Alice veto.
- Shipping native with shell/MCP in the “smallest runtime” without ADR → Alice veto.
- Claiming beat-fx on marketing µs / binary myths → Alice veto.

---

## Phase 3 — vertical slice (2026-09-21)

**Milestone:** first **real** native coding loop — not more scaffold docs.

### Must work end-to-end

USER TASK → MODEL → TOOLS → SEARCH/READ → EDIT → TEST → FIX → SUCCESS

on a **real fixture repo with real bugs** (Thomas). Fake-model CI stays for micro benches; live model is required for the slice demo.

### Product success (this phase)

1. One OpenAI-compatible adapter (not five providers). Outside core.
2. Recorded run: model/tool events + final diff + timings.
3. `harness replay <run>` without a live model when responses are recorded.
4. Published raw metrics (tool waste, tokens, wall time) — not one vanity score.
5. Deny-all + shell opt-in + workspace bind still hold (Jessie).

### Still out

- Feature race with fx / MCP-before-loop / giant sandbox
- Beat-fx marketing
- Growing Python Harness core
