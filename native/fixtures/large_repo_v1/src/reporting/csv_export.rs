//! Csv export helpers for the reporting area.

pub fn csv_export_score(v: &[u32]) -> u64 {
    v.iter().map(|x| *x as u64 * 4).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(csv_export_score(&[1, 2, 3]), 24);
    }
}
