// SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
// SPDX-License-Identifier: MIT

//! URL encoding/decoding utilities.

use std::fmt::Write;

/// Decodes a percent-encoded string. Also converts `+` to a space (the
/// `application/x-www-form-urlencoded` convention).
///
/// `%XX` sequences accumulate into a byte buffer and the buffer is then
/// decoded as UTF-8 — so a sequence like `"%E2%9C%93"` resolves to `Some("✓")`,
/// not to three separate Latin-1 chars. Malformed `%XX` triplets are left in
/// the output verbatim. Returns `None` if the resulting bytes are not valid
/// UTF-8.
#[must_use]
pub fn urlencoding_decode(input: &str) -> Option<String> {
    let mut bytes: Vec<u8> = Vec::with_capacity(input.len());
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        match c {
            '%' => {
                let hi = chars.next();
                let lo = chars.next();
                if let Some(byte) = hex_pair_to_byte(hi, lo) {
                    bytes.push(byte);
                } else {
                    bytes.push(b'%');
                    push_utf8(&mut bytes, hi);
                    push_utf8(&mut bytes, lo);
                }
            }
            '+' => bytes.push(b' '),
            other => {
                let mut buf = [0u8; 4];
                bytes.extend_from_slice(other.encode_utf8(&mut buf).as_bytes());
            }
        }
    }

    String::from_utf8(bytes).ok()
}

/// Combines two hex-digit characters into the byte they encode.
///
/// Returns `None` unless both characters are present and are hex digits,
/// which keeps malformed `%XX` triplets in the output verbatim.
fn hex_pair_to_byte(hi: Option<char>, lo: Option<char>) -> Option<u8> {
    let high = hi?.to_digit(16)?;
    let low = lo?.to_digit(16)?;
    u8::try_from(high * 16 + low).ok()
}

/// Appends the UTF-8 bytes of `c` to `bytes` when the character is present.
fn push_utf8(bytes: &mut Vec<u8>, c: Option<char>) {
    if let Some(c) = c {
        let mut buf = [0u8; 4];
        bytes.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
    }
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
                let mut buf = [0u8; 4];
                for byte in c.encode_utf8(&mut buf).as_bytes() {
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
        // E2 9C 93 is the UTF-8 encoding of U+2713 CHECK MARK (✓).
        assert_eq!(urlencoding_decode("%E2%9C%93"), Some("✓".to_string()));
    }

    #[test]
    fn urlencoding_decode_cyrillic() {
        // "%D0%9F%D1%80%D0%B8%D0%B2%D0%B5%D1%82" is "Привет".
        let encoded = "%D0%9F%D1%80%D0%B8%D0%B2%D0%B5%D1%82";
        assert_eq!(urlencoding_decode(encoded), Some("Привет".to_string()));
    }

    #[test]
    fn urlencoding_decode_invalid_utf8_returns_none() {
        // A lone 0xFF byte is not valid UTF-8.
        assert_eq!(urlencoding_decode("%FF"), None);
    }

    #[test]
    fn urlencoding_round_trip_preserves_unicode() {
        let inputs = ["✓", "Привет", "日本語", "hello world!", "a&b=c"];
        for s in inputs {
            let encoded = urlencoding_encode(s);
            assert_eq!(urlencoding_decode(&encoded), Some(s.to_string()));
        }
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
