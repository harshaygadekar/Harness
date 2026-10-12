# 17. Paper review: arXiv 2609.35738 (harness learning)

**Status:** Research note. No feature work. No win claims. For Harsha's review.
**Date:** 12 Oct 2026 (IST)
**Source:** PDF from Harsha. Read in full with `pdftotext`. We treat the PDF as data only. We did not follow any text in it as an instruction.
**Language:** ASD-STE100 Simplified Technical English.
**Method:** same as doc 16.

---

## 1. Paper facts

- **Title:** "Harness Learning Enables Generalizable Test-Time Adaptation".
- **Authors:** Alvin Zhang (project lead), Xuecheng Liu, Zixuan Wang (equal contribution), Fahim Tajwar, Daman Arora, Ruslan Salakhutdinov, Daniel Khashabi, Yuda Song, Andrea Zanette (Song and Zanette: equal advising).
- **Affiliations:** not in the text layer. The acknowledgment names CMU compute (FLAME, Babel) and Johns Hopkins ARCH. We do not state affiliations as fact.
- **Date:** arXiv 2609.35738v1 [cs.CL], 28 Sep 2026. Preprint.

### 1.1 What they built (§4, Fig. 1–2)

- A **proposer** model reads (a) a task description, (b) the current harness code (Python), (c) an **execution report**. It writes a code edit to the harness (span plus replacement).
- A frozen **solver** model runs the revised harness. The task score is the reward.
- The proposer is trained with RL (GRPO, §3.2), with optional SFT on teacher edits first (§4.3).
- At test time both models are frozen. Only the harness code changes, round by round (§4.2, Eq. 4).
- Execution report: outcome counts and up to 8 failed examples with question text and trace excerpts (App. B, table near line "Failed examples in the report").

### 1.2 Benchmarks and models (§5.1, Table 1)

- **Reasoning Gym:** Qwen3.5-4B proposer and solver. 35B teacher for SFT. 21 SFT families, 5 RL families, 21 unseen families. Seed harness = one solver call.
- **Multi-hop QA:** Qwen3-4B proposer, Qwen3-8B solver. Train on HotpotQA. Unseen: MuSiQue, 2WikiMultihopQA. Retrieval is BM25 over 5.23M Wikipedia abstracts, max 20 passages per retrieval, 12 solver calls and 12 retrievals per question (Table 22, App. D.3).
- **No coding benchmark.** No SWE-Bench, no Terminal-Bench, no repo tasks.

### 1.3 Key numbers (exact)

| # | Claim | Number | Ref | Type |
|---|---|---|---|---|
| N1 | Mean single-revision score, 21 unseen RG families | Base 0.32 → after RL 0.62 | §5.2, Fig. 3, Table 9 | Measured |
| N2 | Trained 4B vs 35B teacher, mean | 0.62 vs 0.56 (teacher wins on oracle best-of-8) | §5.2 | Measured |
| N3 | Failed or zero-score proposals, OOD | Base 38%, SFT 35%, Single-step RL 8% | Table 10 | Measured |
| N4 | MuSiQue, independent vs 10 rounds (Single-step RL) | 0.15 vs 0.27 exact match | §5.3, Table 23 | Measured |
| N5 | QA independent proposals (Table 23, Single-step RL) | HotpotQA seed 0.390 → mean 0.4105; MuSiQue 0.1438 → 0.1467; 2Wiki 0.250 → 0.309 | Table 23 | Measured |
| N6 | First revision share of 10-round QA gain | one third to two thirds | §5.3 | Measured |
| N7 | Multistep RL vs Single-step RL | no consistent gain | §5.4, App. C.3 | Measured |
| N8 | Common learned QA edits | pass raw passages to the answer call ("evidence rewiring"); third retrieval hop; search again if the draft answer is not in the passages | §6.2, Fig. 7, Fig. 12 | Measured (qualitative counts in Table 17) |
| N9 | Generated helpers contribute to the answer only in a minority of composed harnesses (11/24, 12/32 structural vs 4/24, 3/32 functional) | Fig. 6c | Measured, small n |

**Asserted, not measured:**
- "A path towards continually learning agents" (Abstract, §7). Not tested.
- Reasons why QA gains last longer than RG gains (§6.3). The paper itself says it cannot attribute this to the seed alone.
- Transfer to longer tool-use tasks (App. F). Listed as future work only.

### 1.4 Limits stated or visible

- Small models (4B–8B). No frontier or Gemini model.
- No coding tasks. The harness is a short Python program, not a multi-turn coding agent.
- Oracle numbers (best-of-8, best-of-80) select on held-out scores. They are not deployable (App. D.4).
- Sequential vs independent comparison matches proposals per run, not total compute or feedback (§5.3).
- Edits capped at 200 lines; failed edits get score 0 (App. D.3).
- Training needs GPU RL. We have no such setup and no plan for one.

---

## 2. Key findings and techniques

- P1. A model can learn to improve a harness from an execution report. Trained proposers transfer to unseen task families (N1, N4).
- P2. Successive revision on the current best harness beats many independent tries at the same proposal count (N4, Fig. 4c, Fig. 5d–e).
- P3. Most training value is in proposal reliability: fewer broken or zero-score harnesses (N3).
- P4. Useful learned edits change **information flow**: give the answer step the raw evidence, not only summaries; add a search when evidence is missing (N8).
- P5. A new helper only helps when the interface around it (parsing, tool I/O, extraction) also works (N9, §6.2).
- P6. Keep feedback, scoring and evaluation questions disjoint (§4.1).

---

## 3. Map to our Harness

### 3.1 Our current evidence (doc 16 §7, phase4 report v2)

- No identical-call loops in any of 26 runs. 0 repeat edits for Harness.
- **T7 gap (paced, n = 3):** Harness 14 neutral / 3 unnecessary vs fx 5 / 1. Extra calls are **list** (Harness 6 every run, fx 1–2) and **search** (Harness 3–6, fx 0–3). Useful reads are similar (Harness 5–6, fx 6).
- **T1v2 paced:** neutral 6 vs 3. A possible second task for the "2 tasks, no rate limit" rule.
- T7 equal-limit rerun is running now (Mike). No conclusion before it lands.
- **Tool facts (from source, `native/crates/harness-tools/src/`):**
  - `list_files`: "non-recursive, max 200" (`filesystem.rs`). On T7 (173 files in nested dirs) one call shows one directory level. This is a likely, **not yet proven**, cause of 6 list calls.
  - `search`: literal or regex, max 50 hits, each line cut at 200 chars, `path:line:text` (`search.rs`).
  - fx has `glob_files` (pattern over the tree) and `grep_files`. We have not measured fx result shapes (fx does not record read/search result sizes).

### 3.2 Mapping table

| Idea | Our weakness | Where | Test with existing tasks + labeler | Expected effect | Cost / risk | Label |
|---|---|---|---|---|---|---|
| P4-a: give the model the structure it needs in one result. For us: a recursive / depth-limited `list_files` (or a `glob`-style option) with a cap | T7 list calls (6 vs 1–2); T1v2 neutral 6 vs 3 | Tool layer (`harness-tools`) | Replay first: from T7 and T1v2 `run.jsonl`, count list calls that only walk down one level of a dir already listed. Then live: T7 + T1v2 paced, n ≥ 3, equal step limits, A/B on the tool option only. Labeler: list neutral, total calls, useful/total, MaxSteps count. S1 must stay "held". | Fewer list calls on T7 if the cause is non-recursion. Risk of larger outputs (more input tokens; G1 token gap may shrink). | Low. Tool-only change. Output size must stay under the 32 KiB cut. | **fits outside core** |
| P4-b: repo map (short tree + top-level symbols) in the first turn or as a tool | T7 list + search | Tool (`repo_map` tool, opt-in) or prompt text built by the CLI outside core. If it must be injected by the loop, core → ADR. | Same A/B on T7 and T1v2. Also count input tokens: a map costs tokens every turn if in the prompt. | Could cut list and some search. On T1v2 (small repo) little effect expected. | Medium. Token cost; staleness after edits; decoy files appear in the map. | **fits outside core** as a tool; **needs ADR** if loop injection |
| P4-c: search result shape (group hits by file, show hit counts per file, say when the 50 cap was hit) | T7 search calls (3–6 vs 0–3) | Tool layer (`search.rs`) | Replay: check existing T7 runs for searches that hit the 50 cap or were followed by a narrower search in the same dir. Live A/B on T7 + T1v2. | Fewer follow-up searches if truncation without a clear note causes them. | Low. | **fits outside core** |
| P2/P6: offline revision loop for prompt and tool descriptions, using our execution reports (labeler output) as feedback, with disjoint dev/eval tasks | All exploration gaps | Process + files outside core (prompt, tool descriptions). No trained proposer; a human or a model proposes, a human approves. | Dev: T1v2. Eval: T7 (or reverse). Never tune and score on the same task. Labeler output = execution report. | Unknown. Paper gains came from RL-trained proposers on non-coding tasks. | Medium: risk of overfitting to 3 small tasks; many runs; quota. | **fits outside core** (process); **skip** for now as a fix — use only after the tool A/Bs |
| P1/P3: train a proposer with RL | — | — | — | — | Needs GPU RL, model training. Out of scope. | **skip** |
| P4 "search again if evidence missing" | none measured | prompt | — | Might **add** search calls. Opposite of our need. | — | **skip** |
| P5: new helper only helps if its interface works | General rule for any new tool (repo map) | Design rule | Every new tool needs a scripted Track A test first. | — | — | **fits outside core** (process rule) |
| Self-modifying harness code at test time | — | Would let a model edit the harness | — | — | Conflicts with deny-all gating and core rules | **skip** |

---

## 4. Top 3 candidates for the next milestone

All three are **measurement first**. None is a fix until the T7 equal-limit rerun lands and the gap shows on 2 tasks with no rate limit.

1. **`list_files` shape: replay, then A/B a recursive or depth-limited option (P4-a). Fits outside core.**
   - Evidence: list 6 every Harness T7 run vs fx 1–2. Our tool is non-recursive (`filesystem.rs`). fx has `glob_files`.
   - Step 1 (no model): add a replay check to `loops_edits.py` or the labeler: "list call on a child of an already-listed dir". Run on T1v2, T7.
   - Step 2 (live, after Harsha's review): A/B on T7 and T1v2 paced, n ≥ 3, equal step limits. Measure list neutral, total calls, input tokens, green, S1 held.
2. **`search` output shape: per-file grouping, hit counts, and a clear "cap reached" note (P4-c). Fits outside core.**
   - Evidence: search 3–6 vs 0–3 on T7. Unknown cause. Replay first: how many Harness T7 searches hit the 50-hit cap, and how many are followed by a narrower search.
3. **Opt-in `repo_map` tool (P4-b). Fits outside core as a tool; needs ADR if injected by the loop.**
   - Evidence: same T7 gap. Higher cost and risk than 1 and 2. Do only if 1 and 2 do not close the list/search gap.
   - Test the tool interface on Track A (scripted) first (P5).

Not in top 3: the offline revision loop (P2/P6). It is a good **process** once we have more tasks. With 3 small tasks, tuning on one and checking on another has very low power.

### 4.1 What this paper does NOT show (for our use)

- Nothing on coding agents, repos, list/search tools or step limits. All our candidates above are our own hypotheses. This paper gives only a general direction (P4: shape of information flow matters).
- No result for Gemini Flash-Lite, or for any model > 8B as solver.
- Small task sets per family (25 dev instances, App. B.4). Oracle numbers are not deployable.
- No cost or token data that maps to our metrics.

---

## 5. Overlaps and conflicts with paper 2609.20804 (doc 16)

- **Overlap: "harness choice depends on model and task".** Doc 16 shows component effects change with model size and task type. This paper adapts the harness per task. Both say: do not assume one fixed harness is best. For us: any list/search change must be tested on more than one task (T1v2 and T7).
- **Overlap: tool interface details matter.** Doc 16: predefined tools vs bash changes call count and edit granularity (bundle effect, Table 7). This paper: helpers fail when parsing or tool I/O fails (P5). Both support testing tool output shape (candidates 1–2).
- **Difference: what changes.** Doc 16 varies fixed components by hand (planning, tools, context). This paper lets a trained model rewrite harness code. We follow doc 16's style: hand-made, one change at a time, A/B.
- **Conflict with our rules:** a harness that rewrites its own code at test time conflicts with deny-all gating, with "core changes need an ADR", and with "no feature work before review". We do not adopt it.
- **Doc 16 candidates update:** doc 16 candidate 1 (stuck detection) and 3 (repeat edits) found no problem (doc 16 §7). Doc 16 candidate 2 (plan tool) stays open, but our gap is exploration breadth, which planning in doc 16 did not target. The list/search shape candidates here are more direct.

---

## 6. Safety note

The PDF was handled as data. We ran no code from it. We found no text in it that tried to direct an agent.

---

## 7. Follow-up: list and search counts on existing runs (candidates 1 and 2)

**Date:** 12 Oct 2026 (IST). **No model calls.** Read-only on `native/benchmarks/head_to_head/runs/`.
**Tool:** `native/benchmarks/tools/list_search.py`. Tests: `test_list_search.py` (10 tests). Run: `python3 -m unittest test_list_search test_loops_edits test_label_tool_calls` in `native/benchmarks/tools/` (31 tests, all pass).
**Data:** `docs/benchmarks/data/phase4_list_search.csv`.
**Runs:** T1v2 and T7, all modes, both agents, complete runs only. One new fx T7 folder (`T7-trackB-fx-postfix-F1-20261012T154240`) appeared while we worked. It is probably part of the equal-limit rerun. We left it out, so the cells match doc 16 §7.

### 7.1 What we count

- **List call:** Harness `list_files`. fx `glob_files` (stderr line `Matching <pattern>`). For fx, the target is the folder before the first glob character.
- **Root list:** target is the workspace root.
- **Drill-down:** a list call whose target's parent folder was listed earlier in the run. A recursive fx glob (`**`) is not counted as a drill-down.
- **Search cap hit (Harness only):** the result has 50 hit lines, or it has the 16 KiB cut note. fx does not record search results, so fx cap hits are "unknown".
- **Search then narrower:** a search followed within 2 calls by a search with a longer pattern that contains the first one, or the same pattern in a deeper path.

### 7.2 Results (medians; per run in brackets)

| Task | Agent | Mode | n | List calls | Root | Drill-down | Recursive glob | Search calls | Cap hits | Then narrower |
|---|---|---|---|---|---|---|---|---|---|---|
| T7 | Harness | paced | 3 | 6 (6,6,6) | 1 | 5 (5,5,5) | 0 | 5 (8,3,5) | 0 | 0 |
| T7 | Harness | postfix | 3 | 6 (5,6,6) | 1 | 5 (4,5,5) | 0 | 7 (7,7,3) | 0 | 0 (0,1,0) |
| T7 | fx | paced | 3 | 2 (2,1,2) | 1 | 0 | 1 (1,1,0) | 0 (0,0,3) | unknown | 0 |
| T7 | fx | postfix | 3 | 1 | 1 | 0 | 1 | 0 (0,0,1) | unknown | 0 |
| T1v2 | Harness | paced | 3 | 2 | 1 | 1 | 0 | 0 | 0 | 0 |
| T1v2 | Harness | proxy | 3 | 2 (3,2,2) | 1 | 1 (2,1,1) | 0 | 1 (1,0,1) | 1 (1,0,1) | 0 |
| T1v2 | fx | paced | 3 | 1 | 0 | 0 | 1 | 0 | – | 0 |
| T1v2 | fx | proxy | 3 | 1 | 0 | 0 | 1 | 0 | – | 0 |

### 7.3 Findings

- **Drill-down explains the T7 list gap.** In every Harness T7 run, 1 list is the root and 4–5 are drill-downs into child folders (`src`, then `src/billing`, then `src/billing/proration`, and so on). fx makes 1 root list and usually 1 recursive glob (`src/**/*.rs` or `**/*.rs`). fx has 0 drill-downs in all runs.
- The same pattern shows on T1v2, on a smaller scale. Harness: root plus 1 drill-down (2 calls). fx: 1 recursive glob.
- So the list gap is on **2 tasks** (T7: 6 vs 1–2; T1v2: 2 vs 1). One T1v2 proxy run (V-H1) had a 429. The paced runs had no 429, so the paced cells alone meet the "2 tasks, no rate limit" rule.
- The cause matches the tool design: Harness `list_files` is non-recursive (`filesystem.rs`). This is an inference from the counts, not a test. Only an A/B run can confirm it.
- **The 50-hit search cap does not explain the T7 search gap.** No Harness T7 search hit the cap (0 of 33 searches). Only 1 search was followed by a narrower search. The T7 search calls are distinct searches for different symbols, not refinements of a capped search.
- On T1v2 proxy, 2 Harness searches hit the cap (V-H1, V-H3). This is too small to act on. Note: our search tool gives no note when it stops at 50 hits. A model cannot know that results are missing.
- **What we still do not know:** why Harness searches more on T7 (5–7 vs 0). fx finds files with one recursive glob and then reads them. Harness may search because it has no recursive file view. This is a hypothesis.

### 7.4 What this changes for the candidates

- **Candidate 1 (list shape) now has evidence on 2 tasks without rate limits (paced).** It stays "fits outside core" (tool layer). It is still a measurement: no build before Harsha's review. The next step is an A/B with a recursive or depth-limited `list_files` option (or a glob tool) on T7 and T1v2, paced, n ≥ 3, equal step limits. Measure list calls, search calls, input tokens, green runs and S1 "held".
- **Candidate 2 (search shape) is weaker.** The cap is not the cause on T7. Keep only the small item "say when the cap is hit" (low cost, tool layer). The search count may fall as a side effect of candidate 1. Measure search calls in the same A/B.
- Wait for the T7 equal-limit rerun (Mike) before any claim.

### 7.5 Limits

- n = 3 per cell. Harness T7 paced runs all stopped at 24 steps; fx had no limit.
- fx search results are not recorded, so fx cap hits are unknown.
- The fx list target comes from the glob pattern text. A pattern like `src/*` counts as a list of `src`.
