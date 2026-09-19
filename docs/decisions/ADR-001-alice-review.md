# ADR-001 — Alice product review

**Date:** 2026-09-19 (IST)  
**Verdict:** Approve Candidate A (Tiny protocol core).

## Checks against `docs/useful-scope.md`

| Lock | ADR-001 | Pass |
|------|---------|------|
| First demo = one model adapter + one user tool + loop | Explicit “Alice first demo” | Yes |
| No shell / `python_eval` / MCP / crew in core | Rejected + boundary table | Yes |
| Deny-by-default tool gate | C1 deny-all until host Allow | Yes |
| DeepSeek = harness inspiration, not API | Constraints + reject list | Yes |
| No graph DSL in core | Candidate C rejected; Alice veto noted | Yes |
| Plugins without forking core | `Plugin.register` + disposer | Yes |

## Nits (non-blocking)

1. Keep **Status = Proposed** until Jessie C1–C5 tests land (already stated).
2. README must lead with the first demo and **non-goals** from useful-scope. Do not list plugin packs as core features.
3. Host Allow list is part of the first demo story — document one line: “allow this tool by name or the loop never calls it.”

## Asks (in Tech Team only)

@Walter fold this review into the Review trail.  
@Mike spike after Proposed → Accepted with C1–C5. I will write README drafts after the spike matches the demo lock.

No scope growth. If the spike adds a second built-in tool or a finish tool, Alice veto.
