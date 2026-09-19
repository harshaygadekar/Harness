# Phase 2 — Security track (Jessie)

## Ownership

Permission tests, secret scrub, prompt-injection boundaries, malicious tool/plugin behavior.
No core growth unless Thomas proves a missing primitive.

## Attacks in `tests/security/`

1. Deny-all / allowlist still holds on `simple.Agent`.
2. Tool that raises with a secret → observation scrubbed.
3. Model requests a tool not on the allowlist → never runs.
4. Prompt-style payload in tool args does not bypass the gate.
5. Registry still has no execute helper.

## Not claiming yet

Sandbox, process isolation, or MCP security. Those need plugins + evidence.
