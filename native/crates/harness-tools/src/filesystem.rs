use std::fs;
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
        if !path.is_dir() {
            return Err(ToolError::Failed(format!("not a directory: {rel}")));
        }
        let mut names: Vec<String> = fs::read_dir(&path)
            .map_err(|e| ToolError::Failed(e.to_string()))?
            .filter_map(|e| e.ok())
            .map(|e| {
                let name = e.file_name().to_string_lossy().into_owned();
                if e.path().is_dir() {
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
        let meta = fs::metadata(&path).map_err(|e| ToolError::Failed(e.to_string()))?;
        if meta.len() > 256 * 1024 {
            return Err(ToolError::Failed("file too large (>256 KiB)".into()));
        }
        let content = fs::read_to_string(&path).map_err(|e| ToolError::Failed(e.to_string()))?;
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
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| ToolError::Failed(e.to_string()))?;
        }
        fs::write(&path, content).map_err(|e| ToolError::Failed(e.to_string()))?;
        Ok(ToolResult {
            content: format!("wrote {} bytes to {rel}", content.len()),
            ok: true,
        })
    }
}
