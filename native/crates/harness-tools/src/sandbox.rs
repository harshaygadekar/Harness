//! Shell sandbox (docs/security/shell-sandbox.md). Outside core. Linux only.
//!
//! - Landlock, hard mode: the kernel must support ABI 6 in full. If not, the
//!   shell is off (fail closed). It never runs without the sandbox.
//! - Files: read and write in the workspace and a private tmp folder only.
//!   Read and execute in the system folders. No home folders, no keys.
//!   `.env*` files in the workspace get no read right (rules are split).
//! - Network: no TCP bind, no TCP connect. Scopes: signals, abstract UNIX
//!   sockets. UDP is not controlled by Landlock (known limit).
//! - Process: `PR_SET_NO_NEW_PRIVS`. A supervisor process is the
//!   `PR_SET_CHILD_SUBREAPER` for the command. When the command ends or the
//!   time runs out, it kills every process under it, also `setsid` ones.

use std::ffi::CString;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// Landlock ABI that we need in full (ABI 6: TCP rules and scopes).
pub const REQUIRED_ABI: i64 = 6;
/// Test hook: an ABI value that can only LOWER the ABI the kernel reports.
/// It cannot turn the sandbox on, so it can only make the shell fail closed.
pub const ABI_CAP_ENV: &str = "HARNESS_LANDLOCK_MAX_ABI";
/// Supervisor exit code after a timeout.
pub const TIMEOUT_EXIT: i32 = 124;

const SYS_CREATE: libc::c_long = libc::SYS_landlock_create_ruleset;
const SYS_ADD: libc::c_long = libc::SYS_landlock_add_rule;
const SYS_RESTRICT: libc::c_long = libc::SYS_landlock_restrict_self;

const EXECUTE: u64 = 1 << 0;
const WRITE_FILE: u64 = 1 << 1;
const READ_FILE: u64 = 1 << 2;
const READ_DIR: u64 = 1 << 3;
const TRUNCATE: u64 = 1 << 14;
const IOCTL_DEV: u64 = 1 << 15;
/// All file-system rights up to ABI 5.
const FS_ALL: u64 = (1 << 16) - 1;
/// Rights that a rule on a file (not a folder) can carry.
const FILE_ONLY: u64 = EXECUTE | WRITE_FILE | READ_FILE | TRUNCATE | IOCTL_DEV;
const NET_BIND_TCP: u64 = 1;
const NET_CONNECT_TCP: u64 = 2;
const SCOPE_ABSTRACT_UNIX: u64 = 1;
const SCOPE_SIGNAL: u64 = 2;

#[repr(C)]
struct RulesetAttr {
    handled_access_fs: u64,
    handled_access_net: u64,
    scoped: u64,
}

#[repr(C, packed)]
struct PathBeneathAttr {
    allowed_access: u64,
    parent_fd: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Landlock,
    Unavailable,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Status::Landlock => "landlock",
            Status::Unavailable => "unavailable",
        }
    }
}

/// Landlock ABI from the kernel (negative or 0 if none), lowered by the test hook.
pub fn kernel_abi() -> i64 {
    // SAFETY: version query; no pointers are read.
    let real = unsafe { libc::syscall(SYS_CREATE, std::ptr::null::<RulesetAttr>(), 0usize, 1u32) } as i64;
    match std::env::var(ABI_CAP_ENV).ok().and_then(|v| v.parse::<i64>().ok()) {
        Some(cap) => real.min(cap),
        None => real,
    }
}

/// Hard mode: only a full ABI 6 (or newer) turns the sandbox on.
pub fn status_for_abi(abi: i64) -> Status {
    if abi >= REQUIRED_ABI {
        Status::Landlock
    } else {
        Status::Unavailable
    }
}

/// Sandbox state for this kernel: `landlock` or `unavailable`.
pub fn status() -> Status {
    status_for_abi(kernel_abi())
}

fn cpath(p: &Path) -> std::io::Result<CString> {
    CString::new(p.as_os_str().as_encoded_bytes()).map_err(|_| std::io::Error::other("NUL in path"))
}

fn open_path(p: &Path) -> std::io::Result<OwnedFd> {
    let c = cpath(p)?;
    // O_NOFOLLOW: a rule never follows a link to its target.
    // SAFETY: valid C string; fd is owned below.
    let fd = unsafe { libc::open(c.as_ptr(), libc::O_PATH | libc::O_CLOEXEC | libc::O_NOFOLLOW) };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: fresh fd.
    Ok(unsafe { OwnedFd::from_raw_fd(fd) })
}

/// A Landlock rule set that is ready to apply.
pub struct Ruleset {
    fd: OwnedFd,
}

impl Ruleset {
    fn new() -> std::io::Result<Self> {
        let attr = RulesetAttr {
            handled_access_fs: FS_ALL,
            handled_access_net: NET_BIND_TCP | NET_CONNECT_TCP,
            scoped: SCOPE_ABSTRACT_UNIX | SCOPE_SIGNAL,
        };
        // SAFETY: attr is valid for its size.
        let fd = unsafe { libc::syscall(SYS_CREATE, &attr as *const RulesetAttr, std::mem::size_of::<RulesetAttr>(), 0u32) };
        if fd < 0 {
            return Err(std::io::Error::last_os_error());
        }
        // SAFETY: fresh fd (O_CLOEXEC by the kernel).
        Ok(Self { fd: unsafe { OwnedFd::from_raw_fd(fd as i32) } })
    }

    /// Allow `access` under `p`. A path that does not exist is skipped.
    /// A link is skipped (its target gets rights only by its own rule).
    fn allow(&self, p: &Path, access: u64) -> std::io::Result<()> {
        let meta = match std::fs::symlink_metadata(p) {
            Ok(m) => m,
            Err(_) => return Ok(()),
        };
        if meta.file_type().is_symlink() {
            return Ok(());
        }
        let access = if meta.is_dir() { access } else { access & FILE_ONLY };
        let fd = open_path(p)?;
        let rule = PathBeneathAttr { allowed_access: access, parent_fd: fd.as_raw_fd() };
        // SAFETY: rule is valid; 1 = LANDLOCK_RULE_PATH_BENEATH.
        let r = unsafe { libc::syscall(SYS_ADD, self.fd.as_raw_fd(), 1u32, &rule as *const PathBeneathAttr, 0u32) };
        if r != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }
}

/// True for `.env`, `.env.<any>` (any case).
fn is_env(name: &std::ffi::OsStr) -> bool {
    crate::guard::is_env_name(&name.to_string_lossy())
}

/// True if a `.env*` file is somewhere under `dir` (no links followed).
fn has_env_below(dir: &Path, depth: usize) -> bool {
    if depth > 32 {
        return true;
    }
    let Ok(rd) = std::fs::read_dir(dir) else { return false };
    rd.flatten().any(|e| {
        let ft = e.file_type();
        if is_env(&e.file_name()) && !ft.as_ref().map(|t| t.is_dir()).unwrap_or(false) {
            return true;
        }
        let skip = ["target", "node_modules"].contains(&e.file_name().to_string_lossy().as_ref());
        ft.map(|t| t.is_dir()).unwrap_or(false) && !skip && has_env_below(&e.path(), depth + 1)
    })
}

/// Workspace rules. A folder with a `.env*` file below it gets every right
/// except read and execute of files; its other entries get full rights.
fn allow_workspace(rs: &Ruleset, dir: &Path, depth: usize) -> std::io::Result<()> {
    if !has_env_below(dir, depth) {
        return rs.allow(dir, FS_ALL);
    }
    rs.allow(dir, FS_ALL & !READ_FILE & !EXECUTE)?;
    for e in std::fs::read_dir(dir)?.flatten() {
        if is_env(&e.file_name()) {
            continue;
        }
        let ft = e.file_type()?;
        if ft.is_dir() {
            allow_workspace(rs, &e.path(), depth + 1)?;
        } else {
            rs.allow(&e.path(), FS_ALL)?;
        }
    }
    Ok(())
}

/// Build the rule set for one shell call.
pub fn build_ruleset(workspace: &Path, private_tmp: &Path) -> std::io::Result<Ruleset> {
    let rs = Ruleset::new()?;
    let rx = EXECUTE | READ_FILE | READ_DIR;
    for p in ["/usr", "/bin", "/sbin", "/lib", "/lib32", "/lib64", "/libx32"] {
        rs.allow(Path::new(p), rx)?;
    }
    for p in ["/etc", "/proc", "/sys"] {
        rs.allow(Path::new(p), READ_FILE | READ_DIR)?;
    }
    for p in ["/dev/null", "/dev/zero", "/dev/full", "/dev/random", "/dev/urandom"] {
        rs.allow(Path::new(p), READ_FILE | WRITE_FILE | TRUNCATE)?;
    }
    // Rust toolchain: read and execute only. Not the whole CARGO_HOME
    // (it can hold credentials.toml).
    for d in toolchain_dirs() {
        rs.allow(&d, rx)?;
    }
    allow_workspace(&rs, workspace, 0)?;
    rs.allow(private_tmp, FS_ALL)?;
    Ok(rs)
}

/// CARGO_HOME bin and registry, and the rustup home (if any).
pub fn toolchain_dirs() -> Vec<PathBuf> {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let cargo = std::env::var_os("CARGO_HOME").map(PathBuf::from).or_else(|| home.as_ref().map(|h| h.join(".cargo")));
    let rustup = std::env::var_os("RUSTUP_HOME").map(PathBuf::from).or_else(|| home.as_ref().map(|h| h.join(".rustup")));
    let mut v = Vec::new();
    if let Some(c) = cargo {
        v.push(c.join("bin"));
        v.push(c.join("registry"));
        v.push(c.join("git"));
    }
    if let Some(r) = rustup {
        v.push(r.join("toolchains"));
    }
    v
}

/// Environment for the shell: `env -i` plus a short allow list. No keys.
pub fn apply_env(cmd: &mut Command, workspace: &Path, private_tmp: &Path) {
    cmd.env_clear();
    cmd.env("PATH", std::env::var_os("PATH").unwrap_or_else(|| "/usr/local/bin:/usr/bin:/bin".into()));
    cmd.env("HOME", workspace);
    cmd.env("LANG", std::env::var_os("LANG").unwrap_or_else(|| "C.UTF-8".into()));
    let home = std::env::var_os("HOME").map(PathBuf::from);
    if let Some(c) = std::env::var_os("CARGO_HOME").map(PathBuf::from).or_else(|| home.as_ref().map(|h| h.join(".cargo"))) {
        cmd.env("CARGO_HOME", c);
    }
    if let Some(r) = std::env::var_os("RUSTUP_HOME").map(PathBuf::from).or_else(|| home.as_ref().map(|h| h.join(".rustup"))) {
        cmd.env("RUSTUP_HOME", r);
    }
    cmd.env("CARGO_NET_OFFLINE", "true");
    cmd.env("TMPDIR", private_tmp);
}

/// Put the sandbox and the supervisor on `cmd`. Runs in the child after fork.
pub fn apply_pre_exec(cmd: &mut Command, rs: &Ruleset, timeout: Duration) {
    let rfd = rs.fd.as_raw_fd();
    let limit_ns = timeout.as_nanos().min(i64::MAX as u128) as i64;
    // SAFETY: the closure uses only async-signal-safe syscalls, no allocation.
    unsafe {
        cmd.pre_exec(move || {
            // The supervisor stays OUTSIDE the Landlock domain. Only the
            // command child is restricted. So the command cannot signal the
            // supervisor (signal scope), and it stays the subreaper.
            if libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            // Raw clone, not glibc fork(): fork() runs atfork handlers that
            // take locks (malloc) another parent thread may have held at the
            // first fork. That can deadlock here.
            let pid = libc::syscall(libc::SYS_clone, libc::SIGCHLD as libc::c_ulong, 0usize, 0usize, 0usize, 0usize) as libc::pid_t;
            if pid < 0 {
                return Err(std::io::Error::last_os_error());
            }
            if pid == 0 {
                // The command child: restrict, then continue to exec. On an
                // error std reports it and the command does not run.
                if libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                if libc::syscall(SYS_RESTRICT, rfd, 0u32) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                return Ok(());
            }
            supervise(pid, limit_ns)
        });
    }
}

fn now_ns() -> i64 {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    // SAFETY: valid pointer.
    unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) };
    ts.tv_sec * 1_000_000_000 + ts.tv_nsec
}

/// Supervisor loop. Never returns.
fn supervise(main: libc::pid_t, limit_ns: i64) -> ! {
    // SAFETY: async-signal-safe syscalls only.
    unsafe {
        // Drop every fd, also the pipe that the parent waits on for exec.
        libc::syscall(libc::SYS_close_range, 0u32, u32::MAX, 0u32);
        let start = now_ns();
        let code: i32;
        loop {
            let mut st: libc::c_int = 0;
            let r = libc::waitpid(-1, &mut st, libc::WNOHANG);
            if r == main {
                code = if libc::WIFEXITED(st) { libc::WEXITSTATUS(st) } else { 128 + libc::WTERMSIG(st) };
                break;
            }
            if r > 0 {
                continue;
            }
            if now_ns() - start > limit_ns {
                code = TIMEOUT_EXIT;
                break;
            }
            let ts = libc::timespec { tv_sec: 0, tv_nsec: 10_000_000 };
            libc::nanosleep(&ts, std::ptr::null_mut());
        }
        kill_all_below();
        libc::_exit(code);
    }
}

/// Kill and reap every process under this one. As the subreaper, all
/// orphans under us come back to us, so repeat until none is left.
unsafe fn kill_all_below() {
    let me = libc::getpid();
    for _ in 0..10_000 {
        let n = kill_children_of(me);
        loop {
            let mut st = 0;
            let r = libc::waitpid(-1, &mut st, libc::WNOHANG);
            if r <= 0 {
                break;
            }
        }
        if n == 0 {
            let mut st = 0;
            // ECHILD: none left.
            if libc::waitpid(-1, &mut st, libc::WNOHANG) < 0 {
                return;
            }
        }
        let ts = libc::timespec { tv_sec: 0, tv_nsec: 1_000_000 };
        libc::nanosleep(&ts, std::ptr::null_mut());
    }
}

/// Send SIGKILL to each process whose parent is `me`. Returns the count.
unsafe fn kill_children_of(me: libc::pid_t) -> usize {
    let dir = libc::open(c"/proc".as_ptr(), libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC);
    if dir < 0 {
        return 0;
    }
    let mut n = 0;
    let mut buf = [0u8; 8192];
    loop {
        let got = libc::syscall(libc::SYS_getdents64, dir, buf.as_mut_ptr(), buf.len());
        if got <= 0 {
            break;
        }
        let mut off = 0usize;
        while off < got as usize {
            let reclen = u16::from_ne_bytes([buf[off + 16], buf[off + 17]]) as usize;
            let name = &buf[off + 19..off + reclen];
            let mut pid: i64 = 0;
            let mut ok = name[0] != 0;
            for &b in name {
                if b == 0 {
                    break;
                }
                if !b.is_ascii_digit() {
                    ok = false;
                    break;
                }
                pid = pid * 10 + (b - b'0') as i64;
            }
            if ok && pid > 0 && parent_of(dir, name) == Some(me) {
                libc::kill(pid as libc::pid_t, libc::SIGKILL);
                n += 1;
            }
            off += reclen;
        }
    }
    libc::close(dir);
    n
}

/// Parent pid from `/proc/<name>/stat` (field 4, after the last `)`).
unsafe fn parent_of(dir: libc::c_int, name: &[u8]) -> Option<libc::pid_t> {
    let mut path = [0u8; 64];
    let mut i = 0;
    for &b in name {
        if b == 0 || i >= 50 {
            break;
        }
        path[i] = b;
        i += 1;
    }
    for &b in b"/stat\0" {
        path[i] = b;
        i += 1;
    }
    let fd = libc::openat(dir, path.as_ptr() as *const libc::c_char, libc::O_RDONLY | libc::O_CLOEXEC);
    if fd < 0 {
        return None;
    }
    let mut buf = [0u8; 1024];
    let got = libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len());
    libc::close(fd);
    if got <= 0 {
        return None;
    }
    let s = &buf[..got as usize];
    let close = s.iter().rposition(|&b| b == b')')?;
    // ") S 1234 "
    let mut j = close + 1;
    let mut field = 0;
    let mut val: i64 = 0;
    let mut in_num = false;
    while j < s.len() {
        let b = s[j];
        if b == b' ' {
            if in_num {
                return Some(val as libc::pid_t);
            }
            field += 1;
        } else if field == 2 && b.is_ascii_digit() {
            in_num = true;
            val = val * 10 + (b - b'0') as i64;
        }
        j += 1;
    }
    None
}

/// A private tmp folder for one shell call. Removed on drop.
pub struct PrivateTmp(pub PathBuf);

impl PrivateTmp {
    pub fn new() -> std::io::Result<Self> {
        use std::os::unix::fs::DirBuilderExt;
        static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
        let p = std::env::temp_dir().join(format!("harness-shell-tmp-{}-{n}-{t}", std::process::id()));
        std::fs::DirBuilder::new().mode(0o700).create(&p)?;
        Ok(Self(p.canonicalize()?))
    }
}

impl Drop for PrivateTmp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
