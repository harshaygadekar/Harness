//! Tokenize helpers for the text area.

pub fn tokenize_label(id: u32) -> String {
    format!("tokenize-{:06}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(tokenize_label(7).len(), "tokenize-".len() + 6);
    }
}
