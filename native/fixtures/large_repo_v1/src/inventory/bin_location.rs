//! Bin location helpers for the inventory area.

pub fn bin_location_label(id: u32) -> String {
    format!("bin_location-{:02}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(bin_location_label(7).len(), "bin_location-".len() + 2);
    }
}
