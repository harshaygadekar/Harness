# 15 — Phase 4: first fair coding-agent benchmark

**Audience:** Thomas (method), Mike (runs), Jessie (gate), Alice (claims), Tech Team  
**Date:** 2026-10-02 (IST)  
**Status:** Phase 4 open. Measurement mode. No beat-fx claim. No major features.  
**Public row doc:** [`../benchmarks/first-head-to-head.md`](../benchmarks/first-head-to-head.md)  
**Task pack:** [`../../native/benchmarks/head_to_head/T1_broken_calc/`](../../native/benchmarks/head_to_head/T1_broken_calc/)  
**Parents:** `10-fx-beat-goals.md`, `13-tool-efficiency.md`, `14-phase3-metrics.md`

---

## 1. Phase lock

Accepted inventory: frozen Python harness; native Rust coding slice; record/replay; deny-all; metrics method; early size/start numbers.

We have **not** beaten fx.sh.

Phase 4 goal: **one reproducible head-to-head row**, then decide the next engineering task from measured failure only.

---

## 2. Two benchmarks (do not mix)

| Track | Purpose | Model |
|-------|---------|--------|
| **A Runtime** | Tool/loop overhead | Fake or scripted only |
| **B Real agent** | Coding success + work required | Live model(s); document deltas |

If the same model is impossible on both agents, Track B is **MODEL DIFFERENCE** and/or **RUNTIME DIFFERENCE**, not a pure agent IQ contest.

---

## 3. Task design rules (Thomas)

1. Agent must inspect the repo.
2. Success = tests / behavioral check, not string match to a known answer.
3. Prompt must not leak the implementation (no formulas, no patch).
4. Oracle stays off the agent channel.
5. Freeze fixture content SHA before publish.

Row 1 uses `broken_calc` with a **stripped** agent prompt in the task pack (not the Phase 3 README requirements list).

---

## 4. Axes to publish (raw)

From directive + `13`/`14`:

- Startup, first model request, first useful action, wall time
- Turns, tool calls, fails, duplicates, useful / unnecessary (oracle)
- Files read / modified, bytes read when known
- Tests executed, final test result, final diff
- Tokens and context bytes when known; else `unknown`
- Peak RSS when known; else `unknown`

Useful actions / total actions may be shown as a **rate**. It does not replace raw counts. No vanity single score.

---

## 5. Repetitions

Prefer **n ≥ 5** live runs per agent when cost allows; minimum **n ≥ 3** before a public timing sentence.

Report median, p95, min, max for wall time (and startup when stable).

State when variance is model/network-dominated.

---

## 6. Operator checklist

### Freeze

- [ ] Fixture SHA matches manifest
- [ ] Prompt = `TASK.md` fence only
- [ ] Harness SHA / fx version recorded
- [ ] Machine / OS / CPU / RAM recorded

### Track A

- [ ] Scripted `broken_calc` sequence exists (Mike) or row deferred
- [ ] Harness scripted runs recorded
- [ ] fx fake path or explicit N/A

### Track B

- [ ] Same fixture copy procedure
- [ ] Model ids + configs listed
- [ ] Differences called out
- [ ] Harness `run.jsonl` + `metrics.json` + diff saved
- [ ] fx transcript saved
- [ ] Jessie confirms deny-all / allow story for Harness run

### Publish

- [ ] Fill §8 tables in `first-head-to-head.md`
- [ ] Decision gate letter (A–G)
- [ ] One sentence: “What does the data say we should build next?”
- [ ] Alice reviews claim language

---

## 7. After row 1

Only then expand to 5–10 tasks (bug fix, test repair, small feature, refactor, migration, debug, large noise tree, misleading layout, large test output, multi-file).

Until data names a bottleneck: no subagents, memory OS, WASM, fat TUI, MCP zoo, provider zoo, marketplace, giant index, speculative core growth.

Core change still needs: requirement → outside-core try → measure → ADR.

---

## 8. Final question (end of phase)

**What does the data say we should build next?**

Not what fx has. Not what looks impressive.
