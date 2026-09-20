# 12 — Native language bake-off (ADR-005 evidence)

**Audience:** Walter (ADR-005), Thomas (Research Lead), Tech Team native coding-agent track  
**Date:** 2026-09-20 (IST)  
**Status:** Research only. No native source tree yet. Python harness stays frozen.  
**Rule:** Mark **measured** vs **cited**. Say **unknown** when unknown. Do not invent numbers. Do not claim we beat fx.

**Scope:** Language choice for a **tiny native AI coding agent** (separate product track). Not a rewrite of the Python core. DeepSeek Harness = **design inspiration** (plugin seams) only — **not** DeepSeek LLM API for this ADR.

---

## 1. Product honesty

| | **fx (Vercel Labs)** | **Python harness (frozen)** | **Native track (this note)** |
|--|----------------------|-----------------------------|------------------------------|
| What | Zig coding-agent CLI + WASM/libfx | Minimal Python library + plugins | New native binary product |
| Fair fight with fx? | — | Only library axes (`10`) | **Yes** on size / process start / tool-efficiency **after** we measure |
| DeepSeek role | — | Seams inspiration (`06`) | Same: plugin seams only |

Python stays. Native lives under a proposed `native/` tree (see §8). No push in this research task.

---

## 2. Candidates

| Language | On this research host | Toolchain note |
|----------|----------------------|----------------|
| **Zig** | **Not installed** | fx builds with Zig **0.16+** (fx README). No local hello smoke. |
| **Rust** | **Yes** — `rustc` / `cargo` **1.85.1** | Measured hello + ureq+rustls smoke. |
| **C** | **Yes** — `gcc` **14.2.0** | Measured dynamic + `gcc -static` hello. No `musl-gcc`. |
| **Go** | **Yes** — `go` **1.24.4** linux/amd64 | Measured hello + blank-import floors. |

**Platform (all measured rows below):** Linux `6.12.94+` x86_64, Debian 13 (trixie), 2026-09-20 IST. Method: `stat` bytes; startup = Python `perf_counter` around `subprocess.run` (warm process table; not hyperfine).

---

## 3. Smoke measurements (this host)

### 3.1 Hello binary size (**measured**)

Build notes: C `gcc -O2 -s`; C static `gcc -O2 -s -static` (glibc static, not musl); Rust `rustc -O -C strip=symbols` and `-C opt-level=z -C lto -C strip=symbols`; Go `CGO_ENABLED=0 go build -ldflags="-s -w"`.

| Binary | Bytes | MiB | Linking |
|--------|------:|----:|---------|
| C hello (dynamic) | 14,472 | 0.014 | dynamic → `libc` |
| C hello (`-static` glibc) | 676,288 | 0.645 | static glibc |
| Rust hello (`-O` strip) | 391,680 | 0.374 | dynamic → `libc`, `libgcc_s` |
| Rust hello (`-Oz` + LTO strip) | 330,160 | 0.315 | dynamic |
| Go hello (`fmt`, strip) | 1,433,784 | 1.367 | static (Go runtime; `ldd`: not a dynamic executable) |
| Zig hello | — | — | **unknown here** (no `zig`) |

### 3.2 Hello startup (**measured**, n=100, median)

| Binary | Median ms | Mean ms | Min ms |
|--------|----------:|--------:|-------:|
| C hello | 0.535 | 0.563 | 0.364 |
| Rust hello | 0.694 | 0.718 | 0.519 |
| Go hello | 1.078 | 1.087 | 0.833 |
| Zig hello | — | — | **unknown here** |

All three measured medians are **under** fx’s Linux CI **2 ms** process budget (`10` / fx `AGENTS.md`). That does **not** prove an agent binary will stay under 2 ms.

### 3.3 Idle RSS sample (**measured**, sleeping process, `VmRSS`)

| Process | VmRSS (KiB) |
|---------|------------:|
| C `sleep(2)` | 1,404 |
| Go `time.Sleep(2s)` | 1,664 |
| Rust `thread::sleep(2s)` | 2,080 |

Informational only. Not comparable to an agent with TLS/JSON/plugins loaded.

### 3.4 HTTP / TLS size floors

#### Go blank-import floors (**measured**, this host, Go 1.24.4, `CGO_ENABLED=0`, `-trimpath -ldflags='-s -w'`)

| Program | Bytes | MiB |
|---------|------:|----:|
| empty `main` | 999,608 | 0.953 |
| `fmt` hello | 1,425,592 | 1.360 |
| `import _ "crypto/tls"` | 2,384,056 | 2.274 |
| `import _ "net/http"` | 3,231,928 | 3.082 |

These match the order of magnitude in golang/go **#81132** (cited table for go1.24.13 linux/amd64: empty 1,007,800; tls 2,384,056; http 3,231,928).

#### Rust ureq + rustls (**measured**, this host)

- Crate: `ureq` 3.4.2, `default-features = false`, feature `rustls`
- Profile: `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`, `strip = true`
- Binary: **1,509,912** bytes (**1.440 MiB**), still dynamically linked to `libc` / `libgcc_s`
- Startup (create `Agent`, n=50): median **0.824 ms**, min **0.633 ms**
- This is an HTTP client **floor**, not a full coding agent.

#### Cited HTTPS / agent-sized estimates (not measured here)

| Claim | Value | Source | Mark |
|-------|-------|--------|------|
| Zig HTTPS download+exec (std TLS + embedded CA) | ~**460 KB** stripped; ~220 KB after UPX | https://rusty.today/blog/vgi-injector-a-tiny-https-downloader-in-zig/ (2026-03-08) | **cited** secondary |
| Rust `reqwest` blocking + rustls, size flags | ~**1.2 MB** | Same post | **cited** secondary |
| Go same use case | ~**4.5 MB** stripped | Same post | **cited** secondary |
| Rust `httpget` + rustls (project README) | **1.3 MB** TLS build | https://github.com/taliamax/httpget | **cited** |
| Zig `zfetch` ReleaseSmall | **2.5 MB** (vendored BoringSSL) | https://github.com/roobie/zfetch README | **cited** |
| fx homepage binary | **6.17 MiB** | https://fx.sh/ fetch 2026-09-20 | **cited** first-party marketing |
| fx AGENTS production ceiling | **7.800 MiB** | fx `AGENTS.md` (via `10`) | **cited** |
| fx measured linux-x86_64 v0.0.10 | **~11.27 MiB** uncompressed | Local unpack in `10` | **cited** prior research |

**Agent-sized estimate (honest):** A coding agent adds JSON, process spawn, TTY, tool registry, model HTTP streaming, and plugins. Expect **multi-MiB**, not hello size. Target band for fair fx fight: stay under or near the **7.800 MiB** ceiling on the **same** platforms fx gates — **unknown until we build**. Do not project hello bytes to agent MiB.

---

## 4. Per-language matrix

Legend: **M** = measured this host · **C** = cited · **U** = unknown / not verified here.

### 4.1 Zig

| Topic | Assessment | Evidence |
|-------|------------|----------|
| Executable size | Best cited tiny HTTPS floor among candidates; fx ships single-digit–teen MiB agent | C: rusty.today ~460 KB HTTPS; fx.sh 6.17 MiB; `10` 11.27 MiB linux-x86_64 |
| Startup latency | fx Linux CI **2 ms** mean budget; marketing “10 µs” is **not** the engineering SLA | C: `10`, fx AGENTS.md; U for Zig hello here |
| Static vs dynamic | Excellent freestanding / static story (`x86_64-linux-none` cited) | C: rusty.today; fx single binary |
| Memory behavior | Manual alloc; no GC. Idle RSS for agent: **U** | — |
| Async runtime overhead | `std.Io` evolving (evented still experimental in public lessons). Overhead: **U** | C: ziglang Io / learning materials |
| FFI / plugin | C ABI interop is a Zig strength; dlopen possible; fx embeds via WASM/libfx | C: fx README |
| Cross-compilation | First-class (`-target`); often used as linker for other langs | C: zig docs / cargo-zigbuild ecosystem |
| Terminal / TTY | Possible; fx ships shell-like CLI + term WASM | C: fx.sh |
| HTTP / TLS | std HTTP client + TLS 1.3 in std (version-dependent maturity) | C: zig `std.http.Client`; rusty.today |
| JSON | std / community; maturity vs serde: thinner | U for agent-scale benches |
| Process management | OS APIs via std; pattern like C | U |
| Sandboxing | Landlock/seccomp via raw syscalls or thin wrappers; crate depth thinner than Rust | C: secondary comparisons; treat maturity as **weaker ecosystem** |
| WASM potential | fx ships `fx-core.wasm` / `fx-term.wasm` — proven product path | C: fx README |
| Fit vs fx | **Same language as competitor** → fairest size/start comparison | — |

**Pros:** Size headroom; cross-compile; no GC; direct fx parity language.  
**Cons:** Not on this host; language moves fast (fx wants 0.16+); smaller library surface for plugins/sandbox; hiring pool small.

### 4.2 Rust

| Topic | Assessment | Evidence |
|-------|------------|----------|
| Executable size | Hello **M** 0.32–0.37 MiB; ureq+rustls **M** 1.44 MiB; room under 7.8 MiB ceiling if kept lean | M §3; C httpget 1.3 MB |
| Startup latency | Hello **M** ~0.7 ms; ureq agent **M** ~0.8 ms median | M §3 |
| Static vs dynamic | Default dynamic libc; musl/`x86_64-unknown-linux-musl` for static (not tried here — no musl) | M ldd on hello/ureq |
| Memory behavior | No GC; ownership. Sleep RSS **M** ~2.0 MiB (hello-class) | M §3.3 |
| Async runtime | Tokio (or async-std) is optional; blocking ureq avoids runtime. Tokio cost: **U** for our agent | — |
| FFI / plugin | `libloading` for `.so`; **Wasmtime** WASI components for sandboxed plugins | C: Wasmtime WASI plugins docs; rustant_plugins / community agent designs |
| Cross-compilation | Pure Rust: rustup targets. C deps: harder; `cargo-zigbuild` common bridge | C: cargo-zigbuild README |
| Terminal / TTY | Mature crates (`crossterm`, `ratatui`, etc.) | C: crates.io ecosystem (existence, not bench) |
| HTTP / TLS | `ureq`/`reqwest` + rustls; no OpenSSL required | M ureq; C reqwest blogs |
| JSON | `serde_json` de facto | C: ecosystem |
| Process management | `std::process`; `tokio::process` if async | — |
| Sandboxing | Strong crate story: Landlock, seccomp, cap-std; plus WASM fuel/epoch limits | C: landlock/seccomp crates; Wasmtime |
| WASM potential | Host **or** guest; Component Model plugins | C: Wasmtime docs |
| Fit vs fx | Can chase same **numeric** axes; different stack → cite method + SHA, never imply “same binary class” without measure | — |

**Pros:** Safety; plugins/WASM; HTTP/JSON/TTY ecosystem; measured on this host.  
**Cons:** Cross-compile with C deps is sticky; TLS floor already ~1.4 MiB before agent logic; compile times higher than Go/Zig (not measured here).

### 4.3 C

| Topic | Assessment | Evidence |
|-------|------------|----------|
| Executable size | Smallest hello **M** 14 KiB dynamic / 0.65 MiB static glibc | M §3.1 |
| Startup latency | Fastest hello **M** ~0.54 ms | M §3.2 |
| Static vs dynamic | Full control; musl static not available on this host | M |
| Memory behavior | Manual; easy to leak or UAF in agent/plugin host | — |
| Async runtime | Roll your own or libuv-class; overhead on us | U |
| FFI / plugin | Native `dlopen` — also the **unsafest** in-process plugin model | — |
| Cross-compilation | Toolchain per target; Zig-cc often used as helper | C: industry practice |
| Terminal / TTY | termios / ncurses; more glue | — |
| HTTP / TLS | No std client; OpenSSL/libcurl or vendored TLS | Size then dominated by TLS stack (**U** for our tree) |
| JSON | cJSON / yyjson / etc. | — |
| Process management | POSIX; well known | — |
| Sandboxing | seccomp/Landlock via libc wrappers; all footguns on us | — |
| WASM potential | WASI SDK / clang; host embedding possible but heavy lift | C: Wasmtime C API exists |
| Fit vs fx | Can win size in theory; lose on safety and delivery speed for agent product | — |

**Pros:** Minimal bytes; maximal control.  
**Cons:** Memory unsafety; reinvent HTTP/TLS/JSON/plugin ABI; poor match for “deny-all gate + plugins” security story.

### 4.4 Go

| Topic | Assessment | Evidence |
|-------|------------|----------|
| Executable size | Hello **M** 1.37 MiB; `net/http` blank import **M** 3.08 MiB before app logic | M §3 |
| Startup latency | Hello **M** ~1.08 ms (still under 2 ms budget as hello) | M §3.2 |
| Static vs dynamic | Easy static with `CGO_ENABLED=0` | M ldd |
| Memory behavior | GC; sleep RSS **M** ~1.6 MiB hello-class; agent steady-state **U** | M §3.3 |
| Async runtime | Goroutines are the runtime; overhead baked into binary | — |
| FFI / plugin | `plugin` package is limited (same toolchain/OS); cgo for C ABI | Weak for third-party plugins |
| Cross-compilation | Excellent for pure Go (`GOOS`/`GOARCH`); cgo needs extra CC | C: Go docs |
| Terminal / TTY | Good libraries | — |
| HTTP / TLS | Batteries included; Go 1.24+ crypto size growth documented | M; C golang/go #81132 |
| JSON | `encoding/json` std | — |
| Process management | `os/exec` excellent | — |
| Sandboxing | Possible; fewer first-class Landlock helpers than Rust | Secondary |
| WASM potential | `GOOS=js` / wasip1; **large** modules; #81132 shows wasm TLS size pain | C: #81132 |
| Fit vs fx | Hard to enter fx’s **7.8 MiB** ceiling class once HTTP+TLS+agent UX land | M floors |

**Pros:** Fast delivery; std HTTP/JSON; easy cross-compile.  
**Cons:** Binary floor; GC; weak plugin ABI; WASM size.

---

## 5. Recommendation for ADR-005

### Selected: **Rust**

Pick **Rust** as the native coding-agent language for ADR-005.

### Top 5 reasons

1. **Safety without GC** — Long-running agent + in-process or WASM plugins need memory safety. Rust beats C/Zig on safety defaults; beats Go on predictable non-GC latency/RSS class for tight CLIs.
2. **Measured size path under fx’s ceiling** — Hello **0.32–0.37 MiB**; ureq+rustls **1.44 MiB** on this host. Leaves headroom toward fx AGENTS **7.800 MiB** ceiling if core stays lean (still must measure a real agent binary).
3. **Plugin seams match DeepSeek Harness inspiration** — Traits + registry + teardown; optional **Wasmtime** components for capability-limited plugins; `libloading` for trusted native plugins. Aligns with “everything optional except a thin driver” without vendoring Cordis/`dsh`.
4. **Batteries for the agent surface** — HTTP/TLS (rustls), JSON (serde), process, TTY crates, Landlock/seccomp crates — without owning OpenSSL or a GC runtime.
5. **Evidence on this host** — Toolchain present; hello and HTTP smokes recorded. Zig has no local smoke; Go’s measured HTTP floor is already **3.08 MiB**.

### Rejected alternatives

| Language | Why not ADR-005 default |
|----------|-------------------------|
| **Zig** | Best **cited** tiny HTTPS and **same stack as fx** (fairest language-level fight). Rejected as **default** because: no local toolchain/smoke; language/tooling churn (fx pins 0.16+); thinner plugin/sandbox library surface for our seam design. **Keep as explicit revisit** if product later mandates “Zig-only for apples-to-apples fx size claims.” |
| **Go** | Measured `net/http` floor **3.08 MiB** before agent features; hello already **1.37 MiB**; GC; limited plugin story; WASM size regressions documented (#81132). Poor fit for fx-class MiB goals. |
| **C** | Smallest hello, but unsafe host for plugins/tools; no std HTTP/TLS/JSON; delivery risk too high for the coding-agent track. |

---

## 6. Fair beat axes vs fx (native product)

Compete only where the products match: **native coding-agent binary**.

| Axis | Why fair | How to measure | Do **not** claim until |
|------|----------|----------------|------------------------|
| **Stripped release MiB** | fx optimizes binary size | Same OS/arch as fx release assets; report uncompressed **and** archive size | Head-to-head on same machine/tag |
| **Process cold start** | fx Linux CI **2 ms** mean (`FX_BENCH=1` help-class paths) | hyperfine / our runner; document warm vs cold | Measured p50/p95 under budget |
| **Tool-efficiency** | Both run gated tools in a loop | Fake-model dispatch /s; tokens in tool schemas (bytes) | Shared harness script + SHA |
| **Idle / steady RSS** | Users care; fx has **no** first-party memory SLA in `10` | `VmRSS` after `help` and after one fake turn | Both sides measured |

**Unfair / forbidden claims**

- “We beat fx” without method + SHA + machine class (`10` DoD).
- Using marketing **10 µs** as fx’s SLA (engineering budget is **2 ms**).
- Comparing Python cold import to fx process start.
- Counting built-in tool count as a core win (tools stay plugins).

Publish metrics with: platform, commit SHA, build flags, fx version compared.

---

## 7. Core vs must-stay-plugins (native)

Align with `04`, ADR-003 freeze spirit, and `11` §4. DeepSeek Harness inspires **seams**, not a Cordis port.

### Core contents (native)

| Area | In core |
|------|---------|
| Model protocol seam | Request/response + stream events; **no** provider SDK |
| Messages | Canonical message list (v0) |
| Agent loop | Default driver: model → tools → append → stop/continue |
| Tools protocol | Register / call / result types |
| Gate | Deny-all default; allow-by-name hooks |
| Lifecycle | Plugin register + teardown/disposers |
| Cancel | Cooperative cancel token / context |
| Streaming | Stream consumer hooks in core; transport adapters outside |

### Must stay plugins (never core)

| Capability | Why |
|------------|-----|
| Provider SDKs (OpenAI, Anthropic, Gemini, …) | Network + vendor deps |
| Shell | Side effects; opt-in Allow |
| Filesystem tools | Path policy in plugin |
| Git | Domain VCS |
| MCP runtime / transports | Adapter implements `Tool` |
| Sandbox (Docker, microVM, bubblewrap) | Ops product; `Environment` plugin |
| TUI / fancy terminal product | Presentation |
| Long-term memory / RAG | Not required for bare loop |
| Observability backends | Hooks only in core |

---

## 8. Proposed `native/` tree (Python untouched)

Do **not** delete or move `src/agentic_harness/`. Native is additive.

```text
native/
  README.md                 # build, targets, non-goals vs Python
  agent/                    # binary crate: CLI entry, flags, FX_BENCH-style early exit
  cmd/                      # thin command modules (ask, help, status, version)
  runtime/                  # loop, messages, gate, cancel, streaming seams
  plugins/                  # in-tree example plugins only (shell/fs/git/MCP stay here)
    README.md               # load policy: wasm | native dylib | none
  benchmarks/               # size gate, startup, fake-model dispatch
  tests/                    # unit + integration; no network by default
  Cargo.toml                # workspace root (if Rust ADR-005 Accept)
  # OR build.zig            # only if a later ADR switches to Zig
```

Python remains:

```text
src/agentic_harness/        # frozen protocol core
plugins/                    # Python plugins
docs/                       # shared research + decisions
```

No native source code in this research drop — layout proposal only.

---

## 9. What Walter needs for ADR-005

1. **Decision:** Accept **Rust** as native language (or file a dissenting Zig-first option with product lock on fx stack parity).
2. **Non-goals:** No Python rewrite; no DeepSeek LLM API; no claim of beating fx pre-measure.
3. **First milestones after Accept:**
   - Scaffold `native/` workspace (empty crates / hello CLI only).
   - Size + startup CI gates with **explicit budgets** (propose: stripped linux-x86_64 ≤ **8.0 MiB** stretch toward **7.8**; help-class start ≤ **2 ms** mean on Linux CI — match fx *budget shape*, not fx’s binary).
   - Fake-model tool loop bench (fair tool-efficiency axis).
4. **Plugin ADR follow-up:** Native dylib vs WASM-component default (recommend WASM for untrusted; dylib for trusted high-perf).
5. **Zig revisit trigger:** If Thomas/Alice require same-language comparison with fx, open ADR-005b with Zig 0.16+ pinned in CI and repeat §3 smokes.

---

## 10. Sources (short list)

Full pack list: [`sources.md`](sources.md). Key URLs for this note:

- fx homepage: https://fx.sh/ (6.17 MiB line; fetch 2026-09-20 IST)
- fx GitHub / README (Zig 0.16+; 7.8 MiB line; WASM embed)
- Prior pack: [`10-fx-beat-goals.md`](10-fx-beat-goals.md), [`11-coding-agent-product.md`](11-coding-agent-product.md), [`06-deepseek-harness.md`](06-deepseek-harness.md), [`04-keep-out-of-core.md`](04-keep-out-of-core.md)
- Go crypto size: https://github.com/golang/go/issues/81132
- Zig HTTPS size secondary: https://rusty.today/blog/vgi-injector-a-tiny-https-downloader-in-zig/
- Rust httpget sizes: https://github.com/taliamax/httpget
- Wasmtime WASI plugins: https://docs.wasmtime.dev/wasip2-plugins.html
- cargo-zigbuild: https://github.com/rust-cross/cargo-zigbuild

**Local smokes:** `/tmp/native-bakeoff/` on research host (not committed). Re-run commands are reconstructible from §3.

---

## 11. One-paragraph verdict

For a separate native coding-agent track that must chase fx-class **size**, **start**, and **tool-efficiency** without freezing Python, **Rust** is the ADR-005 default: measured sub-millisecond hello/HTTP startup, ~1.44 MiB rustls HTTP floor with headroom under fx’s 7.8 MiB ceiling, and a plugin/WASM/sandbox ecosystem that matches DeepSeek-style seams. **Zig** remains the strongest size/cross-compile alternative and the fairest language match to fx, but lacks a local smoke and brings toolchain churn — revisit if product locks Zig. **Go** and **C** are rejected for binary-floor and safety/delivery reasons respectively. Do not claim a beat on fx until native benches land.
