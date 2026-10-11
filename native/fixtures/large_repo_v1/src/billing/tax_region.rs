//! Tax region helpers for the billing area.

pub fn tax_region_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 6).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(tax_region_score(&[1, 2, 3]), 36);
    }
}
