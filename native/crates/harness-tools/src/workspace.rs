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
        // If absolute path, must still be under root.
        let canon_parent = out
            .parent()
            .map(|p| {
                if p.exists() {
                    p.canonicalize().unwrap_or_else(|_| p.to_path_buf())
                } else {
                    // walk up to existing ancestor
                    let mut a = p.to_path_buf();
                    while !a.exists() {
                        if !a.pop() {
                            break;
                        }
                    }
                    a.canonicalize().unwrap_or(a)
                }
            })
            .unwrap_or_else(|| self.root.clone());

        let joined = if out.exists() {
            out.canonicalize().map_err(WorkspaceError::Io)?
        } else {
            let name = out
                .file_name()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."));
            canon_parent.join(name)
        };

        // Use strip_prefix, not starts_with (Jessie: /tmp/ws vs /tmp/ws_evil).
        if joined.strip_prefix(&self.root).is_err() {
            return Err(WorkspaceError::Escape(rel.to_string()));
        }
        Ok(joined)
    }
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

    fn tempfile_dir() -> PathBuf {
        let d = std::env::temp_dir().join(format!("harness-ws-{}", std::process::id()));
        let _ = fs::create_dir_all(&d);
        d
    }
}
