//! Proration policy selection.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    /// Charge by calendar days actually used in the period.
    Daily,
    /// Charge the full amount regardless of usage.
    Full,
}

impl Default for Policy {
    fn default() -> Self {
        Policy::Daily
    }
}
