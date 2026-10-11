//! Coupon helpers for the billing area.

pub fn coupon_bucket(x: u64) -> u64 {
    (x + 3 - 1) / 3
}

/// Returns true when `x` lies within the configured window for coupon.
pub fn within_window(x: i64) -> bool {
    (-30..=30).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(coupon_bucket(30 + 1), 11);
    }
}
