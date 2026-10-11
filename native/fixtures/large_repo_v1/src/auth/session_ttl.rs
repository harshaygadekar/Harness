//! Session ttl helpers for the auth area.

pub fn session_ttl_bucket(x: u64) -> u64 {
    (x + 2 - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(session_ttl_bucket(20 + 1), 11);
    }
}
