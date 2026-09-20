use std::collections::HashMap;
use std::sync::Arc;

use crate::gate::{AllowDecision, ToolGate};
use crate::messages::{Message, Role};
use crate::model::ModelClient;
use crate::tool::Tool;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStatus {
    Succeeded,
    Failed,
    MaxSteps,
}

#[derive(Debug)]
pub struct RunResult {
    pub status: RunStatus,
    pub messages: Vec<Message>,
    pub steps: usize,
}

pub struct RunConfig {
    pub max_steps: usize,
}

impl Default for RunConfig {
    fn default() -> Self {
        Self { max_steps: 8 }
    }
}

pub struct Runner {
    tools: HashMap<String, Arc<dyn Tool>>,
    gate: Arc<dyn ToolGate>,
}

impl Runner {
    pub fn new(gate: Arc<dyn ToolGate>) -> Self {
        Self {
            tools: HashMap::new(),
            gate,
        }
    }

    pub fn register_tool(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn run(
        &self,
        model: &mut dyn ModelClient,
        user: &str,
        config: RunConfig,
    ) -> RunResult {
        let mut messages = vec![Message {
            role: Role::User,
            content: Some(user.to_string()),
            tool_calls: vec![],
            tool_call_id: None,
            name: None,
        }];
        let tool_names: Vec<String> = self.tools.keys().cloned().collect();

        for step in 0..config.max_steps {
            let response = model.complete(&messages, &tool_names);
            let msg = response.message;
            let calls = msg.tool_calls.clone();
            messages.push(msg);

            if calls.is_empty() {
                return RunResult {
                    status: RunStatus::Succeeded,
                    messages,
                    steps: step + 1,
                };
            }

            for call in calls {
                match self.gate.decide(&call.name) {
                    AllowDecision::Deny { reason } => {
                        messages.push(Message {
                            role: Role::Tool,
                            content: Some(format!("DENIED: {reason}")),
                            tool_calls: vec![],
                            tool_call_id: Some(call.id),
                            name: Some(call.name),
                        });
                    }
                    AllowDecision::Allow => {
                        let content = match self.tools.get(&call.name) {
                            None => format!("ERROR: unknown tool {}", call.name),
                            Some(tool) => match tool.call(&call.arguments) {
                                Ok(r) => r.content,
                                Err(e) => format!("ERROR: {e}"),
                            },
                        };
                        // light scrub: never echo obvious key prefixes in tool text
                        let content = scrub_secrets(&content);
                        messages.push(Message {
                            role: Role::Tool,
                            content: Some(content),
                            tool_calls: vec![],
                            tool_call_id: Some(call.id),
                            name: Some(call.name),
                        });
                    }
                }
            }
        }

        RunResult {
            status: RunStatus::MaxSteps,
            messages,
            steps: config.max_steps,
        }
    }
}

fn scrub_secrets(s: &str) -> String {
    // Minimal scrub: redact AIza / sk- / Bearer-looking tokens.
    let mut out = s.to_string();
    for pat in ["AIza", "sk-", "Bearer "] {
        if let Some(i) = out.find(pat) {
            let end = (i + 12).min(out.len());
            out.replace_range(i..end, "[REDACTED]");
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::AllowList;
    use crate::model::FakeModel;
    use crate::tool::{FunctionTool, ToolResult};
    use serde_json::json;

    #[test]
    fn deny_all_blocks_tool() {
        let gate = Arc::new(crate::gate::DenyAll);
        let mut runner = Runner::new(gate);
        let echo = Arc::new(FunctionTool {
            name: "echo".into(),
            description: "echo".into(),
            parameters: json!({"type":"object"}),
            handler: |args| {
                Ok(ToolResult {
                    content: args["text"].as_str().unwrap_or("").to_string(),
                    ok: true,
                })
            },
        });
        runner.register_tool(echo);
        let mut model = FakeModel::new();
        let result = runner.run(&mut model, "hi", RunConfig::default());
        assert!(result
            .messages
            .iter()
            .any(|m| m.content.as_deref().unwrap_or("").contains("DENIED")));
    }

    #[test]
    fn allowlist_runs_echo() {
        let gate = Arc::new(AllowList::new(["echo"]));
        let mut runner = Runner::new(gate);
        runner.register_tool(Arc::new(FunctionTool {
            name: "echo".into(),
            description: "echo".into(),
            parameters: json!({"type":"object"}),
            handler: |args| {
                Ok(ToolResult {
                    content: args["text"].as_str().unwrap_or("").to_string(),
                    ok: true,
                })
            },
        }));
        let mut model = FakeModel::new();
        let result = runner.run(&mut model, "hi", RunConfig::default());
        assert_eq!(result.status, RunStatus::Succeeded);
        assert!(result
            .messages
            .iter()
            .any(|m| m.role == Role::Tool && m.content.as_deref() == Some("hi")));
    }
}
