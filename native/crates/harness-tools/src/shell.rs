use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

use harness_core::{Tool, ToolError, ToolResult};
use serde_json::{json, Value};

use crate::bound::bound_excerpt;
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
        // Bound output; 30s soft timeout via kill is OS-specific — use try_wait loop.
        let mut child = Command::new("bash")
            .arg("-lc")
            .arg(cmd)
            .current_dir(self.workspace.root())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| ToolError::Failed(e.to_string()))?;

        let start = std::time::Instant::now();
        loop {
            match child.try_wait() {
                Ok(Some(_)) => break,
                Ok(None) if start.elapsed() > Duration::from_secs(30) => {
                    let _ = child.kill();
                    return Err(ToolError::Failed("timeout 30s".into()));
                }
                Ok(None) => std::thread::sleep(Duration::from_millis(50)),
                Err(e) => return Err(ToolError::Failed(e.to_string())),
            }
        }
        let out = child
            .wait_with_output()
            .map_err(|e| ToolError::Failed(e.to_string()))?;
        let mut text = String::new();
        text.push_str(&String::from_utf8_lossy(&out.stdout));
        if !out.stderr.is_empty() {
            text.push_str("\n--- stderr ---\n");
            text.push_str(&String::from_utf8_lossy(&out.stderr));
        }
        text = bound_excerpt(&text, 16 * 1024);
        text.push_str(&format!("\n[exit {}]", out.status.code().unwrap_or(-1)));
        Ok(ToolResult {
            content: text,
            ok: out.status.success(),
        })
    }
}
