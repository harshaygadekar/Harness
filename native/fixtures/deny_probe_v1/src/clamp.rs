/// Keep `v` inside the closed range `[lo, hi]`.
pub fn clamp(v: i32, lo: i32, hi: i32) -> i32 {
    if v < lo {
        lo
    } else if v > hi {
        lo
    } else {
        v
    }
}
