//! Tiering helpers for the accounts area.

pub fn tiering_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 6).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(tiering_score(&[1, 2, 3]), 36);
    }
}
