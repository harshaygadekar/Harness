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

| Run | Build | Request 2 tokens in / out |
|---|---|---|
| rerun3 H1 (14 Oct) | `7205729` | 669 / 0 |
| rerun3 H3 (14 Oct) | `7205729` | 669 / 0 |
| pair1 H1 (21 Oct) | `e0fc51e` | 669 / 0 |
| rerun3 H2 (14 Oct) | `7205729` | did not stop; passed the oracle |

The runs come from 2 builds. `git diff 7205729 e0fc51e -- native/crates/` changes only
`harness-cli/src/main.rs` and `harness-cli/src/metrics.rs`: the `oracle_success` field and
how `metrics.json` writes success. The core loop, the model client, the prompt and the tool
set do not change. So the agent sends the same requests in both builds.

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

To name the cause, the proxy must record the exact upstream end value. Plan (Chief
approved, Jessie's form): add Gemini's known end values to the allow list (`OTHER`,
`MALFORMED_FUNCTION_CALL`, `UNEXPECTED_TOOL_CALL`, `MAX_TOKENS`, `SAFETY`, `RECITATION`,
`BLOCKLIST`, `PROHIBITED_CONTENT`, `SPII`) and log every other value as `other`. It needs
a new pin and Jessie's review. Then a 10-time replay of request 2 gives the value and the rate.
The result goes in this doc.

## Replay of request 2 (2 Nov 2026)

Status: gate failed: 400 `INVALID_ARGUMENT`, no thought signature. No try counts.
This result says nothing about model quality.

- Path: `BENCH_REPLAY=h1req2` in `run_isolated.sh`, commit `8ee0e1e`.
  Pin `/opt/benchproxy/4b02f425bedc94b2829d1d94f84005049af4e602`, proxy.py sha256 `49a946d6…a398`.
  Harness binary `e0fc51e`, sha256 `df9db25d…99c9`.
- Record: `runs-replay/replay-h1req2-20261102T135315/replay_record.json` (out of git).
  `run_meta.json` sha256 `ebfbb03c…8bb8`, logged before the first request.
- Request fields from `e0fc51e`: `model` = `gemini-3.5-flash-lite`, `stream` = false,
  3 messages (no system prompt), 9 tools. Harness sends no `temperature`, `tool_choice`
  or `max_tokens`. So Gemini uses its default and samples.
- Body: 2978 bytes, sha256 `b8791401…1393`. It does not hold the thought signature from
  H1 reply 1. The proxy took that signature out of reply 1 and put it back into request 2
  in H1, but it never logged it.

Result: the calibration try got HTTP 400, error code `INVALID_ARGUMENT`, in 287 ms.
It had no tokens, no end value and no reply model. The gate stopped the run. 0 of 10 tries were sent.

### What this shows

- **We cannot reproduce H1 request 2 without its signature.** This replay tells us nothing
  about the empty-reply rate.
- A likely cause of the 400 is that the earlier tool call in the request has no thought
  signature. This is a lead only. It is not tested.
- If that lead is true, H1 request 2 got a 200, so it carried a signature that Gemini
  accepted. That makes it less likely that a missing signature caused the empty reply.
  It does not rule out a signature that was present but wrong.
- H1 did not log the reply model. Only the request name is on record. So we cannot show
  that a later run uses the same model as H1.

### Harness relies on the proxy for the signature

In H1, the bench proxy carried the thought signature from reply 1 to request 2.
Harness did not. So for this request the proxy is not a neutral pipe: request 2
works only because the proxy changes it. If the signature step is the cause, the fix
goes in the Gemini model adapter (the provider seam in ADR-002), not in the core loop
and not in the bench proxy. No core change is needed now.

### Leads, none proven

1. The proxy step that takes out and puts back the thought signature. fx went through
   the same proxy and passed, so this is a lead only.
2. The model starts a tool call that it cannot form (for example `MALFORMED_FUNCTION_CALL`).
   The new pin logs the exact end value, so the next live Harness run will show it.
3. Default sampling. The prompt was the same in 4 runs and 1 of them (H2) did not stop.

### Next step (Chief's ruling, 3 Nov)

- No placeholder run. It changes the request, so it cannot tell us anything about H1.
- The proxy logs 2 flags, yes or no, never the value: "signature in reply 1" and
  "signature put back in request 2". Jessie reviews it, then a new pin.
- 5 fresh live Harness T1 runs on the `e0fc51e` binary, on the new pin. Label:
  diagnostic, not compared. Every guard rule applies. fx does not run.
- On request 2, each record shows the end value, the reply model and the 2 flags.

With 5 runs the rate stays rough. For example, 3 empty replies in 5 runs gives a
95% interval of about 15% to 95%. This doc will give the interval, not only the count.
