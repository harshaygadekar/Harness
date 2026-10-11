//! Pad helpers for the text area.

pub fn pad_bucket(x: u64) -> u64 {
    (x + 3 - 1) / 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(pad_bucket(30 + 1), 11);
    }
}
