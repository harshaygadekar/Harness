//! Wrap helpers for the text area.

pub fn wrap_label(id: u32) -> String {
    format!("wrap-{:03}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(wrap_label(7).len(), "wrap-".len() + 3);
    }
}
