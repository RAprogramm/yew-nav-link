use yew::prelude::*;
use yew_router::prelude::*;

/// Returns `true` when the given route exactly matches the current URL.
#[hook]
pub fn use_is_active<R>(route: R) -> bool
where
    R: Routable + Clone + PartialEq + 'static
{
    let current = use_route::<R>();
    current.as_ref() == Some(&route)
}

/// Returns `true` when the given route exactly matches the current URL.
///
/// This is an alias for [`use_is_active`]; both perform exact matching.
#[hook]
pub fn use_is_exact_active<R>(route: R) -> bool
where
    R: Routable + Clone + PartialEq + 'static
{
    let current = use_route::<R>();
    current.as_ref() == Some(&route)
}

/// Returns `true` when the current URL starts with the given route's path.
///
/// Matches segment-wise, so `/docs` matches `/docs/api` but not
/// `/documentation`.
#[hook]
pub fn use_is_partial_active<R>(route: R) -> bool
where
    R: Routable + Clone + PartialEq + 'static
{
    let current = use_route::<R>();
    current.as_ref().is_some_and(|current_route| {
        let target_path = route.to_path();
        let current_path = current_route.to_path();
        crate::active_link::is_path_prefix(&target_path, &current_path)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Debug, Routable)]
    enum TestRoute {
        #[at("/")]
        Home,
        #[at("/docs")]
        Docs
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
}
