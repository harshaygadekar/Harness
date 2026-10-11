//! CSV writing.

/// Quotes a field when it contains a comma, quote or newline.
pub fn quote_field(f: &str) -> String {
    if f.contains(',') || f.contains('"') || f.contains('\n') {
        format!("\"{}\"", f.replace('"', "\"\""))
    } else {
        f.to_string()
    }
}

/// Joins fields into one record.
pub fn write_record(fields: &[String]) -> String {
    fields.iter().map(|f| quote_field(f)).collect::<Vec<_>>().join(",")
}
