//! Checksum helpers for the storage area.

pub fn checksum_bucket(x: u64) -> u64 {
    (x + 2 - 1) / 2
}

/// Returns true when `x` lies within the configured window for checksum.
pub fn within_window(x: i64) -> bool {
    (-20..=20).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(checksum_bucket(20 + 1), 11);
    }
}
