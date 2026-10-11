//! Thin read-only git_status / git_diff. Workspace cwd only. Not a sandbox.

use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

use harness_core::{Tool, ToolError, ToolResult};
use serde_json::{json, Value};

use crate::bound::bound_head_tail;
use crate::workspace::Workspace;

fn git_cmd(ws: &Workspace, args: &[&str]) -> Result<(i32, String), ToolError> {
    let mut c = Command::new("git");
    c.args(args).current_dir(ws.root());
    let out = crate::proc::run(c, Duration::from_secs(15))
        .map_err(|e| ToolError::Failed(format!("git: {e}")))?;
    if out.timed_out {
        return Err(ToolError::Failed("git: timeout 15s".into()));
    }
    let mut text = out.stdout.render();
    if out.stderr.total() > 0 {
        if !text.is_empty() {
            text.push('\n');
        }
        text.push_str(&out.stderr.render());
    }
    Ok((out.code.unwrap_or(-1), text))
}

pub struct GitStatus {
    pub workspace: Arc<Workspace>,
}

impl Tool for GitStatus {
    fn name(&self) -> &str {
        "git_status"
    }
    fn description(&self) -> &str {
        "Read-only git status --short in the workspace."
    }
    fn parameters(&self) -> Value {
        json!({"type": "object", "properties": {}})
    }
    fn call(&self, _args: &Value) -> Result<ToolResult, ToolError> {
        let (code, text) = git_cmd(&self.workspace, &["status", "--short"])?;
        let content = if text.trim().is_empty() {
            format!("(clean) [exit {code}]")
        } else {
            format!("{}\n[exit {code}]", bound_head_tail(&text, 16 * 1024))
        };
        Ok(ToolResult {
            content,
            ok: code == 0,
        })
    }
}

pub struct GitDiff {
    pub workspace: Arc<Workspace>,
}

impl Tool for GitDiff {
    fn name(&self) -> &str {
        "git_diff"
    }
    fn description(&self) -> &str {
        "Read-only git diff (unstaged) in the workspace."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "optional pathspec"}
            }
        })
    }
    fn call(&self, args: &Value) -> Result<ToolResult, ToolError> {
        let mut argv = vec!["diff", "--"];
        let path_owned: String;
        if let Some(p) = args.get("path").and_then(|v| v.as_str()) {
            // Resolve under workspace to block escape; pass relative pathspec to git.
            let resolved = self
                .workspace
                .resolve(p)
                .map_err(|e| ToolError::Failed(e.to_string()))?;
            path_owned = resolved
                .strip_prefix(self.workspace.root())
                .unwrap_or(&resolved)
                .to_string_lossy()
                .into_owned();
            argv.push(path_owned.as_str());
        }
        let (code, text) = git_cmd(&self.workspace, &argv)?;
        let content = if text.trim().is_empty() {
            format!("(no diff) [exit {code}]")
        } else {
            format!("{}\n[exit {code}]", bound_head_tail(&text, 16 * 1024))
        };
        Ok(ToolResult {
            content,
            ok: code == 0,
        })
    }
}
