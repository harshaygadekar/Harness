# Beat-fx goals (honest, measurable)

**Audience:** Harsha, Thomas, Mike  
**Date:** 2026-09-19 (IST)  
**Status:** Research goals. **Do not claim we beat fx.** Python library ≠ Zig native coding CLI.  
**Rule:** Numbers below are from primary sources or our local measurement. Unknown stays unknown. No invented figures.

---

## Product honesty (read first)

| | **fx (Vercel Labs)** | **Our harness** |
|--|----------------------|-----------------|
| What it is | Native **coding-agent CLI + embeddable runtime** (Zig; also WASM / libfx) | Minimal **Python library**: loop + tool protocol + deny-all gate + plugins |
| Ship artifact | One binary (install script / GitHub release) | Package import (`agentic_harness`); needs a Python interpreter |
| Built-in coding tools | Yes (files, shell, web, skills, MCP, subagent, …) | **None in core** — host/plugins supply tools |
| Fair fight? | Only on **library-like** axes (import cost, dispatch, LOC, deps, plugin load) | Not on **native binary MiB**, **µs process start**, or **full coding CLI** feature parity |

fx optimizes for “tiny native coding agent in CI/sandbox.” We optimize for “tiny Python core others embed.” Beating fx on binary size or process cold-start is a **different product** (Zig rewrite), not a Phase 2 Python-core goal.

---

## 1. What fx publishes (table + sources)

Fetched / measured **2026-09-19 (IST)**. Tag used for release assets: **v0.0.10**.

| Claim / metric | Value | Source | Notes |
|----------------|-------|--------|-------|
| Homepage binary size | **6.17 MiB** | https://fx.sh/ (fetch 2026-09-19) | Marketing line: “6.17 MiB binary.” Platform not named on the homepage sentence. |
| Production size ceiling | **7.800 MiB** | https://github.com/vercel-labs/fx/blob/main/AGENTS.md (Binary Size Observability) | macOS arm64 PGSO release qualification is authoritative. Preferred headroom **0.250 MiB** in `scripts/pgso` size gate (ceiling **7.800** MiB). |
| PR size delta warn | **≥ 52,429 bytes (0.050000 MiB)** | Same AGENTS.md | Informational on PR matrix; does **not** fail the PR. |
| Linux CI startup budget | **2 ms** mean, **every** measured command | AGENTS.md Benchmarks; `benchmarks/check_budgets.py` `LINUX_BUDGETS` all `0.002` | Commands: `fx (startup)`, `fx help`, `fx status --json`, `fx background --json`, `fx doctor --json`, `fx sessions --json`. Non-Linux runs are informational. |
| Bench mode | `FX_BENCH=1` | AGENTS.md | Arg parse + CLI dispatch, then exit **before** TTY init. Baseline cold-start path: `fx help`. |
| Heavy-work regression gate | **≤ 10%** slower p50/p95 vs control | AGENTS.md / PGSO docs | File indexing, TUI, etc. |
| Release archive sizes (compressed) | linux-aarch64 **5,050,451** B; linux-x86_64 **5,136,058** B; macos-aarch64 **3,953,656** B; macos-x86_64 **5,588,055** B | GitHub release API `v0.0.10` | Compressed `.tar.gz` only in the API. |
| **Measured** linux-x86_64 binary (uncompressed) | **11,821,040** bytes ≈ **11.27 MiB** | Local unpack of `fx-linux-x86_64.tar.gz` from `v0.0.10` (sha256 `45bf4d88…e6bd97af`) | **Does not match** homepage 6.17 MiB. Report both. Do not pretend they are the same number. |
| Built-in tool surface (docs) | **17 named kinds** in the tools table (plus dynamic MCP server tools) | https://fx.sh/docs/capabilities/tools.md | `glob_files`, `grep_files`, `read_file`, `write_file`, `edit_file`, `shell`, `web_search`, `web_fetch`, `vision`, `skill`, `install_skill`, `subagent`, `capability_search`, `mcp_select_tool`, `mcp_features`, `ask_user_question`, `read_tool_result`. |
| v0.0.10 relative speed claims | Turns up to **1.6×** faster; model requests up to **2.5×** faster | GitHub release body `v0.0.10` | Relative to prior fx; **no absolute ms** published. |
| Language / license | Zig 0.16+; Apache-2.0 | README / AGENTS.md | No Node runtime for the main binary. |
| Embed surfaces | `fx acp`, `createFxAgent()` / `fx-core.wasm`, `createFxTerminal()` / `fx-term.wasm`, npm `libfx` | README; https://fx.sh/docs/lib.md | Full coding CLI ≠ our Python package. |

### Secondary (not first-party SLA — do not use as pass/fail)

| Claim | Value | Source | Use |
|-------|-------|--------|-----|
| “10 µs cold start” | Marketing / blog | Third-party posts (e.g. Laura Martel blog) | **Not** in AGENTS.md budgets. Engineering budget is **2 ms** Linux CI. |
| Independent `fx --help` timing | **2.7 ms** (one review) | https://devaireviews.com/blog/vercel-fx-coding-agent-review/ | One machine; not our CI. |
| Independent binary on disk | **6,486,176** B ≈ **6.19 MiB** (reviewer’s download) | Same review | Platform of that download not our linux-x86_64 asset. |
| Baseline memory | “single-digit MiB” (blog) | Secondary blogs | **No first-party memory SLA** found in AGENTS.md / docs fetch. Treat as **unknown** for goals. |
| Box-local smoke (this research host) | `FX_BENCH=1 fx help` median ≈ **0.26 ms** (n=20) | Local run 2026-09-19 IST | Informational only; not fx CI hardware. |

### What we did **not** find as first-party measurable SLAs

- Absolute RSS / heap memory budget for idle CLI  
- Absolute “tool dispatch µs” number for the Zig tool loop  
- Absolute token-budget for system prompt (docs say “concise”; no byte SLA)  
- Published head-to-head vs Claude Code / omp with shared harness

---

## 2. Fair comparison axes (where we CAN compete)

Compete as a **minimal embeddable agent loop library**, not as a coding CLI.

| Axis | Why fair | How we measure |
|------|----------|----------------|
| **Core import / cold load** | Both care about “pay before first useful call.” fx uses process+CLI; we use `import agentic_harness`. | Subprocess cold import median ms (`benchmarks/cold_import.py`) |
| **Tool dispatch with fake model** | Loop + gate + tool call overhead without network/LLM noise | Fake `ModelClient`; N steps; median µs/step and total ms |
| **Core LOC** | Size of the maintained core surface | `wc -l` on `src/agentic_harness/**/*.py` excluding `plugins/` |
| **Runtime dependency count** | fx avoids extra package managers for the binary; we claim empty core deps | `pyproject.toml` `dependencies == []`; import must not pull provider SDKs |
| **Plugin register / teardown** | Both stress extension without core growth | Time to register N no-op plugins + dispose; leak checks |
| **Deny-all gate correctness** | Security default comparable in *spirit* to permission-first design | Attack-suite / unit tests (Jessie); not a latency contest with fx |
| **Reproducible fake-model agent run** | CI without API keys | Scripted tool-using loop; exit code + step count stable |

### Snapshot of **our** side (measured 2026-09-19 IST)

| Metric | Value |
|--------|-------|
| Core LOC (excl. `plugins/`) | **520** |
| All package LOC (incl. Gemini plugin) | **711** |
| Core runtime deps | **0** (`dependencies = []`) |
| Cold import median (existing bench, this box) | **24.72 ms** (5 samples; see `benchmarks/cold_import.py`) |

These are **baselines**, not “beats fx.”

---

## 3. Unfair / different-product axes

| Axis | fx reality | Our stance |
|------|------------|------------|
| Native binary MiB | Homepage **6.17 MiB**; AGENTS ceiling **7.800 MiB**; measured linux-x86_64 **11.27 MiB** | **NON-GOAL** for Python core. Optional **LONG-TERM** only if product decides a native sidecar. |
| Process cold start µs / sub-ms CLI | Linux CI **2 ms** budget; `FX_BENCH=1` path | **NON-GOAL** to match. CPython process start alone is a different floor. |
| Full coding CLI / TUI / ACP / WASM terminal | Shipped product | **NON-GOAL** for core. Coding agent = **plugins + examples** (Phase 2 scope). |
| Built-in tool count (~17 + MCP) | Product feature | **NON-GOAL** to put tools in core. Count plugins/examples separately. |
| “1.6× / 2.5× faster turns” | fx vs prior fx | **NON-GOAL** to cite as our win. Different stack. |
| Zig rewrite / no-GC binary | Their stack | **LONG-TERM product decision**, not Phase 2/3 Python work (Alice scope lock). |

---

## 4. Phase 2 / Phase 3 goals

Priority labels: **MUST** / **SHOULD** / **STRETCH**. Numeric targets are for **our** harness CI, not “fx’s number minus ε.”

### Phase 2 (coding agent **on** the harness — plugins only)

Aligns with `docs/useful-scope-phase2.md`: no “beat fx” marketing without benches.

| ID | Priority | Goal | Numeric target | Notes |
|----|----------|------|----------------|-------|
| P2-1 | MUST | Fake-model bench suite exists and runs in CI | Commands listed in §5 land; exit 0 on main | Mike owns impl |
| P2-2 | MUST | Core stays stdlib-only | `dependencies = []`; cold-import dep ban still passes | Already true — keep |
| P2-3 | MUST | Core LOC stay lean | Core (excl. plugins) **≤ 800 LOC** | Fail CI or release checklist if exceeded without ADR |
| P2-4 | MUST | Cold import budget (library) | Median cold `import agentic_harness` **≤ 50 ms** on Linux CI (CPython 3.11+, warm machine class documented) | Not comparable to fx 2 ms process budget |
| P2-5 | MUST | Fake-model tool loop | **≥ 1000** gated tool dispatches / s on Linux CI with no-op tools (or document measured rate + floor once Mike baselines) | First impl: record baseline; then lock floor at 0.9× baseline |
| P2-6 | SHOULD | Plugin load | Register + dispose **100** empty plugins median **≤ 5 ms** | Prevents registry bloat |
| P2-7 | SHOULD | Coding-agent example via plugins only | Example runs with fake model end-to-end | No shell/fs in core |
| P2-8 | STRETCH | Cold import | Median **≤ 20 ms** | Nice-to-have; do not break APIs for it |
| P2-9 | STRETCH | Documented comparison page cites only measured numbers | This file + `08-harness-comparison.md` stay current | Research Lead / Mike |

### Phase 3 (harden + optional native — still honest)

| ID | Priority | Goal | Numeric target | Notes |
|----|----------|------|----------------|-------|
| P3-1 | MUST | Keep Phase 2 budgets green on every release | Same P2-2…P2-5 | Regression = release blocker |
| P3-2 | SHOULD | Optional thin CLI **wrapper** (Python) | Startup to “ready” **≤ 200 ms** median (import + argparse + fake one-shot) | Still not fx 2 ms |
| P3-3 | SHOULD | Memory of idle Runner + empty registries | RSS delta vs bare Python **documented**; no hard SLA until measured | Measure first |
| P3-4 | STRETCH | Native/Zig or Rust **sidecar** experiment | If pursued: size ceiling **≤ 8 MiB** stripped ReleaseSafe **and** Linux help **≤ 2 ms** — i.e. meet fx’s *own* published budgets | Separate product ADR; not Python core |
| P3-5 | STRETCH | Feature pack parity *as plugins* | MCP client pack, fs/shell packs with deny-all | Compare pack LOC/deps to fx builtins — still not “we are fx” |

---

## 5. Proposed harness benchmark commands (Mike)

All use a **fake model** (deterministic tool-call script). No network. Reproducible seeds.

```bash
# From repo root, PYTHONPATH=src or editable install.

# B1 — cold import (exists; keep)
python benchmarks/cold_import.py
# Expect: cold_import_ms_median=…  deps_ok

# B2 — tool dispatch throughput (planned)
python benchmarks/tool_dispatch.py --steps 1000 --tools 8
# Expect: dispatch_per_s=…  median_us_per_step=…

# B3 — plugin register/dispose (planned)
python benchmarks/plugin_churn.py --n 100
# Expect: register_dispose_ms_median=…

# B4 — fake coding loop (planned)
python benchmarks/fake_coding_loop.py --max-steps 20
# Expect: status=completed  steps=…  tools_ok=…

# B5 — size / deps gate (planned; can be shell)
python benchmarks/size_gate.py
# Expect: core_loc≤800  deps=0
```

**CI shape (proposal):** run B1–B5 on Linux x86_64, CPython 3.11 and 3.12; publish JSON artifacts; fail on MUST budgets. Do **not** fail the build if someone runs `fx` slower/faster on the same runner — that is a different binary.

Stub index: `benchmarks/README.md`.

---

## 6. Definition of done — “beat fx on X”

Use this template. **X must be a fair axis from §2.**

| Field | Rule |
|-------|------|
| **X** | Named fair axis (e.g. “core import ms”, not “binary MiB”) |
| **Ours** | Measurement method + machine class + commit SHA + median/p95 |
| **fx** | Quote **only** a first-party number **or** say “no fx number; internal budget only” |
| **Pass** | Ours meets our published budget **and** comparison text does not imply CLI parity |
| **Fail** | Missing method, invented fx number, or claim on an unfair axis |

### Concrete DoD examples

1. **“Beat fx on core import ms”** — **INVALID as stated.** fx publishes process CLI budgets, not Python import.  
   **Valid substitute:** “Meet P2-4 cold-import ≤ 50 ms” — pass/fail on our CI only.

2. **“Beat fx on tool dispatch”** — **VALID only** if we publish our fake-model dispatch/s **and** do not claim fx’s unpublished Zig dispatch µs. Pass = P2-5 green.

3. **“Beat fx on binary size”** — **NON-GOAL** for Python core. Pass = N/A. LONG-TERM native sidecar would use fx’s **7.800 MiB** ceiling and measured asset sizes as the bar.

4. **“Beat fx on coding-agent product”** — **NON-GOAL** as a single metric. Phase 2 DoD = example + plugins + benches, per Alice scope lock — **not** “faster than fx.sh.”

---

## 7. Working rules for the team

1. Never say “we beat fx” in README, ROADMAP, or demos without a §6 DoD row and linked bench JSON.  
2. Always pair any fx MiB number with **which** source (homepage vs AGENTS ceiling vs measured platform asset).  
3. Prefer AGENTS.md **2 ms** over blog **10 µs** when discussing fx latency.  
4. Python core growth to chase fx CLI features = Alice veto (`useful-scope-phase2.md`).  
5. Refresh this file when fx bumps homepage size, AGENTS budgets, or latest release tag.

---

## Sources (this note)

- https://fx.sh/  
- https://github.com/vercel-labs/fx  
- https://raw.githubusercontent.com/vercel-labs/fx/main/AGENTS.md  
- https://raw.githubusercontent.com/vercel-labs/fx/main/benchmarks/check_budgets.py  
- https://fx.sh/docs/capabilities/tools.md  
- https://fx.sh/llms.txt  
- https://api.github.com/repos/vercel-labs/fx/releases/latest (`v0.0.10`)  
- Local: unpacked `fx-linux-x86_64` binary size; `benchmarks/cold_import.py`; `wc -l` on `src/agentic_harness`  
- Secondary only (cited as such): https://devaireviews.com/blog/vercel-fx-coding-agent-review/  
