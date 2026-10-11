//! Trend helpers for the reporting area.

pub fn trend_bucket(x: u64) -> u64 {
    (x + 6 - 1) / 6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(trend_bucket(60 + 1), 11);
    }
}
