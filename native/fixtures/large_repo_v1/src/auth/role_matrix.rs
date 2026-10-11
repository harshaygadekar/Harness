//! Role matrix helpers for the auth area.

pub fn role_matrix_bucket(x: u64) -> u64 {
    (x + 6 - 1) / 6
}

/// Returns true when `x` lies within the configured window for role matrix.
pub fn within_window(x: i64) -> bool {
    (-60..=60).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(role_matrix_bucket(60 + 1), 11);
    }
}
