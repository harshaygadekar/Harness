//! run_tests: cargo test or pytest; soft fail if neither looks present.

use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

use harness_core::{Tool, ToolError, ToolResult};
use serde_json::{json, Value};

use crate::bound::bound_excerpt;
use crate::workspace::Workspace;

pub struct RunTests {
    pub workspace: Arc<Workspace>,
}

impl Tool for RunTests {
    fn name(&self) -> &str {
        "run_tests"
    }
    fn description(&self) -> &str {
        "Run project tests: cargo test if Cargo.toml, else python -m pytest if pytest-ish. Returns exit code + failure excerpt."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {}
        })
    }
    fn call(&self, _args: &Value) -> Result<ToolResult, ToolError> {
        let root = self.workspace.root();
        let (cmd, label) = if root.join("Cargo.toml").is_file() {
            (vec!["cargo", "test", "--", "--nocapture"], "cargo test")
        } else if looks_pytest(root) {
            (vec!["python", "-m", "pytest", "-q"], "python -m pytest")
        } else {
            return Ok(ToolResult {
                content: "soft-fail: no Cargo.toml and no pytest-ish layout found".into(),
                ok: false,
            });
        };

        let mut child = Command::new(cmd[0])
            .args(&cmd[1..])
            .current_dir(root)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| ToolError::Failed(format!("{label}: {e}")))?;

        let start = std::time::Instant::now();
        loop {
            match child.try_wait() {
                Ok(Some(_)) => break,
                Ok(None) if start.elapsed() > Duration::from_secs(120) => {
                    let _ = child.kill();
                    return Err(ToolError::Failed(format!("{label}: timeout 120s")));
                }
                Ok(None) => std::thread::sleep(Duration::from_millis(50)),
                Err(e) => return Err(ToolError::Failed(e.to_string())),
            }
        }
        let out = child
            .wait_with_output()
            .map_err(|e| ToolError::Failed(e.to_string()))?;
        let code = out.status.code().unwrap_or(-1);
        let mut raw = String::from_utf8_lossy(&out.stdout).into_owned();
        if !out.stderr.is_empty() {
            raw.push_str("\n--- stderr ---\n");
            raw.push_str(&String::from_utf8_lossy(&out.stderr));
        }
        let excerpt = failure_excerpt(&raw);
        let content = format!(
            "[{label}] exit {code}\n{}",
            bound_excerpt(&excerpt, 8 * 1024)
        );
        Ok(ToolResult {
            content,
            ok: out.status.success(),
        })
    }
}

fn looks_pytest(root: &std::path::Path) -> bool {
    root.join("pytest.ini").is_file()
        || root.join("pyproject.toml").is_file()
        || root.join("tests").is_dir()
        || root.join("test").is_dir()
}

/// Prefer FAIL/error lines; else last chunk.
fn failure_excerpt(raw: &str) -> String {
    let lines: Vec<&str> = raw.lines().collect();
    let mut interesting: Vec<&str> = lines
        .iter()
        .copied()
        .filter(|l| {
            let u = l.to_uppercase();
            u.contains("FAIL")
                || u.contains("ERROR")
                || u.contains("ASSERT")
                || u.contains("PANICKED")
                || l.contains("--> ")
                || l.trim_start().starts_with("left:")
                || l.trim_start().starts_with("right:")
        })
        .collect();
    if interesting.is_empty() {
        let start = lines.len().saturating_sub(40);
        return lines[start..].join("\n");
    }
    interesting.truncate(60);
    interesting.join("\n")
}
