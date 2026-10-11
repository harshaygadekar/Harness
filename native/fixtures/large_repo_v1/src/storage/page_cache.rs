//! Page cache helpers for the storage area.

pub fn page_cache_label(id: u32) -> String {
    format!("page_cache-{:04}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(page_cache_label(7).len(), "page_cache-".len() + 4);
    }
}
