//! Native coding-agent CLI (ADR-005).
//! Jessie: deny-all default; not an OS sandbox.

use std::process::ExitCode;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use harness_core::{
    AllowList, DenyAll, FakeModel, FunctionTool, RunConfig, Runner, ToolGate, ToolResult, VERSION,
};
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
        /// Comma-separated tool allowlist (default: deny-all)
        #[arg(long, default_value = "")]
        allow: String,
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
            max_steps,
        } => {
            let names: Vec<String> = allow
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect();

            let gate: Arc<dyn ToolGate> = if names.is_empty() {
                Arc::new(DenyAll)
            } else {
                Arc::new(AllowList::new(names.clone()))
            };

            let mut runner = Runner::new(gate);
            if names.iter().any(|n| n == "echo") {
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

            let mut model = FakeModel::new();
            let result = runner.run(&mut model, &prompt, RunConfig { max_steps });
            println!("{:?}", result.status);
            println!("steps={}", result.steps);
            if let Some(last) = result.messages.last() {
                if let Some(c) = &last.content {
                    println!("{c}");
                }
            }
            ExitCode::SUCCESS
        }
    }
}
