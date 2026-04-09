/// Path matching strategy for NavLink active state detection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Match {
    /// Link is active only when paths match exactly.
    #[default]
    Exact,
    /// Link is active when current path starts with target path (segment-wise).
    Partial
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_default_is_exact() {
        assert_eq!(Match::default(), Match::Exact);
    }

    #[test]
    fn match_equality() {
        assert_eq!(Match::Exact, Match::Exact);
        assert_eq!(Match::Partial, Match::Partial);
        assert_ne!(Match::Exact, Match::Partial);
    }

    #[test]
    fn match_debug() {
        assert_eq!(format!("{:?}", Match::Exact), "Exact");
    }

    #[test]
    fn match_clone() {
        let m = Match::Partial;
        let cloned = m;
        assert_eq!(m, cloned);
    }
}
