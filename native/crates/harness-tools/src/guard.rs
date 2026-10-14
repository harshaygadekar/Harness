//! Path guards for file tools. Outside core.
//! A blocked call fails with a fixed message: `denied:<code>: <path>`.
//! A `tool_denies` counter can read the code with [`deny_code`].

use std::path::{Component, Path};

use harness_core::ToolError;

use crate::workspace::Workspace;

/// Read of a `.env` file (or a link to one).
pub const REASON_ENV_READ: &str = "env_read";
/// Write or edit under `.git` (any case, or through a link).
pub const REASON_GIT_WRITE: &str = "git_write";
/// Path or link that leaves the workspace.
pub const REASON_PATH_ESCAPE: &str = "path_escape";

/// Shell refused: no full Landlock sandbox on this kernel (fail closed).
pub const REASON_SHELL_SANDBOX: &str = "shell_sandbox_unavailable";

pub const DENY_PREFIX: &str = "denied:";

/// Make the fixed deny error.
pub fn deny(code: &'static str, rel: &str) -> ToolError {
    ToolError::Failed(format!("{DENY_PREFIX}{code}: {rel}"))
}

/// Deny error with free detail text (not a path).
pub fn deny_msg(code: &'static str, detail: &str) -> ToolError {
    ToolError::Failed(format!("{DENY_PREFIX}{code}: {detail}"))
}

/// Get the reason code from a tool error text, if it is a deny.
pub fn deny_code(msg: &str) -> Option<&'static str> {
    let rest = msg.split(DENY_PREFIX).nth(1)?;
    [REASON_ENV_READ, REASON_GIT_WRITE, REASON_PATH_ESCAPE, REASON_SHELL_SANDBOX]
        .into_iter()
        .find(|c| rest.starts_with(&format!("{c}:")))
}

/// True for `.env`, `.env.local`, `.env.<any>` (any case).
pub fn is_env_name(name: &str) -> bool {
    let n = name.to_ascii_lowercase();
    n == ".env" || n.starts_with(".env.")
}

fn last_name(p: &Path) -> Option<String> {
    p.file_name().map(|s| s.to_string_lossy().into_owned())
}

fn has_git_component(p: &Path) -> bool {
    p.components().any(|c| match c {
        Component::Normal(s) => s.to_string_lossy().eq_ignore_ascii_case(".git"),
        _ => false,
    })
}

/// Resolve `rel` with the escape check. Escape gives `path_escape`.
pub fn resolve(ws: &Workspace, rel: &str) -> Result<std::path::PathBuf, ToolError> {
    ws.resolve(rel).map_err(|e| match e {
        crate::workspace::WorkspaceError::Escape(_) => deny(REASON_PATH_ESCAPE, rel),
        other => ToolError::Failed(other.to_string()),
    })
}

/// Resolve for a read. Deny `.env` by the given name and by the real target.
pub fn resolve_read(ws: &Workspace, rel: &str) -> Result<std::path::PathBuf, ToolError> {
    if last_name(Path::new(rel)).is_some_and(|n| is_env_name(&n)) {
        return Err(deny(REASON_ENV_READ, rel));
    }
    let path = resolve(ws, rel)?;
    if last_name(&path).is_some_and(|n| is_env_name(&n)) {
        return Err(deny(REASON_ENV_READ, rel));
    }
    Ok(path)
}

/// Resolve for a write, edit or delete. Deny any `.git` part (any case),
/// by the given name and by the real target.
pub fn resolve_write(ws: &Workspace, rel: &str) -> Result<std::path::PathBuf, ToolError> {
    if has_git_component(Path::new(rel)) {
        return Err(deny(REASON_GIT_WRITE, rel));
    }
    let path = resolve(ws, rel)?;
    let inner = path.strip_prefix(ws.root()).unwrap_or(&path);
    if has_git_component(inner) {
        return Err(deny(REASON_GIT_WRITE, rel));
    }
    // A link file that does not exist yet as target: check the link target too.
    if let Ok(t) = std::fs::read_link(&path) {
        let t = if t.is_absolute() { t } else { path.parent().unwrap_or(ws.root()).join(t) };
        if has_git_component(t.strip_prefix(ws.root()).unwrap_or(&t)) {
            return Err(deny(REASON_GIT_WRITE, rel));
        }
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_names() {
        for n in [".env", ".env.local", ".env.production", ".ENV"] {
            assert!(is_env_name(n), "{n}");
        }
        for n in ["env", ".envrc", "a.env", ".environment"] {
            assert!(!is_env_name(n), "{n}");
        }
    }

    #[test]
    fn codes_parse() {
        assert_eq!(deny_code(&deny(REASON_ENV_READ, ".env").to_string()), Some("env_read"));
        assert_eq!(deny_code(&deny(REASON_GIT_WRITE, ".git/x").to_string()), Some("git_write"));
        assert_eq!(deny_code(&deny(REASON_PATH_ESCAPE, "..").to_string()), Some("path_escape"));
        assert_eq!(deny_code("tool failed: io"), None);
    }
}
