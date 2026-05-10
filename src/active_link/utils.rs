// SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Helpers shared between [`crate::NavLink`] and its callers: segment-wise
//! path-prefix matching and the active-class string builder.

/// Checks if `target` path is a segment-wise prefix of `current` path.
#[inline]
#[must_use]
pub fn is_path_prefix(target: &str, current: &str) -> bool {
    let target_iter = target.split('/').filter(|s| !s.is_empty());
    let mut current_iter = current.split('/').filter(|s| !s.is_empty());

    for t in target_iter {
        match current_iter.next() {
            Some(c) if t == c => {}
            _ => return false
        }
    }

    true
}

/// Combines a base CSS class name with an active class name when selected.
///
/// When `is_active` is `true`, returns both the base and active classes
/// separated by a space. Otherwise, returns only the base class.
///
/// # Examples
///
/// ```
/// use yew_nav_link::active_link::utils::build_class;
///
/// assert_eq!(build_class(true, "nav-link", "active"), "nav-link active");
/// assert_eq!(build_class(false, "nav-link", "active"), "nav-link");
/// ```
#[inline]
#[must_use]
pub fn build_class(is_active: bool, base_class: &str, active_class: &str) -> String {
    if is_active {
        format!("{base_class} {active_class}")
    } else {
        base_class.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_class_active() {
        assert_eq!(build_class(true, "nav-link", "active"), "nav-link active");
    }

    #[test]
    fn build_class_inactive() {
        assert_eq!(build_class(false, "nav-link", "active"), "nav-link");
    }

    #[test]
    fn prefix_exact_match() {
        assert!(is_path_prefix("/", "/"));
        assert!(is_path_prefix("/docs", "/docs"));
    }

    #[test]
    fn prefix_valid() {
        assert!(is_path_prefix("/docs", "/docs/api"));
    }

    #[test]
    fn prefix_not_prefix() {
        assert!(!is_path_prefix("/docs/api", "/docs"));
    }

    #[test]
    fn prefix_segment_boundary() {
        assert!(!is_path_prefix("/doc", "/documents"));
    }
}
