//! Quarter helpers for the calendar area.

pub fn quarter_bucket(x: u64) -> u64 {
    (x + 5 - 1) / 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(quarter_bucket(50 + 1), 11);
    }
}
