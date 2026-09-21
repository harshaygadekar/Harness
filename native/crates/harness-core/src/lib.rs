//! Tiny native agent core (ADR-005).
//! Deny-all tool gate by default. No shell/fs/git in core.
//! No provider HTTP in core.

mod events;
mod gate;
mod messages;
mod model;
mod runner;
mod tool;

pub use events::{bound_observation, RunEvent, MAX_OBS_BYTES};
pub use gate::{AllowDecision, AllowList, DenyAll, ToolGate};
pub use messages::{Message, Role, ToolCall};
pub use model::{FakeModel, ModelClient, ModelResponse};
pub use runner::{RunConfig, RunResult, RunStatus, Runner};
pub use tool::{FunctionTool, Tool, ToolError, ToolResult, ToolSpec};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
