# First head-to-head coding-agent benchmark (Phase 4)

**Status:** Method + task frozen for row 1. **Results: empty until runs land.**  
**Date opened:** 2026-10-02 (IST)  
**Owners:** Thomas (task/method), Mike (Harness + fx runs), Jessie (gate), Alice (claim language)  
**Rule:** No “beat fx” claim. No vanity single score. ASD-STE100.

---

## 1. Objective

Produce one reproducible head-to-head row on a controlled coding task.

Separate:

| Track | Name | What it measures |
|-------|------|------------------|
| **A** | Runtime benchmark | Harness/tool overhead with fake or scripted model (no live model quality) |
| **B** | Real coding-agent benchmark | Live model + agent loop on the same task |

If Harness and fx cannot share a model, label Track B as **MODEL DIFFERENCE** and/or **RUNTIME DIFFERENCE**. Do not call it a pure agent-performance win.

---

## 2. Task (row 1)

| Field | Value |
|-------|--------|
| Task id | `T1/broken_calc` |
| Pack | [`native/benchmarks/head_to_head/T1_broken_calc/`](../../native/benchmarks/head_to_head/T1_broken_calc/) |
| Fixture | `native/fixtures/broken_calc/` |
| Fixture content SHA-256 | `e72e0c456c320fe3b1032db6dc9f812c31e70c57a0af52a97b228f9b81bcbc19` |
| Agent prompt | Only the fenced text in `TASK.md` |
| Success | `cargo test` exit 0 in a **copy** of the fixture (`SUCCESS.md`) |
| Oracle | `ORACLE.md` — operators only; never in the agent prompt |

The agent must inspect the tree. The prompt does not name the fix formulas.

---

## 3. Controlled variables (target)

| Variable | Target |
|----------|--------|
| Fixture content SHA | Same for every run |
| Task prompt text | Exact `TASK.md` block |
| Machine | Same host for Harness and fx rows in one publish |
| OS / arch | Recorded below |
| Working copy | Fresh copy of fixture per run (never mutate git fixture) |
| Network | Record: online / offline / restricted |
| Allow / permissions | Document Harness `--allow` and fx permission mode |

---

## 4. Uncontrolled / hard-to-control (document, do not hide)

| Item | Notes |
|------|--------|
| Model identity | If fx cannot use the same model id / API, mark **MODEL DIFFERENCE** |
| Model config | Temperature, tool schema shape, max tokens — list deltas |
| Tool surface | Built-in tools differ; list Harness allowlist vs fx defaults |
| Sandbox | Harness is **not** an OS sandbox; fx may differ — state facts only |
| Binary packaging | Different languages/runtimes |

---

## 5. What to record (every run)

Copy into `native/benchmarks/head_to_head/runs/<run_id>/meta.json` (+ `run.jsonl` / fx log + `metrics.json` + `final.diff`).

### 5.1 Identity

- `run_id`, `track` (`A` \| `B`), `agent` (`harness` \| `fx`)
- Harness git SHA / executable hash
- fx version or git SHA
- Fixture content SHA
- Model name + config (or `scripted` / `fake`)
- Machine: hostname class, OS, CPU model, RAM, arch
- Network condition
- Operator notes

### 5.2 Timing / resources

- Process startup ms (agent binary)
- Time to first model request ms
- Time to first useful action ms (see oracle labels)
- Total wall time ms
- Peak RSS bytes (if available; else `unknown`)

### 5.3 Agent / tools

- Model turns
- Total tool calls, failed tool calls, duplicate tool calls
- Useful / unnecessary labels (oracle) — raw counts, not one score
- Files read, bytes read (if available), files modified
- Tests executed
- Tokens: input, output, tool-result, total (or `unknown`)
- Largest tool result bytes
- Bytes transferred to model (or `unknown`)
- Final test result (`pass` \| `fail`)
- Final diff (`git diff` or equivalent on the worktree)

### 5.4 Aggregation (repetitions)

For timing fields with ≥3 runs: **median, p95, min, max**.  
If model/network variance dominates, say so in plain words.

---

## 6. Track A — runtime (scripted / fake)

**Harness:** `harness ask --model scripted` (or a T1 scripted sequence for `broken_calc` when Mike adds it; until then use `broken_add` **only** as a smoke and say it is not row-1).

**Row-1 requirement:** scripted path on **`broken_calc`** (search/read/patch/test). If missing, Mike adds a scripted sequence **outside** `harness-core` before publishing Track A for T1.

**fx:** run equivalent only if a fake/scripted mode exists. Else Track A is Harness-only + note `fx: N/A`.

---

## 7. Track B — live coding agent

Same fixture SHA, same prompt, same machine.

Record model for each side. If models differ → classify **MODEL DIFFERENCE**; still publish raw rows.

Store complete Harness `run.jsonl` + `metrics.json`. Store fx session log / transcript path.

---

## 8. Raw results (fill after runs)

### Track A

| agent | n | success | wall_ms median | tool_calls median | notes |
|-------|---|---------|----------------|-------------------|-------|
| harness | | | | | |
| fx | | | | | |

### Track B

| agent | model | n | success rate | wall_ms median | tool_calls median | tokens total median | notes |
|-------|-------|---|--------------|----------------|-------------------|---------------------|-------|
| harness | | | | | | | |
| fx | | | | | | | |

Raw per-run files: `native/benchmarks/head_to_head/runs/` (gitignored if large; keep at least one redacted sample + meta).

---

## 9. Decision gate (after first row)

Answer only from data:

| Code | Meaning |
|------|---------|
| A | Lose on startup |
| B | Lose on coding success |
| C | Lose on tool efficiency |
| D | Lose on context efficiency |
| E | Lose on memory |
| F | Win some axes, lose others |
| G | Inconclusive |

**Next build** = fix the measured bottleneck outside core when possible. No feature zoo.

---

## 10. Claim language (Alice lock)

Allowed examples:

- “On task T1/broken_calc, machine X, Harness median wall time was …”
- “fx used model Y; Harness used model Z — MODEL DIFFERENCE.”

Forbidden:

- “We beat fx.”
- Any single efficiency score that hides raw counters.

---

## 11. Limitations (standing)

- One task is not a product verdict.
- Live variance may dominate.
- Tool surfaces differ.
- Early native binary size/start numbers elsewhere are **not** this row.

---

## 12. Run checklist (Mike / Jessie)

See [`../research/15-phase4-first-bench.md`](../research/15-phase4-first-bench.md) § checklist.
