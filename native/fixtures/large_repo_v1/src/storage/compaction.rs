//! Compaction helpers for the storage area.

pub fn compaction_bucket(x: u64) -> u64 {
    (x + 5 - 1) / 5
}

/// Returns true when `x` lies within the configured window for compaction.
pub fn within_window(x: i64) -> bool {
    (-50..=50).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(compaction_bucket(50 + 1), 11);
    }
}
