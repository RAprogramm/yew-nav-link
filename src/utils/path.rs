// SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! # Path Utilities
//!
//! Helper functions for working with URL paths: normalization,
//! absolute path detection, and path joining.
//!
//! # Example
//!
//! ```rust
//! use yew_nav_link::{is_absolute, join_paths, normalize_path};
//!
//! assert_eq!(normalize_path("/docs//api/"), "/docs/api/");
//! assert_eq!(normalize_path("/foo/bar/../baz"), "/foo/baz");
//! assert!(is_absolute("/docs"));
//! assert_eq!(join_paths("/base", "child"), "/base/child");
//! ```
//!
//! # Functions
//!
//! | Function | Signature | Description |
//! |----------|-----------|-------------|
//! | `normalize_path` | `(path: &str) -> String` | Collapse `//`, resolve `.`/`..` without escaping root |
//! | `is_absolute` | `(path: &str) -> bool` | Check if path starts with `/` |
//! | `join_paths` | `(base: &str, path: &str) -> String` | Join or replace base path |

/// Normalizes a path: collapse duplicate separators and resolve `.` and
/// `..` segments without escaping the root.
///
/// - Preserves the leading `/` for absolute paths and a single trailing `/` if
///   the input ended with one.
/// - `..` segments pop the previous component; at the root they are a no-op
///   (the path cannot escape `/`).
/// - `.` segments are dropped.
/// - The bare `/` and `""` inputs round-trip unchanged.
///
/// # Examples
///
/// ```
/// use yew_nav_link::normalize_path;
///
/// assert_eq!(normalize_path("/docs//api/"), "/docs/api/");
/// assert_eq!(normalize_path("/foo/bar/../baz/"), "/foo/baz/");
/// assert_eq!(normalize_path("/a/./b/c/../d"), "/a/b/d");
/// assert_eq!(normalize_path("/../foo"), "/foo");
/// assert_eq!(normalize_path("/"), "/");
/// ```
#[must_use]
pub fn normalize_path(path: &str) -> String {
    if path.is_empty() {
        return String::new();
    }

    let absolute = path.starts_with('/');
    let trailing_slash = path.len() > 1 && path.ends_with('/');

    let mut stack: Vec<&str> = Vec::new();
    for segment in path.split('/').filter(|s| !s.is_empty()) {
        match segment {
            "." => {}
            ".." => {
                stack.pop();
            }
            other => stack.push(other)
        }
    }

    if stack.is_empty() {
        return if absolute {
            "/".to_string()
        } else {
            String::new()
        };
    }

    let body = stack.join("/");
    let mut result = String::with_capacity(body.len() + 2);
    if absolute {
        result.push('/');
    }
    result.push_str(&body);
    if trailing_slash {
        result.push('/');
    }
    result
}

/// Returns `true` if the path starts with `/` (i.e. is absolute).
#[must_use]
pub fn is_absolute(path: &str) -> bool {
    path.starts_with('/')
}

/// Joins a base path with a child path.
///
/// If `path` is absolute, it is returned (normalized) directly.
/// Otherwise the two paths are concatenated and normalized.
#[must_use]
pub fn join_paths(base: &str, path: &str) -> String {
    if path.starts_with('/') {
        normalize_path(path)
    } else {
        let base = base.trim_end_matches('/');
        normalize_path(&format!("{base}/{path}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_collapses_double_slashes() {
        assert_eq!(normalize_path("/docs//api"), "/docs/api");
        assert_eq!(normalize_path("a//b///c"), "a/b/c");
    }

    #[test]
    fn normalize_preserves_trailing_slash_for_directories() {
        assert_eq!(normalize_path("/docs/"), "/docs/");
        assert_eq!(normalize_path("/"), "/");
    }

    #[test]
    fn normalize_resolves_double_dot() {
        assert_eq!(normalize_path("/foo/bar/../baz"), "/foo/baz");
        assert_eq!(normalize_path("/foo/bar/../baz/"), "/foo/baz/");
        assert_eq!(normalize_path("/a/b/c/../../d"), "/a/d");
    }

    #[test]
    fn normalize_resolves_single_dot() {
        assert_eq!(normalize_path("/a/./b/c/../d"), "/a/b/d");
        assert_eq!(normalize_path("./foo/./bar"), "foo/bar");
    }

    #[test]
    fn normalize_does_not_escape_root() {
        assert_eq!(normalize_path("/../foo"), "/foo");
        assert_eq!(normalize_path("/../../foo"), "/foo");
        assert_eq!(normalize_path("/.."), "/");
    }

    #[test]
    fn normalize_handles_empty_input() {
        assert_eq!(normalize_path(""), "");
    }

    #[test]
    fn normalize_relative_paths() {
        assert_eq!(normalize_path("foo/../bar"), "bar");
        assert_eq!(normalize_path("foo/./bar"), "foo/bar");
    }

    #[test]
    fn is_absolute_returns_true_for_rooted_paths() {
        assert!(is_absolute("/"));
        assert!(is_absolute("/docs"));
        assert!(is_absolute("/docs/api"));
    }

    #[test]
    fn is_absolute_returns_false_for_relative_paths() {
        assert!(!is_absolute(""));
        assert!(!is_absolute("docs"));
        assert!(!is_absolute("./docs"));
        assert!(!is_absolute("../docs"));
    }

    #[test]
    fn join_paths_with_absolute() {
        assert_eq!(join_paths("/base", "/docs"), "/docs");
        assert_eq!(join_paths("/base", "/"), "/");
    }

    #[test]
    fn join_paths_with_relative() {
        assert_eq!(join_paths("/base", "docs"), "/base/docs");
        assert_eq!(join_paths("/base/", "docs"), "/base/docs");
    }
}
