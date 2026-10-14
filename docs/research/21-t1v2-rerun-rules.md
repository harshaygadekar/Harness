# 21. T1 v2 cap50 rerun: analysis rules (set before the first run)

Status: fixed before any rerun. Ruling by The Chief, 2026-10-13 (Tech Team).

## Build
- Harness is built from a commit on `origin/main` at run time. It includes the retry fix `9fca9d9`.
- The build SHA goes in each `run_env.json` and in each table row.
- T1 v2 rows are never compared in the same table with T7 cap50 rows (build `92b84dc`).

## Run order
- Runs are interleaved F/H.
- First, run one pair. Continue only if that pair has 0 upstream 503s.
- A slot can be rerun at most 2 times per day. If that is not enough, the whole set waits.

## Upstream-affected runs
- The proxy log gives a 503 count for each run.
- A run with 1 or more 503s is "upstream-affected". It stays in the table, with its 503 count.
- It counts for nothing: not for the pass rate, not for "N of N", not for time and not for tokens. A pass and a fail are treated the same way.
- Reason: Harness retries up to 6 times, and fx handles 503s in a different way. If we counted these runs, the result would measure our retry code, not the task.
- The slot is run again later.
- If Harness passes through 503s where fx stops, that is a separate claim. It stays in the backlog until it has its own planned test.

## Inputs
- Runs that `run_guard.refuse_reason` refuses (fake or killed) are not counted. `time_split.py` and `results_table.py` apply this check.
- For Harness time, use `agent_wall_ms_excl_retry_wait`.
- Request counts: proxy requests minus `model_retries.reasons.http_503` and `reasons.http_429`. Connection retries are matched to proxy rows by time, not subtracted. A retry that cannot be matched is listed.
- fx token numbers are self-reported, and they do not count the review requests.

## Amendment (The Chief, 2026-10-15)
- Runner time limit: 900 s for BOTH agents (fx and Harness). `run_isolated.sh` sets it and writes `agent_timeout_s` in `run_env.json`.
- A transport error at the proxy (usage row with `"status": "error"`) counts the same as a 503. The run is upstream-affected and counts for nothing.
- So the first rerun F1 (`T1-trackB-fx-isolated-cap50-rerun-F1-20261014T165804`, 2 transport errors, 480 s limit) is upstream-affected.
