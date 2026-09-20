use std::fs;
use std::sync::Arc;

use harness_core::{Tool, ToolError, ToolResult};
use serde_json::{json, Value};

use crate::workspace::Workspace;

/// Minimal unified-style replace: old_string -> new_string in one file.
pub struct ApplyPatch {
    pub workspace: Arc<Workspace>,
}

impl Tool for ApplyPatch {
    fn name(&self) -> &str {
        "apply_patch"
    }
    fn description(&self) -> &str {
        "Replace one exact occurrence of old_string with new_string in a file."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string"},
                "old_string": {"type": "string"},
                "new_string": {"type": "string"}
            },
            "required": ["path", "old_string", "new_string"]
        })
    }
    fn call(&self, args: &Value) -> Result<ToolResult, ToolError> {
        let rel = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::Failed("path required".into()))?;
        let old = args
            .get("old_string")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::Failed("old_string required".into()))?;
        let new = args
            .get("new_string")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::Failed("new_string required".into()))?;
        if old.is_empty() {
            return Err(ToolError::Failed("old_string empty".into()));
        }
        let path = self
            .workspace
            .resolve(rel)
            .map_err(|e| ToolError::Failed(e.to_string()))?;
        let text = fs::read_to_string(&path).map_err(|e| ToolError::Failed(e.to_string()))?;
        let count = text.matches(old).count();
        if count == 0 {
            return Err(ToolError::Failed("old_string not found".into()));
        }
        if count > 1 {
            return Err(ToolError::Failed(format!(
                "old_string matches {count} times; must be unique"
            )));
        }
        let updated = text.replacen(old, new, 1);
        fs::write(&path, updated).map_err(|e| ToolError::Failed(e.to_string()))?;
        Ok(ToolResult {
            content: format!("patched {rel}"),
            ok: true,
        })
    }
}
