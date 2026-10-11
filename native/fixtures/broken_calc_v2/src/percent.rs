//! Percentage helper.

/// Return `part` as a percentage of `whole`, rounded to nearest integer
/// (half away from zero). Return `None` when `whole == 0`.
pub fn percent(part: i32, whole: i32) -> Option<i32> {
    if whole == 0 {
        return Some(0);
    }
    Some((part * 100) / whole)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_basic() {
        assert_eq!(percent(1, 4), Some(25));
        assert_eq!(percent(1, 2), Some(50));
    }

    #[test]
    fn percent_zero_whole_is_none() {
        assert_eq!(percent(5, 0), None);
    }

    #[test]
    fn percent_rounds_half_away() {
        // 2/3 * 100 = 66.666... → nearest is 67
        assert_eq!(percent(2, 3), Some(67));
        // 1/3 * 100 = 33.333... → nearest is 33
        assert_eq!(percent(1, 3), Some(33));
    }

    #[test]
    fn percent_negative_rounds_half_away() {
        // -2/3 * 100 = -66.666... → half away from zero → -67
        assert_eq!(percent(-2, 3), Some(-67));
    }
}
