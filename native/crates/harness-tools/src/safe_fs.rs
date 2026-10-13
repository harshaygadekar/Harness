//! Open files one path part at a time from the workspace root, with
//! openat + O_NOFOLLOW. Outside core.
//!
//! The caller first resolves the path (all links expanded, inside the
//! workspace). Here no part may be a link. If a link was put in after the
//! check, the open fails (ELOOP/ENOTDIR) and we deny with path_escape.
//! This closes the check-then-open gap for links below the root.

use std::ffi::CString;
use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::{FromRawFd, OwnedFd, AsRawFd};
use std::os::unix::ffi::OsStrExt;
use std::path::{Component, Path};

use harness_core::ToolError;

use crate::guard::{deny, REASON_PATH_ESCAPE};

fn cstr(s: &std::ffi::OsStr) -> Result<CString, ToolError> {
    CString::new(s.as_bytes()).map_err(|_| ToolError::Failed("bad path".into()))
}

fn open_at(dir: Option<&OwnedFd>, name: &CString, flags: i32, mode: u32) -> std::io::Result<OwnedFd> {
    let base = dir.map(|d| d.as_raw_fd()).unwrap_or(libc::AT_FDCWD);
    // SAFETY: valid C string; fd is owned on success.
    let fd = unsafe { libc::openat(base, name.as_ptr(), flags | libc::O_CLOEXEC, mode) };
    if fd < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(unsafe { OwnedFd::from_raw_fd(fd) })
    }
}

fn map_err(rel: &str, e: std::io::Error) -> ToolError {
    match e.raw_os_error() {
        Some(libc::ELOOP) | Some(libc::ENOTDIR) => deny(REASON_PATH_ESCAPE, rel),
        _ => ToolError::Failed(e.to_string()),
    }
}

/// Open the parent folder of `real` (a resolved path under `root`) part by
/// part. Make missing folders when `create` is true. Return (dir fd, name).
fn open_parent(root: &Path, real: &Path, rel: &str, create: bool) -> Result<(OwnedFd, CString), ToolError> {
    let inner = real.strip_prefix(root).map_err(|_| deny(REASON_PATH_ESCAPE, rel))?;
    let parts: Vec<_> = inner
        .components()
        .map(|c| match c {
            Component::Normal(s) => Ok(s),
            _ => Err(deny(REASON_PATH_ESCAPE, rel)),
        })
        .collect::<Result<_, _>>()?;
    let (last, dirs) = parts.split_last().ok_or_else(|| ToolError::Failed("no file name".into()))?;
    let dflags = libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW;
    let mut cur = open_at(None, &cstr(root.as_os_str())?, dflags, 0).map_err(|e| map_err(rel, e))?;
    for d in dirs {
        let name = cstr(d)?;
        let next = match open_at(Some(&cur), &name, dflags, 0) {
            Err(e) if create && e.raw_os_error() == Some(libc::ENOENT) => {
                // SAFETY: valid fd and C string.
                if unsafe { libc::mkdirat(cur.as_raw_fd(), name.as_ptr(), 0o755) } != 0 {
                    let e = std::io::Error::last_os_error();
                    if e.raw_os_error() != Some(libc::EEXIST) {
                        return Err(map_err(rel, e));
                    }
                }
                open_at(Some(&cur), &name, dflags, 0)
            }
            r => r,
        }
        .map_err(|e| map_err(rel, e))?;
        cur = next;
    }
    Ok((cur, cstr(last)?))
}

/// Read a file. No part of the path may be a link.
pub fn read(root: &Path, real: &Path, rel: &str) -> Result<String, ToolError> {
    let (dir, name) = open_parent(root, real, rel, false)?;
    let fd = open_at(Some(&dir), &name, libc::O_RDONLY | libc::O_NOFOLLOW, 0).map_err(|e| map_err(rel, e))?;
    let mut s = String::new();
    File::from(fd).read_to_string(&mut s).map_err(|e| ToolError::Failed(e.to_string()))?;
    Ok(s)
}

/// Write (create or replace) a file. No part of the path may be a link.
pub fn write(root: &Path, real: &Path, rel: &str, data: &[u8], create: bool) -> Result<(), ToolError> {
    let (dir, name) = open_parent(root, real, rel, create)?;
    let mut flags = libc::O_WRONLY | libc::O_TRUNC | libc::O_NOFOLLOW;
    if create {
        flags |= libc::O_CREAT;
    }
    let fd = open_at(Some(&dir), &name, flags, 0o644).map_err(|e| map_err(rel, e))?;
    File::from(fd).write_all(data).map_err(|e| ToolError::Failed(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::symlink;

    /// Check passes, then a folder is swapped for an outside link before the
    /// open. The open must fail with path_escape and write nothing outside.
    #[test]
    fn swap_after_check_is_denied() {
        let base = std::env::temp_dir().join(format!("harness-safefs-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let (root, out) = (base.join("ws"), base.join("out"));
        fs::create_dir_all(root.join("a")).unwrap();
        fs::create_dir_all(&out).unwrap();
        let ws = crate::Workspace::new(&root).unwrap();
        let real = ws.resolve("a/b.txt").unwrap();
        fs::remove_dir(root.join("a")).unwrap();
        symlink(&out, root.join("a")).unwrap();
        let e = write(ws.root(), &real, "a/b.txt", b"x", true).unwrap_err();
        assert_eq!(crate::guard::deny_code(&e.to_string()), Some("path_escape"));
        let e = read(ws.root(), &real, "a/b.txt").unwrap_err();
        assert_eq!(crate::guard::deny_code(&e.to_string()), Some("path_escape"));
        assert!(!out.join("b.txt").exists());
        // Final part swapped for a link.
        fs::remove_file(root.join("a")).unwrap();
        fs::create_dir_all(root.join("a")).unwrap();
        symlink(out.join("f.txt"), root.join("a/b.txt")).unwrap();
        let e = write(ws.root(), &real, "a/b.txt", b"x", true).unwrap_err();
        assert_eq!(crate::guard::deny_code(&e.to_string()), Some("path_escape"));
        assert!(!out.join("f.txt").exists());
        let _ = fs::remove_dir_all(&base);
    }
}
