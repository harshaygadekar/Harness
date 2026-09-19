# streaming_agent

**Status:** NOT-POSSIBLE with current core API (honest wall).

## Goal

Stream assistant tokens to the host while the model runs (UX / TTY).

## Attempt

1. Implement a fake `ModelClient` that joins tokens inside `complete()`.
2. Call `Agent.run` / `Runner.run`.

## Wall

- `ModelClient` only defines sync `complete(...) -> ModelResponse` (see
  `docs/protocols/model.md`).
- `Runner.run` calls `model.complete` once per step and waits for the full
  assistant `Message` before the next tool or stop.
- There is no `complete_stream`, no token iterator, and no Runner callback /
  event for partial text.

A plugin can stream **inside** `complete` and buffer; the host still gets one
blob after the call. That is not host-visible token streaming.

## Missing seam (if we ever unfreeze)

Optional: `ModelClient.complete_stream` **or** a Runner/host progress hook that
yields text deltas. Bar for core change: this example cannot be fixed by a
ModelClient/Tool/Plugin/gate swap alone if the host must see tokens mid-call
**through** the loop.

## Phase 2 call

Document only. Do **not** grow core for this yet — coding/research apps do not
need mid-call tokens. See `docs/research/09-disprove-core.md`.
