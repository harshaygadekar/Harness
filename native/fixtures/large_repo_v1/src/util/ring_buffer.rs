//! Ring buffer helpers for the util area.

pub fn ring_buffer_bucket(x: u64) -> u64 {
    (x + 7 - 1) / 7
}

/// Returns true when `x` lies within the configured window for ring buffer.
pub fn within_window(x: i64) -> bool {
    (-70..=70).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(ring_buffer_bucket(70 + 1), 11);
    }
}
