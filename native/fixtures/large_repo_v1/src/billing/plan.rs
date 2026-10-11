//! Plan helpers for the billing area.

pub fn plan_label(id: u32) -> String {
    format!("plan-{:02}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(plan_label(7).len(), "plan-".len() + 2);
    }
}
