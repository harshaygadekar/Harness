//! Timezone offset helpers for the calendar area.

pub fn timezone_offset_label(id: u32) -> String {
    format!("timezone_offset-{:02}", id)
}

/// Returns true when `x` lies within the configured window for timezone offset.
pub fn within_window(x: i64) -> bool {
    (-20..=20).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(timezone_offset_label(7).len(), "timezone_offset-".len() + 2);
    }
}
