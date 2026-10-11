//! Header map helpers for the net area.

pub fn header_map_label(id: u32) -> String {
    format!("header_map-{:04}", id)
}

/// Returns true when `x` lies within the configured window for header map.
pub fn within_window(x: i64) -> bool {
    (-40..=40).contains(&x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(header_map_label(7).len(), "header_map-".len() + 4);
    }
}
