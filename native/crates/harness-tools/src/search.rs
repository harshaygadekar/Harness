use std::fs;
use std::sync::Arc;

use harness_core::{Tool, ToolError, ToolResult};
use regex::RegexBuilder;
use serde_json::{json, Value};

use crate::bound::bound_excerpt;
use crate::workspace::Workspace;

pub struct Search {
    pub workspace: Arc<Workspace>,
}

impl Tool for Search {
    fn name(&self) -> &str {
        "search"
    }
    fn description(&self) -> &str {
        "Search file contents under workspace (literal or regex, max 50 hits)."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {"type": "string"},
                "path": {"type": "string", "description": "subdir (default .)"},
                "regex": {"type": "boolean", "default": false}
            },
            "required": ["pattern"]
        })
    }
    fn call(&self, args: &Value) -> Result<ToolResult, ToolError> {
        let pattern = args
            .get("pattern")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::Failed("pattern required".into()))?;
        let rel = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
        let use_re = args.get("regex").and_then(|v| v.as_bool()).unwrap_or(false);
        let root = self
            .workspace
            .resolve(rel)
            .map_err(|e| ToolError::Failed(e.to_string()))?;

        let re = if use_re {
            RegexBuilder::new(pattern)
                .case_insensitive(true)
                .build()
                .map_err(|e| ToolError::Failed(e.to_string()))?
        } else {
            RegexBuilder::new(&regex::escape(pattern))
                .case_insensitive(true)
                .build()
                .map_err(|e| ToolError::Failed(e.to_string()))?
        };

        let mut hits = Vec::new();
        walk(&root, self.workspace.root(), &re, &mut hits, 50)
            .map_err(|e| ToolError::Failed(e.to_string()))?;
        let raw = if hits.is_empty() {
            "(no hits)".into()
        } else {
            hits.join("\n")
        };
        Ok(ToolResult {
            content: bound_excerpt(&raw, 16 * 1024),
            ok: true,
        })
    }
}

fn walk(
    dir: &std::path::Path,
    ws_root: &std::path::Path,
    re: &regex::Regex,
    hits: &mut Vec<String>,
    max: usize,
) -> std::io::Result<()> {
    if hits.len() >= max {
        return Ok(());
    }
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Ok(()),
    };
    for ent in entries.flatten() {
        if hits.len() >= max {
            break;
        }
        let path = ent.path();
        let name = ent.file_name();
        let name = name.to_string_lossy();
        if name.starts_with('.') || name == "target" || name == "node_modules" {
            continue;
        }
        if path.is_dir() {
            walk(&path, ws_root, re, hits, max)?;
        } else if path.is_file() {
            let meta = ent.metadata()?;
            if meta.len() > 256 * 1024 {
                continue;
            }
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            for (i, line) in text.lines().enumerate() {
                if re.is_match(line) {
                    let rel = path.strip_prefix(ws_root).unwrap_or(&path);
                    hits.push(format!("{}:{}:{}", rel.display(), i + 1, truncate(line, 200)));
                    if hits.len() >= max {
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

fn truncate(s: &str, n: usize) -> String {
    if s.len() <= n {
        s.to_string()
    } else {
        format!("{}…", &s[..n])
    }
}
