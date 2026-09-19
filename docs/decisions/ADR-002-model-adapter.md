# ADR-002: Model adapter plugins

| Field | Value |
|-------|-------|
| **Status** | Proposed |
| **Date** | 2026-09-19 (IST) |
| **Owners** | Walter (draft), Mike (implement), Jessie (security), Thomas (review) |
| **Related** | [ADR-001](ADR-001-core-loop.md) (Accepted — tiny core, `ModelClient` protocol), `SECURITY.md`, Jessie scrub (`AIza` / `AQ.`), Gemini plugin spike |

---

## Problem

Hosts need real provider adapters (Gemini first) without putting provider SDKs in the tiny core.

ADR-001 accepted a provider-agnostic `ModelClient` protocol and denied baking product tools or heavy deps into core. The Gemini spike already exists as a **plugin** (not core): API key only in gitignored `.env`; push blocked on PAT so work continues local. We need a clear decision so:

1. Core stays free of `google-genai` / other provider SDKs.
2. Retry/backoff for provider-specific failures (e.g. Gemini 503) lives in the right place.
3. Secrets never appear in logs or error surfaces (Jessie patterns).
4. Tests/CI keep a fake `ModelClient` by default.

---

## Decision

**`ModelClient` is a protocol in core. Each provider is a plugin package/module that implements it.**

| Layer | Owns |
|-------|------|
| Core | `ModelClient` protocol (`complete(messages, tools) → ModelResponse`), `ModelResponse`, fake client for tests |
| Provider plugin | SDK import, auth from env, request/response mapping, provider retries/backoff, error sanitization before re-raise/return |

Core must **not** import `google-genai` or any other provider SDK. Optional extras may list provider deps; cold import of core remains stdlib-only.

---

## Gemini plugin rules

Applies to `agentic_harness.plugins.gemini` (or successor package under the gemini extra).

1. **Credentials** — Read `GEMINI_API_KEY` or `GOOGLE_API_KEY` from the environment only. Never commit keys. Never print or log the key (including debug).
2. **Default model** — `gemini-flash-latest` unless the host overrides.
3. **Retries / backoff** — Transient Gemini **503** / UNAVAILABLE / high-demand retries and backoff live **only in the plugin** (Mike). Core does not retry provider HTTP errors.
4. **Error sanitization** — Before surfacing failures to the runner/host, scrub secrets per Jessie patterns (including Google `AIza…` and `AQ.…` keys, plus existing `sk-` / Bearer / `api_key` / home-path rules in `sanitize.py`). Prefer short safe text; never echo raw SDK payloads that may embed credentials.
5. **No finish tool, no shell** — Adapters implement `ModelClient` only. They do not register a `finish` tool, shell, `python_eval`, or other side-effect tools.
6. **SECURITY.md** — No `Tool.call` outside `Runner`; adapters are not a bypass path for tools.

---

## Core and test defaults

- **Fake `ModelClient`** remains the default for unit tests and CI. No network, no provider SDK required for the green path (ADR-001 spike).
- Hosts opt into a live provider by installing the extra and registering the plugin / constructing the adapter explicitly.
- Core registry may hold at most one active model client; swapping is a host/plugin concern.

---

## Alternatives rejected

| Alternative | Why rejected |
|-------------|--------------|
| **Bake Gemini into core** | Pulls `google-genai` into the default install; violates ADR-001 minimal core and “no provider SDK in core” benchmark. |
| **LiteLLM (or similar) in core** | Large transitive surface, another API to secure and version; providers stay out of core. A LiteLLM *plugin* could be a later ADR if evidence warrants. |

---

## Consequences

- New providers follow the same shape: implement `ModelClient`, optional `Plugin.register`, env-only secrets, sanitize errors, no product tools in the adapter.
- Provider-specific reliability (retries, rate limits) is documented and tested in the plugin, not in `Runner`.
- Jessie reviews scrub coverage when new key formats appear; Mike owns plugin retry behavior.
- Streaming is **not** required for acceptance of this ADR (see Open).

---

## Open questions

1. **Streaming** — Later, as an **optional** method on the `ModelClient` protocol (e.g. `complete_stream`), so non-streaming clients stay valid. Not in scope for the first Gemini plugin cut.
2. Sync vs async `complete` — follow ADR-001 open question (async-first + sync helper).
3. Multi-provider registration / routing — out of scope; one client per run for now.

---

## Review checklist (empty until reviews land)

- [x] Thomas — architecture / protocol boundary ([review](ADR-002-thomas-review.md))
- [x] Mike — Gemini retries only in plugin; fake client default in CI ([review](ADR-002-mike-review.md))
- [ ] Jessie — no key logging; scrub `AIza` / `AQ.` on adapter errors; no `Tool.call` outside Runner
- [ ] Status → Accepted after reviews
