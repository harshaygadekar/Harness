//! Wal helpers for the storage area.

pub fn wal_bucket(x: u64) -> u64 {
    (x + 7 - 1) / 7
}

/// Returns true when `x` lies within the configured window for wal.
pub fn within_window(x: i64) -> bool {
    (-70..=70).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(wal_bucket(70 + 1), 11);
    }
}
