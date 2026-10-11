//! Receipt helpers for the billing area.

pub fn receipt_bucket(x: u64) -> u64 {
    (x + 3 - 1) / 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(receipt_bucket(30 + 1), 11);
    }
}
