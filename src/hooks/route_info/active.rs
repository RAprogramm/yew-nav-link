use yew::prelude::*;
use yew_router::prelude::*;

fn is_exact_match<R>(current: Option<&R>, route: &R) -> bool
where
    R: Routable + Clone + PartialEq + 'static,
{
    current == Some(route)
}

fn is_partial_match<R>(current: Option<&R>, route: &R) -> bool
where
    R: Routable + Clone + PartialEq + 'static,
{
    match current {
        Some(current_route) => {
            let target_path = route.to_path();
            let current_path = current_route.to_path();
            crate::active_link::is_path_prefix(&target_path, &current_path)
        }
        None => false,
    }
}

/// Returns `true` when the given route exactly matches the current URL.
#[hook]
pub fn use_is_active<R>(route: R) -> bool
where
    R: Routable + Clone + PartialEq + 'static,
{
    let current = use_route::<R>();
    is_exact_match(current.as_ref(), &route)
}

/// Returns `true` when the given route exactly matches the current URL.
///
/// This is an alias for [`use_is_active`]; both perform exact matching.
#[hook]
pub fn use_is_exact_active<R>(route: R) -> bool
where
    R: Routable + Clone + PartialEq + 'static,
{
    let current = use_route::<R>();
    is_exact_match(current.as_ref(), &route)
}

/// Returns `true` when the current URL starts with the given route's path.
///
/// Matches segment-wise, so `/docs` matches `/docs/api` but not
/// `/documentation`.
#[hook]
pub fn use_is_partial_active<R>(route: R) -> bool
where
    R: Routable + Clone + PartialEq + 'static,
{
    let current = use_route::<R>();
    is_partial_match(current.as_ref(), &route)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Debug, Routable)]
    enum TestRoute {
        #[at("/")]
        Home,
        #[at("/docs")]
        Docs,
    }

    #[test]
    fn use_is_active_returns_bool() {
        let _ = use_is_active(TestRoute::Home);
    }

    #[test]
    fn use_is_exact_active_returns_bool() {
        let _ = use_is_exact_active(TestRoute::Home);
    }

    #[test]
    fn use_is_partial_active_returns_bool() {
        let _ = use_is_partial_active(TestRoute::Docs);
    }

    #[test]
    fn exact_match_helper_handles_some_and_none() {
        assert!(super::is_exact_match(
            Some(&TestRoute::Home),
            &TestRoute::Home
        ));
        assert!(!super::is_exact_match(
            Some(&TestRoute::Docs),
            &TestRoute::Home
        ));
        assert!(!super::is_exact_match::<TestRoute>(None, &TestRoute::Home));
    }

    #[test]
    fn partial_match_helper_handles_prefix_and_none() {
        assert!(super::is_partial_match(
            Some(&TestRoute::Docs),
            &TestRoute::Docs
        ));
        assert!(!super::is_partial_match(
            Some(&TestRoute::Home),
            &TestRoute::Docs
        ));
        assert!(!super::is_partial_match::<TestRoute>(
            None,
            &TestRoute::Docs
        ));
    }
}
