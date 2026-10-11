//! Token shape helpers for the auth area.

pub fn token_shape_label(id: u32) -> String {
    format!("token_shape-{:03}", id)
}

/// Returns true when `x` lies within the configured window for token shape.
pub fn within_window(x: i64) -> bool {
    (-30..=30).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(token_shape_label(7).len(), "token_shape-".len() + 3);
    }
}
