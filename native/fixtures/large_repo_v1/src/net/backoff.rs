//! Backoff helpers for the net area.

pub fn backoff_label(id: u32) -> String {
    format!("backoff-{:05}", id)
}

/// Returns true when `x` lies within the configured window for backoff.
pub fn within_window(x: i64) -> bool {
    (-50..=50).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(backoff_label(7).len(), "backoff-".len() + 5);
    }
}
