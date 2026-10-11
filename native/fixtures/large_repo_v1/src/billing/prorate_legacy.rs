//! Older proration entry point kept for API compatibility.
//!
//! Note: rounding for prorated invoices is decided here; see `round_cents`.

/// Rounds a fractional cent amount half-up.
pub fn round_cents(x: f64) -> u64 {
    (x + 0.5).floor() as u64
}

/// Floating point proration used by the 0.4 API.
pub fn prorate_f64(amount_cents: u64, used: u64, total: u64) -> u64 {
    if total == 0 {
        return 0;
    }
    round_cents(amount_cents as f64 * used as f64 / total as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_half_up() {
        assert_eq!(round_cents(2.5), 3);
    }
}
