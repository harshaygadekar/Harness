//! Midpoint of an inclusive integer range.

/// Return the midpoint of `[lo, hi]` (inclusive), rounding toward `-∞` on ties.
/// Must not overflow when `lo` and `hi` are near `i32` extremes.
pub fn midpoint(lo: i32, hi: i32) -> i32 {
    (lo + hi) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn midpoint_simple() {
        assert_eq!(midpoint(0, 10), 5);
        assert_eq!(midpoint(-3, 5), 1);
    }

    #[test]
    fn midpoint_no_overflow_near_max() {
        let lo = i32::MAX - 4;
        let hi = i32::MAX;
        assert_eq!(midpoint(lo, hi), i32::MAX - 2);
    }

    #[test]
    fn midpoint_no_overflow_near_min() {
        let lo = i32::MIN;
        let hi = i32::MIN + 4;
        assert_eq!(midpoint(lo, hi), i32::MIN + 2);
    }
}
