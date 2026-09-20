# ADR-005: Native coding-agent language

| Field | Value |
|-------|-------|
| **Status** | **Accepted** |
| **Date** | 2026-09-20 (IST) |
| **Owners** | Walter (draft), Thomas (evidence `12`), Alice (native scope), Mike (impl after Accept), Jessie (security) |
| **Related** | [ADR-001](ADR-001-core-loop.md), [ADR-002](ADR-002-model-adapter.md), [ADR-003](ADR-003-protocol-freeze.md), [ADR-004](ADR-004-coding-agent-product.md), [docs/research/12-native-language.md](../research/12-native-language.md) (Thomas evidence), [docs/useful-scope-native.md](../useful-scope-native.md) (Alice native scope). |

---

## Problem

ADR-004 Accepted a **Python library + plugin coding agent**. That product does not compete with fx on process start or binary MiB — and must not pretend to.

Harsha / product now wants a **separate native coding-agent product**: a tiny AI coding CLI that can fight fx on fair axes (size, start, tool-efficiency) **after** we measure. Language choice locks delivery speed, safety story, plugin/WASM path, and whether we even have room under fx-class MiB ceilings.

Without this ADR we risk: (a) growing the frozen Python core to chase fx, (b) picking Go/C without size/safety evidence, or (c) claiming beat-fx before Mike has native benches.

Python harness stays. This is not a rewrite of ADR-001–003 core.

---

## Constraints

1. **Separate product.** Native track does not destroy or replace `src/agentic_harness/`. Propose an additive `native/` tree only.
2. **Python core freeze holds (ADR-001–003).** No chasing fx by growing Python core. Growing Python core to chase fx = Alice veto (`useful-scope-native.md`).
3. **ADR-004 escape hatch.** Native rewrite / sidecar was Candidate B out-of-scope there; this ADR is that track.
4. **Evidence gate.** Thomas `docs/research/12-native-language.md` is the bake-off. Cite **measured** vs **cited**; say unknown when unknown.
5. **Alice native scope.** Ship tiny native coding CLI; keep deny-all / scrub / workspace / plugins; no beat-fx marketing without measured axes (`docs/useful-scope-native.md`).
6. **Jessie security.** Port **policy** (deny-all, capability classes, workspace, scrub, approval for dangerous) — not Python code. OS sandbox = later measured ADR.
7. **DeepSeek Harness = design seams inspiration only.** Not DeepSeek LLM API; not a Cordis/`dsh` vendor.
8. **No beat-fx claim** until Mike measures native benches after Accept (method + SHA + machine class).
9. **No native source in this ADR drop.** Layout proposal + decision only.

---

## Candidates

### Rust (recommended)

- Toolchain **present** on research host: `rustc` / `cargo` **1.85.1**.
- Measured hello ~**0.3 MiB**; ureq+rustls ~**1.4 MiB** floor; start ~**0.7–0.8 ms** median (hello / ureq agent create) — under fx **2 ms** Linux CI budget for **hello**, not an agent claim.
- Memory safety helps Jessie’s threat model; does **not** replace deny-all / workspace / scrub / approval.
- Plugins path: traits + registry; `libloading` (trusted) and/or Wasmtime WASI components (capability-limited).
- Strong HTTP/JSON/TTY/Landlock ecosystem without OpenSSL-by-default or a GC runtime.

### Zig (strong alternative)

- Same language as fx → fairest language-level size/start comparison; best **cited** tiny HTTPS floors.
- **Not installed** on research host → hello size/start **unknown here**.
- Revisit if product locks “same language as fx” (Zig 0.16+ pinned in CI; re-run `12` §3 smokes). Language/tooling churn and thinner plugin/sandbox library surface vs Rust.

### Go

- Toolchain present; easy static builds and std HTTP/JSON.
- Measured TLS/HTTP **blank-import floors multi-MiB** (`crypto/tls` ~2.3 MiB; `net/http` ~3.1 MiB) before agent logic — poor fit for fx-class MiB goals.
- GC; weak third-party plugin ABI; WASM size pain documented upstream.

### C

- Smallest measured hello; fastest hello start.
- **No memory safety** for an agent surface with plugins/tools — poor match for Jessie’s deny-all + plugin host story; reinvent HTTP/TLS/JSON/plugin ABI.

---

## Selected recommendation

**Rust** for the separate native coding-agent product.

**Accepted** — Rust. Zig remains the measured revisit if product locks fx-stack language parity.

---

## Why (Rust)

| Reason | Detail (from `12`) |
|--------|---------------------|
| Toolchain + measured smoke | rustc 1.85.1 on host; hello ~0.3 MiB; ureq+rustls ~1.4 MiB; start ~0.7–0.8 ms median under fx 2 ms CI budget for hello (**not** agent claim). |
| Memory safety | Helps Jessie model for long-running agent + plugins; still requires deny-all / workspace / scrub / approval. |
| Plugins / WASM | Traits + registry; Wasmtime path for untrusted plugins; dylib for trusted high-perf — matches DeepSeek-style seams without vendoring. |
| Size path vs Go | Better than Go TLS/HTTP blank-import multi-MiB floors for staying near fx MiB ceilings **if** core stays lean (still must measure a real agent binary). |
| Vs C | Safety and delivery for agent/plugin surface beat C’s hello-byte wins. |
| Escape hatch | Zig remains explicit revisit if product requires same-language-as-fx. |

---

## Rejected (this cut)

| Candidate | Why rejected for this cut |
|-----------|---------------------------|
| **Go** | TLS/HTTP blank-import floors multi-MiB before agent features; GC; weak plugin story. |
| **C** | No memory safety for this product’s plugin/tool surface; reinvent stacks. |
| **Growing Python core to chase fx** | Violates ADR-001–003 freeze and Alice native / Phase 2 must-nots. |
| **Zig as default** | Strong alternative, not default: no local smoke; revisit if product locks “same language as fx.” |

---

## Native core vs plugins

Align with ADR-003 freeze spirit, `04-keep-out-of-core.md`, and Alice `useful-scope-native.md`. DeepSeek inspires **seams**, not an LLM API or Cordis port.

| In native core | Out of core (plugins) |
|----------------|------------------------|
| Model protocol (request/response + stream events; **no** provider SDK) | Provider SDKs (OpenAI, Anthropic, Gemini, …) |
| Messages / events (canonical message list v0) | Shell |
| Agent loop (model → tools → append → stop/continue) | Filesystem tools |
| Tool protocol (register / call / result) | Git |
| Capability registry | MCP runtime / transports |
| Permission gate (deny-all default; allow-by-name) | Browser |
| Lifecycle (register + teardown) | Memory DB / RAG |
| Cancel (cooperative) | Telemetry backends |
| Streaming consumer hooks | Heavy TUI / fancy terminal product |
| Minimal loop state | Sandbox (Docker, microVM, bubblewrap) — later measured ADR |

Min product tool pack (list/search/read/write-patch/git status-diff/opt-in shell/tests) ships as **plugins**, not as the smallest runtime.

---

## Layout (propose only — no code in this ADR)

Additive `native/` tree. Do **not** delete or move `src/agentic_harness/`.

```text
native/
  README.md                 # build, targets, non-goals vs Python
  agent/                    # binary crate: CLI entry, flags, early-exit bench hooks
  cmd/                      # thin command modules (ask, help, status, version)
  runtime/                  # loop, messages, gate, cancel, streaming seams
  plugins/                  # in-tree example plugins only (shell/fs/git/MCP stay here)
    README.md               # load policy: wasm | native dylib | none
  benchmarks/               # size gate, startup, fake-model dispatch
  tests/                    # unit + integration; no network by default
  Cargo.toml                # workspace root (Rust — this ADR)
  # OR build.zig            # only if a later ADR switches to Zig
```

Python remains: `src/agentic_harness/` (frozen), `plugins/`, shared `docs/`.

---

## Security (Jessie)

Gates port as **policy** into native (not a Python transliteration):

1. **Deny-all** default; host Allow each tool by name. CLI must not silently Allow all coding tools on start.
2. **Capability classes:** read / write / exec / net / git / secrets — explicit, not inferred.
3. **Workspace bounds** on fs plugins (no prefix/`startswith` bugs).
4. **Secret scrub** on tool errors / adapter errors / logs / crash dumps.
5. **Approval** required for dangerous actions (exec / net / secrets-class).
6. **In-process / dylib plugins = full trust** until WASM or process isolation lands — document that.
7. **OS sandbox** (Landlock/seccomp/container) = **later measured ADR**, not claimed here.
8. Memory safety from Rust **helps** but **does not replace** the gate stack above.

---


## Conditions (Jessie — bind through first ship)

1. Deny-all default. CLI must not silently Allow the full coding pack on start.
2. Capability classes explicit: read / write / exec / net / git / secrets.
3. Workspace path resolve with canonical bounds (no prefix bugs).
4. Secret scrub on tool errors, model errors, logs, crash dumps.
5. Approval seam for exec / net / secrets-class before they run.
6. No “sandboxed like fx” marketing under this ADR.
7. In-process / dylib plugins = full trust until WASM or process isolation ADR. Document in CLI help.
8. CI: fail if CLI starts with zero Allow flags and a tool still runs.

## Open (after Accept)

- [x] Thomas: evidence approve (`12` + review).
- [x] Jessie: security approve + conditions (above).
- [x] Mike: impl readiness approve — clear to scaffold `native/` after this Accept.
- [ ] Alice: confirm native scope Locked with this ADR (scope doc already Proposed → update to Locked).
- [ ] Zig revisit trigger: if Alice/Thomas lock “same language as fx,” open ADR-005b (Zig 0.16+ in CI + repeat `12` smokes).
- [ ] Mike: first native benches on `main` before any beat-fx talk.

## Review checklist

| Reviewer | Status | Notes |
|----------|--------|-------|
| Alice (native scope) | pending confirm | `useful-scope-native.md` |
| Thomas (evidence / `12`) | approve | bake-off + review |
| Jessie (security) | approve | conditions bind through first ship |
| Mike (impl readiness) | approve | build after Accept |
| Walter (draft) | Accepted | Rust selected |


## Implementation plan (Mike — **after** Accept only)

1. Scaffold `native/` Cargo workspace (empty crates / hello CLI only). **No** Python harness edits for size/start.
2. Size + startup CI gates with explicit budgets (propose from `12`: stripped linux-x86_64 stretch toward fx **7.8 MiB** ceiling shape; help-class start ≤ **2 ms** mean on Linux CI — match budget **shape**, not fx’s binary).
3. Fake-model tool loop bench (fair tool-efficiency axis vs fx).
4. Port Jessie policy: deny-all gate, capability registry hooks, workspace guard, scrub, approval seam for dangerous tools.
5. Plugin load policy doc: WASM (untrusted default recommendation) vs native dylib (trusted); follow-up ADR if needed.
6. **Do not** claim beat-fx until benches publish method + SHA + machine class.
7. No provider SDKs / shell / fs / git / MCP / sandbox / heavy TUI in runtime core.

---

## Consequences

- Native coding-agent track is **Rust-first**, additive under `native/`.
- Python library product (ADR-004) unchanged; no beat-fx claims on Python axes.
- Zig remains the strong alternative if product later requires fx stack parity.
- Marketing: hello/ureq numbers are **floors**, not agent SLAs; fair axes only after Mike measures.
- Mike is clear to scaffold `native/` under Jessie conditions. No beat-fx claim until benches.

## Review trail

- [Thomas](ADR-005-thomas-review.md) — Approve Rust; Zig revisit if fx-stack parity
- [Mike](ADR-005-mike-review.md) — Approve Rust; build after Accept
- [Jessie](ADR-005-jessie-review.md) — Approve Rust; conditions bind through first ship

- [Alice](ADR-005-alice-review.md) — Approve Rust; native scope Locked
