//! Native coding-agent CLI (ADR-005).
//! Jessie: deny-all default; workspace bind; shell needs explicit allow.
//! Not an OS sandbox.

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use harness_core::{
    AllowList, DenyAll, FakeModel, FunctionTool, ModelClient, RunConfig, Runner, ToolGate,
    ToolResult, VERSION,
};
use harness_tools::{coding_tool_names, coding_tools, Workspace};
use serde_json::json;

#[derive(Parser, Debug)]
#[command(
    name = "harness",
    about = "Tiny native agent CLI (ADR-005). Deny-all tools by default. Not an OS sandbox."
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Print version
    Version,
    /// Run one fake-model agent turn (CI / smoke)
    Run {
        #[arg(long)]
        prompt: String,
        /// Comma-separated allowlist. Use `coding` for min pack. Default: deny-all.
        #[arg(long, default_value = "")]
        allow: String,
        /// Workspace root for coding tools
        #[arg(long, default_value = ".")]
        workspace: PathBuf,
        /// Also allow `run_shell` when listed via `--allow shell` or `--allow coding,shell`
        #[arg(long, default_value_t = false)]
        shell: bool,
        #[arg(long, default_value_t = 8)]
        max_steps: usize,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Version => {
            println!("{VERSION}");
            ExitCode::SUCCESS
        }
        Cmd::Run {
            prompt,
            allow,
            workspace,
            shell,
            max_steps,
        } => match run_cmd(&prompt, &allow, &workspace, shell, max_steps) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::FAILURE
            }
        },
    }
}

fn expand_allow(allow: &str, shell_flag: bool) -> Vec<String> {
    let tokens: Vec<&str> = allow
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    // --shell alone must not open the gate (Jessie). Need non-empty --allow.
    if tokens.is_empty() {
        return Vec::new();
    }
    let mut names: Vec<String> = Vec::new();
    let mut want_shell = shell_flag;
    for t in tokens {
        match t {
            "coding" => {
                for n in coding_tool_names(false) {
                    names.push(n.to_string());
                }
            }
            "shell" => {
                want_shell = true;
                names.push("run_shell".into());
            }
            other => names.push(other.to_string()),
        }
    }
    if want_shell && !names.iter().any(|n| n == "run_shell") {
        names.push("run_shell".into());
    }
    names
}

fn run_cmd(
    prompt: &str,
    allow: &str,
    workspace: &PathBuf,
    shell: bool,
    max_steps: usize,
) -> Result<(), String> {
    let allow_names = expand_allow(allow, shell);
    let register_shell = allow_names.iter().any(|n| n == "run_shell");

    let gate: Arc<dyn ToolGate> = if allow_names.is_empty() {
        Arc::new(DenyAll)
    } else {
        Arc::new(AllowList::new(allow_names.clone()))
    };

    let mut runner = Runner::new(gate);

    let needs_ws = allow_names.iter().any(|n| {
        matches!(
            n.as_str(),
            "list_files" | "search" | "read_file" | "write_file" | "apply_patch" | "run_shell"
        )
    });

    if needs_ws {
        let ws = Arc::new(Workspace::new(workspace).map_err(|e| format!("workspace: {e}"))?);
        for tool in coding_tools(ws, register_shell) {
            if allow_names.iter().any(|n| n == tool.name()) {
                runner.register_tool(tool);
            }
        }
    }

    if allow_names.iter().any(|n| n == "echo") {
        runner.register_tool(Arc::new(FunctionTool {
            name: "echo".into(),
            description: "Echo text".into(),
            parameters: json!({
                "type": "object",
                "properties": {"text": {"type": "string"}},
                "required": ["text"]
            }),
            handler: |args: &serde_json::Value| {
                Ok(ToolResult {
                    content: args
                        .get("text")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    ok: true,
                })
            },
        }));
    }

    let mut model: FakeModel = FakeModel::new();
    let model_ref: &mut dyn ModelClient = &mut model;
    let result = runner.run(model_ref, prompt, RunConfig { max_steps });
    println!("{:?}", result.status);
    println!("steps={}", result.steps);
    if let Some(last) = result.messages.last() {
        if let Some(c) = &last.content {
            println!("{c}");
        }
    }
    Ok(())
}
