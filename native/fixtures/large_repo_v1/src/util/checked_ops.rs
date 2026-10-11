//! Checked ops helpers for the util area.

pub fn checked_ops_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 6).sum()
}

/// Returns true when `x` lies within the configured window for checked ops.
pub fn within_window(x: i64) -> bool {
    (-60..=60).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(checked_ops_score(&[1, 2, 3]), 36);
    }
}
