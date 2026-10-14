//! Fail closed: with Landlock forced lower than ABI 6 (or off), run_shell
//! refuses and never runs the command without the sandbox.
//! Own test binary, because it sets a process env var.

use std::sync::Arc;

use harness_core::Tool;
use harness_tools::sandbox::{self, Status};
use harness_tools::{deny_code, RunShell, Workspace};
use serde_json::json;

#[test]
fn shell_refuses_without_full_landlock() {
    let ws = std::env::temp_dir().join(format!("harness-sb-fc-{}", std::process::id()));
    std::fs::create_dir_all(&ws).unwrap();
    let t = RunShell { workspace: Arc::new(Workspace::new(&ws).unwrap()) };
    for cap in ["5", "4", "0", "-1"] {
        std::env::set_var(sandbox::ABI_CAP_ENV, cap);
        assert_eq!(sandbox::status(), Status::Unavailable, "cap {cap}");
        let e = t.call(&json!({"command": "touch ran_unsandboxed"})).unwrap_err();
        assert_eq!(deny_code(&e.to_string()), Some("shell_sandbox_unavailable"), "{e}");
        assert!(!ws.join("ran_unsandboxed").exists(), "cap {cap}: command ran");
    }
    // The hook can only lower the ABI: a high value does not turn it on.
    std::env::set_var(sandbox::ABI_CAP_ENV, "99");
    assert_eq!(sandbox::kernel_abi() < 99, true);
    std::env::remove_var(sandbox::ABI_CAP_ENV);
    assert!(t.call(&json!({"command": "touch ran_sandboxed"})).unwrap().ok);
    assert!(ws.join("ran_sandboxed").exists());
    let _ = std::fs::remove_dir_all(&ws);
}
