//! Business days helpers for the calendar area.

pub fn business_days_label(id: u32) -> String {
    format!("business_days-{:02}", id)
}

/// Returns true when `x` lies within the configured window for business days.
pub fn within_window(x: i64) -> bool {
    (-20..=20).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(business_days_label(7).len(), "business_days-".len() + 2);
    }
}
