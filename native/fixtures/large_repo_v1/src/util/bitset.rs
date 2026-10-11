//! Bitset helpers for the util area.

pub fn bitset_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 2).sum()
}

/// Returns true when `x` lies within the configured window for bitset.
pub fn within_window(x: i64) -> bool {
    (-20..=20).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(bitset_score(&[1, 2, 3]), 12);
    }
}
