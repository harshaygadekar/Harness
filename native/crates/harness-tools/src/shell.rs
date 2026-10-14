use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

use harness_core::{Tool, ToolError, ToolResult};
use serde_json::{json, Value};

use crate::bound::bound_head_tail;
use crate::workspace::Workspace;

/// Opt-in shell. The boundary is the Landlock sandbox (see `sandbox.rs` and
/// docs/security/shell-sandbox.md). The deny list below is only a guard
/// against mistakes. It is not a boundary.
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
        "Run a shell command in the workspace (opt-in; Landlock sandbox, no network)."
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
        // Fail closed: no full Landlock ABI 6, no shell. Never unsandboxed.
        if crate::sandbox::status() != crate::sandbox::Status::Landlock {
            return Err(crate::guard::deny(crate::guard::REASON_SHELL_SANDBOX, "run_shell"));
        }
        let timeout = Duration::from_secs(30);
        let out = run_sandboxed(self.workspace.root(), cmd, timeout)
            .map_err(|e| crate::guard::deny_msg(crate::guard::REASON_SHELL_SANDBOX, &e.to_string()))?;
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

/// Run `bash -c cmd` in the sandbox. The supervisor enforces `timeout` and
/// kills every process under it. The parent waits a little longer as a backstop.
pub fn run_sandboxed(root: &std::path::Path, cmd: &str, timeout: Duration) -> std::io::Result<crate::proc::Outcome> {
    let tmp = crate::sandbox::PrivateTmp::new()?;
    let rs = crate::sandbox::build_ruleset(root, &tmp.0)?;
    let mut c = Command::new("/bin/bash");
    c.arg("-c").arg(cmd).current_dir(root);
    crate::sandbox::apply_env(&mut c, root, &tmp.0);
    crate::sandbox::apply_pre_exec(&mut c, &rs, timeout);
    let start = std::time::Instant::now();
    let mut out = crate::proc::run(c, timeout + Duration::from_secs(10))?;
    drop(rs);
    if out.code == Some(crate::sandbox::TIMEOUT_EXIT) && start.elapsed() >= timeout {
        out.timed_out = true;
        out.success = false;
    }
    Ok(out)
}
