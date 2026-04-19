//! URL encoding/decoding utilities.

use std::fmt::Write;

/// Decodes a percent-encoded string (e.g. `"%20"` becomes a space).
///
/// Also converts `+` to a space. Returns `None` only if the input is malformed
/// in a way that prevents decoding (currently always returns `Some`).
#[must_use]
pub fn urlencoding_decode(input: &str) -> Option<String> {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();

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
                    // result.push_str(&format!("%{byte:02X}"));
                    let _ = write!(result, "%{byte:02X}");
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urlencoding_decode_simple() {
        let result = urlencoding_decode("%20");
        assert_eq!(result, Some(" ".to_string()));
    }

    #[test]
    fn urlencoding_decode_plus() {
        let result = urlencoding_decode("+");
        assert_eq!(result, Some(" ".to_string()));
    }

    #[test]
    fn urlencoding_decode_mixed() {
        let result = urlencoding_decode("hello%20world");
        assert_eq!(result, Some("hello world".to_string()));
    }

    #[test]
    fn urlencoding_decode_unencoded() {
        let result = urlencoding_decode("hello");
        assert_eq!(result, Some("hello".to_string()));
    }

    #[test]
    fn urlencoding_decode_empty() {
        let result = urlencoding_decode("");
        assert_eq!(result, Some(String::new()));
    }

    #[test]
    fn urlencoding_decode_multiple_percent() {
        let result = urlencoding_decode("%20%21");
        assert_eq!(result, Some(" !".to_string()));
    }

    #[test]
    fn urlencoding_decode_unicode() {
        let result = urlencoding_decode("%E2%9C%93");
        assert_eq!(result, Some("â\u{9c}\u{93}".to_string()));
    }

    #[test]
    fn urlencoding_decode_partial_hex() {
        let result = urlencoding_decode("%2");
        assert_eq!(result, Some("%2".to_string()));
    }

    #[test]
    fn urlencoding_decode_short_hex() {
        let result = urlencoding_decode("%2X");
        assert_eq!(result, Some("%2X".to_string()));
    }

    #[test]
    fn urlencoding_encode_unreserved() {
        let result = urlencoding_encode("abc123-_.~");
        assert_eq!(result, "abc123-_.~");
    }

    #[test]
    fn urlencoding_encode_space() {
        let result = urlencoding_encode("hello world");
        assert_eq!(result, "hello+world");
    }

    #[test]
    fn urlencoding_encode_special_chars() {
        let result = urlencoding_encode("hello@world");
        assert!(result.contains("hello"));
        assert!(result.contains("%40"));
    }

    #[test]
    fn urlencoding_encode_empty() {
        let result = urlencoding_encode("");
        assert_eq!(result, "");
    }

    #[test]
    fn urlencoding_encode_unicode() {
        let result = urlencoding_encode("✓");
        assert!(result.starts_with('%'));
    }

    #[test]
    fn urlencoding_encode_complex() {
        let result = urlencoding_encode("hello world!");
        assert!(result.contains("hello+world"));
    }
}
