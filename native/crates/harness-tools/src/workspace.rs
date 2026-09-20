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

        if !joined.starts_with(&self.root) {
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

    fn tempfile_dir() -> PathBuf {
        let d = std::env::temp_dir().join(format!("harness-ws-{}", std::process::id()));
        let _ = fs::create_dir_all(&d);
        d
    }
}
