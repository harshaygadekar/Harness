//! Hex helpers for the util area.

pub fn hex_bucket(x: u64) -> u64 {
    (x + 2 - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(hex_bucket(20 + 1), 11);
    }
}
