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
            vec![tc("4", "run_tests", serde_json::json!({}))],
        ],
        "Fixed add(2,2). Tests pass.",
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
