//! Bloom helpers for the storage area.

pub fn bloom_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 4).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(bloom_score(&[1, 2, 3]), 24);
    }
}
