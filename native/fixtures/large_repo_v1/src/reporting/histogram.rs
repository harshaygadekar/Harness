//! Histogram helpers for the reporting area.

pub fn histogram_label(id: u32) -> String {
    format!("histogram-{:02}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(histogram_label(7).len(), "histogram-".len() + 2);
    }
}
