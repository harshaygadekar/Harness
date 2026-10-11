//! Pivot helpers for the reporting area.

pub fn pivot_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(pivot_score(&[1, 2, 3]), 12);
    }
}
