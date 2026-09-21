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
