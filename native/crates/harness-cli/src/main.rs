//! Native coding-agent CLI (ADR-005).
//! Jessie: deny-all default; workspace bind; shell needs explicit allow.
//! Not an OS sandbox.

mod metrics;

use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use harness_core::{
    AllowList, DenyAll, FakeModel, FunctionTool, RunConfig, RunEvent, Runner,
    ToolGate, ToolResult, VERSION,
};
use harness_model::{broken_add_fix_script, OpenAiCompat};
use harness_tools::{coding_tool_names, coding_tools, Workspace};
use metrics::{aggregate, detect_commit_sha, write_metrics, MetricsMeta};
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
    /// Real agent loop: MODEL → TOOLS → EDIT → TEST (default model=fake)
    Ask {
        #[arg(long)]
        prompt: String,
        #[arg(long, default_value = "")]
        allow: String,
        #[arg(long, default_value = ".")]
        workspace: PathBuf,
        /// Model: fake | scripted | openai (default fake for CI safety)
        #[arg(long, default_value = "fake")]
        model: String,
        /// Directory to write run.jsonl events
        #[arg(long)]
        record: Option<PathBuf>,
        #[arg(long, default_value_t = 16)]
        max_steps: usize,
    },
    /// Dump / validate a recorded run.jsonl; optionally emit metrics.json
    Replay {
        /// Path to run.jsonl
        run: PathBuf,
        /// Write Phase 3 metrics.json (default: beside run.jsonl)
        #[arg(long)]
        metrics_out: Option<PathBuf>,
        /// Skip writing metrics.json
        #[arg(long, default_value_t = false)]
        no_metrics: bool,
        #[arg(long, default_value = "unknown")]
        task_id: String,
        #[arg(long, default_value = "unknown")]
        fixture_path: String,
        #[arg(long, default_value = "0.1.0")]
        fixture_version: String,
        #[arg(long, default_value = "other")]
        model_mode: String,
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
        Cmd::Ask {
            prompt,
            allow,
            workspace,
            model,
            record,
            max_steps,
        } => match ask_cmd(&prompt, &allow, &workspace, &model, record.as_deref(), max_steps)
        {
            Ok(code) => code,
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::FAILURE
            }
        },
        Cmd::Replay {
            run,
            metrics_out,
            no_metrics,
            task_id,
            fixture_path,
            fixture_version,
            model_mode,
        } => match replay_cmd(
            &run,
            metrics_out.as_deref(),
            no_metrics,
            &task_id,
            &fixture_path,
            &fixture_version,
            &model_mode,
        ) {
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

fn coding_tool_name_set() -> &'static [&'static str] {
    &[
        "list_files",
        "search",
        "read_file",
        "write_file",
        "apply_patch",
        "run_tests",
        "git_status",
        "git_diff",
        "run_shell",
    ]
}

fn build_runner(allow_names: &[String], workspace: &Path) -> Result<Runner, String> {
    let gate: Arc<dyn ToolGate> = if allow_names.is_empty() {
        Arc::new(DenyAll)
    } else {
        Arc::new(AllowList::new(allow_names.to_vec()))
    };

    let mut runner = Runner::new(gate);
    let register_shell = allow_names.iter().any(|n| n == "run_shell");
    let needs_ws = allow_names
        .iter()
        .any(|n| coding_tool_name_set().contains(&n.as_str()));

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

    Ok(runner)
}

fn write_events(record_dir: &Path, events: &[RunEvent]) -> Result<PathBuf, String> {
    fs::create_dir_all(record_dir).map_err(|e| format!("record dir: {e}"))?;
    let path = record_dir.join("run.jsonl");
    let mut f = File::create(&path).map_err(|e| format!("record file: {e}"))?;
    for ev in events {
        let line = serde_json::to_string(ev).map_err(|e| format!("serialize: {e}"))?;
        writeln!(f, "{line}").map_err(|e| format!("write: {e}"))?;
    }
    Ok(path)
}

fn run_cmd(
    prompt: &str,
    allow: &str,
    workspace: &PathBuf,
    shell: bool,
    max_steps: usize,
) -> Result<(), String> {
    let allow_names = expand_allow(allow, shell);
    let runner = build_runner(&allow_names, workspace)?;
    let mut model: FakeModel = FakeModel::new();
    let result = runner.run(&mut model, prompt, RunConfig { max_steps });
    println!("{:?}", result.status);
    println!("steps={}", result.steps);
    if let Some(last) = result.messages.last() {
        if let Some(c) = &last.content {
            println!("{c}");
        }
    }
    Ok(())
}

fn ask_cmd(
    prompt: &str,
    allow: &str,
    workspace: &Path,
    model_name: &str,
    record: Option<&Path>,
    max_steps: usize,
) -> Result<ExitCode, String> {
    let allow_names = expand_allow(allow, false);
    let runner = build_runner(&allow_names, workspace)?;

    let result = match model_name {
        "fake" => {
            let mut model = FakeModel::new();
            runner.run(&mut model, prompt, RunConfig { max_steps })
        }
        "scripted" => {
            let mut model = broken_add_fix_script();
            runner.run(&mut model, prompt, RunConfig { max_steps })
        }
        "openai" => {
            let mut model =
                OpenAiCompat::from_env().map_err(|e| format!("openai model: {e}"))?;
            runner.run(&mut model, prompt, RunConfig { max_steps })
        }
        other => return Err(format!("unknown model '{other}' (use fake|scripted|openai)")),
    };

    let record_dir = match record {
        Some(d) => d.to_path_buf(),
        None => workspace.join(".harness").join("runs"),
    };
    let path = write_events(&record_dir, &result.events)?;
    let metrics_path = record_dir.join("metrics.json");
    let meta = MetricsMeta {
        task_id: format!("ask/{model_name}"),
        fixture_path: workspace.display().to_string(),
        fixture_version: "0.1.0".into(),
        model_mode: model_name.to_string(),
        commit_sha: detect_commit_sha(),
        machine_class: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
        source_run_jsonl: path.display().to_string(),
    };
    let metrics = aggregate(&result.events, &meta);
    write_metrics(&metrics_path, &metrics)?;
    println!("status={:?}", result.status);
    println!("steps={}", result.steps);
    println!("recorded={}", path.display());
    println!("metrics={}", metrics_path.display());
    if let Some(last) = result.messages.last() {
        if let Some(c) = &last.content {
            println!("{c}");
        }
    }

    // For scripted fixture: surface tool failures clearly.
    let tests_ok = result.events.iter().any(|e| matches!(
        e,
        RunEvent::ToolResult { name, ok: true, .. } if name == "run_tests"
    ));
    if model_name == "scripted" && !tests_ok {
        eprintln!("scripted slice: run_tests did not succeed");
        return Ok(ExitCode::FAILURE);
    }

    Ok(match result.status {
        harness_core::RunStatus::Succeeded => ExitCode::SUCCESS,
        _ => ExitCode::FAILURE,
    })
}

fn replay_cmd(
    run: &Path,
    metrics_out: Option<&Path>,
    no_metrics: bool,
    task_id: &str,
    fixture_path: &str,
    fixture_version: &str,
    model_mode: &str,
) -> Result<(), String> {
    let f = File::open(run).map_err(|e| format!("open: {e}"))?;
    let reader = BufReader::new(f);
    let mut n = 0usize;
    let mut events: Vec<RunEvent> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| format!("read line {}: {e}", i + 1))?;
        if line.trim().is_empty() {
            continue;
        }
        let ev: RunEvent = serde_json::from_str(&line)
            .map_err(|e| format!("schema line {}: {e}\n  text: {line}", i + 1))?;
        n += 1;
        match &ev {
            RunEvent::ModelRequest {
                ts_ms,
                message_count,
                tool_count,
            } => println!("[{ts_ms}] model_request messages={message_count} tools={tool_count}"),
            RunEvent::ModelResponse {
                ts_ms,
                content,
                tool_calls,
            } => {
                println!(
                    "[{ts_ms}] model_response tool_calls={} content={}",
                    tool_calls.len(),
                    content.as_deref().unwrap_or("-").chars().take(80).collect::<String>()
                );
            }
            RunEvent::ToolCall {
                ts_ms, id, name, ..
            } => println!("[{ts_ms}] tool_call id={id} name={name}"),
            RunEvent::ToolResult {
                ts_ms,
                id,
                name,
                ok,
                content,
            } => {
                let preview: String = content.chars().take(100).collect();
                println!("[{ts_ms}] tool_result id={id} name={name} ok={ok} {preview}");
            }
            RunEvent::Error { ts_ms, message } => println!("[{ts_ms}] error {message}"),
            RunEvent::StateChange { ts_ms, status } => {
                println!("[{ts_ms}] state_change {status}")
            }
            RunEvent::Done {
                ts_ms,
                status,
                steps,
            } => println!("[{ts_ms}] done status={status} steps={steps}"),
        }
        events.push(ev);
    }
    println!("validated {n} events");

    if !no_metrics {
        let out = metrics_out
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| {
                run.parent()
                    .unwrap_or_else(|| Path::new("."))
                    .join("metrics.json")
            });
        let meta = MetricsMeta {
            task_id: task_id.to_string(),
            fixture_path: fixture_path.to_string(),
            fixture_version: fixture_version.to_string(),
            model_mode: model_mode.to_string(),
            commit_sha: detect_commit_sha(),
            machine_class: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
            source_run_jsonl: run.display().to_string(),
        };
        let metrics = aggregate(&events, &meta);
        write_metrics(&out, &metrics)?;
        println!("metrics={}", out.display());
    }
    Ok(())
}
