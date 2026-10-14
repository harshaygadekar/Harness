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
        // Step-by-step openat with O_NOFOLLOW for every folder and file.
        let dir = crate::safe_fs::open_dir(self.workspace.root(), &root, rel)?;
        let start = root.strip_prefix(self.workspace.root()).unwrap_or(std::path::Path::new("")).to_path_buf();
        walk(&self.workspace, &dir, &start, &re, &mut hits, &mut skipped, 50, 0);
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

#[allow(clippy::too_many_arguments)]
fn walk(
    ws: &Workspace,
    dir: &std::os::fd::OwnedFd,
    rel_dir: &std::path::Path,
    re: &regex::Regex,
    hits: &mut Vec<String>,
    skipped: &mut Vec<String>,
    max: usize,
    depth: usize,
) {
    use crate::safe_fs::Kind;
    if hits.len() >= max || depth > 64 {
        return;
    }
    let Ok(mut entries) = crate::safe_fs::list_dir(dir) else { return };
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    for (name_os, kind) in entries {
        if hits.len() >= max {
            break;
        }
        let name = name_os.to_string_lossy();
        if name.starts_with('.') || name == "target" || name == "node_modules" {
            continue;
        }
        let rel = rel_dir.join(&name_os);
        let text = match kind {
            Kind::Dir => {
                if let Ok(sub) = crate::safe_fs::open_child_dir(dir, &name_os) {
                    walk(ws, &sub, &rel, re, hits, skipped, max, depth + 1);
                }
                continue;
            }
            Kind::File => match crate::safe_fs::read_child_os(dir, &name_os, 256 * 1024) {
                Ok(Some(t)) => t,
                _ => continue,
            },
            Kind::Link => {
                // A link that leaves the workspace: one note, link path only
                // (never the target path or content). Linked folders are not
                // walked (no loops). A link to a file inside is read through
                // its real path, again with openat + O_NOFOLLOW.
                let rel_s = rel.to_string_lossy().into_owned();
                let Ok(real) = ws.resolve(&rel_s) else {
                    skipped.push(rel_s);
                    continue;
                };
                if crate::guard::is_env_name(real.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default().as_str()) {
                    continue;
                }
                match crate::safe_fs::read_limited(ws.root(), &real, &rel_s, 256 * 1024) {
                    Ok(t) => t,
                    Err(_) => continue,
                }
            }
            Kind::Other => continue,
        };
        for (i, line) in text.lines().enumerate() {
            if re.is_match(line) {
                hits.push(format!("{}:{}:{}", rel.display(), i + 1, truncate(line, 200)));
                if hits.len() >= max {
                    break;
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
