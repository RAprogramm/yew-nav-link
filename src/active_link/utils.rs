/// Checks if `target` path is a segment-wise prefix of `current` path.
#[inline]
#[must_use]
pub fn is_path_prefix(target: &str, current: &str) -> bool {
    let mut target_iter = target.split('/').filter(|s| !s.is_empty());
    let mut current_iter = current.split('/').filter(|s| !s.is_empty());

    loop {
        match (target_iter.next(), current_iter.next()) {
            (Some(t), Some(c)) if t == c => continue,
            (Some(_), Some(_)) => return false,
            (Some(_), None) => return false,
            (None, _) => return true
        }
    }
}

#[inline]
#[must_use]
pub fn build_class(is_active: bool, base_class: &str, active_class: &str) -> String {
    if is_active {
        format!("{} {}", base_class, active_class)
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
