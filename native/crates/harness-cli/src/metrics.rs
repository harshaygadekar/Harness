//! Aggregate run.jsonl into Phase 3 metrics.json (schema: native/benchmarks/phase3_schema.json).
//! Heuristic counters from events. Oracle-labeled waste stays "unknown" until fixtures ship labels.
//! No single score. No beat-fx claim.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use harness_core::RunEvent;
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct MetricsMeta {
    pub task_id: String,
    pub fixture_path: String,
    pub fixture_version: String,
    pub model_mode: String,
    pub commit_sha: String,
    pub machine_class: String,
    pub source_run_jsonl: String,
}

impl Default for MetricsMeta {
    fn default() -> Self {
        Self {
            task_id: "unknown".into(),
            fixture_path: "unknown".into(),
            fixture_version: "0.1.0".into(),
            model_mode: "other".into(),
            commit_sha: detect_commit_sha(),
            machine_class: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
            source_run_jsonl: "unknown".into(),
        }
    }
}

pub fn detect_commit_sha() -> String {
    Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into())
}

fn measured_at_ist() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Asia/Calcutta = UTC+5:30
    let ist = secs + 5 * 3600 + 1800;
    let days = ist / 86400;
    let tod = ist % 86400;
    let hour = tod / 3600;
    let min = (tod % 3600) / 60;
    let sec = tod % 60;
    // Civil date from days since 1970-01-01 (Howard Hinnant algorithm)
    let z = days as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!(
        "{y:04}-{m:02}-{d:02}T{hour:02}:{min:02}:{sec:02}+05:30"
    )
}

fn path_from_args(args: &Value) -> Option<String> {
    args.get("path")
        .or_else(|| args.get("file"))
        .or_else(|| args.get("filepath"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn search_key(args: &Value) -> String {
    serde_json::to_string(args).unwrap_or_default()
}

fn call_fingerprint(name: &str, args: &Value) -> String {
    format!("{name}|{}", serde_json::to_string(args).unwrap_or_default())
}

/// Build Phase 3 metrics object from recorded events.
pub fn aggregate(events: &[RunEvent], meta: &MetricsMeta) -> Value {
    let mut total_tool_calls = 0u64;
    let mut tool_calls_success = 0u64;
    let mut tool_calls_fail = 0u64;
    let mut gate_denies = 0u64;
    let mut tool_errors = 0u64;
    let mut writes = 0u64;
    let mut tests_run = 0u64;
    let mut model_turns = 0u64;
    let mut files_read: HashSet<String> = HashSet::new();
    let mut files_modified: HashSet<String> = HashSet::new();
    let mut read_seen: HashSet<String> = HashSet::new();
    let mut search_seen: HashSet<String> = HashSet::new();
    let mut call_seen: HashMap<String, u64> = HashMap::new();
    let mut redundant_reads = 0u64;
    let mut redundant_searches = 0u64;
    let mut duplicate_tool_calls = 0u64;
    let mut useful_actions = 0u64;
    let mut first_useful_ts: Option<u64> = None;
    let mut success_ts: Option<u64> = None;
    let mut t0: Option<u64> = None;
    let mut t_done: Option<u64> = None;
    let mut done_status = String::from("fail");
    let mut pending: HashMap<String, (String, Value)> = HashMap::new();

    for ev in events {
        match ev {
            RunEvent::ModelRequest { ts_ms, .. } => {
                t0.get_or_insert(*ts_ms);
            }
            RunEvent::ModelResponse { ts_ms, .. } => {
                t0.get_or_insert(*ts_ms);
                model_turns += 1;
            }
            RunEvent::ToolCall {
                ts_ms,
                id,
                name,
                arguments,
            } => {
                t0.get_or_insert(*ts_ms);
                total_tool_calls += 1;
                let fp = call_fingerprint(name, arguments);
                let n = call_seen.entry(fp).or_insert(0);
                *n += 1;
                if *n > 1 {
                    duplicate_tool_calls += 1;
                }
                pending.insert(id.clone(), (name.clone(), arguments.clone()));
            }
            RunEvent::ToolResult {
                ts_ms,
                id,
                name,
                content,
                ok,
            } => {
                t0.get_or_insert(*ts_ms);
                let args = pending
                    .remove(id)
                    .map(|(_, a)| a)
                    .unwrap_or(Value::Null);

                if *ok {
                    tool_calls_success += 1;
                } else {
                    tool_calls_fail += 1;
                    if content.starts_with("DENIED:") {
                        gate_denies += 1;
                    } else {
                        tool_errors += 1;
                    }
                }

                match name.as_str() {
                    "read_file" => {
                        if let Some(p) = path_from_args(&args) {
                            if *ok {
                                if !read_seen.insert(p.clone()) {
                                    redundant_reads += 1;
                                } else {
                                    files_read.insert(p);
                                    useful_actions += 1;
                                    first_useful_ts.get_or_insert(*ts_ms);
                                }
                            }
                        }
                    }
                    "search" => {
                        let key = search_key(&args);
                        if *ok {
                            if !search_seen.insert(key) {
                                redundant_searches += 1;
                            } else {
                                useful_actions += 1;
                                first_useful_ts.get_or_insert(*ts_ms);
                            }
                        }
                    }
                    "write_file" | "apply_patch" => {
                        writes += 1;
                        if let Some(p) = path_from_args(&args) {
                            files_modified.insert(p);
                        } else if name == "apply_patch" {
                            // patch may use different arg shapes; still count write
                            if let Some(p) = args.get("path").and_then(|v| v.as_str()) {
                                files_modified.insert(p.to_string());
                            }
                        }
                        if *ok {
                            useful_actions += 1;
                            first_useful_ts.get_or_insert(*ts_ms);
                        }
                    }
                    "run_tests" => {
                        tests_run += 1;
                        if *ok {
                            useful_actions += 1;
                            first_useful_ts.get_or_insert(*ts_ms);
                            success_ts = Some(*ts_ms);
                        }
                    }
                    _ => {}
                }
            }
            RunEvent::Error { ts_ms, .. } => {
                t0.get_or_insert(*ts_ms);
            }
            RunEvent::StateChange { ts_ms, .. } => {
                t0.get_or_insert(*ts_ms);
            }
            RunEvent::Done { ts_ms, status, .. } => {
                t0.get_or_insert(*ts_ms);
                t_done = Some(*ts_ms);
                done_status = status.clone();
            }
        }
    }

    let wall_time_ms = match (t0, t_done) {
        (Some(a), Some(b)) if b >= a => b - a,
        (Some(a), None) => {
            // last event ts
            let last = events.iter().rev().find_map(|e| match e {
                RunEvent::ModelRequest { ts_ms, .. }
                | RunEvent::ModelResponse { ts_ms, .. }
                | RunEvent::ToolCall { ts_ms, .. }
                | RunEvent::ToolResult { ts_ms, .. }
                | RunEvent::Error { ts_ms, .. }
                | RunEvent::StateChange { ts_ms, .. }
                | RunEvent::Done { ts_ms, .. } => Some(*ts_ms),
            });
            last.map(|b| b.saturating_sub(a)).unwrap_or(0)
        }
        _ => 0,
    };

    let status_lower = done_status.to_lowercase();
    let (exit_status, task_success, run_ok) = if status_lower.contains("succeed") {
        ("success", true, true)
    } else if status_lower.contains("max") {
        ("max_turns", false, true)
    } else if status_lower.contains("timeout") {
        ("timeout", false, true)
    } else if status_lower.contains("fail") || status_lower.contains("error") {
        // harness finished with Fail status still counts as run_ok if Done was emitted
        ("fail", false, t_done.is_some())
    } else if t_done.is_some() {
        ("fail", false, true)
    } else {
        ("fail", false, false)
    };

    // task_success: oracle-independent heuristic (Done=Succeeded or any passing run_tests).
    // exit_status is the stop reason from `done` only; a passing run_tests must not
    // overwrite it (T7 H2: tests passed but the run stopped at MaxSteps).
    let task_success = task_success || success_ts.is_some();

    let time_to_first_useful: Value = match (t0, first_useful_ts) {
        (Some(a), Some(b)) if b >= a => json!(b - a),
        _ => json!("unknown"),
    };
    let time_to_success: Value = if task_success {
        match (t0, success_ts.or(t_done)) {
            (Some(a), Some(b)) if b >= a => json!(b - a),
            _ => json!("unknown"),
        }
    } else {
        json!("unknown")
    };

    let unnecessary = redundant_reads + redundant_searches; // heuristic floor; not full M1 oracle

    let mut files_read_paths: Vec<String> = files_read.iter().cloned().collect();
    files_read_paths.sort();
    let mut files_modified_paths: Vec<String> = files_modified.iter().cloned().collect();
    files_modified_paths.sort();

    let model_mode = match meta.model_mode.as_str() {
        "fake" | "scripted" | "live" | "other" => meta.model_mode.clone(),
        "openai" => "live".into(),
        _ => "other".into(),
    };

    json!({
        "schema_version": "phase3.v1",
        "task_id": meta.task_id,
        "fixture_path": meta.fixture_path,
        "fixture_version": meta.fixture_version,
        "commit_sha": meta.commit_sha,
        "machine_class": meta.machine_class,
        "model_mode": model_mode,
        "measured_at": measured_at_ist(),
        "exit_status": exit_status,
        "task_success": task_success,
        "run_ok": run_ok,
        "total_tool_calls": total_tool_calls,
        "tool_calls_success": tool_calls_success,
        "tool_calls_fail": tool_calls_fail,
        "gate_denies": gate_denies,
        "tool_errors": tool_errors,
        "unnecessary_tool_calls": unnecessary,
        "redundant_reads": redundant_reads,
        "redundant_searches": redundant_searches,
        "duplicate_tool_calls": duplicate_tool_calls,
        "useful_actions": useful_actions,
        "writes": writes,
        "tests_run": tests_run,
        "files_read": files_read.len() as u64,
        "files_modified": files_modified.len() as u64,
        "files_read_paths": files_read_paths,
        "files_modified_paths": files_modified_paths,
        "model_turns": model_turns,
        "tokens_prompt": "unknown",
        "tokens_completion": "unknown",
        "tokens_total": "unknown",
        "tool_schema_bytes": "unknown",
        "context_bytes": "unknown",
        "wall_time_ms": wall_time_ms,
        "time_to_first_useful_action_ms": time_to_first_useful,
        "time_to_success_ms": time_to_success,
        "source_run_jsonl": meta.source_run_jsonl,
        "metrics_source": "replay_aggregate_v1",
        "notes": "Heuristic aggregate from run.jsonl. unnecessary_tool_calls = redundant_reads + redundant_searches only (not full M1 oracle). No beat-fx claim."
    })
}

pub fn write_metrics(path: &Path, metrics: &Value) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("metrics dir: {e}"))?;
    }
    let text = serde_json::to_string_pretty(metrics).map_err(|e| format!("serialize metrics: {e}"))?;
    std::fs::write(path, text + "\n").map_err(|e| format!("write metrics: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use harness_core::RunEvent;
    use serde_json::json;

    #[test]
    fn counts_tools_and_denies() {
        let events = vec![
            RunEvent::model_request(1, 2),
            RunEvent::model_response(None, vec![]),
            RunEvent::tool_call("1".into(), "read_file".into(), json!({"path": "a.rs"})),
            RunEvent::tool_result("1".into(), "read_file".into(), "ok".into(), true),
            RunEvent::tool_call("2".into(), "read_file".into(), json!({"path": "a.rs"})),
            RunEvent::tool_result("2".into(), "read_file".into(), "ok".into(), true),
            RunEvent::tool_call("3".into(), "shell".into(), json!({})),
            RunEvent::tool_result(
                "3".into(),
                "shell".into(),
                "DENIED: no".into(),
                false,
            ),
            RunEvent::tool_call("4".into(), "run_tests".into(), json!({})),
            RunEvent::tool_result("4".into(), "run_tests".into(), "ok".into(), true),
            RunEvent::done("Succeeded", 2),
        ];
        let mut meta = MetricsMeta::default();
        meta.task_id = "T1/test".into();
        meta.fixture_path = "native/fixtures/x".into();
        meta.model_mode = "scripted".into();
        meta.source_run_jsonl = "/tmp/run.jsonl".into();
        let m = aggregate(&events, &meta);
        assert_eq!(m["schema_version"], "phase3.v1");
        assert_eq!(m["total_tool_calls"], 4);
        assert_eq!(m["redundant_reads"], 1);
        assert_eq!(m["gate_denies"], 1);
        assert_eq!(m["task_success"], true);
        assert_eq!(m["tests_run"], 1);
        assert!(m.get("score").is_none());
    }

    #[test]
    fn passing_tests_do_not_overwrite_max_steps_stop_reason() {
        let events = vec![
            RunEvent::model_request(1, 2),
            RunEvent::model_response(None, vec![]),
            RunEvent::tool_call("1".into(), "run_tests".into(), json!({})),
            RunEvent::tool_result("1".into(), "run_tests".into(), "ok".into(), true),
            RunEvent::done("MaxSteps", 24),
        ];
        let m = aggregate(&events, &MetricsMeta::default());
        assert_eq!(m["exit_status"], "max_turns");
        assert_eq!(m["task_success"], true);
        assert_eq!(m["run_ok"], true);
    }
}
