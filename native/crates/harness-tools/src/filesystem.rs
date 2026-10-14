use std::sync::Arc;

use harness_core::{Tool, ToolError, ToolResult};
use serde_json::{json, Value};

use crate::workspace::Workspace;

pub struct ListFiles {
    pub workspace: Arc<Workspace>,
}

impl Tool for ListFiles {
    fn name(&self) -> &str {
        "list_files"
    }
    fn description(&self) -> &str {
        "List files under a workspace-relative path (non-recursive, max 200)."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Relative directory (default .)" }
            }
        })
    }
    fn call(&self, args: &Value) -> Result<ToolResult, ToolError> {
        let rel = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
        let path = crate::guard::resolve(&self.workspace, rel)?;
        // Step-by-step openat with O_NOFOLLOW (no check-then-open gap).
        let dir = crate::safe_fs::open_dir(self.workspace.root(), &path, rel)
            .map_err(|e| if crate::guard::deny_code(&e.to_string()).is_some() { e } else { ToolError::Failed(format!("not a directory: {rel}")) })?;
        let mut names: Vec<String> = crate::safe_fs::list_dir(&dir)
            .map_err(|e| ToolError::Failed(e.to_string()))?
            .into_iter()
            .map(|(n, k)| {
                let name = n.to_string_lossy().into_owned();
                if k == crate::safe_fs::Kind::Dir {
                    format!("{name}/")
                } else {
                    name
                }
            })
            .collect();
        names.sort();
        names.truncate(200);
        Ok(ToolResult {
            content: names.join("\n"),
            ok: true,
        })
    }
}

pub struct ReadFile {
    pub workspace: Arc<Workspace>,
}

impl Tool for ReadFile {
    fn name(&self) -> &str {
        "read_file"
    }
    fn description(&self) -> &str {
        "Read a text file under the workspace (max 256 KiB)."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string"}
            },
            "required": ["path"]
        })
    }
    fn call(&self, args: &Value) -> Result<ToolResult, ToolError> {
        let rel = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::Failed("path required".into()))?;
        let path = crate::guard::resolve_read(&self.workspace, rel)?;
        // Step-by-step openat with O_NOFOLLOW; size check on the open fd.
        let content = crate::safe_fs::read_limited(self.workspace.root(), &path, rel, 256 * 1024)?;
        Ok(ToolResult {
            content,
            ok: true,
        })
    }
}

pub struct WriteFile {
    pub workspace: Arc<Workspace>,
}

impl Tool for WriteFile {
    fn name(&self) -> &str {
        "write_file"
    }
    fn description(&self) -> &str {
        "Write a text file under the workspace (creates parents)."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string"},
                "content": {"type": "string"}
            },
            "required": ["path", "content"]
        })
    }
    fn call(&self, args: &Value) -> Result<ToolResult, ToolError> {
        let rel = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::Failed("path required".into()))?;
        let content = args
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::Failed("content required".into()))?;
        if content.len() > 512 * 1024 {
            return Err(ToolError::Failed("content too large (>512 KiB)".into()));
        }
        let path = crate::guard::resolve_write(&self.workspace, rel)?;
        crate::safe_fs::write(self.workspace.root(), &path, rel, content.as_bytes(), true)?;
        Ok(ToolResult {
            content: format!("wrote {} bytes to {rel}", content.len()),
            ok: true,
        })
    }
}
