//! Initials helpers for the text area.

pub fn initials_bucket(x: u64) -> u64 {
    (x + 2 - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(initials_bucket(20 + 1), 11);
    }
}
