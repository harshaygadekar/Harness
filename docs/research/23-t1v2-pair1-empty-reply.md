# 23. T1 v2 pair 1: the empty reply again

Status: research note. No win claim. No fix proposed.

## Run set

- Pair run on 21 Oct 2026. Weak model (Gemini Flash-Lite), through the proxy pin `588290c`.
- Harness built from `e0fc51e`, sandbox absent (one-time exception by the Chief). Bench scripts from `fed68a5`.
- Folders, under `native/benchmarks/head_to_head/runs/`:
  - H1: `T1-trackB-harness-isolated-cap50-pair1-H1-20261021T101802`
  - F1: `T1-trackB-fx-isolated-cap50-pair1-F1-20261021T101823`
- Both runs: 0 × 429, 0 × 503, 0 transport errors. The guard counts both.
- The first H1 try (`…H1-20261021T101627`) did not start (exit 127, no `time` binary). It is not counted.

## Result

| Run | Oracle | Requests | Tokens in / out | Edits |
|---|---|---|---|---|
| H1 (Harness) | fail (`cargo test` exit 101) | 2 | 1289 / 10 | none |
| F1 (fx) | pass (10 tests) | 12 | 64722 / 661 | `src/midpoint.rs`, `src/percent.rs` |

## What H1 did

From `rec/run.jsonl` and `proxy_usage.jsonl`:

1. Request 1: 620 tokens in, 10 out, finish label `tool_calls`. The model called `list_files`.
2. Request 2: 669 tokens in, **0 out**, finish label `other`, HTTP 200. No text and no tool call.
3. Core ended the run as `Succeeded` after 2 steps, as the protocol says (`succeeded` is not a pass).

## Comparison with doc 22

The same stop now shows in 3 of 4 Harness runs on T1 v2:

| Run | Request 2 tokens in / out |
|---|---|
| rerun3 H1 (14 Oct) | 669 / 0 |
| rerun3 H3 (14 Oct) | 669 / 0 |
| pair1 H1 (21 Oct) | 669 / 0 |
| rerun3 H2 (14 Oct) | did not stop; passed the oracle |

The input count is the same each time. So the model gets the same prompt at the same point,
after the same `list_files` result, and it often gives nothing back. fx never stopped this
way. fx sends a different prompt, so it never sends this exact request.

## What the finish label tells us

The label is `other`. In the pin `588290c`, `other` means one of 2 things:
the upstream sent the word `other`, or it sent a value that is not on the allow list
(`stop`, `length`, `tool_calls`, `content_filter`, `safety`, `recitation`).
The log cannot tell these 2 cases apart.

It is not `stop`, so this is not a normal end of turn. It is not `length`, and the reply is
far below any limit. It is not `content_filter` or `safety`.

Gemini's own API has more end reasons than the OpenAI-compatible list, for example
`OTHER`, `MALFORMED_FUNCTION_CALL` and `UNEXPECTED_TOOL_CALL`. A tool call that the
model started but could not form would give 0 output tokens with no text. **This is an
inference. The logs do not prove it.**

## What this does and does not show

- It shows a repeatable stop on 1 task (T1 v2), at 1 point (request 2).
- It does not show the stop on a second task. The T7 and T9 scan found 0 empty replies.
- Under Harsha's rule, no fix is proposed. The parked `ModelClient` wrapper stays parked.

## Open question for the team

To name the cause, the proxy must record the exact upstream end value. A safe way:
log the raw value only if it matches `^[A-Za-z_]{1,40}$`, else log `other`.
That logs a fixed word, never model text. It changes `proxy.py`, so it needs a new pin
and Jessie's review. The Chief decides if and when.
