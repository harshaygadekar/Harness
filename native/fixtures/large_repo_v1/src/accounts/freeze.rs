//! Freeze helpers for the accounts area.

pub fn freeze_label(id: u32) -> String {
    format!("freeze-{:02}", id)
}

/// Returns true when `x` lies within the configured window for freeze.
pub fn within_window(x: i64) -> bool {
    (-20..=20).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(freeze_label(7).len(), "freeze-".len() + 2);
    }
}
