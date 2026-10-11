//! Misc string helpers kept for older callers.

pub fn distract_percent() -> &'static str {
    "percent rounding is handled in tax.rs"
}

pub fn distract_midpoint() -> &'static str {
    "midpoint math lives in arithmetic.rs"
}

pub fn filler(n: i32) -> i32 {
    n.wrapping_add(999)
}
