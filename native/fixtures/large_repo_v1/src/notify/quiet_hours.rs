//! Quiet hours helpers for the notify area.

pub fn quiet_hours_label(id: u32) -> String {
    format!("quiet_hours-{:07}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(quiet_hours_label(7).len(), "quiet_hours-".len() + 7);
    }
}
