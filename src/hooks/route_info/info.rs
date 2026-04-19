use yew::prelude::*;
use yew_router::prelude::*;

/// Returns the currently active route, or `None` if no route is matched.
#[hook]
pub fn use_route_info<R: Routable + 'static>() -> Option<R> {
    use_route::<R>()
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
        #[at("/about")]
        About
    }

    #[test]
    fn use_route_info_returns_option() {
        let _ = use_route_info::<TestRoute>();
    }

    #[test]
    fn use_route_info_with_nested_routes() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum NestedRoute {
            #[at("/")]
            Home,
            #[at("/users")]
            Users,
            #[at("/users/:id")]
            User { id: String }
        }

        let _ = use_route_info::<NestedRoute>();
    }

    #[test]
    fn test_route_is_routable() {
        assert_eq!(TestRoute::Home.to_path(), "/");
        assert_eq!(TestRoute::Docs.to_path(), "/docs");
        assert_eq!(TestRoute::About.to_path(), "/about");
    }

    #[test]
    fn use_route_info_with_complex_routes() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum ComplexRoute {
            #[at("/")]
            Home,
            #[at("/about")]
            About,
            #[at("/users/:id")]
            User { id: String }
        }

        let _ = use_route_info::<ComplexRoute>();
    }

    #[test]
    fn test_route_with_route_params() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum RouteWithParams {
            #[at("/users/:id")]
            User { id: String }
        }

        let user_route = RouteWithParams::User {
            id: "123".to_string()
        };
        assert_eq!(user_route.to_path(), "/users/123");
    }
}
