// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Helpers shared between [`crate::NavLink`] and its callers: segment-wise
//! path-prefix matching and the active-class string builder.

/// Checks if `target` path is a segment-wise prefix of `current` path.
///
/// A root or empty `target` (zero non-empty segments) matches only a root or
/// empty `current`. Without this rule `"/"` would prefix every path and a
/// partial-matching Home link would be active on every page; React Router
/// defaults its root links to exact matching for the same reason.
#[inline]
#[must_use]
pub fn is_path_prefix(target: &str, current: &str) -> bool {
    let mut target_iter = target.split('/').filter(|s| !s.is_empty());
    let mut current_iter = current.split('/').filter(|s| !s.is_empty());

    let mut matched_any = false;
    for t in target_iter.by_ref() {
        match current_iter.next() {
            Some(c) if t == c => matched_any = true,
            _ => return false
        }
    }

    matched_any || current_iter.next().is_none()
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

    #[test]
    fn prefix_root_target_matches_only_root() {
        assert!(!is_path_prefix("/", "/docs"));
        assert!(!is_path_prefix("", "/docs"));
        assert!(is_path_prefix("", ""));
        assert!(is_path_prefix("/", "//"));
    }
}
