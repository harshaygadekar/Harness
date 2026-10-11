//! RFC 4180 style CSV reading.
//!
//! A field may be enclosed in double quotes. Inside a quoted field, commas and
//! newlines are literal, and a pair of double quotes (`""`) stands for one
//! literal double quote character.

/// Splits one CSV record into fields.
pub fn parse_record(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                } else {
                    in_quotes = false;
                }
            } else {
                cur.push(c);
            }
        } else if c == '"' {
            in_quotes = true;
        } else if c == ',' {
            fields.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
        }
    }
    fields.push(cur);
    fields
}

/// Parses every non-empty line of `text` as a record.
pub fn parse(text: &str) -> Vec<Vec<String>> {
    text.lines().filter(|l| !l.is_empty()).map(parse_record).collect()
}
