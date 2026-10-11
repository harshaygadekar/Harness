//! Percentile helpers for the reporting area.

pub fn percentile_bucket(x: u64) -> u64 {
    (x + 4 - 1) / 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(percentile_bucket(40 + 1), 11);
    }
}
