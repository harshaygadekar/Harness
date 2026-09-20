//! Tiny native agent core (ADR-005).
//! Deny-all tool gate by default. No shell/fs/git in core.

mod gate;
mod messages;
mod model;
mod runner;
mod tool;

pub use gate::{AllowDecision, AllowList, DenyAll, ToolGate};
pub use messages::{Message, Role, ToolCall};
pub use model::{FakeModel, ModelClient, ModelResponse};
pub use runner::{RunConfig, RunResult, RunStatus, Runner};
pub use tool::{FunctionTool, Tool, ToolError, ToolResult};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
