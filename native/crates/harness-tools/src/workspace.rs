use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("path escapes workspace: {0}")]
    Escape(String),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub fn new(root: impl AsRef<Path>) -> Result<Self, WorkspaceError> {
        let root = root.as_ref().canonicalize().map_err(WorkspaceError::Io)?;
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Resolve `rel` under root. Rejects `..` escape (Jessie workspace bind).
    pub fn resolve(&self, rel: &str) -> Result<PathBuf, WorkspaceError> {
        let candidate = if Path::new(rel).is_absolute() {
            PathBuf::from(rel)
        } else {
            self.root.join(rel)
        };
        // Normalize without requiring the path to exist yet.
        let mut out = PathBuf::new();
        for comp in candidate.components() {
            match comp {
                std::path::Component::ParentDir => {
                    if !out.pop() {
                        return Err(WorkspaceError::Escape(rel.to_string()));
                    }
                }
                std::path::Component::CurDir => {}
                std::path::Component::RootDir => {
                    out.push(comp);
                }
                other => out.push(other),
            }
        }
        // Resolve every link on the path with symlink_metadata/read_link,
        // also dangling links and chains. The link text is resolved against
        // the link's parent folder. Parts that do not exist yet are kept.
        let joined = resolve_links(&out).ok_or_else(|| WorkspaceError::Escape(rel.to_string()))?;

        // Use strip_prefix, not starts_with (Jessie: /tmp/ws vs /tmp/ws_evil).
        if joined.strip_prefix(&self.root).is_err() {
            return Err(WorkspaceError::Escape(rel.to_string()));
        }
        Ok(joined)
    }
}

/// Resolve all links in an absolute, normalized path, without needing the
/// path to exist. Returns None on a link loop (more than 40 links).
fn resolve_links(path: &Path) -> Option<PathBuf> {
    use std::path::Component;
    let mut todo: Vec<std::ffi::OsString> = Vec::new();
    let push_rev = |todo: &mut Vec<std::ffi::OsString>, p: &Path| {
        for c in p.components().rev() {
            todo.push(c.as_os_str().to_os_string());
        }
    };
    push_rev(&mut todo, path);
    let mut out = PathBuf::new();
    let mut links = 0;
    while let Some(c) = todo.pop() {
        match Path::new(&c).components().next() {
            Some(Component::RootDir) => out = PathBuf::from("/"),
            Some(Component::CurDir) | None => {}
            Some(Component::ParentDir) => {
                out.pop();
            }
            Some(_) => {
                out.push(&c);
                let is_link = std::fs::symlink_metadata(&out)
                    .map(|m| m.file_type().is_symlink())
                    .unwrap_or(false);
                if is_link {
                    links += 1;
                    if links > 40 {
                        return None;
                    }
                    let target = std::fs::read_link(&out).ok()?;
                    out.pop(); // the link's parent folder
                    push_rev(&mut todo, &target); // absolute target resets via RootDir
                }
            }
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn blocks_parent_escape() {
        let dir = tempfile_dir();
        let ws = Workspace::new(&dir).unwrap();
        assert!(ws.resolve("../etc/passwd").is_err());
        assert!(ws.resolve("ok.txt").is_ok());
    }

    #[test]
    fn blocks_prefix_sibling_escape() {
        let base = std::env::temp_dir().join(format!("harness-pfx-{}", std::process::id()));
        let root = base.join("ws");
        let evil = base.join("ws_evil");
        fs::create_dir_all(&root).unwrap();
        fs::create_dir_all(&evil).unwrap();
        fs::write(evil.join("secret.txt"), b"nope").unwrap();
        let ws = Workspace::new(&root).unwrap();
        // Absolute path into sibling that shares a string prefix must fail.
        assert!(ws.resolve(evil.join("secret.txt").to_str().unwrap()).is_err());
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn new_nested_path_keeps_middle_parts() {
        let dir = std::env::temp_dir().join(format!("harness-ws-nested-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let ws = Workspace::new(&dir).unwrap();
        assert_eq!(ws.resolve("a/b/c").unwrap(), ws.root().join("a/b/c"));
        fs::create_dir_all(dir.join("a")).unwrap();
        assert_eq!(ws.resolve("a/b/c").unwrap(), ws.root().join("a/b/c"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn new_path_through_outside_link_denied() {
        let base = std::env::temp_dir().join(format!("harness-ws-link-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let root = base.join("ws");
        let out = base.join("out");
        fs::create_dir_all(&root).unwrap();
        fs::create_dir_all(&out).unwrap();
        std::os::unix::fs::symlink(&out, root.join("l")).unwrap();
        let ws = Workspace::new(&root).unwrap();
        for p in ["l/x", "l/new/deep/x", "l/new/../x"] {
            assert!(matches!(ws.resolve(p), Err(WorkspaceError::Escape(_))), "{p}");
        }
        let _ = fs::remove_dir_all(&base);
    }

    fn tempfile_dir() -> PathBuf {
        let d = std::env::temp_dir().join(format!("harness-ws-{}", std::process::id()));
        let _ = fs::create_dir_all(&d);
        d
    }
}
