//! Min coding tool pack (ADR-005). Outside core.
//! Workspace-bound paths. Shell is opt-in and runs in a Landlock sandbox.

mod bound;
mod filesystem;
pub mod guard;
mod git;
mod patch;
mod safe_fs;
pub mod sandbox;
mod search;
mod shell;
mod tests_tool;
pub mod proc;
mod workspace;

pub use guard::{deny_code, REASON_ENV_READ, REASON_GIT_WRITE, REASON_PATH_ESCAPE, REASON_SHELL_SANDBOX};
pub use filesystem::{ListFiles, ReadFile, WriteFile};
pub use git::{GitDiff, GitStatus};
pub use patch::ApplyPatch;
pub use search::Search;
pub use shell::{run_sandboxed, RunShell};
pub use tests_tool::RunTests;
pub use workspace::{Workspace, WorkspaceError};

use std::sync::Arc;

use harness_core::Tool;

/// Default pack: list/search/read/write/patch/run_tests/git_status/git_diff.
/// Shell only if `include_shell`.
pub fn coding_tools(ws: Arc<Workspace>, include_shell: bool) -> Vec<Arc<dyn Tool>> {
    let mut v: Vec<Arc<dyn Tool>> = vec![
        Arc::new(ListFiles { workspace: ws.clone() }),
        Arc::new(Search { workspace: ws.clone() }),
        Arc::new(ReadFile { workspace: ws.clone() }),
        Arc::new(WriteFile { workspace: ws.clone() }),
        Arc::new(ApplyPatch { workspace: ws.clone() }),
        Arc::new(RunTests { workspace: ws.clone() }),
        Arc::new(GitStatus { workspace: ws.clone() }),
        Arc::new(GitDiff { workspace: ws.clone() }),
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
        "run_tests",
        "git_status",
        "git_diff",
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

    #[test]
    fn coding_names_include_tests_and_git() {
        let names = coding_tool_names(false);
        assert!(names.contains(&"run_tests"));
        assert!(names.contains(&"git_status"));
        assert!(names.contains(&"git_diff"));
        assert!(!names.contains(&"run_shell"));
        assert!(coding_tool_names(true).contains(&"run_shell"));
    }

    fn tool<'a>(tools: &'a [Arc<dyn harness_core::Tool>], n: &str) -> &'a Arc<dyn harness_core::Tool> {
        tools.iter().find(|t| t.name() == n).unwrap()
    }

    #[test]
    fn shell_400kb_returns_quickly_with_tail() {
        let ws = tmp_ws();
        let tools = coding_tools(ws, true);
        let t = std::time::Instant::now();
        let r = tool(&tools, "run_shell")
            .call(&json!({"command": "for i in $(seq 1 6000); do echo \"noise $i ...........................................................\"; done; echo TAIL_FAILURE_NAME"}))
            .unwrap();
        assert!(t.elapsed() < std::time::Duration::from_secs(10), "took {:?}", t.elapsed());
        assert!(r.content.contains("…[omitted "), "no omitted marker");
        assert!(r.content.contains("TAIL_FAILURE_NAME"), "tail missing");
        assert!(r.ok);
    }

    #[test]
    fn run_tests_large_output_shows_tail_failure() {
        let ws = tmp_ws();
        let root = ws.root().to_path_buf();
        fs::write(root.join("Cargo.toml"), "[package]\nname = \"bigout\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\n").unwrap();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(
            root.join("src/lib.rs"),
            "#[test] fn noisy() { for i in 0..6000 { println!(\"noise {i} ............................................................\"); } }\n#[test] fn zz_named_failure() { assert_eq!(1, 2, \"named failure message\"); }\n",
        )
        .unwrap();
        let tools = coding_tools(ws, false);
        let r = tool(&tools, "run_tests").call(&json!({})).unwrap();
        assert!(!r.ok);
        assert!(r.content.contains("zz_named_failure"), "{}", &r.content[..r.content.len().min(2000)]);
    }

    #[test]
    fn fake_key_in_tool_output_is_scrubbed_before_model() {
        use harness_core::{AllowList, Message, ModelClient, ModelResponse, Role, RunConfig, RunEvent, Runner, ToolCall, ToolSpec};
        struct M(usize);
        impl ModelClient for M {
            fn complete(&mut self, _m: &[Message], _t: &[ToolSpec]) -> ModelResponse {
                self.0 += 1;
                let tool_calls = if self.0 == 1 {
                    vec![ToolCall { id: "c1".into(), name: "run_shell".into(), arguments: json!({"command": "echo key=sk-FAKEKEY0123456789abcdef"}), extra_content: None }]
                } else { vec![] };
                ModelResponse { message: Message { role: Role::Assistant, content: Some("ok".into()), tool_calls, tool_call_id: None, name: None } }
            }
        }
        let ws = tmp_ws();
        let mut runner = Runner::new(Arc::new(AllowList::new(["run_shell"])));
        for t in coding_tools(ws, true) { runner.register_tool(t); }
        let res = runner.run(&mut M(0), "x", RunConfig { max_steps: 4 });
        let seen: Vec<String> = res.events.iter().filter_map(|e| match e { RunEvent::ToolResult { content, .. } => Some(content.clone()), _ => None }).collect();
        assert_eq!(seen.len(), 1);
        assert!(seen[0].contains("[REDACTED]"), "{}", seen[0]);
        assert!(!seen[0].contains("FAKEKEY"));
    }
}
