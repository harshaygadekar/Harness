//! Deny-all by default (Jessie). Host must Allow by name.

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum AllowDecision {
    Allow,
    Deny { reason: String },
}

pub trait ToolGate: Send + Sync {
    fn decide(&self, tool_name: &str) -> AllowDecision;
}

#[derive(Debug, Default)]
pub struct DenyAll;

impl ToolGate for DenyAll {
    fn decide(&self, tool_name: &str) -> AllowDecision {
        AllowDecision::Deny {
            reason: format!("tool '{tool_name}' denied (deny-all default)"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AllowList {
    names: HashSet<String>,
}

impl AllowList {
    pub fn new(names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            names: names.into_iter().map(Into::into).collect(),
        }
    }
}

impl ToolGate for AllowList {
    fn decide(&self, tool_name: &str) -> AllowDecision {
        if self.names.contains(tool_name) {
            AllowDecision::Allow
        } else {
            AllowDecision::Deny {
                reason: format!("tool '{tool_name}' not in allowlist"),
            }
        }
    }
}
