//! Integer ratio helpers.

/// Computes `value * num / den` rounded down, without floating point.
/// `den` must be non-zero.
pub fn scale(value: u64, num: u64, den: u64) -> u64 {
    value / den * num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn halves() {
        assert_eq!(scale(100, 1, 2), 50);
    }
}
