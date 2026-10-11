//! Index key helpers for the storage area.

pub fn index_key_bucket(x: u64) -> u64 {
    (x + 5 - 1) / 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(index_key_bucket(50 + 1), 11);
    }
}
