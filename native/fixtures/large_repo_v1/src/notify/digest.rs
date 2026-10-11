//! Digest helpers for the notify area.

pub fn digest_label(id: u32) -> String {
    format!("digest-{:03}", id)
}

/// Returns true when `x` lies within the configured window for digest.
pub fn within_window(x: i64) -> bool {
    (-30..=30).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(digest_label(7).len(), "digest-".len() + 3);
    }
}
