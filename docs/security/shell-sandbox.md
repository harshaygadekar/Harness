# Design note: shell sandbox (outside core)

Status: Proposed. Owner: Walter. Needs Harsha's approval because it changes tool behavior.
This is not an ADR. It does not change `harness-core`.

## Problem
- `run_shell` uses a deny list of 7 text strings. It is not a boundary.
- Model-free tests: 17 of 38 escape cases still pass, and all 17 go through the shell.
- Every live benchmark row runs with shell on. So no row with shell on can carry a security result.

## Decision
The operating system enforces the boundary. Text matching does not.

1. Landlock (Linux kernel file-access limits). The box kernel is 6.12 and Landlock is active.
   - The shell child process applies a Landlock rule set before `exec`.
   - Read and write: the workspace only.
   - Read and execute: the system tool folders (`/usr`, `/bin`, `/lib*`, `/etc` read-only), the Rust toolchain folders, and `/tmp` under a private folder per run.
   - No access: home folders, `/proc/*/environ` of other processes, the key file, `/opt/benchproxy`.
   - `.env` and `.git` stay protected by the file tools. In the shell, `.git` is read-only if Landlock can express it; if not, it is a known limit.
   - Network (Landlock ABI 4+): allow TCP connect only to the proxy port on 127.0.0.1, or deny all. Default: deny all.
2. Environment: the shell starts with `env -i` plus a short allow list (`PATH`, `HOME` set to the workspace, `LANG`, `CARGO_HOME`). No keys.
3. Process limits: own process group, kill the group on timeout (closes the open process-kill gap), and `PR_SET_NO_NEW_PRIVS`.
4. Fail closed: if the kernel has no Landlock, `run_shell` is turned off and the run records `shell_sandbox=unavailable`. It never falls back to the deny list.
5. The deny list stays only as a guard against mistakes. It is not called a boundary in docs or reports.

## Where the code goes
- `harness-tools`: the sandbox setup in the shell tool, using the `landlock` crate (one new dependency) or raw syscalls if we want zero new crates.
- `harness-cli`: a flag that shows the sandbox state in `metrics.json` (`shell_sandbox=landlock|off|unavailable`).
- `harness-core`: no change. If the tool result type cannot carry a deny reason code, stop and bring it to Walter first.

## Also in scope: the read-side known limit (from 579ebfb)
- `read_file`, `list_files` and `search` still check a path first and open it later.
- Fix: use the same step-by-step `openat` with `O_NOFOLLOW` that `write_file` and `apply_patch` use.
- The root swap above the workspace stays a known limit. Landlock covers it for the shell.

## Pass conditions
- All 17 shell escape cases in `sandbox_escapes` pass with the sandbox on.
- Full suite passes. `harness-core` diff is empty.
- A test proves fail-closed: with Landlock forced off, `run_shell` refuses to run.
- `cargo test` still runs inside the sandbox on T1 v2, T7 and T9 fixtures (no false blocks).
- The key check from `run_isolated.sh` still passes.
- Only then may a row with shell on use the label "held".

## Not in scope
- Containers or VMs. Too heavy for a small agent. Revisit only if Landlock fails a pass condition.
- macOS and Windows. Shell stays off there until a separate note.
