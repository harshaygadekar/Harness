# ADR-001 security review

**Reviewer:** Jessie  
**Date:** 2026-09-19  
**Target:** `docs/decisions/ADR-001-core-loop.md`  
**Status:** Conditionally accepted

---

## Verdict

ADR-001 takes the right cuts. Tiny protocol core. No Cordis port. No shell / `python_eval` in core. Tool gate in the loop. Sequential tools in v0. Good.

Ship the spike only if the conditions below are in code and tests.

---

## Accepts (already in ADR)

1. Deny-by-default `can_use_tool` before `tool.call`.
2. No shell / `python_eval` / built-in side-effect tools in core.
3. Tool errors sanitized; no secrets in observations or `RunResult.error`.
4. v0 tool calls run one at a time.
5. Plugins must not call `execute` outside the registry pipeline.
6. No network or credential store in core.
7. Plugin `register` returns a disposer.

---

## Conditions (block merge if missing)

### C1. Default deny means deny all tools

Do not special-case “side-effect tools.” The host must allow each tool by name (or a clear allow list). Core default for an unset gate is **Deny** for every tool call.

If the host wants a pure/local tool, the host sets Allow. Core does not guess “pure.”

**Test:** register a tool, run with no `can_use_tool` override → tool never runs; observation is a deny reason.

### C2. Gate cannot be skipped

`ToolRegistry` / `Runner` must be the only path to `call`. No public `tool.call` helper that bypasses the gate for plugins.

**Test:** attempt invoke via any non-Runner path fails or is not exported.

### C3. Secret scrub is concrete

Define patterns before code freeze: `sk-`, `Bearer `, `api_key`, `API_KEY`, `password=`, AWS-like keys, home directory prefixes. Prefer error codes (`TOOL_FAILED`) plus a short safe string.

**Test:** raise `Exception("key=sk-test-SECRET")` from a tool → observation and `RunResult.error` contain no `sk-test-SECRET`.

### C4. Hook / plugin trust

v0 plugins run in-process. Treat them as **full trust** (same process as the host). Document that. Do not claim isolation.

Prompt-injection note: `on_step` / message-mutating hooks can rewrite history. Only load plugins the host chose. No remote plugin install in v0.

### C5. Cancel actually stops work

After cancel, do not start a new tool call. In-flight call should check cancel where possible; document if it cannot.

**Test:** cancel mid-run → status `cancelled`; no further tool side effects after the flag.

---

## Attacks still open (track, do not block ADR text)

| Attack | Why open | Follow-up |
|--------|----------|-----------|
| Malicious in-process plugin | Same process = RCE by design | Sandbox / subprocess runner = later plugin |
| Model returns huge parallel tool lists | Sequential still burns tokens/time | `max_tool_calls_per_step` later |
| Denial observation trains the model to retry forever | Loop until `max_steps` | Host policy / backoff plugin |
| Logging adapters leak prompts | Model plugin is host-supplied | SECURITY.md: never log raw prompts by default |

---

## DeepSeek Harness notes (inspiration only)

Take: pre-execute waterfall, monotonic deny, loop seams, no product tools in core.

Reject for security copy-paste: claiming Cordis-style isolation without a process boundary; HMR unload races; shipping shell in a “Minimal” profile as if it were safe defaults.

Our Minimal demo = user function tool + model adapter. Not shell + editor.

---

## Sign-off

- ADR-001 design: **Accept** with C1–C5 required in the spike.
- Code merge: **Hold** until Mike’s CI covers gate, scrub, and no shell in core (ADR checklist).

Jessie
