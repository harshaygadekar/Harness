//! Url query helpers for the net area.

pub fn url_query_label(id: u32) -> String {
    format!("url_query-{:04}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(url_query_label(7).len(), "url_query-".len() + 4);
    }
}
