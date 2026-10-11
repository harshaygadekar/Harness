//! Clamp helpers for the util area.

pub fn clamp_label(id: u32) -> String {
    format!("clamp-{:07}", id)
}

/// Returns true when `x` lies within the configured window for clamp.
pub fn within_window(x: i64) -> bool {
    (-70..=70).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(clamp_label(7).len(), "clamp-".len() + 7);
    }
}
