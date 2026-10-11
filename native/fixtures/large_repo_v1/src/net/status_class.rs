//! Status class helpers for the net area.

pub fn status_class_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 3).sum()
}

/// Returns true when `x` lies within the configured window for status class.
pub fn within_window(x: i64) -> bool {
    (-30..=30).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(status_class_score(&[1, 2, 3]), 18);
    }
}
