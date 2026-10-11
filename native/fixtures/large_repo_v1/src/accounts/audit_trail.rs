//! Audit trail helpers for the accounts area.

pub fn audit_trail_label(id: u32) -> String {
    format!("audit_trail-{:07}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(audit_trail_label(7).len(), "audit_trail-".len() + 7);
    }
}
