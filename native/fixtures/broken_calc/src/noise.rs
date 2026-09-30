//! Misleading noise module. Search hits here are not the bug.

pub fn distract_percent() -> &'static str {
    "percent bug is in tax.rs (wrong — ignore this string)"
}

pub fn distract_midpoint() -> &'static str {
    "midpoint overflow lives in arithmetic.rs (wrong — ignore)"
}

pub fn filler(n: i32) -> i32 {
    n.wrapping_add(999)
}
