//! Statement helpers for the accounts area.

pub fn statement_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 6).sum()
}

/// Returns true when `x` lies within the configured window for statement.
pub fn within_window(x: i64) -> bool {
    (-60..=60).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(statement_score(&[1, 2, 3]), 36);
    }
}
