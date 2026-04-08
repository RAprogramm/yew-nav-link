use yew::prelude::*;
use yew_router::prelude::*;

/// Returns the currently active route, or `None` if no route is matched.
#[hook]
pub fn use_route_info<R: Routable + 'static>() -> Option<R> {
    use_route::<R>()
}
