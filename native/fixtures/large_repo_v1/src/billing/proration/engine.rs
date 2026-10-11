//! Applies a proration policy to an amount.

use super::policy::Policy;
use crate::util::ratio::scale;

/// Prorates `amount_cents` for `used` days out of `total` days.
/// The result is rounded down to whole cents.
pub fn apply(policy: Policy, amount_cents: u64, used: u64, total: u64) -> u64 {
    match policy {
        Policy::Full => amount_cents,
        Policy::Daily => {
            if total == 0 {
                return 0;
            }
            scale(amount_cents, used.min(total), total)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_ignores_usage() {
        assert_eq!(apply(Policy::Full, 1234, 1, 30), 1234);
    }

    #[test]
    fn whole_period() {
        assert_eq!(apply(Policy::Daily, 3000, 30, 30), 3000);
    }
}
