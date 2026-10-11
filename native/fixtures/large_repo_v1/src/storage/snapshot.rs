//! Snapshot helpers for the storage area.

pub fn snapshot_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 5).sum()
}

/// Returns true when `x` lies within the configured window for snapshot.
pub fn within_window(x: i64) -> bool {
    (-50..=50).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(snapshot_score(&[1, 2, 3]), 30);
    }
}
