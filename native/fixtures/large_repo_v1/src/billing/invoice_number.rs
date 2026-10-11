//! Invoice number helpers for the billing area.

pub fn invoice_number_label(id: u32) -> String {
    format!("invoice_number-{:06}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(invoice_number_label(7).len(), "invoice_number-".len() + 6);
    }
}
