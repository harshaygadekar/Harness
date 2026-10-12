# 18. Token overhead: fx vs Harness

Label: weak model (Gemini Flash-Lite) through the proxy; raw, not reviewed.

All fx token numbers in this document are self-reported; fx --auto review requests not counted.

Limit (Walter): Part of the 920 vs 6,330 gap is that Harness sends no system prompt.

The fx license is not checked. Thus this document holds no fx prompt text and no fx tool text; only counts and summaries.

Run data: `native/benchmarks/head_to_head/runs/` in the shared checkout (not in git).
No live model run was done for this study.

## 1. Request bodies: not saved

No request bodies are saved. The proxy (`native/benchmarks/tools/oai_stream_proxy/proxy.py`, README "Security properties") logs only method, path and status. `proxy.log` in each run folder has lines such as `POST /chat/completions -> 200` only. `proxy_hold.json` has counts only. Harness `rec/run.jsonl` has `model_request` events with `message_count` and `tool_count` only. fx `stdout.txt` has the total `usage` only, not usage per request.

Thus we cannot compare request N with request N+1, and we have no first-request input tokens for either agent.

### Minimal model-free way to capture one first request (proposal only, not done)

1. Start a fake upstream on 127.0.0.1. It writes the received JSON body to a file, after scrub, and returns one fixed SSE answer with `finish_reason: "stop"` and no tool calls.
2. Start the proxy with `PROXY_UPSTREAM` set to the fake upstream.
3. Run each agent once on the task with a dummy key (not a real key). The agent stops after one request.
4. Do not log the `Authorization` header. Scrub the body with the `scrub_secrets` rules before you save it.

Risk: fx can use its own saved provider login. Make sure fx sends to the proxy and has no real credentials in that run.

## 2. Fixed overhead per request

| Item | Harness | fx | Source |
|---|---|---|---|
| System prompt | none (first request has `message_count: 1`) | base prompt 5,889 chars (about 1,470 tokens), plus other context (measured in section 8) | `rec/run.jsonl`; `fx-prompt-cleaned.md` (counts only) |
| Tool schema | 9 tools, 2,221 chars JSON (about 550-750 tokens) | not known; tool names seen: shell, glob_files, grep_files, read_file, edit_file | temporary test on `coding_tools(ws, true)`; fx `stdout.txt` |
| Task prompt | 940-1,023 chars (TASK.md) | same | `head_to_head/*/TASK.md` |
| Fixed tokens per request (fit) | about 965 (T1v2), about 1,070 (T7), about 1,350-1,650 (T9) | measured about 6,334 cl100k (section 8; self-reported; fx --auto review requests not counted) | `token_overhead.py` |

The Harness fit uses one chars-per-token ratio per run, so that the rebuilt chars match `tokens_prompt` in `rec/metrics.json`. The ratio is 3.2-3.3 on T1v2, 3.0 on T7 and 2.0-2.4 on T9. A low ratio on T9 shows that the large CSV and test text gives more tokens per char. The fixed split is thus approximate.

## 3. Split per run: fixed x requests vs growth

Harness (`python3 native/benchmarks/tools/token_overhead.py RUN/rec/run.jsonl RUN/rec/metrics.json FIXED_CHARS`):

| Run | Requests | Input tokens | Fixed | Growth | Fixed share | Test-runner output kept in context | Share |
|---|---|---|---|---|---|---|---|
| T1v2 P-H2 | 14 | 28,833 | 13,508 | 15,325 | 46.8% | 4,422 | 15.3% |
| T7 postfix H1 | 24 | 79,208 | 26,458 | 52,750 | 33.4% | 8,052 | 10.2% |
| T7 postfix H2 | 24 | 71,352 | 25,734 | 45,618 | 36.1% | 7,707 | 10.8% |
| T7 postfix H3 | 24 | 70,893 | 25,703 | 45,190 | 36.3% | 7,813 | 11.0% |
| T9 postfix H1 | 17 | 268,387 | 22,937 | 245,450 | 8.5% | 75,590 | 28.2% |
| T9 postfix H2 | 20 | 454,306 | 29,010 | 425,296 | 6.4% | 158,142 | 34.8% |
| T9 postfix H3 | 21 | 349,559 | 32,159 | 317,400 | 9.2% | 77,868 | 22.3% |
| T9 postfix15 H1 | 24 | 803,716 | 39,763 | 763,953 | 4.9% | 138,993 | 17.3% |
| T9 postfix15 H2 | 22 | 624,817 | 34,228 | 590,589 | 5.5% | 168,644 | 27.0% |

"Test-runner output" = results of `run_tests`, and of `run_shell` with `cargo test`, counted once for each later request.
T9 postfix15 H3 is ABORTED (no `rec/`) and is not in the table.

### 3.1 fx

fx token numbers: self-reported; fx --auto review requests not counted.

fx gives only total input tokens. We cannot split fx tokens from data. Totals:

| Run | Requests (steps) | Input tokens | Mean per request | Source |
|---|---|---|---|---|
| T1v2 P-F3 | 15 | 75,499 | 5,033 | docs/benchmarks/first-head-to-head.md |
| T7 postfix F1/F2/F3 | 14/13/14 | 132,109/122,539/133,270 | about 9,400 | `summary.json` |
| T9 postfix F1/F2/F3 | 18/20/16 | 583,414/679,544/524,144 | about 32,000 | `summary.json` |
| T9 postfix15 F1/F2/F3 | 17/17/18 | 522,772/483,488/464,132 | about 28,400 | `summary.json` |

Old estimate (superseded by section 8) for T1v2 only: if fx growth is near Harness growth (15,325 tokens), fx fixed is (75,499 - 15,325) / 15 = about 4,000 tokens per request. This is about 4 times the Harness fixed part. The fx base prompt alone is about 1,470 tokens. This estimate is NOT verified.

## 4. Does fx shorten old tool results? (not verified)

We cannot answer from data, because we have no bodies and no usage per request. Evidence that we have:
- In fx `stdout.txt`, each `cargo test` that fails on T9 has `stdout_bytes` 430,969 or 416,579 with `truncated: true` and an `output_file`. Thus fx caps command output before the model sees it.
- The fx binary has text for a cap on tool results sent to the model, a `read_tool_result` tool to open older results by ID, and compaction. This shows that fx can archive or compact old results. We cannot see if it did so in these runs.
- Harness keeps each tool result (up to 32 KiB, MAX_OBS_BYTES) for all later requests. The data agree: results of 32,806-32,807 chars stay in each later request.

## 5. Why Harness used more on T9 (714K vs 483K median, postfix15)

- Harness medians: 803,716 and 624,817 (2 valid runs, median 714,266). fx median 483,488 (3 runs; self-reported; fx --auto review requests not counted).
- At about request 10, Harness reads `tests/generated.rs`, `data/orders.csv` and (H1) `data/orders_expected.txt`. Each result is about 32.8K chars (capped). It also runs `cargo test` output of 8-16K chars. All of these stay in context for the next 12-14 requests.
- Non-test tool results (mainly these file reads) give 415K-619K tokens. Test-runner output gives 139K-169K tokens (17-27%). Fixed overhead gives only 34K-40K tokens (about 5%).
- Harness also does more requests (22-24, H1 hit MaxSteps 24) than fx (17-18).
- Thus the cause is growth from large kept results x many requests, not fixed overhead. On T9 fx has a larger fixed part but a smaller growth.

## 6. Does the same growth show on T7?

Partly. On T7, growth is 64-67% of Harness input, but the largest result is only 1,540-2,332 chars and test output is only 10-11%. Growth on T7 comes from many small results over 24 requests. The large-result growth shows only on T9. Thus test-output growth is clear on 1 task (T9), not on 2.

## 7. Could not verify

- fx system prompt as sent, fx tool schemas, fx tool count, fx first-request tokens.
- Whether fx truncates, archives or summarizes old results in later requests.
- Harness chars-per-token is a fit per run, not a measured first request.

## 8. Measured fixed overhead (first request, no model)

Label: no model call. Captured on 2026-10-12 with a fake upstream.

### Method

- A fake OpenAI-compatible server on 127.0.0.1:18999 (not the bench proxy, port 8787 not touched). It saved each request body and gave one plain `stop` answer with no tool calls and `usage` 1/1.
- Both agents ran as user `benchagent` (`sudo -n -u benchagent env -i ...`). Env had only `PATH`, `HOME` (a new temp home), the dummy key and the base URL or settings. No real key was present.
- Harness: binary built from this branch (base 74124ff). `HARNESS_BASE_URL=http://127.0.0.1:18999/v1`, `HARNESS_MODEL=fake-model`, `--allow coding,shell`.
- fx 0.0.13: `~/.fx/settings.json` in the temp home with one custom `openai-chat-completions` connection to the fake server and bearer auth from a dummy env var. `fx ask --json --no-save --auto --model fake-model`.
- fx used the fake server: the fake server saved the request, fx printed the fake answer text, and fx reported `usage` 1/1 (the fake values). With no settings, fx stopped with `MissingCredentials`, so it did not use a saved login.
- Workspaces: copies of `native/fixtures/broken_calc_v2` (T1v2) and `native/fixtures/large_repo_v1` (T7) in /tmp. Prompt = `TASK.md`.
- Each agent sent exactly 1 request per task. The fake server was stopped after the runs.
- Harness bodies are in `18-token-overhead/bodies/` (scrubbed; the auth header was removed). fx bodies are kept outside git (`/workspace/local-only/fx-prompt/`), because the fx license is not checked.

### Numbers (first request)

| Item | Harness T1v2 | Harness T7 | fx T1v2 | fx T7 |
|---|---|---|---|---|
| System messages | 0 | 0 | 7 | 7 |
| System prompt chars | 0 | 0 | 8,161 | 8,155 |
| Tool count | 9 | 9 | 14 | 14 |
| Tool schema chars (JSON) | 2,378 | 2,378 | 18,642 | 18,642 |
| User prompt chars | 931 | 1,016 | 931 | 1,016 |
| Total body chars | 3,452 | 3,523 | 28,257 | 28,322 |
| Total, chars/4 | 863 | 881 | 7,064 | 7,081 |
| Total, tiktoken cl100k | 920 | 932 | 6,334 | 6,340 |
| System, cl100k | 0 | 0 | 1,647 | 1,641 |
| Tools, cl100k | 630 | 630 | 4,243 | 4,243 |

Token counts are approximate. cl100k is not the Gemini tokenizer.

Harness tools: apply_patch, git_diff, git_status, list_files, read_file, run_shell, run_tests, search, write_file.
fx tools: read_file, glob_files, grep_files, edit_file, write_file, shell, capability_search, skill, install_skill, mcp_select_tool, mcp_features, ask_user_question, web_fetch, read_tool_result.
fx system parts (chars): base prompt 5,900; web search note 440; project rules note 88; MCP note 467; turn context 497; permission runtime context 564; language control 205.

Check: the Harness fit in section 2 gave about 965 fixed tokens per request on T1v2. The measured value with cl100k is 920. They agree.

### T1v2 fx split with measured fixed overhead

All fx numbers here: self-reported; fx --auto review requests not counted.

Limit (Walter): Part of the 920 vs 6,330 gap is that Harness sends no system prompt.

The fx JSON `usage` counts main-agent requests only. With `--auto`, fx also sends review requests. The proxy saw 15 upstream requests in P-F3, but fx reported 10 steps. Thus main-agent requests are about 11 (steps + 1, not verified).

| Run | fx steps | Input tokens | Fixed (6,334 x (steps+1)) | Growth | Fixed share |
|---|---|---|---|---|---|
| P-F2 | 8 | 57,656 | 57,006 | 650 | about 99% |
| P-F3 | 10 | 75,499 | 69,674 | 5,825 | about 92% |
| P-F1 | 14 | 164,664 | 95,010 | 69,654 | about 58% |

Compare Harness P-H2: 28,833 input tokens, 46.8% fixed. On T1v2 the fx fixed part (about 6,300 tokens per request, about 6.9 times Harness) explains most of the gap.
This replaces the estimate "about 4,000 tokens per request" in section 3.1. That estimate divided by 15 requests, but 15 included the review requests.

### Not verified

- Exact count of fx main-agent requests per run (assumed steps + 1).
- Gemini token counts. We used cl100k and chars/4 only.
- fx `--auto` review request bodies. They did not occur in this capture, because the fake server sent no tool calls.
- Later requests (N+1). The fake server ends each run after 1 request, so we did not capture compaction or truncation of old results.
