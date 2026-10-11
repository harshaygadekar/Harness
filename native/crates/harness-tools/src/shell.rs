use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

use harness_core::{Tool, ToolError, ToolResult};
use serde_json::{json, Value};

use crate::bound::bound_head_tail;
use crate::workspace::Workspace;

/// Opt-in shell. Soft deny list. Not an OS sandbox (Jessie).
pub struct RunShell {
    pub workspace: Arc<Workspace>,
}

const DENY: &[&str] = &[
    "rm -rf /",
    "mkfs",
    "dd if=",
    ":(){",
    "curl | sh",
    "wget | sh",
    "chmod 777",
];

impl Tool for RunShell {
    fn name(&self) -> &str {
        "run_shell"
    }
    fn description(&self) -> &str {
        "Run a shell command in the workspace (opt-in; not a sandbox)."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "command": {"type": "string"}
            },
            "required": ["command"]
        })
    }
    fn call(&self, args: &Value) -> Result<ToolResult, ToolError> {
        let cmd = args
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::Failed("command required".into()))?;
        let lower = cmd.to_lowercase();
        for d in DENY {
            if lower.contains(d) {
                return Err(ToolError::Failed(format!("soft-deny matched: {d}")));
            }
        }
        let mut c = Command::new("bash");
        c.arg("-lc").arg(cmd).current_dir(self.workspace.root());
        let out = crate::proc::run(c, Duration::from_secs(30))
            .map_err(|e| ToolError::Failed(e.to_string()))?;
        if out.timed_out {
            return Err(ToolError::Failed("timeout 30s".into()));
        }
        // Bound each stream on its own (16 KiB total) so the stdout tail
        // (e.g. cargo's final failure list) is not pushed out by stderr.
        let err = out.stderr.render();
        let err_budget = if err.is_empty() { 0 } else { err.len().min(4 * 1024) };
        let mut text = bound_head_tail(&out.stdout.render(), 16 * 1024 - err_budget);
        if !err.is_empty() {
            text.push_str("\n--- stderr ---\n");
            text.push_str(&bound_head_tail(&err, err_budget));
        }
        text.push_str(&format!("\n[exit {}]", out.code.unwrap_or(-1)));
        Ok(ToolResult {
            content: text,
            ok: out.success,
        })
    }
}
