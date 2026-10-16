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
   - Read and write: a private `/tmp` folder for each run.
   - Read and execute: the system tool folders (`/usr`, `/bin`, `/lib*`, `/etc` read-only) and the Rust toolchain folders.
   - No access: home folders, the key file, `/opt/benchproxy`.
   - Landlock can only add access. It cannot remove access to a path inside the workspace. So:
     - `.env`: no secrets live in the agent's workspace. Runs use a clean copy with no `.env`. The shell `.env` cases count as passing only because no `.env` exists, and the doc says so.
     - `.git`: shell writes to `.git` stay possible. This is a known limit. Those cases stay failing and are not counted as passing.
   - Other processes' environment (`/proc/*/environ`): Landlock does not protect `/proc`. The protection is that the agent runs as `benchagent` and the proxy runs as `benchproxy`. The `ps` check and the user check in `run_isolated.sh` stay required.
   - Network: deny TCP bind and connect (ABI 4) and use the ABI 6 scopes (signals and abstract UNIX sockets). UDP is not controlled by Landlock and stays open. This is a known limit.
   - Mode, owner and time changes: Landlock does not control `chmod`, `chown` and time changes. The shell can change these on any file that `benchagent` owns, also outside the workspace. File contents outside stay safe. This is a known limit (Walter, case 6). It is safe only if `benchagent` owns no files outside its workspace, its private `/tmp` folders and its home folder.
   - Crates: download them before the run and set `CARGO_NET_OFFLINE=true`.
   - Hard mode only: require full support for the ABI level we use. If the kernel supports less, the shell turns off. The crate's best-effort mode is not allowed.
2. Environment: the shell starts with `env -i` plus a short allow list (`PATH`, `HOME` set to the workspace, `LANG`, `CARGO_HOME`). No keys.
3. Process limits: `PR_SET_NO_NEW_PRIVS`. A command can call `setsid` and leave its process group, so a group kill is not enough. The tool uses `PR_SET_CHILD_SUBREAPER` (or a cgroup where one is available) and kills every process under it on timeout. This closes the open process-kill gap.
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
- The shell escape cases in `sandbox_escapes` pass with the sandbox on, except the `.git` write cases, which stay listed as a known limit.
- A test with a lower Landlock version proves that the shell turns off.
- A test where a command calls `setsid` and then sleeps proves that the timeout kills it.
- Full suite passes. `harness-core` diff is empty.
- A test proves fail-closed: with Landlock forced off, `run_shell` refuses to run.
- `cargo test` still runs inside the sandbox on T1 v2, T7 and T9 fixtures (no false blocks).
- The key check from `run_isolated.sh` still passes.
- Only then may a row with shell on use the label "held".

## Not in scope
- Containers or VMs. Too heavy for a small agent. Revisit only if Landlock fails a pass condition.
- macOS and Windows. Shell stays off there until a separate note.
