# Phase 4: cost estimate for a stronger-model paired run

**Status:** Estimate. Prices checked on 2026-10-11. No run on these models has happened yet.

## 1. Experiment shape

- 3 tasks (T1v2, T7, T9) × 2 agents (Harness, fx) × n = 5 runs = 30 runs per model.
- One model per experiment. Same model for both agents.

## 2. Token data used

Source: raw run folders in `native/benchmarks/head_to_head/runs/` (paced runs, `gemini-3.5-flash-lite`, 5 s pacing).

- Harness: `rec/metrics.json` (`tokens_prompt`, `tokens_completion`).
- fx: `usage.input_tokens`, `usage.output_tokens` in `stdout.txt`.
- Requests: `upstream_requests` in `proxy_hold.json` (proxy count).

| Task | Agent | n | Input median | Input max | Output median | Output max | Requests median | Requests max |
|------|-------|---|--------------|-----------|---------------|------------|-----------------|--------------|
| T1v2 | Harness | 3 | 28,833 | 36,012 | 839 | 855 | 14 | 16 |
| T1v2 | fx | 3 | 75,499 | 164,664 | 747 | 5,439 | 15 | 19 |
| T7 | Harness | 3 | 80,016 | 91,514 | 554 | 723 | 24 | 24 |
| T7 | fx | 3 | 159,406 | 202,485 | 658 | 740 | 22 | 23 |
| T9 | Harness | 1 | 241,118 | 241,118 | 450 | 450 | 32 | 32 |
| T9 | fx | 1 | 238,307 | 238,307 | 452 | 452 | 16 | 16 |

Notes on the data:

- T1v2: 3 paced runs per agent. All 6 runs were green.
- T7: 3 runs per agent. All 3 Harness runs stopped at the step limit (`MaxSteps`, 24 steps). Only 1 of them was green. So Harness T7 tokens are capped by the step limit. A run without that limit can use more tokens.
- T9: only 1 complete run per agent. fx run F2 timed out (`agent_exit` 124) and has no usage data. fx run `paced15-F1` has no exit file. The Harness T9 run failed, and 10 of its 32 requests got HTTP 429. So T9 numbers rest on n = 1 and are weak.
- Requests are not used in the price formula. Providers bill per token. Requests are shown for rate-limit planning.

## 3. Method

- Low estimate = median input and output tokens per (task, agent) × 5 runs, summed over 6 cells.
- High estimate = max input and output tokens per (task, agent) × 1.5 (safety) × 5 runs, summed over 6 cells.
- Token totals: low = 4,115,895 input and 18,500 output. High = 7,305,750 input and 64,943 output.
- Price = standard (not batch) list price, no cache discount. Prompt caching can lower the real cost. We did not model it.
- Token counts come from the Gemini tokenizer. Other models count tokens differently. Anthropic says its newer tokenizer gives about 30% more tokens for the same text. That is not in these numbers.
- A stronger model can take a different number of turns. That changes tokens. This estimate assumes the same token use as Flash-Lite.

## 4. Prices (USD per 1M tokens, standard tier)

| Model | Input | Output | Source |
|-------|-------|--------|--------|
| Gemini 3.1 Pro Preview (`gemini-3.1-pro-preview`) | 2.00 (prompts ≤ 200k) | 12.00 (prompts ≤ 200k) | https://ai.google.dev/gemini-api/docs/pricing (page "Last updated 2026-10-09 UTC") |
| Claude Sonnet 5.5 | 2.00 | 10.00 | https://platform.claude.com/docs/en/about-claude/pricing |
| OpenAI `gpt-6.1-sol` | 2.00 (≤ 272K) | 10.00 (≤ 272K) | https://developers.openai.com/api/docs/pricing |
| DeepSeek V4 Pro (`deepseek-v4-pro`, open weight) | 1.32 (peak, cache miss) | 3.96 (peak) | https://api-docs.deepseek.com/quick_start/pricing |

Notes:

- Gemini 3.1 Pro: above 200k tokens in one prompt, the price is 4.00 / 18.00. The token totals above are summed over many requests. We do not have per-request prompt sizes. On T9, one request may go above 200k if a large test output stays in the history. Then the Gemini cost is higher than shown.
- DeepSeek off-peak prices are half of peak (0.66 / 1.98). We use peak for a safe estimate.
- Claude and OpenAI models are reachable through OpenAI-compatible routes (OpenAI direct; Claude via its OpenAI SDK compatibility or a router such as OpenRouter). Router prices can differ. We did not check router prices.

## 5. FX rate

1 USD = 96.6149 INR. Source: RBI reference rate (FBIL), as at 1:00 pm, 2026-10-09, shown on https://www.rbi.org.in/ . Market mid-rate on 2026-10-11 was close (about 96.7 to 96.8).

## 6. Cost estimate (30 runs per model)

| Model | Low USD | Low INR | High USD | High INR |
|-------|---------|---------|----------|----------|
| Gemini 3.1 Pro Preview | 8.45 | 817 | 15.39 | 1,487 |
| Claude Sonnet 5.5 | 8.42 | 813 | 15.26 | 1,474 |
| GPT-6.1 Sol | 8.42 | 813 | 15.26 | 1,474 |
| DeepSeek V4 Pro (peak) | 5.51 | 532 | 9.90 | 957 |
| **All 4 models** | **30.80** | **2,975** | **55.81** | **5,392** |

Split by task (USD, low / high):

| Task | Gemini 3.1 Pro | Claude Sonnet 5.5 | GPT-6.1 Sol | DeepSeek V4 Pro |
|------|----------------|-------------------|-------------|-----------------|
| T1v2 | 1.14 / 3.58 | 1.12 / 3.48 | 1.12 / 3.48 | 0.72 / 2.17 |
| T7 | 2.47 / 4.54 | 2.45 / 4.52 | 2.45 / 4.52 | 1.60 / 2.95 |
| T9 | 4.85 / 7.27 | 4.84 / 7.26 | 4.84 / 7.26 | 3.18 / 4.77 |

## 7. Which numbers are strong

- T1v2 rows rest on 3 green runs per agent. They are the most reliable.
- T7 rows rest on 3 runs per agent. Harness T7 is capped by the 24-step limit. Real use can be higher.
- T9 rows rest on 1 run per agent. Treat them as unknown range. T9 is the largest share of cost.
- Input tokens are more than 99% of the cost on every model. Output price matters little here.
- Retries after HTTP 429 or timeouts add cost. They are not in this estimate.
