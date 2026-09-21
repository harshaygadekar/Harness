//! JSON-serializable run events for record / replay.

use serde::{Deserialize, Serialize};

use crate::messages::ToolCall;

/// Max tool observation size after scrub (32 KiB).
pub const MAX_OBS_BYTES: usize = 32 * 1024;

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RunEvent {
    ModelRequest {
        ts_ms: u64,
        message_count: usize,
        tool_count: usize,
    },
    ModelResponse {
        ts_ms: u64,
        content: Option<String>,
        tool_calls: Vec<ToolCall>,
    },
    ToolCall {
        ts_ms: u64,
        id: String,
        name: String,
        arguments: serde_json::Value,
    },
    ToolResult {
        ts_ms: u64,
        id: String,
        name: String,
        content: String,
        ok: bool,
    },
    Error {
        ts_ms: u64,
        message: String,
    },
    StateChange {
        ts_ms: u64,
        status: String,
    },
    Done {
        ts_ms: u64,
        status: String,
        steps: usize,
    },
}

impl RunEvent {
    pub fn model_request(message_count: usize, tool_count: usize) -> Self {
        Self::ModelRequest {
            ts_ms: now_ms(),
            message_count,
            tool_count,
        }
    }

    pub fn model_response(content: Option<String>, tool_calls: Vec<ToolCall>) -> Self {
        Self::ModelResponse {
            ts_ms: now_ms(),
            content,
            tool_calls,
        }
    }

    pub fn tool_call(id: String, name: String, arguments: serde_json::Value) -> Self {
        Self::ToolCall {
            ts_ms: now_ms(),
            id,
            name,
            arguments,
        }
    }

    pub fn tool_result(id: String, name: String, content: String, ok: bool) -> Self {
        Self::ToolResult {
            ts_ms: now_ms(),
            id,
            name,
            content,
            ok,
        }
    }

    pub fn error(message: String) -> Self {
        Self::Error {
            ts_ms: now_ms(),
            message,
        }
    }

    pub fn state_change(status: impl Into<String>) -> Self {
        Self::StateChange {
            ts_ms: now_ms(),
            status: status.into(),
        }
    }

    pub fn done(status: impl Into<String>, steps: usize) -> Self {
        Self::Done {
            ts_ms: now_ms(),
            status: status.into(),
            steps,
        }
    }
}

/// Truncate observation text to MAX_OBS_BYTES with a note.
pub fn bound_observation(s: &str) -> String {
    if s.len() <= MAX_OBS_BYTES {
        return s.to_string();
    }
    let mut out = s[..MAX_OBS_BYTES].to_string();
    out.push_str(&format!(
        "\n…[truncated; full length {} bytes]",
        s.len()
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn events_roundtrip_json() {
        let e = RunEvent::done("Succeeded", 3);
        let s = serde_json::to_string(&e).unwrap();
        let back: RunEvent = serde_json::from_str(&s).unwrap();
        match back {
            RunEvent::Done { status, steps, .. } => {
                assert_eq!(status, "Succeeded");
                assert_eq!(steps, 3);
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn bound_truncates() {
        let big = "x".repeat(MAX_OBS_BYTES + 100);
        let out = bound_observation(&big);
        assert!(out.len() < big.len());
        assert!(out.contains("truncated"));
    }
}
