//! Reorder point helpers for the inventory area.

pub fn reorder_point_bucket(x: u64) -> u64 {
    (x + 6 - 1) / 6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(reorder_point_bucket(60 + 1), 11);
    }
}
