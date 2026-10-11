# oai_stream_proxy (benchmark tooling only, not product)

A small Python stdlib proxy used only by the head-to-head bench to put fx and Harness
on the same normalised Gemini OpenAI-compatible stream. Harness crates do not use it.
fx is not patched.

    python3 proxy.py 8787          # listens on 127.0.0.1:8787 only
    python3 -m unittest test_proxy # recorded, made-up sample chunks

Upstream: `https://generativelanguage.googleapis.com/v1beta/openai` (override with `PROXY_UPSTREAM`).
Every path (`/chat/completions`, `/models`, ...) is forwarded to it.

## Security properties (gate rules)
- Binds `127.0.0.1` only (`BIND_HOST`, asserted in `make_server`, covered by `test_binds_loopback_only`).
- The `Authorization` header is passed through to upstream unchanged. It is never stored,
  logged or written anywhere. Request logging is method, path and status only, with no headers or bodies.
- Only `Content-Type`, `Accept`, and `Authorization` are forwarded upstream.

## What it rewrites
Streaming (`text/event-stream`) responses, per `data:` JSON chunk:
1. **`index` on tool_call deltas.** Gemini omits it. When missing, the proxy numbers tool calls by position within the
   choice (0, 1, ...) for the whole response. The index stays stable for the same `id`. A delta without an `id` reuses the
   previous index. An `index` that is already present is left unchanged.
2. **`extra_content` removed** at chunk, choice, delta and tool_call level.
3. **Chunk dropped** if, after step 2, every choice has an empty delta (`{}`) and no `finish_reason`. An example is
   Gemini's trailing chunk whose delta holds only `extra_content`.
4. **`usage` kept only on the last chunk.** It is removed from every chunk and only the most recent value is kept.
   That value is attached to the chunk that carries a `finish_reason`. If none arrives before `[DONE]`, it is
   emitted as a `{"choices":[],"usage":...}` chunk.
5. **`finish_reason` `"stop"` changed to `"tool_calls"`** when that choice emitted tool calls in this response. Gemini
   sends `"stop"`. Without this change fx 0.0.13 fails at 0 steps with `InconsistentFinishReason`. The proxy smoke
   test found this; it was not in the original spec. Non-stream responses get the same change.
6. `[DONE]` is passed through. Non-`data:` lines (comments, blank keep-alives) are not forwarded. Each event is
   re-terminated with a blank line.

Non-stream JSON responses get the same `extra_content` removal from `choices[].message.tool_calls[]`.
Error responses (non-200) are passed through unchanged.

## Thought signatures (request side)
Gemini places `thought_signature` in `tool_calls[].extra_content.google`. Gemini expects it to come back on later turns.
Because the proxy strips it from responses, in the default `PROXY_SIG_MODE=reinject` it keeps an
in-memory map `tool_call id -> extra_content` for the life of the process. Outgoing request messages whose
`tool_calls[]` entries lack `extra_content` get it re-added by id. Requests that already carry
`extra_content` are left as they are. `PROXY_SIG_MODE=strip` turns off re-injection. It was used only to test
whether re-injection is needed 

**Empirical result (2026-10-11):** with `strip`, the second turn returns HTTP 400 `Function call is missing a
thought_signature in functionCall parts`. With `reinject`, it returns 200. So signatures must be round-tripped, and
`reinject` is used for all bench runs for both agents.

## Pacing (`--min-interval-ms`)
    python3 proxy.py 8787 --min-interval-ms 5000 --hold-file /path/run/proxy_hold.json

Optional, default `0` (no holding; behaviour unchanged). When set, each upstream request is held until at least
that many ms have passed since the previous upstream request started (starts are serialised under a lock).
Pacing lives in the proxy so every agent behind it gets the same delay; neither agent is changed.
Counts are kept per process (use a fresh process, or `Pacer.reset()`, per run): `upstream_requests`,
`held_requests`, `hold_ms`, `status_429`. With `--hold-file` they are rewritten as JSON after every upstream
response, and printed as a `pacing {...}` line on shutdown. Counts only: no headers, keys or bodies.
Covered by the `Pacing` tests in `test_proxy.py`.
