//! Column statistics.

/// Sum of a numeric column; non-numeric cells count as zero.
pub fn column_sum(rows: &[Vec<String>], col: usize) -> i64 {
    rows.iter().filter_map(|r| r.get(col)).map(|c| c.trim().parse::<i64>().unwrap_or(0)).sum()
}

/// Widest cell in a column, in chars.
pub fn column_width(rows: &[Vec<String>], col: usize) -> usize {
    rows.iter().filter_map(|r| r.get(col)).map(|c| c.chars().count()).max().unwrap_or(0)
}
