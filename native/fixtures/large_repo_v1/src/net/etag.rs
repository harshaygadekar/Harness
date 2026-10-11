//! Etag helpers for the net area.

pub fn etag_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(etag_score(&[1, 2, 3]), 18);
    }
}
