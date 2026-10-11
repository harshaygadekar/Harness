//! Valuation helpers for the inventory area.

pub fn valuation_bucket(x: u64) -> u64 {
    (x + 6 - 1) / 6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(valuation_bucket(60 + 1), 11);
    }
}
