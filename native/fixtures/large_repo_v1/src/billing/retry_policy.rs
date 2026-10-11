//! Retry policy helpers for the billing area.

pub fn retry_policy_label(id: u32) -> String {
    format!("retry_policy-{:02}", id)
}

/// Returns true when `x` lies within the configured window for retry policy.
pub fn within_window(x: i64) -> bool {
    (-20..=20).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(retry_policy_label(7).len(), "retry_policy-".len() + 2);
    }
}
