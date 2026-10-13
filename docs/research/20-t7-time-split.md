# 20. T7 cap50: where the wall time goes

Label: weak model (Gemini Flash-Lite) through the proxy.

Runs: `native/benchmarks/head_to_head/runs/T7-trackB-{fx,harness}-isolated-cap50-*-20261012T17*` (3 per agent; the 2 ABORTED fx attempts are not used). Results doc: `docs/benchmarks/first-head-to-head.md` (T7 cap50 section).

Stray-process scan (Mike, 2026-10-13): all T7 cap50 runs F1-F3 and H1-H3 are clear. Build `92b84dc`.
Script: `native/benchmarks/tools/time_split.py RUN_DIR...` (tests: `test_time_split.py`). No model was called for this study.

## 1. Method

| Part | Harness | fx |
|---|---|---|
| (a) Pacing hold | measured: `hold_ms` in `proxy_hold.json` (proxy waits until 5 s after the previous request start) | measured, same |
| (b) Model/proxy wait | measured: sum of `model_request` -> `model_response` in `rec/run.jsonl`, minus (a) and (c) | inferred: wall - (a) - (c) - (d). Includes (e) |
| (c) 429/retry backoff | measured: `retry_wait_ms` (summary.json) | 0 (no 429 in `proxy_hold.json`, `fx_429_notices` 0) |
| (d) Tool time | measured: `tool_call` -> `tool_result` | measured for shell only (`duration_ms` in `stdout.txt`); other tools assumed near 0 |
| (e) Agent overhead | measured: wall - request time - tool time | not separable; inside (b) |

Request gaps per request: Harness from `rec/run.jsonl`; fx from `ts_ms` gaps in `proxy_usage.jsonl` (time at end of each response; includes hold and tool time).
fx upstream counts include `--auto` review requests (2-4 per run). fx token numbers are self-reported by the proxy usage log.

## 2. Per-run table (ms)

| Run | Wall | (a) Hold | (b) Model | (c) Retry | (d) Tool | (e) Overhead | Upstream req | Agent req | First edit (step) | (b) per upstream req | Out tok / req |
|---|---|---|---|---|---|---|---|---|---|---|---|
| fx F1 | 91,535 | 71,299 | 19,603* | 0 | 633 | in (b) | 19 | 16 | 12 | 1,032* | 40.4 |
| fx F2 | 149,045 | 62,345 | 85,643* | 0 | 1,057 | in (b) | 17 | 15 | 12 | 5,038* | 40.3 |
| fx F3 | 105,900 | 75,382 | 29,704* | 0 | 814 | in (b) | 22 | 19 | 14 | 1,350* | 33.4 |
| Harness H1 | 413,083 | 131,054 | 280,020 | 0 | 1,983 | 26 | 33 | 33 | 26 | 8,485 | 28.7 |
| Harness H2 | 292,080 | 60,515 | 223,093 | 7,181 | 1,269 | 22 | 42 | 39 | 32 | 5,312 | 25.3 |
| Harness H3 | 295,945 | 77,039 | 216,088 | 2,021 | 756 | 41 | 21 | 20 | 15 | 10,290 | 35.0 |

\* inferred (includes fx overhead). fx "agent req" = steps + 1 (not verified). All runs passed the oracle.
Medians: wall fx 105,900 vs Harness 295,945. Hold fx 71,299 vs Harness 77,039. Model fx 29,704 (inferred) vs Harness 223,093. Tool fx 814 vs Harness 1,269.

## 3. One long request per run

Most requests (Harness and fx) end 4-5 s after the previous one. That is the 5 s pacing floor; real model time per request is short (below 1 s for most).

- Harness H1: one request took 255 s (request 21; 20-24 output tokens, status 200). Without it, H1 model time is about 29 s.
- Harness H3: one request took 203 s (request 19; 10 output tokens, status 200). Without it, H3 model time is about 17 s.
- Harness H2: no single stall. Requests 1-22 took 4-22 s each (slow upstream), and 3 upstream 503s were retried (7.2 s backoff).
- fx F2: one gap of 73 s (request 5). fx F1 and F3: no gap above 9 s.

The long requests had short outputs (10-24 tokens). Thus long outputs do not explain them. They are slow upstream answers (Gemini side or network). We cannot see inside the upstream.

## 4. Conclusion: what makes the wall-time gap (not interleaved)

- Main cause: slower requests, from a few upstream stalls. The model-wait part (b) differs by about 190 s at the median (223 s vs about 30 s, fx value inferred), and this is nearly all the wall gap (296 s vs 106 s). In H1 and H3, one request of 200-255 s gives most of it. In H2, many slow requests and 503 retries give it.
- Second cause: more requests. Harness sent 21-42 requests; fx 17-22. With the 5 s pacing floor, each extra request costs about 5 s. Pacing hold itself is similar (median 77 s vs 71 s), except H1 (131 s, 33 requests).
- Not a cause: 429s (none), tool time (about 1-2 s per run for both), Harness overhead (below 50 ms).
- Confounder: the runs were not interleaved. fx ran 22:38-22:46 IST and Harness 22:46-23:08 IST on 2026-10-12. The upstream can be slower at a different time (H2 got 503s). Thus the stalls may come from the time of the run, not from the agent. This is inferred, not verified.
- If we remove the single largest request in each run, the Harness walls are about 158 s (H1), 270 s (H2) and 93 s (H3). This is a rough check only.

## 5. T1v2 cap50

Not complete. There is no T1v2 cap50 run folder. There is one T1 (v1) cap50 run, `T1-trackB-fx-isolated-cap50-F1-20261013T162343`: fx only, `oracle_success` false, stop reason `no_json (agent_exit 124)`. There are no Harness runs. We did not analyze it.
