//! Bound large tool outputs: excerpt + note full length.

pub fn bound_excerpt(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    format!(
        "{}\n…[truncated; full length {} bytes]",
        &s[..max],
        s.len()
    )
}

/// Keep the first and last `max / 2` bytes (char-safe) with an omitted-bytes marker.
/// Test/command output puts failure names at the end, so the tail must survive.
pub fn bound_head_tail(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut h = max / 2;
    while !s.is_char_boundary(h) {
        h -= 1;
    }
    let mut t = s.len() - (max - max / 2);
    while !s.is_char_boundary(t) {
        t += 1;
    }
    format!(
        "{}\n…[omitted {} bytes of {} total]…\n{}",
        &s[..h],
        t - h,
        s.len(),
        &s[t..]
    )
}

#[cfg(test)]
mod head_tail_tests {
    use super::bound_head_tail;
    #[test]
    fn keeps_both_ends() {
        let s = format!("START{}END", "x".repeat(1000));
        let b = bound_head_tail(&s, 20);
        assert!(b.starts_with("START") && b.ends_with("END") && b.contains("omitted"));
        assert_eq!(bound_head_tail("short", 20), "short");
        let m = "é".repeat(50);
        let _ = bound_head_tail(&m, 11);
    }
}
