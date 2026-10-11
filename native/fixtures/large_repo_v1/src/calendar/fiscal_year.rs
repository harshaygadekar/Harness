//! Fiscal year helpers for the calendar area.

pub fn fiscal_year_label(id: u32) -> String {
    format!("fiscal_year-{:04}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(fiscal_year_label(7).len(), "fiscal_year-".len() + 4);
    }
}
