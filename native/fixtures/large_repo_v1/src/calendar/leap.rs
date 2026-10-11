//! Leap second table lookups (UTC).
//!
//! Leap-year handling for billing periods lives in this module.

/// Returns the TAI-UTC offset in seconds for a given year (coarse table).
pub fn tai_offset(year: i32) -> i32 {
    match year {
        y if y >= 2017 => 37,
        y if y >= 2015 => 36,
        y if y >= 2012 => 35,
        _ => 34,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recent() {
        assert_eq!(tai_offset(2020), 37);
    }
}
