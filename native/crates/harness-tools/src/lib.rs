//! Min coding tool pack (ADR-005). Outside core.
//! Workspace-bound paths. Shell is opt-in. Not an OS sandbox.

mod filesystem;
mod patch;
mod search;
mod shell;
mod workspace;

pub use filesystem::{ListFiles, ReadFile, WriteFile};
pub use patch::ApplyPatch;
pub use search::Search;
pub use shell::RunShell;
pub use workspace::{Workspace, WorkspaceError};

use std::sync::Arc;

use harness_core::Tool;

/// Default pack: list/search/read/write/patch. Shell only if `include_shell`.
pub fn coding_tools(ws: Arc<Workspace>, include_shell: bool) -> Vec<Arc<dyn Tool>> {
    let mut v: Vec<Arc<dyn Tool>> = vec![
        Arc::new(ListFiles { workspace: ws.clone() }),
        Arc::new(Search { workspace: ws.clone() }),
        Arc::new(ReadFile { workspace: ws.clone() }),
        Arc::new(WriteFile { workspace: ws.clone() }),
        Arc::new(ApplyPatch { workspace: ws.clone() }),
    ];
    if include_shell {
        v.push(Arc::new(RunShell { workspace: ws }));
    }
    v
}

pub fn coding_tool_names(include_shell: bool) -> Vec<&'static str> {
    let mut n = vec![
        "list_files",
        "search",
        "read_file",
        "write_file",
        "apply_patch",
    ];
    if include_shell {
        n.push("run_shell");
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;

    fn tmp_ws() -> Arc<Workspace> {
        let d = std::env::temp_dir().join(format!(
            "harness-tools-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&d).unwrap();
        fs::write(d.join("a.txt"), "hello world\n").unwrap();
        Arc::new(Workspace::new(&d).unwrap())
    }

    #[test]
    fn read_write_search_patch() {
        let ws = tmp_ws();
        let tools = coding_tools(ws.clone(), false);
        let read = tools.iter().find(|t| t.name() == "read_file").unwrap();
        let r = read.call(&json!({"path":"a.txt"})).unwrap();
        assert!(r.content.contains("hello"));

        let search = tools.iter().find(|t| t.name() == "search").unwrap();
        let s = search.call(&json!({"pattern":"hello"})).unwrap();
        assert!(s.content.contains("a.txt"));

        let patch = tools.iter().find(|t| t.name() == "apply_patch").unwrap();
        patch
            .call(&json!({
                "path":"a.txt",
                "old_string":"hello world",
                "new_string":"hello harness"
            }))
            .unwrap();
        let r2 = read.call(&json!({"path":"a.txt"})).unwrap();
        assert!(r2.content.contains("harness"));
    }

    #[test]
    fn escape_blocked() {
        let ws = tmp_ws();
        let read = ReadFile {
            workspace: ws,
        };
        let err = read.call(&json!({"path":"../etc/passwd"}));
        assert!(err.is_err());
    }
}
