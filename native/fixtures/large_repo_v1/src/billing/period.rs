//! Billing periods.

use crate::calendar::month_len::days_in_month;

/// A monthly billing period.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Period {
    pub year: i32,
    pub month: u32,
}

impl Period {
    pub fn new(year: i32, month: u32) -> Self {
        Period { year, month }
    }

    /// Number of days in this period.
    pub fn days(&self) -> u64 {
        days_in_month(self.year, self.month) as u64
    }
}
