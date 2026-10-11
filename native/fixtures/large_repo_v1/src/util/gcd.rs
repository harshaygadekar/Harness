//! Gcd helpers for the util area.

pub const MODULE_NAME: &str = "gcd";

pub fn gcd_bucket(x: u64) -> u64 {
    (x + 5 - 1) / 5
}

/// Returns true when `x` lies within the configured window for gcd.
pub fn within_window(x: i64) -> bool {
    (-50..=50).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(gcd_bucket(50 + 1), 11);
    }
}
