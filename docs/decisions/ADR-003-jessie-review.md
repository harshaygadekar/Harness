# ADR-003 review — Jessie (security)

**Verdict:** Approve. Freeze holds.

## Agree

- Core freeze until Thomas files a failing example.
- `simple.Agent` stays a façade — not a second gate/loop.
- Deny-all / allowlist stays on `RunConfig.can_use_tool`.
- No shell / MCP / sandbox promoted into core via “DX.”

## Freeze invariants (must not regress)

1. Default gate remains deny-all when the host sets no policy.
2. `Registry` must not grow an `execute` / `run_tool` bypass.
3. Secret scrub stays on tool errors and model-adapter errors.
4. Façade allowlist must not silently Allow tools the host did not pass in.

## Nit

Protocol docs say `FunctionTool` / `deny_all` in places; code uses `FunctionTool` vs `FunctionTool` — align names with `src/` (same as Mike’s naming nit). Security does not care which spelling, only that docs match the gate entrypoint hosts actually call.

Jessie — 2026-09-19
