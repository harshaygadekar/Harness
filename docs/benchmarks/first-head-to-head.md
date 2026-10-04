# First head-to-head coding-agent benchmark (Phase 4)

**Status:** Method + task frozen for row 1. **Track A (Harness scripted) recorded; fx Track A TBD / blocked.** **Track B = one real coding-agent run on Gemini (Harness only; failed before a fix). fx Track B not run (TBD).**  
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

**What Track A measures:** Harness / tool-loop **runtime overhead only** with a fixed scripted sequence. It is **not** agent IQ and **not** a claim vs fx.

### 6.1 How to select scripted `broken_calc` vs `broken_add`

```bash
cd native
cargo build --release -p harness-cli

# Recommended: fresh copy of the fixture (never mutate git fixture)
FIX=$(mktemp -d)
cp -a fixtures/broken_calc/. "$FIX/"

# Option A — workspace detect (Cargo.toml name `broken_calc` or path contains broken_calc)
./target/release/harness ask \
  --model scripted \
  --allow coding,shell \
  --workspace "$FIX" \
  --prompt "fix the failing tests" \
  --record /tmp/harness-t1-trackA
# Prefer the exact fenced prompt from TASK.md for published rows (operators paste it).

# Option B — explicit script flag (back-compat default is broken_add)
./target/release/harness ask \
  --model scripted \
  --script broken_calc \
  --allow coding,shell \
  --workspace "$FIX" \
  --prompt "fix the failing tests" \
  --record /tmp/harness-t1-trackA

# broken_add smoke (Phase 3; not row-1)
./target/release/harness ask --model scripted --script broken_add \
  --allow coding,shell --workspace "$ADD_COPY" --prompt "fix the failing tests"
```

`--script` accepts `broken_calc` | `broken_add` (aliases `calc` | `add`). If omitted with `--model scripted`, the CLI detects from workspace path or `Cargo.toml` `name`; otherwise defaults to **`broken_add`** for back-compat.

**fx:** run equivalent only if a fake/scripted mode exists. Else Track A is Harness-only + note `fx: N/A (no fake-model API)` / blocked.

---

## 7. Track B — live coding agent

Same fixture SHA, same prompt, same machine.

Record model for each side. If models differ → classify **MODEL DIFFERENCE**; still publish raw rows.

Store complete Harness `run.jsonl` + `metrics.json`. Store fx session log / transcript path.

---

## 8. Raw results (fill after runs)

### Track A

**Machine (shared host for this publish):**  
`uname -a`: `Linux grok-bot-vm-6854201 6.12.94+ #1 SMP PREEMPT_DYNAMIC Fri Sep 18 16:05:18 UTC 2026 x86_64 GNU/Linux`  
CPU: Intel(R) Xeon(R) Processor (8 logical CPUs) · RAM: 16397616 kB (~16 GiB) · arch: x86_64  

**Harness Track A (T1/broken_calc, model=scripted):**

| Field | Value |
|-------|--------|
| agent | harness |
| track | A (RUNTIME only — not agent IQ vs fx) |
| model | `scripted` (`broken_calc_fix_script`) |
| harness commit SHA | `14fafacf3e97b36875ff5eab242176ed66f8477d` |
| fixture content SHA-256 | `e72e0c456c320fe3b1032db6dc9f812c31e70c57a0af52a97b228f9b81bcbc19` |
| allow | `coding,shell` (Jessie: deny-all default; shell only via allow) |
| network | offline (no live model API) |
| n | 1 |
| success | yes (`cargo test` exit 0 on fixture **copy**) |
| wall_time_ms | 553 |
| model_turns | 8 |
| total_tool_calls | 7 (all success; 0 fail / 0 duplicate) |
| files_read / modified | 2 / 2 (`src/percent.rs`, `src/midpoint.rs`) |
| useful_actions (heuristic) | 7 |
| recorded run | `native/benchmarks/head_to_head/runs/T1-broken_calc-trackA-harness-20261002-060541/` (`run.jsonl` + `metrics.json` + `meta.json`; dir gitignored) |
| measured_at | 2026-10-02T06:05:42+05:30 (IST) |

| agent | n | success | wall_ms median | tool_calls median | notes |
|-------|---|---------|----------------|-------------------|-------|
| harness | 1 | yes | 553 | 7 | scripted `broken_calc`; RUNTIME only |
| fx | — | not run | TBD | TBD | **blocked / not run yet** — no fx scripted/fake path exercised this session |

**fx columns:** empty / TBD. Do not invent fx numbers. Track A Harness row is **not** a beat-fx claim.

### 8.1 Harness commit for Track A row

Harness commit SHA that contains `broken_calc_fix_script` + `--script` CLI: `14fafacf3e97b36875ff5eab242176ed66f8477d` (docs-fill follow-up may tip HEAD).

### Track B

**Label:** Track B = real coding-agent run on Gemini (live model + agent loop). Not Track A. Not a beat-fx claim.

**MODEL DIFFERENCE vs fx:** unknown until fx runs the **same** model (`gemini-3.5-flash` via the OpenAI-compatible Gemini endpoint). fx was **not** run. fx columns stay TBD. Do not read this row as an agent-performance win.

**Harness Track B (T1/broken_calc, n=1):**

| Field | Value |
|-------|--------|
| agent | harness |
| track | B (live coding agent) |
| model | `gemini-3.5-flash` (`HARNESS_MODEL`; `HARNESS_BASE_URL` = Gemini OpenAI-compatible `v1beta/openai`; CLI `--model openai`) |
| harness commit SHA | `c7ed4ce83ca7af5685fc45caabe33b0e9fe02388` |
| fixture content SHA-256 | `e72e0c456c320fe3b1032db6dc9f812c31e70c57a0af52a97b228f9b81bcbc19` |
| allow | `coding,shell` |
| network | online (live model API) |
| max_steps | 24 |
| n | 1 |
| success | **no** (`task_success` false, `exit_status` fail, run status `Failed`) |
| wall_time_ms | 111115 |
| model_turns | 3 |
| total_tool_calls | 2 (2 success, **0 fail**, 0 duplicate) |
| files_read / modified | 0 / 0 (only `list_files` on workspace root and `src/`) |
| tests executed by agent | 0 |
| tokens (prompt / completion / total) | not present in `metrics.json` (`unknown`) |
| final test result | fail — agent did not edit the copy (`final.diff` empty). Operator `cargo test` on that copy after the run: exit 101 (5 passed, 5 failed). |
| recorded run | `native/benchmarks/head_to_head/runs/T1-broken_calc-trackB-harness-20261004-125242/` (`run.jsonl` + `metrics.json` + `final.diff` + `machine.txt`; dir gitignored) |
| measured_at | 2026-10-04T12:54:34+05:30 (IST) |
| error (scrubbed) | Third `chat/completions` read timed out (`MODEL_ERROR` … `timed out reading response`). No API key in the error text. Two earlier model turns returned. |

| agent | model | n | success rate | wall_ms median | tool_calls median | tokens total median | notes |
|-------|-------|---|--------------|----------------|-------------------|---------------------|-------|
| harness | `gemini-3.5-flash` | 1 | 0/1 | 111115 | 2 | unknown | live Track B; stopped on HTTP read timeout before any edit |
| fx | — | — | not run | TBD | TBD | TBD | **not run** — MODEL DIFFERENCE vs Harness unknown until fx uses the same model |

**Track B repeat (run 2 of n):** still not a vs-fx result. fx was not run. No coding-quality verdict and no loss-to-fx claim.

| Field | Value |
|-------|--------|
| run | 2 |
| agent | harness |
| model | `gemini-3.5-flash` (same endpoint and CLI flags as run 1) |
| n | 2 (two live runs; not a comparison with fx) |
| success | **no** (`task_success` false, `exit_status` fail, run status `Failed`) |
| wall_time_ms | 55782 |
| model_turns | 3 |
| total_tool_calls | 2 (1 success `list_files`, 1 fail `run_tests`, 0 duplicate) |
| files modified | 0 (`final.diff` empty) |
| stop reason | Third model turn returned HTTP 503 UNAVAILABLE (“high demand”); the agent had not edited the copy. |
| operator `cargo test` | exit 101 (5 passed, 5 failed) |
| recorded run | `native/benchmarks/head_to_head/runs/T1-broken_calc-trackB-harness-r2-20261004-125829/` (gitignored) |
| measured_at | 2026-10-04T12:59:25+05:30 (IST) |
| harness commit at measurement | `e435905cf2ebee199339ec5364885910776a851f` |


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
