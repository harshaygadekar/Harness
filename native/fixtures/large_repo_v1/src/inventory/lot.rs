//! Lot helpers for the inventory area.

pub fn lot_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 4).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(lot_score(&[1, 2, 3]), 24);
    }
}
