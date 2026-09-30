//! Tiny lib with an intentional add bug for the Phase 3 scripted slice.

mod noise;

/// Add two integers. Intentionally wrong: returns a+b+1.
pub fn add(a: i32, b: i32) -> i32 {
    a + b + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_two() {
        assert_eq!(add(2, 2), 4);
    }
}
