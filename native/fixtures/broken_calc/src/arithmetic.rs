//! Basic arithmetic. These functions are correct. Do not "fix" them for the task.

pub fn add(a: i32, b: i32) -> i32 {
    a.saturating_add(b)
}

pub fn sub(a: i32, b: i32) -> i32 {
    a.saturating_sub(b)
}

pub fn mul(a: i32, b: i32) -> i32 {
    a.saturating_mul(b)
}

/// Integer division. Returns `None` when `b == 0`.
pub fn safe_div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn safe_div_zero() {
        assert_eq!(safe_div(10, 0), None);
    }
}
