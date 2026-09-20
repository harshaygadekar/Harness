use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("tool failed: {0}")]
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct ToolResult {
    pub content: String,
    pub ok: bool,
}

pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> Value;
    fn call(&self, args: &Value) -> Result<ToolResult, ToolError>;
}

pub struct FunctionTool<F>
where
    F: Fn(&Value) -> Result<ToolResult, ToolError> + Send + Sync,
{
    pub name: String,
    pub description: String,
    pub parameters: Value,
    pub handler: F,
}

impl<F> Tool for FunctionTool<F>
where
    F: Fn(&Value) -> Result<ToolResult, ToolError> + Send + Sync,
{
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn parameters(&self) -> Value {
        self.parameters.clone()
    }
    fn call(&self, args: &Value) -> Result<ToolResult, ToolError> {
        (self.handler)(args)
    }
}
