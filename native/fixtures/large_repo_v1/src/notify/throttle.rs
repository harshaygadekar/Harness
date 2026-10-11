//! Throttle helpers for the notify area.

pub fn throttle_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 7).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(throttle_score(&[1, 2, 3]), 42);
    }
}
