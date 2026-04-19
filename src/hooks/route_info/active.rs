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

    #[test]
    fn test_route_to_path() {
        assert_eq!(TestRoute::Home.to_path(), "/");
        assert_eq!(TestRoute::Docs.to_path(), "/docs");
    }

    #[test]
    fn test_route_clone() {
        let route = TestRoute::Home.clone();
        assert_eq!(route, TestRoute::Home);
    }

    #[test]
    fn test_route_partial_eq() {
        assert_eq!(TestRoute::Home, TestRoute::Home);
        assert_ne!(TestRoute::Home, TestRoute::Docs);
    }

    #[test]
    fn test_route_debug() {
        let debug = format!("{:?}", TestRoute::Home);
        assert!(debug.contains("Home"));
    }

    #[test]
    fn use_is_active_with_home_route() {
        let _ = use_is_active(TestRoute::Home);
    }

    #[test]
    fn use_is_active_with_docs_route() {
        let _ = use_is_active(TestRoute::Docs);
    }

    #[test]
    fn use_is_exact_active_with_home_route() {
        let _ = use_is_exact_active(TestRoute::Home);
    }

    #[test]
    fn use_is_exact_active_with_docs_route() {
        let _ = use_is_exact_active(TestRoute::Docs);
    }

    #[test]
    fn use_is_partial_active_with_home_route() {
        let _ = use_is_partial_active(TestRoute::Home);
    }

    #[test]
    fn use_is_partial_active_with_docs_route() {
        let _ = use_is_partial_active(TestRoute::Docs);
    }

    #[test]
    fn test_route_to_path_more_variants() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum ExtendedTestRoute {
            #[at("/")]
            Home,
            #[at("/about")]
            About,
            #[at("/users/:id")]
            User { id: String },
            #[at("/posts/:year/:month/:day")]
            Post { year: u32, month: u32, day: u32 }
        }

        assert_eq!(ExtendedTestRoute::Home.to_path(), "/");
        assert_eq!(ExtendedTestRoute::About.to_path(), "/about");

        let user_route = ExtendedTestRoute::User {
            id: "123".to_string()
        };
        assert_eq!(user_route.to_path(), "/users/123");

        let post_route = ExtendedTestRoute::Post {
            year:  2023,
            month: 12,
            day:   25
        };
        assert_eq!(post_route.to_path(), "/posts/2023/12/25");
    }

    #[test]
    fn test_route_clone_more_variants() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum CloneTestRoute {
            #[at("/")]
            Home,
            #[at("/settings")]
            Settings,
            #[at("/items/:id")]
            Item { id: String }
        }

        let home = CloneTestRoute::Home;
        let home_clone = home.clone();
        assert_eq!(home, home_clone);

        let settings = CloneTestRoute::Settings;
        let settings_clone = settings.clone();
        assert_eq!(settings, settings_clone);

        let item = CloneTestRoute::Item {
            id: "item-42".to_string()
        };
        let item_clone = item.clone();
        assert_eq!(item, item_clone);
    }

    #[test]
    fn test_route_partial_eq_more_variants() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum EqTestRoute {
            #[at("/")]
            Home,
            #[at("/about")]
            About,
            #[at("/contact")]
            Contact
        }

        assert_eq!(EqTestRoute::Home, EqTestRoute::Home);
        assert_eq!(EqTestRoute::About, EqTestRoute::About);
        assert_eq!(EqTestRoute::Contact, EqTestRoute::Contact);
        assert_ne!(EqTestRoute::Home, EqTestRoute::About);
        assert_ne!(EqTestRoute::Home, EqTestRoute::Contact);
        assert_ne!(EqTestRoute::About, EqTestRoute::Contact);
    }

    #[test]
    fn test_route_debug_more_variants() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum DebugTestRoute {
            #[at("/")]
            Home,
            #[at("/dashboard")]
            Dashboard
        }

        let home_debug = format!("{:?}", DebugTestRoute::Home);
        assert!(home_debug.contains("Home"));

        let dashboard_debug = format!("{:?}", DebugTestRoute::Dashboard);
        assert!(dashboard_debug.contains("Dashboard"));
    }

    #[test]
    fn test_route_to_path_with_parameters() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum ParamTestRoute {
            #[at("/search/:query")]
            Search { query: String },
            #[at("/date/:year/:month/:day")]
            Date { year: u32, month: u32, day: u32 },
            #[at("/product/:category/:id")]
            Product { category: String, id: u32 }
        }

        let search = ParamTestRoute::Search {
            query: "rust".to_string()
        };
        assert_eq!(search.to_path(), "/search/rust");

        let date = ParamTestRoute::Date {
            year:  2023,
            month: 2,
            day:   14
        };
        assert_eq!(date.to_path(), "/date/2023/2/14");

        let product = ParamTestRoute::Product {
            category: "books".to_string(),
            id:       101
        };
        assert_eq!(product.to_path(), "/product/books/101");
    }

    #[test]
    fn test_route_enum_with_unit_variants() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum UnitTestRoute {
            #[at("/")]
            Home,
            #[at("/about")]
            About,
            #[at("/contact")]
            Contact
        }

        assert_eq!(UnitTestRoute::Home.to_path(), "/");
        assert_eq!(UnitTestRoute::About.to_path(), "/about");
        assert_eq!(UnitTestRoute::Contact.to_path(), "/contact");

        let home1 = UnitTestRoute::Home;
        let home2 = UnitTestRoute::Home;
        assert_eq!(home1, home2);

        let about = UnitTestRoute::About;
        assert_ne!(home1, about);
    }

    #[test]
    fn test_route_with_more_variants() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum ExtendedTestRoute {
            #[at("/")]
            Home,
            #[at("/docs")]
            Docs,
            #[at("/docs/api")]
            Api,
            #[at("/users/:id")]
            User { id: String },
            #[at("/posts/:year/:month")]
            Post { year: u32, month: u32 }
        }

        assert_eq!(ExtendedTestRoute::Home.to_path(), "/");
        assert_eq!(ExtendedTestRoute::Docs.to_path(), "/docs");
        assert_eq!(ExtendedTestRoute::Api.to_path(), "/docs/api");

        // Test that we can create instances
        let user_route = ExtendedTestRoute::User {
            id: "123".to_string()
        };
        assert_eq!(user_route.to_path(), "/users/123");

        let post_route = ExtendedTestRoute::Post {
            year:  2023,
            month: 5
        };
        assert_eq!(post_route.to_path(), "/posts/2023/5");
    }

    #[test]
    fn test_route_equality_with_complex_routes() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum ComplexTestRoute {
            #[at("/admin/dashboard")]
            AdminDashboard,
            #[at("/admin/settings")]
            AdminSettings,
            #[at("/api/v1/:resource")]
            ApiResource { resource: String }
        }

        let route1 = ComplexTestRoute::AdminDashboard;
        let route2 = ComplexTestRoute::AdminDashboard;
        let route3 = ComplexTestRoute::AdminSettings;

        assert_eq!(route1, route2);
        assert_ne!(route1, route3);

        let api1 = ComplexTestRoute::ApiResource {
            resource: "users".to_string()
        };
        let api2 = ComplexTestRoute::ApiResource {
            resource: "users".to_string()
        };
        let api3 = ComplexTestRoute::ApiResource {
            resource: "posts".to_string()
        };

        assert_eq!(api1, api2);
        assert_ne!(api1, api3);
    }

    #[test]
    fn test_route_clone_preserves_values() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum CloneTestRoute {
            #[at("/profile/:id")]
            Profile { id: String }
        }

        let original = CloneTestRoute::Profile {
            id: "user-123".to_string()
        };
        let cloned = original.clone();

        assert_eq!(original, cloned);
        let CloneTestRoute::Profile {
            id: orig_id
        } = original;
        let CloneTestRoute::Profile {
            id: cloned_id
        } = cloned;
        assert_eq!(orig_id, cloned_id);
    }

    #[test]
    fn use_is_active_with_different_routes() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum MultiRoute {
            #[at("/")]
            Home,
            #[at("/about")]
            About,
            #[at("/contact")]
            Contact
        }

        let _ = use_is_active(MultiRoute::Home);
        let _ = use_is_active(MultiRoute::About);
        let _ = use_is_active(MultiRoute::Contact);
    }

    #[test]
    fn use_is_exact_active_with_different_routes() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum MultiRoute {
            #[at("/")]
            Home,
            #[at("/about")]
            About
        }

        let _ = use_is_exact_active(MultiRoute::Home);
        let _ = use_is_exact_active(MultiRoute::About);
    }

    #[test]
    fn use_is_partial_active_with_different_routes() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum MultiRoute {
            #[at("/")]
            Home,
            #[at("/docs")]
            Docs,
            #[at("/docs/api")]
            Api
        }

        let _ = use_is_partial_active(MultiRoute::Home);
        let _ = use_is_partial_active(MultiRoute::Docs);
        let _ = use_is_partial_active(MultiRoute::Api);
    }

    #[test]
    fn test_route_with_many_variants() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum ManyVariantRoute {
            #[at("/")]
            Home,
            #[at("/about")]
            About,
            #[at("/services")]
            Services,
            #[at("/products")]
            Products,
            #[at("/contact")]
            Contact
        }

        assert_eq!(ManyVariantRoute::Home.to_path(), "/");
        assert_eq!(ManyVariantRoute::About.to_path(), "/about");
        assert_eq!(ManyVariantRoute::Services.to_path(), "/services");
        assert_eq!(ManyVariantRoute::Products.to_path(), "/products");
        assert_eq!(ManyVariantRoute::Contact.to_path(), "/contact");
    }
}
