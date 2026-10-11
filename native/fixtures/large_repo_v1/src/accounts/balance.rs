//! Balance helpers for the accounts area.

pub fn balance_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(balance_score(&[1, 2, 3]), 12);
    }
}
