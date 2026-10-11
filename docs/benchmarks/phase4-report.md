# Phase 4 report: gains, losses and mistakes (T7/T9 pending)

**Audience:** Harsha (owner), Tech Team (Thomas, Jessie, Walter, Alice, The Chief, Mike)  
**Date:** 2026-10-11 (IST)  
**Status:** Interim. T7 and T9 runs are in progress. Their results are not in this report.  
**Main data:** [`first-head-to-head.md`](first-head-to-head.md). Method: [`../research/13-tool-efficiency.md`](../research/13-tool-efficiency.md), [`../research/14-phase3-metrics.md`](../research/14-phase3-metrics.md), [`../research/15-phase4-first-bench.md`](../research/15-phase4-first-bench.md).  
**Rules for this report:** Facts from the bench doc, the run folders and the team only. No invented numbers. No claim that Harness beats fx or is better than fx. "Team input" marks facts that a team member gave for this report.

---

## 1. Summary

Phase 4 had one goal: a reproducible, fair head-to-head row against fx (fx.sh coding CLI, v0.0.13, build `4d966e272cfc`). We worked in measurement mode only and added no features.

We have one fair paired row today. It is task T1 v2 on `gemini-3.5-flash-lite`, through our bench stream proxy, with 5 s pacing for both agents.

In that row, both agents solved the task 3 of 3 times. Harness used a median of 28.8K input tokens and fx used 75.5K. Harness took more turns (14) than fx took steps (10), but these units are not the same. Wall time minus pacing was close (Harness 11.0 s, fx 12.8 s).

All other sets are weaker. They use T1 v1 (the fixture had fix hints), or one agent could not start, or free-tier quota stopped the runs.

Most of the phase went into free-tier quotas, provider stream formats and approval waits. We also found and fixed two real key-leak bugs.

The data supports gate result F at most, on one task, one weak model and one provider. It does not yet name a Harness weakness to build a fix for. The answer to "what do we build next" is "nothing yet", until T7 and T9 land.

---

## 2. What we measured

### 2.1 Tasks

| Task | Fixture | Status | Note |
|------|---------|--------|------|
| T1 v1 `broken_calc` | `native/fixtures/broken_calc/` | Run | Bugs labeled in source. The tree had `BUG:` comments, a formula hint in a test comment, a README with the requirements and notes that named the bug files. All v1 rows are weak. |
| T1 v2 `broken_calc_v2` | `native/fixtures/broken_calc_v2/` (frozen at `20baad1`) | Run | Same bugs and test asserts. All hints removed. |
| T7 `large_repo_v1` | `native/fixtures/large_repo_v1/` (frozen at `2a99d06`) | Running | 173 files, 8,325,651 bytes. Measures how much context an agent reads to find a bug in a big repo with decoys. |
| T9 `noisy_tests_v1` | `native/fixtures/noisy_tests_v1/` (frozen at `2a99d06`) | Running | Baseline `cargo test` output is 431,640 bytes. Measures how an agent handles very large test output. |

### 2.2 Tracks

- **Track A (runtime):** Harness with a scripted model. fx has no scripted mode, so fx Track A is N/A.
- **Track B (live model):** both agents with the same live model, where the provider let both start.

### 2.3 Sets and run counts

| Set | Task | Model | Provider path | Harness runs | fx runs |
|-----|------|-------|---------------|--------------|---------|
| Track A v1 | T1 v1 | scripted | none | 1 | N/A |
| Track A v2 | T1 v2 | scripted | none | 1 | N/A |
| Gemini Flash r1–r2 | T1 v1 | `gemini-3.5-flash` | Gemini OpenAI-compatible, direct | 2 | 0 |
| Gemini Flash r3–r5 + fx-1, fx-2 | T1 v1 | `gemini-3.5-flash` | direct | 3 | 2 |
| Flash-Lite direct (L1–L3, F1) | T1 v1 | `gemini-3.5-flash-lite` | direct | 3 | 1 |
| Flash-Lite via proxy (P1–P3, F1–F3) | T1 v1 | `gemini-3.5-flash-lite` | bench stream proxy | 3 | 3 |
| Cerebras (C-H1–3, C-F1–3) | T1 v1 | `qwen-3.8-27b` | Cerebras, direct | 3 | 3 |
| T1 v2 unpaced (V-H1–3, V-F1–3) | T1 v2 | `gemini-3.5-flash-lite` | proxy | 3 | 3 |
| T1 v2 paced 5 s (P-H1–3, P-F1–3) | T1 v2 | `gemini-3.5-flash-lite` | proxy, `--min-interval-ms 5000` | 3 | 3 |

All live runs used the same box (Linux x86_64, 8 vCPU Xeon, about 16 GiB RAM), the exact fenced `TASK.md` prompt, a fresh fixture copy per run and an 8-minute timeout. Success is operator `cargo test` exit 0 on the copy after the run.

### 2.4 Differences we did not control

- System prompt.
- Tool surface (fx built-in tools, Harness coding and shell packs).
- Step cap (Harness `--max-steps 24`, fx default).
- Permission modes. fx ran `--full-access` in early sets and `--auto` later. Harness ran `--allow coding,shell` in every live row, so shell was on. No row tests the Harness deny-all default.
- Units. fx "steps" and Harness "model turns" are counted differently.
- Retry behaviour. Harness reports its retries. fx retries 429s internally and does not report the count or the wait.

---

## 3. Where Harness gained

Each item gives the numbers, the task and model, and the confidence. "n" is runs per agent.

### G1. Fewer input tokens on T1 v2 (strongest signal, one task)

- **Paced set (T1 v2, Flash-Lite, proxy, 5 s pacing):** Harness median 28,833 input tokens (min 22,406, max 36,012). fx median 75,499 (min 57,656, max 164,664). Both 3 of 3.
- **Unpaced set (T1 v2, Flash-Lite, proxy):** Harness median 53,438 over 2 finished runs. fx median 132,457 over 3 finished runs.
- In both v2 sets, every Harness run used fewer input tokens than every fx run. This includes the failed run V-H1 (41,167).
- **Confidence:** n = 3 per agent per set, 1 task, 1 model, 1 provider through our proxy. Low.
- **Allowed claim (Alice):** "On T1 v2 with `gemini-3.5-flash-lite` and equal pacing, both agents solved it 3 of 3 times. Harness used a median of 28.8K input tokens and fx used 75.5K, on this one task. Harness took more turns, and wall time minus pacing was close."
- **Not allowed:** "faster", "uses less context" in general, "beats fx".
- **What we do not know:** we have no fx per-call token breakdown. So we cannot say why the gap exists (prompt size, tool schema size, tool output size or history).
- **Condition to upgrade this signal (team input, Thomas):** the token gap counts as a strength only if it holds on T7. On T7 the 32 KiB output cut is unlikely to matter. On T9 a gap is partly a design choice (the cut), not agent skill.

### G2. Fewer failed and duplicate tool calls in the paced set (weak)

- **Paced T1 v2:** median tool calls ok/fail/dup: Harness 12/1/2, fx 8/2/4.
- **Confidence:** n = 3, 1 task. Weak. fx "dup" means repeated identical name+status+command entries in its tool list. That measure is coarser than the Harness one. The two columns are not the same measure.

### G3. Harness started and finished where fx could not start (compatibility, not quality)

- **Flash-Lite direct (T1 v1):** Harness 3 of 3. fx 0 of 1 with `InvalidChunk` at 0 steps. Cause: fx cannot parse the Gemini OpenAI-compatible stream (tool-call deltas without `index`, an extra `extra_content` field, finish_reason `stop` with tool calls).
- **Cerebras `qwen-3.8-27b` (T1 v1):** Harness 3 of 3 (median 6 turns, 12 tool calls, 16,722 input tokens). fx 0 of 3 with HTTP 400 "System message must be at the beginning".
- **Confidence:** this is provider compatibility. It says nothing about coding quality. Both sets are inconclusive as comparisons (see section 5).

### G4. Track A runtime numbers exist (Harness only)

- T1 v1 scripted: success, 553 ms, 8 model turns, 7 tool calls (0 fail, 0 duplicate), n = 1, commit `14fafac`.
- T1 v2 scripted: success, 352 ms, 8 model turns, 7 tool calls (0 fail, 0 duplicate), n = 1, commit `e013766`.
- **Confidence:** n = 1 each. There is no fx side, because fx has no scripted mode. These are not a comparison.

### G5. What held (team input, Jessie and The Chief)

- No key appeared in any run folder, doc or commit across all scans (Jessie).
- No run edited tests or files outside the task (Jessie). The oracle checks in the bench doc agree. One cosmetic extra edit exists: C-H2 changed a comment on line 1 of `lib.rs`.
- Every row had a claim line, a security scan and labeled differences. Nobody claimed a win (The Chief).
- Core changed only once, for a bug fix (PR #10 `be14c4c`) that Walter acked with no ADR (The Chief).

---

## 4. Where Harness lost or is weak

### W1. More model turns than fx steps (watch; units differ)

- **T1 v2 paced:** Harness median 14 turns, fx median 10 steps.
- **T1 v2 unpaced:** Harness median 17 turns, fx median 14 steps.
- **T1 v1 proxy:** Harness 12–15 turns, fx 8–9 steps.
- **Important limit:** fx steps and Harness turns are not the same unit. The paced set also has a like-for-like count from the proxy: upstream model requests per run. fx: 19, 12, 15 (median 15). Harness: 16, 14, 12 (median 14). On that count the gap is small. No 429 occurred in the paced set.
- **Confidence:** n = 3 per set, 1 task. Alice's rule puts turn count under "watch", not "fix".

### W2. Rate-limit fragility (watch; rate limit in play)

- **r3–r5 (T1 v1, `gemini-3.5-flash`, retry build `29d1f58`):** all 3 failed on HTTP 429. The free-tier limit was 5 requests per window. Retry wait was 16,115 ms, 15,333 ms and 16,005 ms of 25,094, 15,804 and 17,957 ms wall time. The retry budget (max 3 retries, total wait cap 30 s) does not outlast that window.
- **T1 v2 unpaced:** Harness 2 of 3, fx 3 of 3. V-H1 stopped on a 429 after 3 retries, with `midpoint.rs` fixed and `percent.rs` not fixed. fx saw 7, 13 and 8 429s at the proxy and finished all 3 runs. The fx retry wait is unknown.
- **What this means:** under quota pressure, fx kept going and Harness gave up. With equal pacing, no 429 reached either agent and both solved 3 of 3.
- **Confidence:** n = 3 per set. A rate limit is in play by definition. Alice's rule says this cannot be a fix item. It stays under "watch". It is also a run-setup problem (see M3 in section 6).

### W3. No retry at the start of the phase (fixed)

- **r1 and r2 (T1 v1, `gemini-3.5-flash`, build `c7ed4ce` / `e435905`):** both failed before any edit. r1 hit a model read timeout (wall 111,115 ms). r2 got HTTP 503 (wall 55,782 ms). The agent had no retry.
- **Change:** bounded retry outside core in the OpenAI-compatible adapter (`29d1f58`), a read-timeout test and a separate retry-wait metric (`2a6168c`).
- **Side effect:** a retry after a read timeout can send the same request twice. That uses quota and can be billed twice. Every retry is counted in `model_retries`.

### W4. The 32 KiB output cut may hide failures on T9 (pending)

- **Design fact:** Harness cuts each tool output at 32 KiB (`MAX_OBS_BYTES` in `native/crates/harness-core/src/events.rs`). It keeps the start and adds a note with the full length.
- **Risk:** the T9 baseline `cargo test` output is 431,640 bytes. If the failing test name or the assert diff is after the first 32 KiB, the model may not see it.
- **Status:** no data yet. T9 is running.

### W5. Decoy reads (weak signal)

- In T1 v2 unpaced, V-H1 and V-H3 read `notes/OLD_BUGS.md` (a stale-notes decoy). Neither run edited it. No paced run read it.
- We cannot compare this with fx, because fx read paths are not in its JSON output.
- T7 has decoys and will give more data.

### W6. Wall time is not a gain

- Unpaced T1 v2 shows a lower Harness wall time (median 16,977 ms over 2 runs, fx 73,366 ms). This number includes fx's internal 429 back-off, so it is not a fair speed number. With equal pacing, wall time minus hold was close. We make no speed claim.

---

## 5. Ties or inconclusive results

| Result | Data | Reading |
|--------|------|---------|
| Success on T1 v2 paced | Both 3 of 3 | Tie |
| Wall time minus pacing, T1 v2 paced | fx min/median/max 10,573 / 12,762 / 27,760 ms; Harness 9,397 / 11,025 / 11,834 ms | Close. n = 3. No speed claim. |
| Upstream model requests, T1 v2 paced | fx median 15, Harness median 14 | Close |
| Success on T1 v1 proxy | Both 3 of 3 | Tie on a weak (hinted) fixture |
| Wall time, T1 v1 proxy | fx median 10,334 ms, Harness 10,519 ms | Close. Harness tokens were not recorded in this set. |
| Cerebras `qwen-3.8-27b` | Harness 3 of 3, fx 0 of 3 (HTTP 400) | Inconclusive. No paired run. |
| Flash-Lite direct | Harness 3 of 3, fx 0 of 1 (`InvalidChunk`) | Inconclusive. Compatibility only. |
| `gemini-3.5-flash` r1–r5, fx-1, fx-2 | No run of either agent succeeded | Inconclusive |
| Track A | Harness only | No comparison possible |

---

## 6. Mistakes and process problems

### M1. The T1 v1 fixture leaked fix hints

- **What went wrong (team input, Thomas):** this was Thomas's mistake. The tree had `BUG:` comments, a fix formula in a test comment, a README with the requirements, and notes that named the real files. The task did not really require inspection.
- **Impact:** every v1 row is weaker. Turn and tool-call numbers on v1 can be smaller than on a task with no hints. We relabelled all v1 rows "T1 v1 (bugs labeled in source)" and marked the tool-efficiency reading weak.
- **Related claim mistake (team input, Alice):** Alice accepted the v1 turn gap as result C before the hint scan. That claim line was wrong.
- **What we changed:** T1 v2 (`20baad1`) removes all hints. T7 and T9 have a recorded hint scan in their oracle files and `leaks_implementation_in_tree: false` in their manifests.
- **Rule (Thomas):** run a hint scan of the fixture tree before any run, not only before the claim line. **Rule (Alice):** no claim line until the hint scan is done.

### M2. "Blocked on a key" when the key was already present

- **What went wrong:** Mike first said Track B was blocked on a key. The Gemini key was already in the gitignored `.env`. The Chief called it a process miss.
- **Impact (team input, The Chief):** about 2 days lost.
- **Rule (The Chief):** check the machine before asking Harsha.

### M3. Live runs on free-tier quota with no preflight

- **What went wrong:** r3–r5 all failed on 429 with a request limit of 5. Most of their wall time was retry backoff. The Chief counts the first 5 live runs as wasted on quota.
- **Impact:** no usable Harness row from r1–r5. The unpaced v2 set still had a 429 failure (V-H1).
- **What we changed:** proxy pacing (`df3670e`). The paced set had zero 429s.
- **Rule (The Chief):** check rate limits before running.

### M4. fx ran late

- **What went wrong (team input, The Chief):** fx did not run until well into the phase. The first fx attempts then failed on `InvalidChunk` at 0 steps.
- **Impact:** the provider compatibility problem showed late. We needed a bench proxy (`db0915b`, `37c488f`) to get any paired row.
- **Rule (The Chief):** a phase starts by running both agents.

### M5. The retry change went to main before review

- **What went wrong:** `29d1f58` landed on main before Jessie reviewed it.
- **Impact (team input, Jessie):** nothing had to be reverted, but the gate came too late.
- **What we changed:** later bench and adapter changes (proxy, token capture `7080c56`, scrub fixes) went through Jessie before use in published rows.

### M6. Two key-leak bugs in the scrub

- **Adapter scrub:** it hid only 8 characters after `sk-` and only the first match. A 52-character `csk-` key would leak about 40 characters. Fixed in `cfbb242`.
- **First fix side effect:** it redacted normal words such as `task-list`. Jessie notes that this changed what the model saw. Fixed with a word-boundary rule in `1b7c38c`.
- **Core `scrub_secrets`:** same class of bug (12 characters, first match only, could crash on multibyte text). Fixed in PR #10 `be14c4c`. No ADR, because Walter classed it as a bug fix in an existing function.
- **Impact:** no key appeared in any run folder, doc or commit across all scans (Jessie).

### M7. Harness tokens not recorded in early paired sets

- **What went wrong:** the T1 v1 proxy pairing had fx tokens but no Harness tokens.
- **What we changed:** token capture in the adapter only, no core change (`7080c56`).

### M8. Approval waits

- **What went wrong:** pushes and run jobs from the group room were often blocked. Each needed Harsha's approval in a 1:1 chat.
- **Impact (team input, The Chief):** the biggest single delay of the phase.
- **Rule (The Chief):** batch approvals into one ask.

### M9. Time spent on providers, not on tasks

- Free-tier quotas and provider compatibility used most of the phase. We ran one task family (T1) in 8 live sets. T7 and T9 started only at the end.

---

## 7. Measurement gaps

1. **fx token breakdown.** We have fx totals per run only. We cannot split them into prompt, tool schema, tool output and history. So we cannot explain the token gap.
2. **fx read paths.** fx JSON output does not list the files it reads. We count fx `read_file` calls only. We cannot compare decoy reads.
3. **Steps and turns.** fx steps and Harness turns are counted differently. The turn gap is not like-for-like (team input, Thomas). Proxy upstream-request counts help, but we have them for the paced set only.
4. **Unnecessary tool calls are a floor (team input, Thomas).** `unnecessary_tool_calls` in `metrics.json` counts only repeat reads and repeat searches. It is not the full oracle-labeled count from doc 13. Useful/total numbers are a floor, not the real figure.
5. **Small samples (team input, Thomas).** With n = 3 per cell, p95 equals the max. This report gives min, median and max for that reason.
6. **One weak model, one provider, our own proxy (team input, Thomas).** All paired rows use `gemini-3.5-flash-lite` through our bench proxy. There is no paired row on a stronger model, because fx could not start on Cerebras.
7. **Track A is Harness only (team input, Thomas).** fx has no scripted mode, so we measure runtime overhead for Harness only.
8. **Deny-all is not tested (team input, Jessie).** Harness ran every live row with `--allow coding,shell`. fx ran `--full-access` or `--auto`. No row tests the Harness deny-all default.
9. **No sandbox (team input, Jessie).** No live run used an OS sandbox. We make no sandbox claim.
10. **fx retry waits.** fx does not report its 429 retry count or wait.
11. **Startup and memory.** Track B rows do not record process startup or peak RSS for either agent.
12. **Proxy rewrites.** The proxy changes the stream for both agents (adds `index`, rewrites finish_reason, adds Gemini signatures back). This is a controlled difference, but it is not how either agent talks to Gemini without the proxy.

---

## T7 and T9 results: pending (placeholder)

**Status:** runs in progress. No result is in this report. Do not read anything into this section until the bench doc has the T7 and T9 sections.

Questions these runs must answer:

1. **Does the token gap hold?** On T7 (173 files, about 8 MB, decoys), does Harness still use fewer input tokens than fx at equal pacing? This is the key question. Per Thomas, G1 counts as a strength only if it holds on T7, where the 32 KiB cut is unlikely to matter.
2. **Does the 32 KiB cut help or hide failures?** On T9 (431,640 bytes of baseline test output), does Harness see the failing test name and assert diff? Per Alice, if Harness uses fewer tokens on T9, the claim must say how much of that comes from the 32 KiB cut, and whether the failing test name was inside the first 32 KiB. The cut is a design choice, not proof that the agent is smarter.
3. **Success on a large repo.** Does each agent solve T7, and in how many runs?
4. **Decoy edits.** Does either agent edit a decoy file on T7, or a test or expected-output file on T9?
5. **Rate limits.** Did any 429 reach either agent? If yes, the row cannot support a fix item (Alice's rule).

---

## 8. Decision-gate reading

| Code | Meaning | Evidence today | Reading |
|------|---------|----------------|---------|
| A | Lose on startup | Not measured in Track B. Track A has no fx side. | No data |
| B | Lose on coding success | Paced T1 v2: both 3 of 3. Unpaced T1 v2: Harness 2 of 3, fx 3 of 3, and the miss was a 429. | No loss without a rate limit |
| C | Lose on tool efficiency | Harness has more turns than fx has steps. Units differ. Upstream requests are close (median 14 vs 15). Unnecessary-call counts are a floor. | Not shown |
| D | Lose on context efficiency | Harness used fewer input tokens on T1 v2 in both sets. | No loss on T1 v2 |
| E | Lose on memory | Peak RSS not recorded | No data |
| F | Win some axes, lose others | Lower tokens on one task. More turns in a unit that is not like-for-like. Weaker under quota pressure. | **At most F**, on 1 task, 1 weak model, 1 provider path |
| G | Inconclusive | Cerebras, Flash-Lite direct, Gemini Flash r1–r5, Track A | Large parts are G |

**Reading:** F at most. With one task and n = 3, the evidence is close to G.

---

## 9. Inputs for the improvement plan

### 9.1 What does the data say we should build next?

**Today: nothing yet. The answer is pending T7 and T9.** (Rule from The Chief.)

A fix item counts only if it is a Harness miss on 2 or more tasks with no rate limit in play (Alice's ranking rule). Today we have one task family (T1 v1 and v2). The only Harness miss without a hinted fixture (V-H1) was a 429. So no product fix item qualifies. If T7 and T9 do not show a Harness miss with no rate limit in play, the answer stays "nothing yet, add tasks".

Each item below follows the core rule: prove the need, try outside core, measure, then write an ADR before any core change.

### 9.2 Fix items (product)

None qualify today.

### 9.3 Watch items (ranked)

These have a measured signal but do not meet the 2-task rule, or are excluded by rule (turns, startup, size).

1. **32 KiB output cut on large test output (W4).**
   - Evidence: `MAX_OBS_BYTES` keeps the start only. T9 baseline output is 431,640 bytes. No run data yet.
   - Experiment: on T9, record for each `cargo test` observation whether the failing test names fall inside the first 32 KiB. Then, outside core, try a tool-side output filter in the shell/test plugin (for example, keep failure lines and the tail) and re-run T9 at equal pacing.
   - Result that justifies building: Harness misses T9, or needs more calls than on T1, because the failure text was cut, and the same pattern shows on a second large-output task with no 429.
2. **Rate-limit fragility (W2).**
   - Evidence: r3–r5 failed on 429. V-H1 failed on 429 after 3 retries while fx finished under 7–13 429s.
   - Experiment: outside core, in the adapter, try a longer retry budget that honours the quota window, and measure duplicate requests and wait time. Run at a fixed request rate below the limit.
   - Result that justifies building: Harness fails under a quota the team expects in real use, while fx finishes under the same quota, on 2 or more tasks. Today this stays excluded, because a rate limit is in play by definition.
3. **Decoy reads (W5).**
   - Evidence: V-H1 and V-H3 read `notes/OLD_BUGS.md`. No edits. No fx comparison possible.
   - Experiment: count decoy reads and decoy edits on T7 against the oracle list.
   - Result that justifies building: decoy edits, or a failed run caused by decoys, on 2 or more tasks.
4. **Turn count (W1).**
   - Evidence: Harness 14 turns vs fx 10 steps (paced v2). Upstream requests 14 vs 15.
   - Experiment: report upstream requests at the proxy for every paired set, so both agents use one unit.
   - Result that justifies building: a turn or request gap in one unit on 2 or more tasks that also costs success or tokens. Per Alice, turn count is not a work item yet.
5. **Startup and binary size.** No Track B data. Not a work item (Alice).

### 9.4 Measurement fixes (ranked)

1. **fx token breakdown.** Log usage counts per request at the bench proxy (counts only, no content, no headers). This explains G1.
2. **One unit for turns.** Record upstream requests at the proxy for every paired run.
3. **Full oracle labels for unnecessary calls.** Build the doc-13 labels in replay, outside core, so useful/total is not a floor.
4. **fx read paths.** Count tool-call names and paths from the model stream at the proxy, with no file content.
5. **More runs and tasks.** n ≥ 5 per cell where quota allows (doc 15), and more tasks after T7 and T9.
6. **A stronger model and a second provider.** Find a paired path where both agents start without a proxy, or document why none exists.
7. **A deny-all row.** One paired set with Harness at its default deny-all plus the minimum allow list, and the fx mode named.
8. **Startup and peak RSS** in Track B rows.

### 9.5 Process items (evidenced mistakes)

1. **Batch approvals into one ask** (M8, The Chief: biggest delay).
2. **Rate-limit preflight before any live run** (M3).
3. **Hint scan of the fixture tree before any run, and before any claim line** (M1, Thomas and Alice).
4. **Run both agents first** at the start of a phase (M4).
5. **Check the machine before asking Harsha** for keys or access (M2).
6. **Review before main** for adapter and bench changes (M5).

---

## 10. Appendix: result tables

The tables below are copied from [`first-head-to-head.md`](first-head-to-head.md) at commit `2a99d06`. Each heading gives the build SHA and the doc commit that added the rows. We checked the Harness numbers against the gitignored `metrics.json` files in `native/benchmarks/head_to_head/runs/`. Small wall-time differences (a few ms) exist because the bench doc uses operator-measured wall time around the process.


### A.1 Track A, T1 v1 (bugs labeled in source). Build `14fafac`; doc commits `3ee3719`, `c7ed4ce`

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

### A.2 Track A, T1 v2 (no hints). Build `e013766`; doc commit `e8600b4`

Harness only. RUNTIME only, not agent quality, not a comparison with fx (fx has no scripted mode).

| Field | Value |
|-------|--------|
| agent | harness |
| model | `scripted` (`broken_calc_v2_fix_script`, CLI `--script broken_calc_v2` / `calc2`) |
| harness commit SHA | `e0137666556cf1e32b7504789cf4d4fd02ceb527` |
| fixture | `native/fixtures/broken_calc_v2/`, content SHA-256 `3180ddf315384f5e66015878ff8a1b6e4e5ebfabb94c6203eff80a5eb014f7b6` |
| prompt | exact fenced block of `T1v2_broken_calc/TASK.md` |
| allow | `coding,shell` |
| n | 1 |
| success | yes (operator `cargo test` exit 0 on a fresh copy; before the run: exit 101) |
| wall_time_ms | 352 |
| model_turns | 8 |
| tool calls ok/fail/dup | 7 / 0 / 0 |
| files read / modified | 2 / 2 (`src/percent.rs`, `src/midpoint.rs`) |
| recorded run | `native/benchmarks/head_to_head/runs/T1v2-broken_calc-trackA-harness-20261011T035835/` (gitignored; folder name timestamp is UTC) |
| measured_at | 2026-10-11T09:28:35+05:30 (IST) |

The v2 script has the same steps as v1. Only the `apply_patch` anchors changed, because the v2 source has no BUG comments.

### A.3 Track B, T1 v1, `gemini-3.5-flash` r1 and r2. Builds `c7ed4ce` and `e435905`; doc commits `e435905`, `47a1537`

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

### A.4 Track B, T1 v1, r3–r5 and fx-1, fx-2 on the retry build. Build `29d1f58`; doc commit `c962cb7`

**Build change:** harness SHA `29d1f583c42cca84e3bf68a4b97028e3a3656157` adds bounded provider retry in `harness-model` OpenAiCompat (outside core): retry on transport error/timeout and HTTP 429/500/502/503/504, max 3 retries, backoff ~2s/4s/8s + jitter, total added wait capped at 30s, Retry-After honored. r3–r5 are **not** the same build as r1/r2 (`e435905` era). `model_retries` is now written to metrics.json and `model_retries.txt`.

**Duplicate-request caveat:** a retry after a read timeout can send the same request to the provider twice, because the first request may already have been received and processed when the client gave up waiting. That uses provider quota and can be billed twice. Every retry is counted in `model_retries`.

**Retry wait reported separately (bench rule):** from the build after `29d1f58`, metrics.json also has `model_retry_wait_ms` (measured total backoff sleep in the adapter) and `agent_wall_ms_excl_retry_wait = wall_time_ms - model_retry_wait_ms`. r3–r5 ran before these fields existed; the values below are recovered from the `model retry N/3 after X ms` lines in each run's `harness.stderr` (the requested sleep per retry, summed), not from a measured counter.

| Run | wall_time_ms | Retry wait (ms, from stderr) | agent_wall_ms_excl_retry_wait |
|-----|--------------|------------------------------|-------------------------------|
| r3 | 25094 | 16115 (2222 + 4802 + 9091) | 8979 |
| r4 | 15804 | 15333 (2139 + 4691 + 8503) | 471 |
| r5 | 17957 | 16005 (2264 + 4053 + 9688) | 1952 |

Same machine (linux-x86_64, 8 vCPU Xeon, 16 GB), model `gemini-3.5-flash` via Gemini OpenAI-compatible endpoint, flags `ask --model openai --allow coding,shell --max-steps 24`, exact TASK.md prompt, fresh fixture copy, 8-min timeout.

| Run | Agent | Success (`cargo test`) | wall_time_ms | Tool calls | Fails | Model turns | Files modified | Retries | Stop reason |
|-----|-------|------------------------|--------------|------------|-------|-------------|----------------|---------|-------------|
| r3 | Harness | no (exit 101) | 25094 | 7 | 1 | 8 | 0 | 3 | After run_tests + 6 reads, provider returned HTTP 429 (free-tier request quota, limit 5) on every attempt; gave up after 3 retries before any edit. |
| r4 | Harness | no (exit 101) | 15804 | 0 | 0 | 1 | 0 | 3 | First model request got HTTP 429 quota on all 4 attempts; no tool calls. |
| r5 | Harness | no (exit 101) | 17957 | 1 | 1 | 2 | 0 | 3 | After one run_tests call, HTTP 429 quota on all 4 attempts; no edit. |
| fx-1 | fx 0.0.13 (rev 4d966e272cfc) | no (exit 101) | 1476 | 0 | — | 0 steps | 0 | n/a | fx exited 1 with `error: InvalidChunk` before any step; run started right after r5 hit quota, root cause (quota error body vs. stream incompatibility) not determined. |
| fx-2 | fx 0.0.13 (rev 4d966e272cfc) | no (exit 101) | 1427 | 0 | — | 0 steps | 0 | n/a | Same `InvalidChunk` after a 75 s wait; not determined whether quota or provider stream format. |

**Finished runs:** none reached success, so no min/median/max wall time is reported.

**Observation:** the retry fired as designed (3 per run), but the failure in r3–r5 was a quota 429 with a per-window request limit of 5; a ≤30 s retry budget does not outlast that window. No conclusion about agent quality is drawn from these rows.

### A.5 Flash-Lite direct, T1 v1. Build `2a6168c`; doc commit `7dfc175`

Date 2026-10-11 (IST). Harness SHA `2a6168ce486347d051a649a33636af782ebcaa8e`. fx 0.0.13 (/tmp/fxbin). Machine: Linux x86_64, 8 vCPU (box). Model `gemini-3.5-flash-lite` via `https://generativelanguage.googleapis.com/v1beta/openai`, same key, free tier.

| Label | Agent | Run | Success (operator `cargo test` exit) | wall_time_ms | model_retries | retry_wait_ms | agent_wall_ms_excl_retry_wait | model turns | tool calls ok/fail/dup | files read / modified | tokens | stop reason |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| same weak model, free tier | fx | F1 | no (101) | 692 | n/a | n/a | n/a | 0 | 0/0/0 | 0 / 0 | null | fx exited 1 at 0 steps with `error: InvalidChunk`. |
| same weak model, free tier | Harness | L1 | yes (0) | 10321 | 0 | 0 | 10321 | 13 | 11/1/2 | 3 / 2 | not recorded | Agent stopped after `cargo test` passed (done: Succeeded, 13 steps). |
| same weak model, free tier | Harness | L2 | yes (0) | 9386 | 0 | 0 | 9386 | 12 | 10/1/2 | 2 / 2 | not recorded | Agent stopped after `cargo test` passed (done: Succeeded, 12 steps). |
| same weak model, free tier | Harness | L3 | yes (0) | 22459 | 0 | 0 | 22459 | 13 | 11/1/2 | 3 / 2 | not recorded | Agent stopped after `cargo test` passed (done: Succeeded, 13 steps). |

("fail" = tool_result without ok=true.) Finished: Harness 3/3, fx 0/1. Harness finished-run wall time min/median/max: 9386 / 10321 / 22459 ms. fx: no finished runs; F2–F3 not attempted because F1 failed before any model turn.

**fx InvalidChunk cause:** not quota — a direct streaming curl to the same endpoint/model returned HTTP 200 immediately before and after F1, and no 429 occurred in this session. The fx failure happened in 692 ms with 0 steps. Gemini's OpenAI-compatible stream chunks are non-standard: tool_call deltas carry no `index` field and include a provider-specific `extra_content` field, the role is repeated on every chunk, the final chunk has a delta with only `extra_content`, and `usage` appears on every chunk. The cause is most likely fx's chunk parser rejecting this stream shape. fx 0.0.13 `ask` has no documented non-streaming option. fx was not patched.

**Controlled:** model id, endpoint, key/tier, TASK.md prompt (exact fenced text), fresh fixture copy with `target/` removed, no ORACLE.md, 8-min timeout, machine, operator `cargo test` as success check.
**Uncontrolled:** tool surface (Harness coding+shell vs fx built-in tools), permission mode (Harness `--allow coding,shell` vs fx `--full-access`), system prompts, streaming handling (fx streams; whether streaming succeeds depends on how each client parses the stream), step cap (Harness 24; fx default).

### A.6 Flash-Lite via stream proxy, T1 v1. Build `2a6168c`, proxy `db0915b` + `37c488f`; doc commit `46ad900`

Label: **same weak model, free tier**. This set has its own table and is not pooled with r1–r5 or L1–L3. Date 2026-10-11 (IST). Harness binary built at SHA `2a6168c` (harness crates unchanged since). fx 0.0.13 (/tmp/fxbin), not patched. Model `gemini-3.5-flash-lite`, same key, free tier. Proxy at `db0915b` + `37c488f`, reviewed by Jessie. Run order: F1, P1, F2, P2, F3, P3, with 60 s between runs and a fresh proxy process for each run.

**Controlled difference: the stream proxy.** Both agents talked to `http://127.0.0.1:8787` → `generativelanguage.googleapis.com/v1beta/openai` through [`native/benchmarks/tools/oai_stream_proxy`](../../native/benchmarks/tools/oai_stream_proxy/README.md). The README lists every rewrite: tool_call `index` added, `extra_content` stripped, empty-delta chunks dropped, `usage` only on the last chunk, `finish_reason` `stop`→`tool_calls` after tool calls, `[DONE]` passed through.
**Signature handling (controlled difference):** the proxy keeps Gemini's signature field (`extra_content.google.thought_signature`) in memory, keyed by tool_call id, and adds it back to outgoing requests that lack it. Without that, Gemini returns HTTP 400 on turn 2 (tested with `PROXY_SIG_MODE=strip`). fx needs the re-add. Harness's adapter round-trips that field itself when it receives it, but behind the proxy it never receives it, so Harness requests also rely on the proxy re-adding it. Both agents therefore send identical signature data.

| Label | Agent | Run | Success (operator `cargo test` exit) | wall_ms | retries | retry_wait_ms | model turns / fx steps | tool calls ok/fail/dup | files read / modified | tokens (in/out) | stop reason |
|---|---|---|---|---|---|---|---|---|---|---|---|
| same weak model, free tier | fx | F1 | yes (0) | 9723 | n/a | n/a | 8 steps, 8 tools | 7/1/2 | 2 read_file calls / 2 | 58591 / 626 | fx exit 0, final answer |
| same weak model, free tier | Harness | P1 | yes (0) | 9218 | 0 | 0 | 12 | 10/1/2 | 3 / 2 | not recorded | done: Succeeded, 12 steps |
| same weak model, free tier | fx | F2 | yes (0) | 10334 | n/a | n/a | 9 steps, 9 tools | 8/1/3 | 2 read_file calls / 2 | 64116 / 607 | fx exit 0, final answer |
| same weak model, free tier | Harness | P2 | yes (0) | 10519 | 0 | 0 | 13 | 11/1/2 | 3 / 2 | not recorded | done: Succeeded, 13 steps |
| same weak model, free tier | fx | F3 | yes (0) | 10952 | n/a | n/a | 8 steps, 8 tools | 7/1/2 | 2 read_file calls / 2 | 59281 / 868 | fx exit 0, final answer |
| same weak model, free tier | Harness | P3 | yes (0) | 15190 | 1 (429) | 2496 | 15 | 12/2/3 | 3 / 2 | not recorded | done: Succeeded, 15 steps |

Modified files in every run: `src/midpoint.rs`, `src/percent.rs`. fx files-read paths are not in its JSON output. fx "dup" means identical name+status+command entries in fx's tool list, which is a coarser measure than Harness's. P3's single 429 was retried and the run finished, so it was not a daily-quota stop. P3 wall excluding retry wait: 12694 ms.

Finished: fx 3/3, Harness 3/3. Wall ms min/median/max over finished runs: fx 9723 / 10334 / 10952; Harness 9218 / 10519 / 15190.

**Uncontrolled:** system prompt, tool surface (fx built-in tools vs Harness coding+shell), step cap (Harness `--max-steps 24`; fx default), permission mode. The permission modes are a named difference: Harness ran deny-all with an allow list `--allow coding,shell`. fx ran `--auto` (its default mode: rules apply, then unresolved sensitive calls are auto-reviewed; no rules configured). `--auto` is narrower than the `--full-access` used in earlier sets, and a headless smoke test showed it can write files and run shell commands.

**Permission note (added 2026-10-11):** wherever fx ran `--auto` in this set, Harness ran with `--allow coding,shell`, so shell was on. This set is not a test of the Harness deny-all default.

### A.7 Cerebras `qwen-3.8-27b`, T1 v1. Build `7080c56`; doc commits `08ad4b4`, `e8600b4`

Token capture: Harness token fields come from commit `7080c56`, approved by Jessie.

Harness finished T1 v1 3 of 3 times on `qwen-3.8-27b`. fx could not start, because Cerebras rejected its message order.

Result: **inconclusive** — fx did not complete any run (0/3), so there is no paired comparison. Rows are measurements only, not pooled with any other set.

**Controlled:** model `qwen-3.8-27b` for both agents; provider Cerebras, base URL `https://api.cerebras.ai/v1`, direct (no proxy); fixture `broken_calc`, fresh copy per run (ORACLE.md removed); exact fenced TASK.md prompt; 8-minute timeout; 30 s between runs; order C-F1, C-H1, C-F2, C-H2, C-F3, C-H3; same box; success = operator `cargo test` exit 0 on the copy after the run.

**Uncontrolled:** system prompt; tool surface (fx built-in tools vs Harness coding+shell); step cap (Harness `--max-steps 24`, fx default); permission modes — fx `fx ask --json --no-save --auto`; Harness `--allow coding,shell` (shell on; not a test of the Harness deny-all default).

Commands: Harness `harness ask --model openai --allow coding,shell --workspace <copy> --max-steps 24 --record <dir>` (Harness commit `7080c56`, sends `User-Agent: harness/<version>`). fx 0.0.13 with a custom `openai-chat-completions` connection to Cerebras, bearer key from env var; `~/.fx/settings.json` restored from backup afterwards.

| Run | Agent | cargo test | wall_ms | retries / wait ms | turns | tool calls ok/fail/dup | files read / modified | tokens in / out / reasoning | stop reason |
|---|---|---|---|---|---|---|---|---|---|
| C-F1 | fx (`--auto`; Harness used `--allow coding,shell`) | fail (unmodified) | 213 | n/a | 0 | 0/0/0 | 0 / 0 | null / null / n/a | HTTP 400 from Cerebras (see below) |
| C-H1 | Harness (`--allow coding,shell`) | pass | 9949 | 0 / 0 | 6 | 12/0/0 | 6 / 2 (midpoint.rs, percent.rs) | 16722 / 8011 / 6443 | model finished (status success) |
| C-F2 | fx (`--auto`; Harness used `--allow coding,shell`) | fail (unmodified) | 226 | n/a | 0 | 0/0/0 | 0 / 0 | null / null / n/a | HTTP 400 from Cerebras |
| C-H2 | Harness (`--allow coding,shell`) | pass | 5513 | 0 / 0 | 6 | 14/0/0 | 6 / 3 (lib.rs, midpoint.rs, percent.rs; lib.rs = comment on line 1 only) | 16304 / 3871 / 2363 | model finished (status success) |
| C-F3 | fx (`--auto`; Harness used `--allow coding,shell`) | fail (unmodified) | 209 | n/a | 0 | 0/0/0 | 0 / 0 | null / null / n/a | HTTP 400 from Cerebras |
| C-H3 | Harness (`--allow coding,shell`) | pass | 7157 | 0 / 0 | 7 | 12/0/0 | 6 / 2 (midpoint.rs, percent.rs) | 18307 / 6544 / 5407 | model finished (status success) |

**C-H2 `lib.rs` edit:** per Thomas's oracle check, the `lib.rs` change was only a comment on line 1. It is a cosmetic extra edit, not a quality failure.

fx error (identical all three runs, before any model turn):
`HTTP 400: {"message":"Failed to apply chat template to messages due to error: System message must be at the beginning.","type":"invalid_request_error","param":"messages","code":"wrong_api_format"}`.
This is a request-shape rejection by the Cerebras qwen chat template, not a stream-format problem. Per instructions the fx side was stopped; fx was not patched and no proxy was built.

**Summary over finished runs:**

| Agent | finished / success | wall ms min / median / max | median turns | median tool calls | median input tokens |
|---|---|---|---|---|---|
| fx | 0 / 0 of 3 | n/a | n/a | n/a | n/a |
| Harness | 3 / 3 of 3 | 5513 / 7157 / 9949 | 6 | 12 | 16722 |

Token use for the set: about 51k input / 18k output tokens (Harness only; fx sent no completed requests). Run folders: `native/benchmarks/head_to_head/runs/T1-broken_calc-trackB-{fx,harness}-cerebras-C-*` (gitignored).

### A.8 T1 v2 Flash-Lite via proxy, unpaced. Build `e013766`, fixture `20baad1`; doc commit `d9fbdc6`

Date 2026-10-11 (IST). Fixture `native/fixtures/broken_calc_v2/` (content SHA-256 `3180ddf3…f7b6`), task pack `native/benchmarks/head_to_head/T1v2_broken_calc/` frozen at `20baad1`. Harness binary built at `e013766` (harness crates as `7080c56` plus the scripted v2 sequence; the openai path is unchanged). fx 0.0.13 (`/tmp/fxbin`, binary sha256 `00def23f…120c`, reinstalled from `fx.sh/setup.sh`, script sha256 `254c2d44…f924f`), not patched. Order V-F1, V-H1, V-F2, V-H2, V-F3, V-H3, 60 s between runs.

**Controlled:** model `gemini-3.5-flash-lite` for both agents; same key, free tier; both agents pointed at the same stream proxy (`native/benchmarks/tools/oai_stream_proxy`, `127.0.0.1:8787` only, `PROXY_SIG_MODE=reinject` signature add-back, `finish_reason` rewrite; fresh proxy process per run); prompt = only the fenced block of the v2 TASK.md; fresh v2 copy per run with `target/` removed and no ORACLE.md or SUCCESS.md; 8-minute timeout; same box; success = operator `cargo test` exit 0 on the copy after the run.

**Uncontrolled:** system prompt; tool surface (fx built-in tools vs Harness coding+shell); step cap (Harness `--max-steps 24`, fx default); permission modes (fx `fx ask --json --no-save --auto`; Harness `--allow coding,shell`). Harness ran with `--allow coding,shell`, so shell was on and this is not a test of deny-all. Retry behaviour also differs: Harness retries a 429 at most 3 times (≤30 s); fx retries 429s internally and does not report the count or wait, so the fx column gives the 429 count seen at the proxy and "unknown" for wait.

| Run | Agent | cargo test | wall_ms | retries / wait ms | turns (Harness) / steps (fx) | tool calls ok/fail/dup | files read / modified | tokens in / out / reasoning | stop reason |
|---|---|---|---|---|---|---|---|---|---|
| V-F1 | fx (`--auto`) | pass | 78981 | 7 × 429 at proxy / unknown | 13 | 10/3/7 | 5 read_file calls / 2 (midpoint.rs, percent.rs) | 109171 / 782 / n/a | fx exit 0, final answer |
| V-H1 | Harness (`--allow coding,shell`) | fail (7 passed, 3 failed) | 28299 | 3 / 16586 | 17 | 13/3/3 | 7 / 1 (midpoint.rs) | 41167 / 355 / null | HTTP 429 quota on 4 attempts after 3 retries; stopped before a percent.rs edit |
| V-F2 | fx (`--auto`) | pass | 73366 | 13 × 429 at proxy / unknown | 19 | 12/7/12 | 5 read_file calls / 2 (midpoint.rs, percent.rs) | 186435 / 2019 / n/a | fx exit 0, final answer |
| V-H2 | Harness (`--allow coding,shell`) | pass | 12747 | 0 / 0 | 17 | 13/3/3 | 6 / 2 (midpoint.rs, percent.rs) | 45249 / 766 / null | done: Succeeded |
| V-F3 | fx (`--auto`) | pass | 36887 | 8 × 429 at proxy / unknown | 14 | 9/5/6 | 2 read_file calls / 2 (midpoint.rs, percent.rs) | 132457 / 1862 / n/a | fx exit 0, final answer |
| V-H3 | Harness (`--allow coding,shell`) | pass | 21206 | 0 / 0 | 17 | 14/2/2 | 7 / 2 (midpoint.rs, percent.rs) | 61626 / 4025 / null | done: Succeeded |

wall_ms is operator-measured around the agent process (includes retry waits and fx's internal 429 back-off). fx files-read paths are not in its JSON output. fx "dup" = repeated identical name+status+command entries, a coarser measure than Harness's. Harness V-H1 and V-H3 read `notes/OLD_BUGS.md` (a stale-notes decoy); neither edited it. The V-H1 429 body does not state the quota window; later runs got HTTP 200, so it was not a daily-quota stop.

**Oracle check:** every diff touched only `src/percent.rs` and/or `src/midpoint.rs`. No run changed a test. V-H1 fixed only `midpoint.rs` before it stopped.

**Summary over finished runs (finished = operator `cargo test` pass):**

| Agent | finished of runs | wall ms min / median / max | median turns or steps | median tool calls | median input tokens |
|---|---|---|---|---|---|
| fx | 3 of 3 | 36887 / 73366 / 78981 | 14 | 14 | 132457 |
| Harness | 2 of 3 | 12747 / 16977 / 21206 | 17 | 16 | 53438 |

Harness median over two finished runs is the mean of the two. The fx and Harness turn counts measure different things (fx steps vs Harness model turns) and are not directly comparable. Run folders: `native/benchmarks/head_to_head/runs/T1v2-broken_calc-trackB-{fx,harness}-proxy-V-*` (gitignored; usage numbers only).

### A.9 T1 v2 Flash-Lite via proxy, paced 5 s. Build `e013766`, pacing `df3670e`; doc commit `2d39e9c`

Date 2026-10-11, runs 09:45–10:17 IST. Fixture `native/fixtures/broken_calc_v2/`, task pack `native/benchmarks/head_to_head/T1v2_broken_calc/` unchanged. Harness binary as in the previous v2 set (`e013766`; harness crates not touched). fx 0.0.13 (`/tmp/fxbin`, binary sha256 `00def23f…`), not patched; `~/.fx/settings.json` backed up, pointed at the proxy for the runs, and restored afterwards. Order P-F1, P-H1, P-F2, P-H2, P-F3, P-H3, at least 90 s between runs.

**Controlled:** model `gemini-3.5-flash-lite` for both agents; same key, free tier; both agents pointed at the same stream proxy (`127.0.0.1:8787` only, `PROXY_SIG_MODE=reinject`, `finish_reason` rewrite; fresh proxy process per run, so the hold counter starts at zero each run); **pacing: proxy `--min-interval-ms 5000`** (each upstream request held until ≥5 s after the previous upstream request started; same for both agents; neither agent changed); prompt = only the fenced block of the v2 TASK.md; fresh v2 copy per run with `target/` removed and no ORACLE.md or SUCCESS.md; 8-minute timeout; same box; success = operator `cargo test` exit 0 on the copy after the run.

**Uncontrolled:** system prompt; tool surface (fx built-in tools vs Harness coding+shell); step cap (Harness `--max-steps 24`, fx default); permission modes (fx `fx ask --json --no-save --auto --model gemini-3.5-flash-lite`; Harness `--allow coding,shell`). Harness ran with `--allow coding,shell`, so shell was on and this is not a test of deny-all. Retry behaviour differs (Harness reports retries; fx does not), but no 429 reached either agent in this set. The number of model requests per run differs between agents, so total hold time differs too.

| Run | Agent | cargo test | wall_ms | proxy hold ms | wall − hold ms | 429s at proxy | retries / wait ms | upstream requests | turns (Harness) / steps (fx) | tool calls ok/fail/dup | files read / modified | tokens in / out / reasoning | stop reason |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| P-F1 | fx (`--auto`) | pass | 92386 | 64626 | 27760 | 0 | n/a (fx does not report) | 19 | 14 | 12/2/6 | 5 read_file calls / 2 (midpoint.rs, percent.rs) | 164664 / 5439 / n/a | fx exit 0, final answer |
| P-H1 | Harness (`--allow coding,shell`) | pass | 76112 | 64278 | 11834 | 0 | 0 / 0 | 16 | 16 | 14/1/2 | 6 / 2 (midpoint.rs, percent.rs) | 36012 / 855 / null | done: Succeeded |
| P-F2 | fx (`--auto`) | pass | 56091 | 45518 | 10573 | 0 | n/a | 12 | 8 | 7/1/2 | 2 read_file calls / 2 (midpoint.rs, percent.rs) | 57656 / 747 / n/a | fx exit 0, final answer |
| P-H2 | Harness (`--allow coding,shell`) | pass | 65876 | 54851 | 11025 | 0 | 0 / 0 | 14 | 14 | 12/1/2 | 4 / 2 (midpoint.rs, percent.rs) | 28833 / 839 / null | done: Succeeded |
| P-F3 | fx (`--auto`) | pass | 71059 | 58297 | 12762 | 0 | n/a | 15 | 10 | 8/2/4 | 3 read_file calls / 2 (midpoint.rs, percent.rs) | 75499 / 661 / n/a | fx exit 0, final answer |
| P-H3 | Harness (`--allow coding,shell`) | pass | 55861 | 46464 | 9397 | 0 | 0 / 0 | 12 | 12 | 10/1/2 | 2 / 2 (midpoint.rs, percent.rs) | 22406 / 579 / null | done: Succeeded |

wall_ms is operator-measured around the agent process and includes proxy hold time; "wall − hold" subtracts the proxy's own hold total (`proxy_hold.json` in each run folder, counts only). fx files-read paths are not in its JSON output. fx "dup" = repeated identical name+status+command entries, a coarser measure than Harness's. No run in this set read `notes/OLD_BUGS.md`.

**Oracle check:** every diff touched only `src/percent.rs` and `src/midpoint.rs`. No run changed a test (no added/removed `assert`, `#[test]` or `mod tests` lines). No decoy edits.

**Harness runs that failed with no 429:** none. All three Harness runs finished with `done: Succeeded` and zero 429s.

**Summary (finished = operator `cargo test` pass):**

| Agent | finished of runs | wall ms min / median / max | wall − hold ms min / median / max | median turns or steps | median tool calls (ok/fail/dup) | median input tokens |
|---|---|---|---|---|---|---|
| fx | 3 of 3 | 56091 / 71059 / 92386 | 10573 / 12762 / 27760 | 10 | 10 (8/2/4) | 75499 |
| Harness | 3 of 3 | 55861 / 65876 / 76112 | 9397 / 11025 / 11834 | 14 | 13 (12/1/2) | 28833 |

Medians of ok/fail/dup are taken per column. fx steps and Harness model turns measure different things and are not directly comparable. Three runs per agent; spread is wide relative to the sample. Run folders: `native/benchmarks/head_to_head/runs/T1v2-broken_calc-trackB-{fx,harness}-paced-P-*` (gitignored; usage numbers only).

---

*End of report. T7 and T9 sections will be added to `first-head-to-head.md` when those runs finish. This report will need an update then.*
