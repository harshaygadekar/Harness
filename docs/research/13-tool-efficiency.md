# 13 — Tool-efficiency metrics (native coding-agent track)

**Audience:** Mike (native impl after ADR-005), Thomas (Research Lead), Tech Team  
**Date:** 2026-09-20 (IST)  
**Status:** Research method only. No native source in this note. No beat-fx claim.  
**Rule:** Measure intelligent tool use, not only speed. Mark **measured** vs **scripted**. Say **unknown** when unknown. Do not invent fx numbers.

**Scope:** How the native Rust coding agent (ADR-005 Accepted) proves **useful work per tool call**, per turn, and per token. Complements size/start benches in [`12-native-language.md`](12-native-language.md) and honesty rules in [`10-fx-beat-goals.md`](10-fx-beat-goals.md). Alice success item 3 in [`useful-scope-native.md`](../useful-scope-native.md): publish a tool-efficiency bench that counts wasteful reads, searches, and tests.

---

## 1. Why this note exists

Speed alone is not enough. An agent can be fast and still waste calls:

- read the same file many times
- search with no new information
- fail a tool and retry without change
- take many model turns before the first useful edit

Product story (Alice):

> Useful coding-agent work per byte, per ms, per tool call, and per model token.

We never say “we beat fx on tool-efficiency” without the method in this note and a published run (SHA + machine + scripts).

---

## 2. Terms

| Term | Meaning |
|------|---------|
| **Tool call** | One gated invoke of a named tool (list, search, read, write/patch, git, shell, tests, …). |
| **Model turn** | One model response (fake or live) that may contain zero or more tool calls, or a final answer. |
| **Useful action** | A tool call that **changes** workspace state toward the task goal, or **new** information that later enables such a change. First definition for CI: see §3. |
| **Unnecessary tool call** | A call that does not add new information and does not change state needed for success. |
| **Redundant read/search** | A read or search whose path/query matches a prior call in the same run and whose result is unchanged. |
| **Failed call** | Gate deny, tool error, bad args, or non-zero exit when the tool reports failure. |
| **Fake model** | Deterministic script of model responses. No network. No API key. |
| **Time to first useful action (TTFU)** | Wall time from run start to the first useful action. |
| **Time to success (TTS)** | Wall time from run start to task success predicate true. |

---

## 3. Metrics (what we count)

Publish these fields on every tool-efficiency run. Use integers or ms. Use **unknown** if the harness cannot record the field yet.

### 3.1 Waste and quality

| Metric ID | Name | How to count | Lower is better? |
|-----------|------|--------------|------------------|
| **M1** | Unnecessary tool calls | Calls labeled unnecessary by the harness oracle (§5.3) | Yes |
| **M2** | Redundant reads | `read` (or equivalent) with same resolved path as an earlier successful read, and file mtime/hash unchanged | Yes |
| **M3** | Redundant searches | `search`/`grep`/`glob` with same query+root as an earlier successful search, and workspace hash unchanged | Yes |
| **M4** | Failed calls | Gate deny + tool error + failed shell/test invoke | Context: prefer zero **unexpected** fails; expected fails in locate-bug tasks are OK if labeled |
| **M5** | Model turns | Count of model responses until success or max-turns | Usually yes for same task |
| **M6** | Useful actions / turn | `useful_actions / model_turns` (0 if turns = 0) | Higher is better |
| **M7** | Time to first useful action | ms from process start (or harness `t0`) to first useful action | Yes |
| **M8** | Time to success | ms from `t0` to success predicate | Yes |
| **M9** | Tokens (if available) | Prompt + completion tokens if the adapter reports them; else **unknown**. Fake-model CI may report **schema bytes** of tool defs instead | Lower is better when comparable |

### 3.2 Supporting counters (always emit)

| Field | Purpose |
|-------|---------|
| `total_tool_calls` | Denominator for waste rates |
| `useful_actions` | Numerator for M6 |
| `gate_denies` | Split of M4 |
| `tool_errors` | Split of M4 |
| `writes` / `patches` | State-changing calls |
| `tests_run` | Test-tool invokes |
| `exit_status` | `success` \| `fail` \| `timeout` \| `max_turns` |
| `task_id` | Category + fixture name |
| `commit_sha` | Agent binary / tree under test |
| `machine_class` | OS, arch, CI label |

### 3.3 Derived rates (optional publish)

- `unnecessary_rate = M1 / total_tool_calls`
- `redundant_read_rate = M2 / max(1, read_calls)`
- `fail_rate = M4 / total_tool_calls`

Do not treat a rate as a beat-fx claim. fx does **not** publish these rates as first-party SLAs in our pack (`10` §1 “not found”).

### 3.4 What is a “useful action” (CI default)

Label a tool call **useful** if **any** of these is true:

1. It **writes or patches** a file that the success predicate later requires.
2. It is the **first** successful read/search that returns content the scripted oracle marks as required for the next write (fixture metadata).
3. It runs **tests** and the result flips from fail→pass or is the first observation of the failing test name (locate / repair tasks).

Label **unnecessary** if:

1. Duplicate read/search under M2/M3 rules.
2. List/search that returns nothing new after the workspace hash is known and the oracle already listed needed paths.
3. Write that the success predicate does not need (wrong file or reverted before success).
4. Shell/test invoke with identical command and identical result as an earlier call in the same run.

Live-model scoring may refine labels later. Fake-model CI uses **oracle labels in the fixture** so scores stay deterministic.

---

## 4. Task categories

Start with **fake-model scripts**. Live models are optional later and must not be the only CI proof.

| ID | Category | Goal | Success predicate (examples) | Fixture shape |
|----|----------|------|------------------------------|---------------|
| **T1** | Bug fix | Make a known failing unit test pass by editing production code | Named test exits 0; no unrelated files required | Small repo + one broken function + test |
| **T2** | Test repair | Fix a broken or outdated test (code under test is correct) | Test suite green; production hash unchanged (or allow-list empty) | Green code + wrong assertion/import in test |
| **T3** | Small feature | Add a small, specified behavior | New test passes; public API matches spec file | Spec markdown + failing stub test |
| **T4** | Locate hidden bug | Find the faulty file/function before fixing | Agent reports correct location **or** applies fix only in allow-listed path | Bug seeded far from obvious entry; decoy files |

### 4.1 Order of work for Mike

1. **T1–T4 fake-model scripts** in `native/benchmarks/tool_efficiency/` (or agreed path under `native/benchmarks/`).
2. Wire counters M1–M9 into the harness JSON artifact.
3. Gate CI on **determinism** (same scores on repeat) before gating on absolute budgets.
4. Only then set numeric floors (e.g. max unnecessary calls per task) from measured baselines — do not invent budgets in this research note.
5. Live-model runs: same fixtures, separate job, network allowed only with secrets; never replace fake-model CI.

### 4.2 Shared rules for all categories

- Workspace is a **temp copy** of a fixture. No write into the source tree.
- Default **deny-all**; Allow only the min coding pack needed for the task (Jessie).
- **No network** in fake-model CI (see §6).
- Max turns and max wall time are fixture fields.
- Prefer opt-in shell. Prefer tests via an explicit `run_tests` tool when possible.

---

## 5. Fair vs unfair vs fx

Native product may fight fx on tool-efficiency **only** after both sides use a **comparable method**. Until fx publishes a matching harness, compare **our** scores to **our** baselines and to shared open scripts — not to invented fx figures.

### 5.1 Fair (native ↔ fx class)

| Axis | Why fair | How we measure | When a comparison is valid |
|------|----------|----------------|------------------------------|
| Useful work per tool call | Both are coding CLIs with gated tools | M1–M6 on shared task scripts | Same fixtures + published scripts + SHAs |
| Fake-model dispatch cost | Loop + gate overhead without LLM noise | Tool calls / s or µs/call on no-op tools | Ours only unless fx publishes the same method |
| Tokens / schema bytes for tools | Cost of tool surface | M9 when available; else tool-schema byte size | Same tool pack description language |
| Time to success on scripted tasks | End-to-end usefulness | M8 on T1–T4 | Same machine class and fixture versions |

### 5.2 Unfair / forbidden

| Claim | Why forbidden |
|-------|----------------|
| “We beat fx on tool-efficiency” with no shared script | No method |
| Citing fx **1.6× / 2.5×** turn speed as our win | fx vs **prior fx** only (`10`) |
| Citing unpublished fx Zig dispatch µs | fx has **no** first-party absolute tool-dispatch SLA in our pack (`10`, `11`) |
| Comparing Python `~122k/s` dispatch to native or to fx | That number is **our Python** fake-model bench, not fx |
| Using marketing **10 µs** start as related to tool skill | Wrong axis; engineering start budget is **2 ms** process help-class |
| Counting more built-in tools as “more efficient” | Tools stay plugins; count is not efficiency |
| Live-only demos as the only proof | Alice / Phase rules: fake-model first |

### 5.3 vs our Python harness

| | Python harness | Native track |
|--|----------------|--------------|
| Role | Library + plugins; Phase 2 benches | Separate coding CLI |
| Fair shared idea | Fake-model loop, gate, JSON artifacts | Same idea under `native/benchmarks/` |
| Unfair | Import-ms vs process-ms; LOC vs MiB | Do not mix into one “we win” sentence |

Python benches (`benchmarks/tool_dispatch.py`, `fake_coding_loop.py`) stay. They do **not** replace native T1–T4 tool-efficiency tasks.

---

## 6. Fake-model harness design (CI, no network)

### 6.1 Goals

1. **Deterministic** scores for M1–M9 on T1–T4.
2. **No network**, no provider SDK, no API key.
3. Runs on Linux CI with the native binary (or `cargo test` harness driver).
4. Emits one JSON document per task for artifacts.

### 6.2 Components

```text
native/benchmarks/tool_efficiency/
  README.md                 # how to run; no-network rule
  fixtures/
    T1_bug_fix/...
    T2_test_repair/...
    T3_small_feature/...
    T4_locate_hidden/...
  scripts/                  # fake-model response sequences (JSON or Rust)
  oracle/                   # success predicates + useful/unnecessary labels
  run_suite.rs              # or cargo bench / xtask entry
  schema.json               # output artifact schema
```

Layout is a **proposal**. Mike may rename crates; keep the seams.

### 6.3 Fake-model script format (conceptual)

Each fixture ships an ordered list of model turns. Each turn is one of:

- `tool_calls`: list of `{id, name, arguments}`
- `final`: assistant text (stop)

The runner:

1. Sets `t0`.
2. Applies deny-all + Allow list from fixture.
3. For each turn: record turn, execute tools, append observations, update counters (M1–M9).
4. Stops on success predicate, max turns, or timeout.
5. Writes JSON artifact.

**Baseline scripts** should be **competent but imperfect**: include at least one redundant read or extra search so M2/M3 are non-zero and the counters are tested. Also ship an **optimal** script (minimal useful path) as a lower bound reference — not as a live-model claim.

### 6.4 Oracle

Per fixture:

| Oracle field | Role |
|--------------|------|
| `success` | Shell/test command or file hash checks |
| `required_paths` | Paths that must be read or written |
| `allow_write_paths` | Writes outside this set → unnecessary or fail |
| `hidden_bug_path` | T4 only — correct location |
| `label_overrides` | Optional per-call useful/unnecessary tags for scripted turns |

### 6.5 CI shape (proposal)

```bash
# From native workspace — exact commands land with Mike’s scaffold.
# No network. Fake model only.

cargo xtask tool-efficiency --suite T1,T2,T3,T4 --out target/te-report.json
# Expect: all tasks exit_status=success for the "optimal" scripts
# Expect: baseline scripts emit M1–M9 fields; suite is deterministic (n≥3 identical)
```

**Fail the build when:**

- any required fixture fails determinism check
- network is attempted (block or unset proxy; fail on DNS/connect if a guard exists)
- artifact missing required metric fields
- gate opens all tools with zero Allow flags (Jessie CI condition)

**Do not fail the build when:**

- fx is slower/faster on the same runner
- live-model optional job is red (keep optional)

### 6.6 Isolation

- Temp workspace per task.
- Restore nothing from prior tasks.
- Seed RNG only if a tool needs it; default tools are deterministic.
- Record tool-schema bytes once per suite for M9 substitute when tokens are unknown.

---

## 7. Definition of done — publishing a number

Use this checklist before README, ROADMAP, demo, or social copy cites a tool-efficiency number.

| Field | Rule |
|-------|------|
| **Metric** | Named from §3 (e.g. M1, M8), not a vague “more efficient” |
| **Task** | Named T1–T4 fixture id + fixture version/hash |
| **Method** | Fake-model or live; link to script path; state no-network for CI numbers |
| **Ours** | Value + median/p95 if timed; commit SHA; machine class; build flags |
| **fx** | Quote a **first-party** fx number **or** write **“no fx number; internal only”**. Never invent |
| **Fair axis?** | Must match §5.1. If unfair (§5.2), do not publish as a comparison |
| **Artifact** | JSON (or equivalent) attached or linked from the commit |
| **Pass** | Checklist complete; Alice scope not violated; Jessie gate rules held |
| **Fail** | Missing method, missing SHA, invented fx figure, live-only proof, or unfair axis |

### 7.1 Concrete examples

1. **“Native agent uses 40% fewer tool calls than fx”** — **INVALID** until a shared fixture pack and fx-side method exist.  
   **Valid substitute:** “On fixture T1_v1, optimal fake script: total_tool_calls=N, unnecessary=0 (SHA…, Linux CI).”

2. **“Time to success 1.2 s on T3”** — **VALID** as an **internal** number with method + SHA + machine. Not a beat-fx claim.

3. **“Beat fx on tool dispatch µs”** — **INVALID** (no fx first-party absolute dispatch SLA in our pack).  
   **Valid substitute:** Publish our fake-model dispatch/s for native; compare only to our prior SHAs.

4. **“Useful actions/turn = 1.5 on T4 baseline script”** — **VALID** when M6 is in the artifact and the script is checked in.

### 7.2 Refresh rule

When fixtures change, bump fixture version and re-baseline. Do not compare new fixture scores to old scores without a note.

---

## 8. Relation to other docs

| Doc | Relation |
|-----|----------|
| [`10-fx-beat-goals.md`](10-fx-beat-goals.md) | Honesty + DoD template; process/size axes |
| [`11-coding-agent-product.md`](11-coding-agent-product.md) | Myth fix: ~122k/s is ours, not fx |
| [`12-native-language.md`](12-native-language.md) | Language bake-off; fair native axes include tool-efficiency |
| ADR-005 | Rust native track Accepted; Mike implements benches after Accept |
| [`useful-scope-native.md`](../useful-scope-native.md) | Success requires published tool-efficiency bench |

---

## 9. Non-goals (this note)

- No native or Python **core code** changes in this research drop.
- No push.
- No invented fx tool-efficiency numbers or budgets.
- No claim that fake-model skill equals live-model skill.
- No OS sandbox marketing (Jessie: later measured ADR).

---

## 10. One-paragraph verdict

Tool-efficiency for the native coding agent means **counting waste and useful work**, not only ms and MiB. Mike’s first CI path is a **no-network fake-model harness** on task categories bug fix, test repair, small feature, and locate-hidden-bug, emitting unnecessary calls, redundant reads/searches, failed calls, turns, useful actions per turn, time to first useful action, time to success, and tokens-or-schema-bytes when available. Fair comparison with fx needs shared scripts and first-party fx numbers; until then we publish **our** artifacts with method, SHA, and machine class only. Do not claim beat-fx on tool-efficiency without the §7 DoD row complete.
