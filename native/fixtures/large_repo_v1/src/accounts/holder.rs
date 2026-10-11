//! Holder helpers for the accounts area.

pub fn holder_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 4).sum()
}

/// Returns true when `x` lies within the configured window for holder.
pub fn within_window(x: i64) -> bool {
    (-40..=40).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(holder_score(&[1, 2, 3]), 24);
    }
}
