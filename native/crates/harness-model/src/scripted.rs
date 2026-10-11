//! Fixed ModelResponse sequence for CI / vertical-slice proof.

use harness_core::{Message, ModelClient, ModelResponse, Role, ToolCall, ToolSpec};

/// Plays a fixed list of responses in order. Extra calls get a terminal "done".
pub struct ScriptedModel {
    responses: Vec<ModelResponse>,
    index: usize,
}

impl ScriptedModel {
    pub fn new(responses: Vec<ModelResponse>) -> Self {
        Self {
            responses,
            index: 0,
        }
    }

    pub fn from_tool_then_text(calls: Vec<Vec<ToolCall>>, finals: &str) -> Self {
        let mut responses: Vec<ModelResponse> = calls
            .into_iter()
            .map(|tool_calls| ModelResponse {
                message: Message {
                    role: Role::Assistant,
                    content: None,
                    tool_calls,
                    tool_call_id: None,
                    name: None,
                },
            })
            .collect();
        responses.push(ModelResponse {
            message: Message {
                role: Role::Assistant,
                content: Some(finals.to_string()),
                tool_calls: vec![],
                tool_call_id: None,
                name: None,
            },
        });
        Self::new(responses)
    }
}

impl ModelClient for ScriptedModel {
    fn complete(&mut self, _messages: &[Message], _tools: &[ToolSpec]) -> ModelResponse {
        if self.index < self.responses.len() {
            let r = self.responses[self.index].clone();
            self.index += 1;
            r
        } else {
            ModelResponse {
                message: Message {
                    role: Role::Assistant,
                    content: Some("done (scripted; no more steps)".into()),
                    tool_calls: vec![],
                    tool_call_id: None,
                    name: None,
                },
            }
        }
    }
}

fn tc(id: &str, name: &str, args: serde_json::Value) -> ToolCall {
    ToolCall {
        id: id.into(),
        name: name.into(),
        arguments: args,
        extra_content: None,
    }
}

/// Scripted sequence for `fixtures/broken_add`:
/// search → read → apply_patch → run_tests → success text.
pub fn broken_add_fix_script() -> ScriptedModel {
    ScriptedModel::from_tool_then_text(
        vec![
            vec![tc(
                "1",
                "search",
                serde_json::json!({"pattern": "add"}),
            )],
            vec![tc(
                "2",
                "read_file",
                serde_json::json!({"path": "src/lib.rs"}),
            )],
            vec![tc(
                "3",
                "apply_patch",
                serde_json::json!({
                    "path": "src/lib.rs",
                    "old_string": "a + b + 1",
                    "new_string": "a + b"
                }),
            )],
            vec![tc(
                "4",
                "run_tests",
                serde_json::json!({}),
            )],
        ],
        "Fixed add(2,2). Tests pass.",
    )
}


/// Scripted sequence for `fixtures/broken_calc` (Track A / T1):
/// search → read percent + midpoint → apply_patch both → run_tests → success text.
/// Avoids decoys under docs/, notes/, data/, tax.rs, noise.rs.
pub fn broken_calc_fix_script() -> ScriptedModel {
    ScriptedModel::from_tool_then_text(
        vec![
            vec![tc(
                "1",
                "search",
                serde_json::json!({"pattern": "fn percent", "path": "src"}),
            )],
            vec![tc(
                "2",
                "search",
                serde_json::json!({"pattern": "fn midpoint", "path": "src"}),
            )],
            vec![tc(
                "3",
                "read_file",
                serde_json::json!({"path": "src/percent.rs"}),
            )],
            vec![tc(
                "4",
                "read_file",
                serde_json::json!({"path": "src/midpoint.rs"}),
            )],
            vec![tc(
                "5",
                "apply_patch",
                serde_json::json!({
                    "path": "src/percent.rs",
                    "old_string": "    if whole == 0 {\n        // BUG: should return None\n        return Some(0);\n    }\n    // BUG: truncates toward zero; no half-away rounding\n    Some((part * 100) / whole)",
                    "new_string": "    if whole == 0 {\n        return None;\n    }\n    Some(((f64::from(part) * 100.0) / f64::from(whole)).round() as i32)"
                }),
            )],
            vec![tc(
                "6",
                "apply_patch",
                serde_json::json!({
                    "path": "src/midpoint.rs",
                    "old_string": "    // BUG: overflowing add for e.g. lo = i32::MAX - 2, hi = i32::MAX\n    (lo + hi) / 2",
                    "new_string": "    lo + (hi - lo) / 2"
                }),
            )],
            vec![tc(
                "7",
                "run_tests",
                serde_json::json!({}),
            )],
        ],
        "Fixed percent (zero-whole + half-away round) and overflow-safe midpoint. Tests pass.",
    )
}

/// Scripted sequence for `fixtures/broken_calc_v2` (Track A / T1 v2, no hints).
/// Same steps as v1; patch anchors match the v2 source (BUG comments removed).
pub fn broken_calc_v2_fix_script() -> ScriptedModel {
    ScriptedModel::from_tool_then_text(
        vec![
            vec![tc(
                "1",
                "search",
                serde_json::json!({"pattern": "fn percent", "path": "src"}),
            )],
            vec![tc(
                "2",
                "search",
                serde_json::json!({"pattern": "fn midpoint", "path": "src"}),
            )],
            vec![tc(
                "3",
                "read_file",
                serde_json::json!({"path": "src/percent.rs"}),
            )],
            vec![tc(
                "4",
                "read_file",
                serde_json::json!({"path": "src/midpoint.rs"}),
            )],
            vec![tc(
                "5",
                "apply_patch",
                serde_json::json!({
                    "path": "src/percent.rs",
                    "old_string": "    if whole == 0 {\n        return Some(0);\n    }\n    Some((part * 100) / whole)",
                    "new_string": "    if whole == 0 {\n        return None;\n    }\n    Some(((f64::from(part) * 100.0) / f64::from(whole)).round() as i32)"
                }),
            )],
            vec![tc(
                "6",
                "apply_patch",
                serde_json::json!({
                    "path": "src/midpoint.rs",
                    "old_string": "    (lo + hi) / 2",
                    "new_string": "    lo + (hi - lo) / 2"
                }),
            )],
            vec![tc(
                "7",
                "run_tests",
                serde_json::json!({}),
            )],
        ],
        "Fixed percent (zero-whole + half-away round) and overflow-safe midpoint. Tests pass.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn script_advances() {
        let mut m = ScriptedModel::from_tool_then_text(
            vec![vec![tc("1", "echo", serde_json::json!({"text":"x"}))]],
            "ok",
        );
        let r1 = m.complete(&[], &[]);
        assert_eq!(r1.message.tool_calls.len(), 1);
        let r2 = m.complete(&[], &[]);
        assert_eq!(r2.message.content.as_deref(), Some("ok"));
    }
}
