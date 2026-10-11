//! Holiday helpers for the calendar area.

pub fn holiday_label(id: u32) -> String {
    format!("holiday-{:03}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(holiday_label(7).len(), "holiday-".len() + 3);
    }
}
