# 14 — Phase 3 metric schema (per-run artifact)

**Audience:** Mike (native replay + emit), Thomas (Research Lead), Tech Team  
**Date:** 2026-09-21 (IST)  
**Status:** Schema contract for Phase 3. No harness-core loop change in this drop. No beat-fx claim.  
**Parent method:** [`13-tool-efficiency.md`](13-tool-efficiency.md) (M1–M9, T1–T4, publish DoD).  
**Machine schema:** [`../../native/benchmarks/phase3_schema.json`](../../native/benchmarks/phase3_schema.json)  
**Rule:** ASD-STE100. Mark **measured** vs **derived** vs **unknown**. Emit raw fields. Do **not** collapse into one score.

---

## 1. Purpose

Each Phase 3 agent run must publish one JSON metrics document. Mike builds that document from a recorded `run.jsonl` (and fixture oracle metadata). The document lists counters. It does not invent a single “efficiency score.”

Use this note for field meaning. Use `phase3_schema.json` for validation.

---

## 2. Raw publish rule

1. Emit every field in §3 on every run.
2. If the harness cannot know a value, set the JSON value to the string `"unknown"` (or omit only if the schema marks the field optional — prefer `"unknown"` for required numeric substitutes).
3. Do not replace a set of counters with one weighted score in CI artifacts or README claims.
4. Derived rates (from `13` §3.3) are optional extra keys. They never replace the raw counters.
5. Before any public number, complete the publish DoD in `13` §7 (metric name, task id, method, SHA, machine class, fair axis). Never claim beat-fx without that row.

---

## 3. Per-run JSON fields

### 3.1 Identity

| Field | Type | Meaning |
|-------|------|---------|
| `schema_version` | string | Fixed `"phase3.v1"` for this contract |
| `task_id` | string | Category + fixture, e.g. `T1/broken_calc` |
| `fixture_path` | string | Repo-relative path, e.g. `native/fixtures/broken_calc` |
| `fixture_version` | string | Fixture semver or content hash |
| `commit_sha` | string | Agent / tree under test |
| `machine_class` | string | OS, arch, CI label |
| `model_mode` | string | `fake` \| `scripted` \| `live` \| other |
| `measured_at` | string | ISO-8601 timestamp (prefer IST offset `+05:30` when local) |

### 3.2 Outcome

| Field | Type | Meaning |
|-------|------|---------|
| `exit_status` | string | `success` \| `fail` \| `timeout` \| `max_turns` |
| `task_success` | boolean | Success predicate true (tests green / oracle pass) |
| `run_ok` | boolean | Process finished without harness crash (may still be task fail) |

### 3.3 Tool calls (from `13` M1–M4 + supporting)

| Field | Type | Meaning |
|-------|------|---------|
| `total_tool_calls` | integer | All gated invokes |
| `tool_calls_success` | integer | Tool results with `ok: true` |
| `tool_calls_fail` | integer | Gate deny + tool error + failed shell/test (`13` M4 split) |
| `gate_denies` | integer | Gate deny count |
| `tool_errors` | integer | Tool error count (not gate) |
| `unnecessary_tool_calls` | integer \| `"unknown"` | `13` M1 |
| `redundant_reads` | integer \| `"unknown"` | `13` M2 |
| `redundant_searches` | integer \| `"unknown"` | `13` M3 |
| `duplicate_tool_calls` | integer \| `"unknown"` | Same name+args (+ same result when applicable) as an earlier call |
| `useful_actions` | integer \| `"unknown"` | `13` useful-action count |
| `writes` | integer | write/patch calls |
| `tests_run` | integer | `run_tests` (or equivalent) invokes |

### 3.4 Files

| Field | Type | Meaning |
|-------|------|---------|
| `files_read` | integer | Distinct paths successfully read |
| `files_modified` | integer | Distinct paths written or patched |
| `files_read_paths` | array of string | Optional detail list |
| `files_modified_paths` | array of string | Optional detail list |

### 3.5 Model / tokens / context

| Field | Type | Meaning |
|-------|------|---------|
| `model_turns` | integer | Model responses until stop (`13` M5) |
| `tokens_prompt` | integer \| `"unknown"` | Prompt tokens if adapter reports |
| `tokens_completion` | integer \| `"unknown"` | Completion tokens if adapter reports |
| `tokens_total` | integer \| `"unknown"` | Sum when both known; else `"unknown"` |
| `tool_schema_bytes` | integer \| `"unknown"` | Byte size of tool defs (fake-model substitute for M9) |
| `context_bytes` | integer \| `"unknown"` | Peak or final serialized context / transcript bytes fed to the model |

### 3.6 Time

| Field | Type | Meaning |
|-------|------|---------|
| `wall_time_ms` | integer | Wall ms from run `t0` to Done (`13` M8 family) |
| `time_to_first_useful_action_ms` | integer \| `"unknown"` | `13` M7 |
| `time_to_success_ms` | integer \| `"unknown"` | `13` M8 when `task_success`; else `"unknown"` or omit with `"unknown"` |

### 3.7 Provenance

| Field | Type | Meaning |
|-------|------|---------|
| `source_run_jsonl` | string | Path to the recorded event log |
| `metrics_source` | string | How counters were built, e.g. `replay_aggregate` |
| `notes` | string | Optional short free text (no secrets) |

### 3.8 Forbidden

- No field named `score`, `efficiency_score`, or `grade` in the Phase 3 artifact.
- No invented fx comparison fields.

Optional derived rates may appear under an `rates` object only:

```json
"rates": {
  "unnecessary_rate": 0.0,
  "redundant_read_rate": 0.0,
  "fail_rate": 0.0,
  "useful_actions_per_turn": 0.0
}
```

Rates are supplemental. Raw counters remain required.

---

## 4. How Mike wires the schema

1. Run the agent with `--record <dir>` so events land in `<dir>/run.jsonl` (`RunEvent` lines).
2. After the run (or in `harness replay`), aggregate events:
   - Count `tool_call` / `tool_result` → tool fields.
   - Count `model_response` → `model_turns`.
   - Diff `ts_ms` of first event vs `done` → `wall_time_ms`.
   - Track read/write tool args → file counters.
   - Apply oracle rules from `13` §3.4 / §6.4 for unnecessary / redundant / useful when labels exist; else `"unknown"`.
3. Merge fixture metadata (`task_id`, `fixture_path`, success predicate result).
4. Write `metrics.json` beside the run (or under `native/benchmarks/out/`).
5. Validate with the JSON Schema in `native/benchmarks/phase3_schema.json` (Draft 2020-12).
6. CI fails if required fields are missing. CI does **not** fail on absolute budgets until baselines exist (`13` §4.1).

Python harness remains frozen. Do not change `harness-core` loop for this schema drop; aggregation may live in CLI replay, xtask, or a small bench helper.

---

## 5. Example (illustrative only)

```json
{
  "schema_version": "phase3.v1",
  "task_id": "T1/broken_calc",
  "fixture_path": "native/fixtures/broken_calc",
  "fixture_version": "0.1.0",
  "commit_sha": "UNKNOWN_UNTIL_COMMIT",
  "machine_class": "linux-x86_64-ci",
  "model_mode": "scripted",
  "measured_at": "2026-09-21T09:30:00+05:30",
  "exit_status": "success",
  "task_success": true,
  "run_ok": true,
  "total_tool_calls": 6,
  "tool_calls_success": 6,
  "tool_calls_fail": 0,
  "gate_denies": 0,
  "tool_errors": 0,
  "unnecessary_tool_calls": 1,
  "redundant_reads": 1,
  "redundant_searches": 0,
  "duplicate_tool_calls": 1,
  "useful_actions": 3,
  "writes": 2,
  "tests_run": 2,
  "files_read": 3,
  "files_modified": 2,
  "model_turns": 4,
  "tokens_prompt": "unknown",
  "tokens_completion": "unknown",
  "tokens_total": "unknown",
  "tool_schema_bytes": 4096,
  "context_bytes": 12000,
  "wall_time_ms": 850,
  "time_to_first_useful_action_ms": 120,
  "time_to_success_ms": 850,
  "source_run_jsonl": "/tmp/harness-run/run.jsonl",
  "metrics_source": "replay_aggregate"
}
```

Numbers above are placeholders. Do not cite them as measured results.

---

## 6. Relation to other docs

| Doc | Relation |
|-----|----------|
| [`13-tool-efficiency.md`](13-tool-efficiency.md) | Metric definitions M1–M9; task categories; DoD |
| [`10-fx-beat-goals.md`](10-fx-beat-goals.md) | Honesty; no beat-fx without method |
| [`../benchmarks/native-phase3-slice.md`](../benchmarks/native-phase3-slice.md) | Scripted slice notes |
| Fixture `native/fixtures/broken_calc/` | T1 sample for search + edge-case math |

---

## 7. Non-goals

- No change to Python `src/agentic_harness/` core loop.
- No change required inside `harness-core` for this research/schema drop.
- No secrets in fixtures or metric notes.
- No single score. No beat-fx claim.

---

## 8. One-paragraph verdict

Phase 3 publishes **raw per-run counters** (tools, duplicates, files, turns, tokens-or-unknown, wall time, task success, context bytes) under schema `phase3.v1`. Mike aggregates `run.jsonl` into JSON that validates against `native/benchmarks/phase3_schema.json`. Comparison and budgets come later from measured baselines. Do not ship a single efficiency score or a beat-fx sentence from this schema alone.
