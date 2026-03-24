//! Integration tests for utils module.

use yew_nav_link::utils::{is_absolute, join_paths, normalize_path};

#[test]
fn normalize_removes_double_slashes() {
    assert_eq!(normalize_path("/docs//api"), "/docs/api");
    assert_eq!(normalize_path("a//b///c"), "a/b/c");
    assert_eq!(normalize_path("///"), "/");
}

#[test]
fn normalize_removes_trailing_slash() {
    assert_eq!(normalize_path("/docs/"), "/docs");
    assert_eq!(normalize_path("/"), "/");
    assert_eq!(normalize_path(""), "");
}

#[test]
fn normalize_preserves_single_slash() {
    assert_eq!(normalize_path("/"), "/");
}

#[test]
fn normalize_handles_empty_string() {
    assert_eq!(normalize_path(""), "");
}

#[test]
fn is_absolute_on_rooted_paths() {
    assert!(is_absolute("/"));
    assert!(is_absolute("/docs"));
    assert!(is_absolute("/docs/api/v1"));
}

#[test]
fn is_absolute_on_relative_paths() {
    assert!(!is_absolute(""));
    assert!(!is_absolute("docs"));
    assert!(!is_absolute("./docs"));
    assert!(!is_absolute("../docs"));
}

#[test]
fn join_paths_with_absolute_path() {
    assert_eq!(join_paths("/base", "/docs"), "/docs");
    assert_eq!(join_paths("/base", "/"), "/");
}

#[test]
fn join_paths_with_relative_path() {
    assert_eq!(join_paths("/base", "docs"), "/base/docs");
    assert_eq!(join_paths("/base/", "docs"), "/base/docs");
}

#[test]
fn join_paths_handles_trailing_slash_in_base() {
    assert_eq!(join_paths("/base/", "sub/path"), "/base/sub/path");
}
