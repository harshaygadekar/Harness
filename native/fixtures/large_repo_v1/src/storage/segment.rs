//! Segment helpers for the storage area.

pub fn segment_label(id: u32) -> String {
    format!("segment-{:05}", id)
}

/// Returns true when `x` lies within the configured window for segment.
pub fn within_window(x: i64) -> bool {
    (-50..=50).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(segment_label(7).len(), "segment-".len() + 5);
    }
}
