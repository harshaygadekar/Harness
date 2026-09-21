//! Model adapters (ADR-005). Outside core — no HTTP in harness-core.
//! OpenAI-compatible chat completions + ScriptedModel for CI.

mod openai;
mod scripted;

pub use openai::{OpenAiCompat, OpenAiError};
pub use scripted::{broken_add_fix_script, ScriptedModel};

pub use harness_core::{ModelClient, ModelResponse, ToolSpec};

#[cfg(test)]
mod slice_tests {
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::Arc;

    use harness_core::{AllowList, RunConfig, Runner};
    use harness_tools::{coding_tool_names, coding_tools, Workspace};

    use crate::broken_add_fix_script;

    fn fixture_src() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/broken_add")
            .canonicalize()
            .expect("fixtures/broken_add")
    }

    fn copy_tree(src: &std::path::Path, dst: &std::path::Path) {
        fs::create_dir_all(dst).unwrap();
        for ent in fs::read_dir(src).unwrap() {
            let ent = ent.unwrap();
            let to = dst.join(ent.file_name());
            if ent.file_type().unwrap().is_dir() {
                if ent.file_name() == "target" {
                    continue;
                }
                copy_tree(&ent.path(), &to);
            } else {
                fs::copy(ent.path(), &to).unwrap();
            }
        }
    }

    #[test]
    fn scripted_broken_add_vertical_slice() {
        let src = fixture_src();
        let dst = std::env::temp_dir().join(format!(
            "harness-slice-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        copy_tree(&src, &dst);

        // Confirm fixture starts broken.
        let before = Command::new("cargo")
            .args(["test", "-q"])
            .current_dir(&dst)
            .output()
            .expect("cargo test before");
        assert!(
            !before.status.success(),
            "fixture should fail before fix"
        );

        let ws = Arc::new(Workspace::new(&dst).unwrap());
        let names: Vec<String> = coding_tool_names(true)
            .into_iter()
            .map(str::to_string)
            .collect();
        let gate = Arc::new(AllowList::new(names.clone()));
        let mut runner = Runner::new(gate);
        for tool in coding_tools(ws, true) {
            if names.iter().any(|n| n == tool.name()) {
                runner.register_tool(tool);
            }
        }

        let mut model = broken_add_fix_script();
        let result = runner.run(
            &mut model,
            "fix the failing tests",
            RunConfig { max_steps: 16 },
        );
        assert_eq!(result.status, harness_core::RunStatus::Succeeded);
        assert!(
            result.events.iter().any(|e| matches!(
                e,
                harness_core::RunEvent::ToolResult { name, ok: true, .. } if name == "run_tests"
            )),
            "run_tests should succeed; events={:?}",
            result.events
        );

        // Record events under temp .harness/runs/
        let rec = dst.join(".harness").join("runs");
        fs::create_dir_all(&rec).unwrap();
        let jsonl = rec.join("run.jsonl");
        let mut body = String::new();
        for ev in &result.events {
            body.push_str(&serde_json::to_string(ev).unwrap());
            body.push('\n');
        }
        fs::write(&jsonl, body).unwrap();
        assert!(jsonl.metadata().unwrap().len() > 0);

        let after = Command::new("cargo")
            .args(["test", "-q"])
            .current_dir(&dst)
            .output()
            .expect("cargo test after");
        assert!(
            after.status.success(),
            "tests should pass after scripted fix: {}",
            String::from_utf8_lossy(&after.stderr)
        );

        let _ = fs::remove_dir_all(&dst);
    }
}
