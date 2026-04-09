//! URL encoding/decoding utilities.

/// Decodes a percent-encoded string (e.g. `"%20"` becomes a space).
///
/// Also converts `+` to a space. Returns `None` only if the input is malformed
/// in a way that prevents decoding (currently always returns `Some`).
#[must_use]
pub fn urlencoding_decode(input: &str) -> Option<String> {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if hex.len() == 2 {
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    result.push(byte as char);
                } else {
                    result.push('%');
                    result.push_str(&hex);
                }
            } else {
                result.push('%');
                result.push_str(&hex);
            }
        } else if c == '+' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }

    Some(result)
}

/// Percent-encodes a string for safe use in URLs.
///
/// Unreserved characters (`a-z`, `A-Z`, `0-9`, `-`, `_`, `.`, `~`) are
/// passed through unchanged. Spaces are encoded as `+`.
#[must_use]
pub fn urlencoding_encode(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 3);

    for c in input.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                result.push(c);
            }
            ' ' => result.push('+'),
            _ => {
                for byte in c.to_string().as_bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }

    result
}
