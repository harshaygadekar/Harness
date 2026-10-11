//! Moving avg helpers for the util area.

pub fn moving_avg_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 6).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(moving_avg_score(&[1, 2, 3]), 36);
    }
}
