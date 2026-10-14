# 22. T1 v2 cap50 rerun: time and request split

Label: weak model (Gemini Flash-Lite) through the proxy. Build `7205729`. Proxy pin `8ce5135`. Jessie reviewed the pin after the runs, not before. This is not a win claim. Harsha reviews first.
Rules: doc 21. All 6 runs have 0 503s and 0 upstream transport errors. `run_guard` refused none of them.
Runs: `native/benchmarks/head_to_head/runs/T1-trackB-{fx,harness}-isolated-cap50-rerun3-*-20261014T19*` (interleaved). Tool: `native/benchmarks/tools/time_split.py`.

## Result
fx passed 3 of 3. Harness passed 1 of 3. H1 and H3 are failures, not reruns.

| run | oracle | wall | hold | model | tool | requests | first edit |
|---|---|---|---|---|---|---|---|
| F1 | pass | 64.4 s | 34.5 s | 27.7 s* | 2.2 s | 12 | 5 |
| F2 | pass | 57.3 s | 44.0 s | 11.7 s* | 1.7 s | 12 | 5 |
| F3 | pass | 55.9 s | 43.9 s | 11.1 s* | 0.9 s | 12 | 5 |
| H1 | fail | 6.0 s | 4.6 s | 1.5 s | 0 s | 2 | none |
| H2 | pass | 60.8 s | 50.2 s | 9.4 s | 1.2 s | 13 | 6 |
| H3 | fail | 6.3 s | 4.6 s | 1.7 s | 0 s | 2 | none |

\* inferred: fx wall time minus hold and tool time. It includes fx's own overhead.
fx requests are counted at the proxy and include the `--auto` review requests.

## Findings
1. Time: in H2 and in all fx runs, most of the wall time is pacing hold (54% to 83%). Model time per request is about 0.7 s for Harness and 0.9 to 2.3 s for fx (inferred). The T7 wall-time gap (doc 20) does not show on T1.
2. Requests and first edit: H2 made 13 requests and edited first at step 6. fx made 12 and edited first at step 5. The T7 pattern of more requests and a later first edit shows on 1 task only. By the two-task rule, no fix is proposed, and the system-prompt test stays parked.
3. Empty reply: H1 and H3 failed in the same way. Request 1 had 620 input tokens and 10 output tokens, and it called `list_files`. Request 2 had 669 input tokens and returned HTTP 200 with 0 output tokens. Under ADR-001, a reply with no tool call ends the run, so Harness stopped with no edit. The context was small, so context length did not cause it.
4. The proxy logs from these runs have no finish reason. We cannot tell a normal stop from a safety stop or another stop reason. Mike added a `finish_reason` label after these runs. Until it has data, the cause of the empty reply is unknown.
5. H1 `metrics.json` reports `task_success: true`, but the oracle failed. This is a separate metrics bug. Mike is tracing it.

## Next
- Look for 0-token replies in the T7 and T9 logs. Those logs have no finish reason.
- If the empty reply shows on a second task: the first fix goes in a plugin (Walter's ruling under ADR-003). Core changes only if a plugin cannot see the empty reply. That needs a failing `examples/` path.
