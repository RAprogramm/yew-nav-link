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
//! assert_eq!(normalize_path("/docs//api/"), "/docs/api");
//! assert!(is_absolute("/docs"));
//! assert_eq!(join_paths("/base", "child"), "/base/child");
//! ```
//!
//! # Functions
//!
//! | Function | Signature | Description |
//! |----------|-----------|-------------|
//! | `normalize_path` | `(path: &str) -> String` | Collapse double slashes, trim trailing `/` |
//! | `is_absolute` | `(path: &str) -> bool` | Check if path starts with `/` |
//! | `join_paths` | `(base: &str, path: &str) -> String` | Join or replace base path |

/// Normalizes a path by collapsing duplicate slashes and removing trailing
/// slashes.
///
/// Preserves the root `/` path.
pub fn normalize_path(path: &str) -> String {
    let mut result = String::with_capacity(path.len());
    let mut prev_was_slash = false;

    for ch in path.chars() {
        if ch == '/' {
            if !prev_was_slash {
                result.push(ch);
                prev_was_slash = true;
            }
        } else {
            result.push(ch);
            prev_was_slash = false;
        }
    }

    if result.ends_with('/') && result.len() > 1 {
        result.pop();
    }

    result
}

/// Returns `true` if the path starts with `/` (i.e. is absolute).
pub fn is_absolute(path: &str) -> bool {
    path.starts_with('/')
}

/// Joins a base path with a child path.
///
/// If `path` is absolute, it is returned (normalized) directly.
/// Otherwise the two paths are concatenated and normalized.
pub fn join_paths(base: &str, path: &str) -> String {
    if path.starts_with('/') {
        normalize_path(path)
    } else {
        let base = base.trim_end_matches('/');
        normalize_path(&format!("{}/{}", base, path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_removes_double_slashes() {
        assert_eq!(normalize_path("/docs//api"), "/docs/api");
        assert_eq!(normalize_path("a//b///c"), "a/b/c");
    }

    #[test]
    fn normalize_removes_trailing_slash() {
        assert_eq!(normalize_path("/docs/"), "/docs");
        assert_eq!(normalize_path("/"), "/");
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
