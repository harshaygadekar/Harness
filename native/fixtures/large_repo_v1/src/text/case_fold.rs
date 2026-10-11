//! Case fold helpers for the text area.

pub fn case_fold_label(id: u32) -> String {
    format!("case_fold-{:06}", id)
}

/// Returns true when `x` lies within the configured window for case fold.
pub fn within_window(x: i64) -> bool {
    (-60..=60).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(case_fold_label(7).len(), "case_fold-".len() + 6);
    }
}
