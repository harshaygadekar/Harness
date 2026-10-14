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

/// Read a file of at most `max` bytes. No part of the path may be a link.
/// The size check uses the open fd (fstat), not the path.
pub fn read_limited(root: &Path, real: &Path, rel: &str, max: u64) -> Result<String, ToolError> {
    let (dir, name) = open_parent(root, real, rel, false)?;
    read_child(&dir, &name, max).map_err(|e| map_err(rel, e))?
        .ok_or_else(|| ToolError::Failed(format!("file too large (>{} KiB)", max / 1024)))
}

/// Open a folder part by part (the root itself for an empty path).
pub fn open_dir(root: &Path, real: &Path, rel: &str) -> Result<OwnedFd, ToolError> {
    let inner = real.strip_prefix(root).map_err(|_| deny(REASON_PATH_ESCAPE, rel))?;
    let dflags = libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW;
    let mut cur = open_at(None, &cstr(root.as_os_str())?, dflags, 0).map_err(|e| map_err(rel, e))?;
    for c in inner.components() {
        let Component::Normal(d) = c else { return Err(deny(REASON_PATH_ESCAPE, rel)) };
        cur = open_at(Some(&cur), &cstr(d)?, dflags, 0).map_err(|e| map_err(rel, e))?;
    }
    Ok(cur)
}

/// Open a child folder of `dir` with O_NOFOLLOW.
pub fn open_child_dir(dir: &OwnedFd, name: &std::ffi::OsStr) -> std::io::Result<OwnedFd> {
    let n = CString::new(name.as_bytes()).map_err(|_| std::io::Error::other("bad name"))?;
    open_at(Some(dir), &n, libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW, 0)
}

/// Read a child file of `dir` with O_NOFOLLOW. None if larger than `max`.
pub fn read_child(dir: &OwnedFd, name: &CString, max: u64) -> std::io::Result<Option<String>> {
    let fd = open_at(Some(dir), name, libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_NONBLOCK, 0)?;
    let f = File::from(fd);
    let meta = f.metadata()?;
    if !meta.is_file() {
        return Err(std::io::Error::other("not a regular file"));
    }
    if meta.len() > max {
        return Ok(None);
    }
    let mut s = String::new();
    f.take(max + 1).read_to_string(&mut s)?;
    Ok(Some(s))
}

/// Same as [`read_child`], with an OS name.
pub fn read_child_os(dir: &OwnedFd, name: &std::ffi::OsStr, max: u64) -> std::io::Result<Option<String>> {
    let n = CString::new(name.as_bytes()).map_err(|_| std::io::Error::other("bad name"))?;
    read_child(dir, &n, max)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Dir,
    File,
    Link,
    Other,
}

/// List the entries of an open folder (no `.` and `..`). Links are not followed.
pub fn list_dir(dir: &OwnedFd) -> std::io::Result<Vec<(std::ffi::OsString, Kind)>> {
    // SAFETY: dup gives a new fd that fdopendir owns; closedir frees it.
    let d = unsafe { libc::dup(dir.as_raw_fd()) };
    if d < 0 {
        return Err(std::io::Error::last_os_error());
    }
    let dp = unsafe { libc::fdopendir(d) };
    if dp.is_null() {
        unsafe { libc::close(d) };
        return Err(std::io::Error::last_os_error());
    }
    unsafe { libc::rewinddir(dp) };
    let mut v = Vec::new();
    loop {
        let ent = unsafe { libc::readdir(dp) };
        if ent.is_null() {
            break;
        }
        let name = unsafe { std::ffi::CStr::from_ptr((*ent).d_name.as_ptr()) };
        let b = name.to_bytes();
        if b == b"." || b == b".." {
            continue;
        }
        let mut kind = match unsafe { (*ent).d_type } {
            libc::DT_DIR => Kind::Dir,
            libc::DT_REG => Kind::File,
            libc::DT_LNK => Kind::Link,
            libc::DT_UNKNOWN => Kind::Other,
            _ => Kind::Other,
        };
        if unsafe { (*ent).d_type } == libc::DT_UNKNOWN {
            let mut st: libc::stat = unsafe { std::mem::zeroed() };
            if unsafe { libc::fstatat(dir.as_raw_fd(), name.as_ptr(), &mut st, libc::AT_SYMLINK_NOFOLLOW) } == 0 {
                kind = match st.st_mode & libc::S_IFMT {
                    libc::S_IFDIR => Kind::Dir,
                    libc::S_IFREG => Kind::File,
                    libc::S_IFLNK => Kind::Link,
                    _ => Kind::Other,
                };
            }
        }
        v.push((std::ffi::OsStr::from_bytes(b).to_os_string(), kind));
    }
    unsafe { libc::closedir(dp) };
    Ok(v)
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

    /// Read side (read_file, list_files, search): openat + O_NOFOLLOW.
    /// A folder swapped for an outside link after the check is denied, and
    /// a final-part link is never followed.
    #[test]
    fn read_side_swap_after_check_is_denied() {
        use harness_core::Tool;
        let base = std::env::temp_dir().join(format!("harness-safefs-r-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let (root, out) = (base.join("ws"), base.join("out"));
        fs::create_dir_all(root.join("a")).unwrap();
        fs::create_dir_all(&out).unwrap();
        fs::write(root.join("a/b.txt"), "inside").unwrap();
        fs::write(out.join("b.txt"), "OUTSIDE-SECRET").unwrap();
        let ws = crate::Workspace::new(&root).unwrap();
        let real_file = ws.resolve("a/b.txt").unwrap();
        let real_dir = ws.resolve("a").unwrap();
        assert_eq!(read_limited(ws.root(), &real_file, "a/b.txt", 1024).unwrap(), "inside");
        assert_eq!(list_dir(&open_dir(ws.root(), &real_dir, "a").unwrap()).unwrap().len(), 1);
        // Swap the folder for a link to the outside folder.
        fs::rename(root.join("a"), base.join("a_old")).unwrap();
        symlink(&out, root.join("a")).unwrap();
        let e = read_limited(ws.root(), &real_file, "a/b.txt", 1024).unwrap_err();
        assert_eq!(crate::guard::deny_code(&e.to_string()), Some("path_escape"));
        let e = open_dir(ws.root(), &real_dir, "a").unwrap_err();
        assert_eq!(crate::guard::deny_code(&e.to_string()), Some("path_escape"));
        // Final part is a link: O_NOFOLLOW refuses it.
        let root_fd = open_dir(ws.root(), ws.root(), ".").unwrap();
        assert!(read_child_os(&root_fd, std::ffi::OsStr::new("a"), 1024).is_err());
        // Tools: search walks with fds and never reads the outside file.
        let ws = std::sync::Arc::new(ws);
        let s = crate::Search { workspace: ws.clone() }.call(&serde_json::json!({"pattern": "OUTSIDE"})).unwrap();
        assert!(!s.content.contains("OUTSIDE-SECRET"), "{}", s.content);
        let _ = fs::remove_dir_all(&base);
    }
}
