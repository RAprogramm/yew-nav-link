#![cfg(test)]

use yew_nav_link::utils::{urlencoding_decode, urlencoding_encode};

#[test]
fn urlencoding_decode_simple() {
    assert_eq!(urlencoding_decode("hello"), Some("hello".to_string()));
}

#[test]
fn urlencoding_decode_space_plus() {
    assert_eq!(
        urlencoding_decode("hello+world"),
        Some("hello world".to_string())
    );
}

#[test]
fn urlencoding_decode_percent_encoded() {
    assert_eq!(urlencoding_decode("%20"), Some(" ".to_string()));
    assert_eq!(urlencoding_decode("%2F"), Some("/".to_string()));
    assert_eq!(urlencoding_decode("%3F"), Some("?".to_string()));
}

#[test]
fn urlencoding_decode_mixed() {
    assert_eq!(
        urlencoding_decode("hello%20world"),
        Some("hello world".to_string())
    );
}

#[test]
fn urlencoding_decode_invalid_percent() {
    let result = urlencoding_decode("%ZZ");
    assert!(result.is_some());
}

#[test]
fn urlencoding_decode_empty() {
    assert_eq!(urlencoding_decode(""), Some(String::new()));
}

#[test]
fn urlencoding_decode_special_chars() {
    assert_eq!(urlencoding_decode("%3C%3E"), Some("<>".to_string()));
}

#[test]
fn urlencoding_encode_simple() {
    assert_eq!(urlencoding_encode("hello"), "hello".to_string());
}

#[test]
fn urlencoding_encode_space() {
    assert_eq!(urlencoding_encode("hello world"), "hello+world".to_string());
}

#[test]
fn urlencoding_encode_unreserved() {
    assert_eq!(urlencoding_encode("abc123-_.~"), "abc123-_.~".to_string());
}

#[test]
fn urlencoding_encode_special() {
    let result = urlencoding_encode("<>");
    assert!(result.contains("%3C"));
    assert!(result.contains("%3E"));
}

#[test]
fn urlencoding_encode_empty() {
    assert_eq!(urlencoding_encode(""), "");
}

#[test]
fn urlencoding_encode_mixed() {
    assert_eq!(
        urlencoding_encode("hello world!"),
        "hello+world%21".to_string()
    );
}

#[test]
fn urlencoding_roundtrip() {
    let original = "Hello World! Test 123";
    let encoded = urlencoding_encode(original);
    let decoded = urlencoding_decode(&encoded);
    assert_eq!(decoded, Some(original.to_string()));
}

#[test]
fn urlencoding_decode_multiple_pluses() {
    assert_eq!(urlencoding_decode("a+++b"), Some("a   b".to_string()));
}
