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
        let root = crate::guard::resolve(&self.workspace, rel)?;

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
        let mut skipped = Vec::new();
        walk(&root, self.workspace.root(), &re, &mut hits, &mut skipped, 50);
        // One note per skipped link. The note has the link path in the
        // workspace only, never the target path or content.
        let mut lines: Vec<String> = skipped
            .iter()
            .map(|p| format!("{}{}: {p} (link leaves workspace; skipped)", crate::guard::DENY_PREFIX, crate::guard::REASON_PATH_ESCAPE))
            .collect();
        if hits.is_empty() {
            lines.push("(no hits)".into());
        } else {
            lines.extend(hits);
        }
        let raw = lines.join("\n");
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
    skipped: &mut Vec<String>,
    max: usize,
) {
    if hits.len() >= max {
        return;
    }
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
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
        // Resolve the real path. Skip links that leave the workspace
        // (path_escape). Do not descend into linked dirs (no loops).
        // canonicalize reads only the link, not the target content.
        let Ok(real) = path.canonicalize() else { continue };
        if real.strip_prefix(ws_root).is_err() {
            let rel = path.strip_prefix(ws_root).unwrap_or(&path);
            skipped.push(rel.display().to_string());
            continue;
        }
        let is_link = ent.file_type().map(|t| t.is_symlink()).unwrap_or(true);
        if real.is_dir() {
            if is_link {
                continue;
            }
            walk(&path, ws_root, re, hits, skipped, max);
        } else if real.is_file() {
            let Ok(meta) = fs::metadata(&real) else { continue };
            if meta.len() > 256 * 1024 {
                continue;
            }
            let Ok(text) = fs::read_to_string(&real) else {
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
}

fn truncate(s: &str, n: usize) -> String {
    if s.len() <= n {
        s.to_string()
    } else {
        format!("{}…", &s[..n])
    }
}
