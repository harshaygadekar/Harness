//! Currency helpers for the billing area.

pub fn currency_label(id: u32) -> String {
    format!("currency-{:06}", id)
}

/// Returns true when `x` lies within the configured window for currency.
pub fn within_window(x: i64) -> bool {
    (-60..=60).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(currency_label(7).len(), "currency-".len() + 6);
    }
}
