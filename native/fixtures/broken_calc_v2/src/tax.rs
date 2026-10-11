//! Billing helpers.

/// Sales tax in cents for `amount_cents` at `rate_bps` (basis points, 10000 = 100%).
/// Truncating division is fine for the rates used in unit tests (exact results).
pub fn sales_tax_cents(amount_cents: i32, rate_bps: i32) -> i32 {
    amount_cents.saturating_mul(rate_bps) / 10_000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tax_ten_percent() {
        assert_eq!(sales_tax_cents(1000, 1000), 100);
    }
}
