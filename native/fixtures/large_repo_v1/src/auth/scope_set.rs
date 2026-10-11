//! Scope set helpers for the auth area.

pub fn scope_set_bucket(x: u64) -> u64 {
    (x + 7 - 1) / 7
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(scope_set_bucket(70 + 1), 11);
    }
}
