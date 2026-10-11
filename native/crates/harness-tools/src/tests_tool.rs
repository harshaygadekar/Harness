//! run_tests: cargo test or pytest; soft fail if neither looks present.

use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

use harness_core::{Tool, ToolError, ToolResult};
use serde_json::{json, Value};

use crate::bound::bound_head_tail;
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

        let mut c = Command::new(cmd[0]);
        c.args(&cmd[1..]).current_dir(root);
        let out = crate::proc::run(c, Duration::from_secs(120))
            .map_err(|e| ToolError::Failed(format!("{label}: {e}")))?;
        if out.timed_out {
            return Err(ToolError::Failed(format!("{label}: timeout 120s")));
        }
        let code = out.code.unwrap_or(-1);
        let mut raw = out.stdout.render();
        if out.stderr.total() > 0 {
            raw.push_str("\n--- stderr ---\n");
            raw.push_str(&out.stderr.render());
        }
        let excerpt = failure_excerpt(&raw);
        let content = format!(
            "[{label}] exit {code}\n{}",
            bound_head_tail(&excerpt, 8 * 1024)
        );
        Ok(ToolResult {
            content,
            ok: out.success,
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
    let interesting: Vec<&str> = lines
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
    // Keep the first and last 30 matches: summary/failure names come last.
    if interesting.len() > 60 {
        let n = interesting.len();
        return format!(
            "{}\n…[{} matching lines omitted]…\n{}",
            interesting[..30].join("\n"),
            n - 60,
            interesting[n - 30..].join("\n")
        );
    }
    interesting.join("\n")
}
