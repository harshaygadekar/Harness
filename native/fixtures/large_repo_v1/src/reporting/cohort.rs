//! Cohort helpers for the reporting area.

pub fn cohort_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 5).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(cohort_score(&[1, 2, 3]), 30);
    }
}
