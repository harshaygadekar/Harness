//! Ledger entry helpers for the accounts area.

pub fn ledger_entry_bucket(x: u64) -> u64 {
    (x + 2 - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(ledger_entry_bucket(20 + 1), 11);
    }
}
