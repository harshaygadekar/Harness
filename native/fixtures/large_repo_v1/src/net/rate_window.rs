//! Rate window helpers for the net area.

pub fn rate_window_bucket(x: u64) -> u64 {
    (x + 3 - 1) / 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(rate_window_bucket(30 + 1), 11);
    }
}
