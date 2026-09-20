# Useful scope — coding-agent product

**Owner:** Alice  
**Date:** 2026-09-20  
**Status:** Locked. ADR-004 Accepted (Candidate A). Core freeze still holds.

---

## Product

Ship a **coding agent people can install and run**, built **on** the harness.

The harness core stays thin. The product is plugins + CLI + examples + docs.

We do **not** claim to beat fx.sh unless measured numbers say so after ADR-004.

---

## What we sell in this phase

| In | Out (this phase) |
|----|------------------|
| CLI that runs a coding agent on a workspace | Full IDE / LSP product |
| Plugins: fs, search, patch, git status, opt-in shell | Shell/MCP/sandbox inside core |
| Deny-all + path bounds + scrub (Jessie) | “Safer than fx” marketing |
| Fake-model benches + honest size/start numbers | Fake µs claims copied from marketing |
| Optional Gemini (or other) as model plugin | Provider SDK in core |

---

## Architecture rule (product)

1. Core freeze until ADR-004 Accepted.
2. New power = plugin first. Core only if Thomas shows a failing example that cannot be a plugin.
3. Native rewrite (Rust/Zig) or single-binary pack = **packaging track**, not silent core growth.
4. DeepSeek Harness = design inspiration only. Not an API for ADRs.

---

## Success for “ready”

1. Clone repo → install → run coding-agent example on a sample workspace.
2. Dangerous tools need explicit Allow.
3. Benches published; no beat-fx claim without green MUST and an Accepted product ADR.
4. Secrets never in git or chat.

---

## Kill / reopen

- If the brief forces shell/MCP into core → Alice veto.
- If we market “beat fx 10 µs / 6 MB Zig” without evidence → Alice veto.
- fx parity as a native product = new ADR after Thomas’s `11-coding-agent-product.md`.
