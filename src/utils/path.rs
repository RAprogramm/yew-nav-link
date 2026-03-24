//! Utility functions for navigation.
//!
//! Provides helper functions for working with paths and URLs.
//!
//! # Example
//!
//! ```ignore
//! use yew_nav_link::normalize_path;
//! use yew_nav_link::is_absolute;
//!
//! let path = normalize_path("/docs//api/");
//! assert_eq!(path, "/docs/api");
//! ```

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

pub fn is_absolute(path: &str) -> bool {
    path.starts_with('/')
}

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
