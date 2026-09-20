use crate::messages::{Message, ToolCall};

pub struct ModelResponse {
    pub message: Message,
}

pub trait ModelClient: Send + Sync {
    fn complete(&mut self, messages: &[Message], tool_names: &[String]) -> ModelResponse;
}

/// Deterministic fake model for benches / CI (no network).
pub struct FakeModel {
    step: usize,
}

impl FakeModel {
    pub fn new() -> Self {
        Self { step: 0 }
    }
}

impl Default for FakeModel {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelClient for FakeModel {
    fn complete(&mut self, _messages: &[Message], tool_names: &[String]) -> ModelResponse {
        self.step += 1;
        if self.step == 1 && tool_names.iter().any(|n| n == "echo") {
            return ModelResponse {
                message: Message {
                    role: crate::messages::Role::Assistant,
                    content: None,
                    tool_calls: vec![ToolCall {
                        id: "1".into(),
                        name: "echo".into(),
                        arguments: serde_json::json!({"text": "hi"}),
                    }],
                    tool_call_id: None,
                    name: None,
                },
            };
        }
        ModelResponse {
            message: Message {
                role: crate::messages::Role::Assistant,
                content: Some("done (fake model)".into()),
                tool_calls: vec![],
                tool_call_id: None,
                name: None,
            },
        }
    }
}
