# Harness comparison (for Thomas)

**Audience:** Thomas (AI/ML Research Lead)  
**Date:** 2026-09-19 (IST)  
**Status:** Factual snapshot. Sizes only from project sites, release assets, or local measurement. Unknown marked unknown.

**Our repo:** `/workspace/agentic-harness` (GitHub: https://github.com/harshaygadekar/Harness)

---

## Projects identified

| Nickname | Actual project | Repo / site |
|----------|----------------|-------------|
| **Our harness** | Minimal Python agentic harness (ADR-001/002) | local `/workspace/agentic-harness` |
| **fx.sh / Vercel** | **fx** (Vercel Labs) — “tiny, open, native coding agent” / agent harness + CLI | https://github.com/vercel-labs/fx · https://fx.sh/ |
| **oh-my-pi** | **Oh My Pi (omp)** — coding agent fork of Pi | https://github.com/can1357/oh-my-pi · https://omp.sh / https://ohmypi.xyz/ |
| **DeepSeek Harness (dsh)** | DeepSeek Harness — “everything is a plugin” | https://github.com/deepseek-ai/deepseek-harness · https://deepseek.com/harness/en/ |

**Identity notes**

- User’s “fx.sh by vercel under 6.8 mb written in zig” matches **vercel-labs/fx**. Homepage (fetched 2026-09-19 IST) claims a **6.17 MiB** native binary; release archives are smaller compressed. “Harness” there = coding-agent runtime/CLI meant to be embedded (libfx / WASM / ACP), not a Python library.
- “oh-my-pi” is **can1357/oh-my-pi** (CLI `omp`), not “ohmyapi” / “Oh My OpenCode”. TypeScript agent surface + ~80k LoC Rust natives claimed in README.

---

## Our harness — local facts

**Docs:** README, ARCHITECTURE.md, ROADMAP.md (product lock: thin core; plugins for the rest).

**Python under `src/` (measured 2026-09-19 IST):** **638 LOC** total (`wc -l` on package `.py` files).

| Module | Role |
|--------|------|
| `runner.py` | ReAct/`max_steps` loop; only path that executes tools |
| `gate.py` | Deny-all `can_use_tool` until host allowlists by name |
| `plugin.py` | `Plugin.register` + disposer; tool/model registry |
| `model.py` | `ModelClient` protocol (provider-agnostic) |
| `tool.py` | `Tool` / `ToolSpec` / host `FunctionTool` |
| `messages.py` | Message / ToolCall / Usage |
| `result.py` | `RunResult` / status / `SafeError` |
| `sanitize.py` | Secret scrubbing for errors/text |
| `plugins/gemini.py` | Optional Gemini `ModelClient` (extra; not core import) |

**`pyproject.toml` dependencies**

- Core: **`dependencies = []`** (stdlib-only core)
- Optional: `dev = ["pytest>=8.0"]`, `gemini = ["google-genai>=1.0"]`
- License text: **MIT**; Python `>=3.11`

**Explicit non-goals in core:** shell, `python_eval`, MCP runtime, multi-agent, Docker/sandbox, RAG, memory OS, observability SaaS, server.

---

## Comparison table

| Dimension | Our harness | fx (Vercel Labs) | oh-my-pi (omp) | DeepSeek Harness (dsh) |
|-----------|-------------|------------------|----------------|------------------------|
| **Language** | Python 3.11+ | Zig (native + WASM) | TypeScript + Rust (N-API natives) | TypeScript (Cordis); Python SDK is a stdio driver around Node, not a port |
| **Approx. size** | **638 LOC** Python under `src/`; install = empty deps + interpreter (no binary claim) | **~6.17 MiB** binary claimed on fx.sh (2026-09-19); release **v0.0.10** `.tar.gz` ≈ **3.8–5.3 MiB** compressed (platform-dependent). Uncompressed binary per asset not listed in release API. | Release **v18.2.6** binaries: **~179–241 MB** (e.g. `omp-darwin-arm64` 178.6 MB, `omp-linux-x64` 240.7 MB). `@oh-my-pi/pi-coding-agent` npm `dist.unpackedSize` ≈ **47.3 MB** (package alone; not full binary). | Meta npm `@deepseek-ai/dsh@0.1.5-rc.2` unpack ≈ **49 KB** (launcher). **Full install/tree size: unknown** from official docs. Secondary clone measurement (Developers Digest, 2026-08-13): ~**453k** TS LoC / ~219 packages — not an official size claim. |
| **What “harness” means** | Thin library: loop + tool protocol + deny-all gate + plugins | Full coding-agent CLI/runtime optimized for embed (CI, sandboxes, browser via libfx) | Batteries-included terminal coding agent (“IDE wired in”) | Full agent runtime where nearly every capability is a Cordis plugin |
| **Core loop** | Yes — `Runner` in core (ADR-001) | Yes — agent turn loop in native binary | Yes — agent session / tool loop | Yes — loop itself is a **replaceable plugin** |
| **Tools** | Host-supplied only; none built-in | Built-in coding tools + skills; MCP tools load on demand | **31** built-in tools claimed (files, bash, LSP, DAP, browser, memory, …) | Mode-dependent (Standard / Code / Minimal / Creator); Minimal ≈ shell + editor |
| **Plugins** | Yes — `Plugin.register` + disposer; providers as plugins (ADR-002) | Extensible via skills, custom tools, libfx host hooks | Extensions (TS modules), marketplaces, inherit Cursor/Claude/etc. configs | **Everything is a plugin** (models, tools, sessions, sandboxes, storage, UI, loop) |
| **MCP** | No (roadmap: plugin pack later) | Yes — **MCP client** (stdio / HTTP / SSE; project trust) | Yes — MCP client (enabled by default in SDK path) | MCP as client/bridge (ecosystem + docs; client-oriented) |
| **Sandbox** | No in core | Permission system; designed to run *in* sandboxes; not the same as OS confinement APIs | Workspace isolation helpers (`pi-iso`); shell retains host authority unless you add OS/container isolation | Yes — fail-closed ladder (Linux bubblewrap/Landlock, macOS Seatbelt, Windows restricted token) per secondary + docs theme |
| **Multi-agent** | No | Yes — subagents | Yes — first-class `task` / Agent Hub / advisor role | Yes — subagents/workflows in Standard-class modes |
| **Memory** | Message list only; no memory OS | Sessions / transcript (product memory OS: not a headline feature) | Yes — retain/recall/learn; backends local / Hindsight / Mnemopi | Append-only session event log + compaction seams; durable persistence via plugins |
| **Streaming** | Sync `ModelClient.complete` (no stream API in core) | Interactive shell + ACP streaming surfaces | Yes — stream rules, session events, TUI | Yes — model streaming via Cordis/`ctx.llm` seams |
| **Security / permissions** | **Deny-all** tool gate; host must allowlist by name; error scrubbing | Modes `ask` / `auto` / `full-access`; rules + session grants; MCP project trust | ACP `session/request_permission`; tool allowlists; not a full OS sandbox by default | Approval policy + sandbox policy; missing backend → deny (fail-closed) |
| **License** | MIT (`pyproject.toml`) | Apache-2.0 | MIT | MIT (developer preview; breaking changes warned) |

---

## Tight takeaways

1. **Our harness** is a *library-sized* Python core (~0.6k LOC, zero runtime deps) for embedding a gated tool loop — closest in *spirit* to dsh’s “seams + gate,” not in *product weight*.
2. **fx** is the real “tiny Zig binary ~6 MiB” coding-agent harness from Vercel Labs; “harness” = embeddable native agent, not a thin protocol package.
3. **oh-my-pi** is a large, feature-rich coding agent (hundreds of MB binary); comparing its size to our LOC or fx’s MiB is category error.
4. **dsh** is the architectural reference (Cordis plugins, loop-as-plugin, sandbox/approval). Its npm meta-package size is not the install weight.

---

## Five honest caveats

1. **Python LOC vs Zig/TS binary MiB is not apples-to-apples.** Our 638 LOC excludes the CPython runtime and any optional `google-genai` tree; fx’s ~6 MiB is a self-contained native binary; omp’s ~180–240 MB ships Rust natives + Bun/agent surface.
2. **Product scope differs.** Ours is a *minimal embeddable loop*; fx/omp/dsh are *shipping coding agents* (CLI/TUI/web) with built-in tools. Feature cells (MCP, multi-agent, memory) favor products that already solved “useful for coding,” not library cores.
3. **fx size claims move.** Homepage said **6.17 MiB** on fetch day; older writeups cited ~6.0–6.8 / 7.8 MiB; GitHub release lists **compressed** archives only. Do not treat “under 6.8 MB” as a fixed SLA.
4. **dsh install size is unknown from first-party packaging alone.** The published `@deepseek-ai/dsh` tarball is a tiny launcher; real weight is the pnpm/npx dependency closure + optional UI. Secondary LoC/package counts are measurements of a clone at a point in time, not release artifacts.
5. **Security models are not interchangeable.** Deny-all allowlists (us), interactive/auto permission modes (fx), ACP prompts (omp), and OS sandboxes (dsh) solve different threat models. Shipping shell/browser tools without confinement is not “more complete security.”

---

## Sources (fetched / measured 2026-09-19 IST)

- Local: `README.md`, `ARCHITECTURE.md`, `ROADMAP.md`, `pyproject.toml`, `src/**/*.py` (`wc -l`)
- fx: https://fx.sh/ , https://github.com/vercel-labs/fx , https://fx.sh/docs/configure-fx/permissions.md , https://fx.sh/docs/capabilities/mcp , release API `v0.0.10`
- oh-my-pi: https://github.com/can1357/oh-my-pi README, https://ohmypi.xyz/ , release `v18.2.6` assets, `npm view @oh-my-pi/pi-coding-agent`
- dsh: https://github.com/deepseek-ai/deepseek-harness/blob/master/README.md , https://deepseek.com/harness/en/ , https://deepseek-harness.github.io/deepseek-harness/en/reference/ , `npm view @deepseek-ai/dsh` ; secondary: https://www.developersdigest.tech/blog/deepseek-harness-dsh-first-look
