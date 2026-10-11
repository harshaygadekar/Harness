//! Invoice line computation.

use crate::billing::period::Period;
use crate::billing::proration::engine::apply;
use crate::billing::proration::policy::Policy;

/// One invoice line for a subscription that was active `days_active` days in `period`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub period: Period,
    pub monthly_cents: u64,
    pub days_active: u64,
}

impl Line {
    /// Amount due in cents, rounded down to whole cents.
    pub fn amount_due(&self) -> u64 {
        apply(Policy::default(), self.monthly_cents, self.days_active, self.period.days())
    }
}

/// Sum of all line amounts.
pub fn total_due(lines: &[Line]) -> u64 {
    lines.iter().map(Line::amount_due).sum()
}
