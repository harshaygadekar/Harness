//! OpenAI-compatible chat completions (non-stream first).
//! Env: HARNESS_API_KEY | OPENAI_API_KEY, HARNESS_BASE_URL, HARNESS_MODEL.
//! Streaming: TODO — wire SSE when non-stream path is solid.

use std::time::Duration;

use harness_core::{Message, ModelClient, ModelResponse, Role, ToolCall, ToolSpec};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const DEFAULT_BASE: &str = "https://api.openai.com/v1";
const DEFAULT_TIMEOUT_SECS: u64 = 60;

#[derive(Debug, Error)]
pub enum OpenAiError {
    #[error("missing API key (set HARNESS_API_KEY or OPENAI_API_KEY)")]
    MissingKey,
    #[error("missing model (set HARNESS_MODEL)")]
    MissingModel,
    #[error("http: {0}")]
    Http(String),
    #[error("parse: {0}")]
    Parse(String),
}

#[derive(Debug, Clone)]
pub struct OpenAiCompat {
    api_key: String,
    base_url: String,
    model: String,
    timeout: Duration,
}

impl OpenAiCompat {
    pub fn from_env() -> Result<Self, OpenAiError> {
        let api_key = std::env::var("HARNESS_API_KEY")
            .or_else(|_| std::env::var("OPENAI_API_KEY"))
            .map_err(|_| OpenAiError::MissingKey)?;
        let base_url = std::env::var("HARNESS_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE.into());
        let model = std::env::var("HARNESS_MODEL").map_err(|_| OpenAiError::MissingModel)?;
        Ok(Self {
            api_key,
            base_url: base_url.trim_end_matches('/').to_string(),
            model,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
        })
    }

    pub fn new(api_key: impl Into<String>, base_url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            model: model.into(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn complete_inner(
        &self,
        messages: &[Message],
        tools: &[ToolSpec],
    ) -> Result<ModelResponse, OpenAiError> {
        let body = ChatRequest {
            model: self.model.clone(),
            messages: messages.iter().map(to_openai_message).collect(),
            tools: if tools.is_empty() {
                None
            } else {
                Some(tools.iter().map(to_openai_tool).collect())
            },
            stream: Some(false),
        };

        let url = format!("{}/chat/completions", self.base_url);
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(10))
            .timeout_read(self.timeout)
            .build();

        let resp = agent
            .post(&url)
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .send_json(&body)
            .map_err(|e| OpenAiError::Http(e.to_string()))?;

        let parsed: ChatResponse = resp
            .into_json()
            .map_err(|e| OpenAiError::Parse(e.to_string()))?;

        let choice = parsed
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| OpenAiError::Parse("no choices".into()))?;

        Ok(ModelResponse {
            message: from_openai_message(choice.message),
        })
    }
}

impl ModelClient for OpenAiCompat {
    fn complete(&mut self, messages: &[Message], tools: &[ToolSpec]) -> ModelResponse {
        match self.complete_inner(messages, tools) {
            Ok(r) => r,
            Err(e) => ModelResponse {
                message: Message {
                    role: Role::Assistant,
                    content: Some(format!("MODEL_ERROR: {e}")),
                    tool_calls: vec![],
                    tool_call_id: None,
                    name: None,
                },
            },
        }
    }
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<OaiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OaiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct OaiMessage {
    role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OaiToolCall>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct OaiToolCall {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    function: OaiFunctionCall,
}

#[derive(Serialize, Deserialize)]
struct OaiFunctionCall {
    name: String,
    arguments: String,
}

#[derive(Serialize)]
struct OaiTool {
    #[serde(rename = "type")]
    type_: String,
    function: OaiFunctionDef,
}

#[derive(Serialize)]
struct OaiFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: OaiMessage,
}

fn to_openai_tool(t: &ToolSpec) -> OaiTool {
    OaiTool {
        type_: "function".into(),
        function: OaiFunctionDef {
            name: t.name.clone(),
            description: t.description.clone(),
            parameters: t.parameters.clone(),
        },
    }
}

fn to_openai_message(m: &Message) -> OaiMessage {
    let role = match m.role {
        Role::System => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    };
    let tool_calls = if m.tool_calls.is_empty() {
        None
    } else {
        Some(
            m.tool_calls
                .iter()
                .map(|c| OaiToolCall {
                    id: c.id.clone(),
                    type_: "function".into(),
                    function: OaiFunctionCall {
                        name: c.name.clone(),
                        arguments: c.arguments.to_string(),
                    },
                })
                .collect(),
        )
    };
    OaiMessage {
        role: role.into(),
        content: m.content.clone(),
        tool_calls,
        tool_call_id: m.tool_call_id.clone(),
        name: m.name.clone(),
    }
}

fn from_openai_message(m: OaiMessage) -> Message {
    let role = match m.role.as_str() {
        "system" => Role::System,
        "user" => Role::User,
        "tool" => Role::Tool,
        _ => Role::Assistant,
    };
    let tool_calls = m
        .tool_calls
        .unwrap_or_default()
        .into_iter()
        .map(|c| {
            let arguments = serde_json::from_str(&c.function.arguments)
                .unwrap_or_else(|_| serde_json::json!({"_raw": c.function.arguments}));
            ToolCall {
                id: c.id,
                name: c.function.name,
                arguments,
            }
        })
        .collect();
    Message {
        role,
        content: m.content,
        tool_calls,
        tool_call_id: m.tool_call_id,
        name: m.name,
    }
}

// TODO(stream): add `complete_stream` with SSE reader when product needs TTFB display.
