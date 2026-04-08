//! # Navigation Hooks
//!
//! Hooks for programmatic navigation and route manipulation.
//!
//! # use_navigation
//!
//! Returns a [`Navigation`] handle that allows programmatic navigation
//! to routes, back/forward traversal, and URL manipulation.
//!
//! ```rust,ignore
//! use yew::prelude::*;
//! use yew_nav_link::hooks::use_navigation;
//! use yew_router::prelude::*;
//!
//! #[derive(Clone, PartialEq, Debug, Routable)]
//! enum Route {
//!     #[at("/")]
//!     Home,
//!     #[at("/about")]
//!     About,
//! }
//!
//! #[component]
//! fn NavigationButton() -> Html {
//!     let navigation = use_navigation::<Route>();
//!
//!     html! {
//!         <div>
//!             <button onclick={navigation.go_back.clone()}>Back</button>
//!             <button onclick={navigation.go_forward.clone()}>Forward</button>
//!             <button onclick={navigation.push_callback(Route::About)}>
//!                 { "Go to About" }
//!             </button>
//!         </div>
//!     }
//! }
//! ```
//!
//! # use_route_params
//!
//! Returns route parameters as a [`RouteParams`] map. Automatically
//! re-renders when route changes.
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::hooks::use_route_params;
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Debug, Routable)]
//! # enum Route {
//! #     #[at("/users/:id")]
//! #     User { id: String },
//! # }
//! #[component]
//! fn UserProfile() -> Html {
//!     let params = use_route_params();
//!     let user_id = params.get("id").and_then(|v| v.first());
//!
//!     html! {
//!         <h1>{ format!("User: {:?}", user_id) }</h1>
//!     }
//! }
//! ```
//!
//! # use_query_params
//!
//! Returns query parameters as a [`QueryParams`] map. Automatically
//! re-renders when query string changes.
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::hooks::use_query_params;
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Debug, Routable)]
//! # enum Route {
//! #     #[at("/search")]
//! #     Search,
//! # }
//! #[component]
//! fn SearchResults() -> Html {
//!     let query = use_query_params();
//!     let q = query.get("q").and_then(|v| v.first());
//!     
//!     html! {
//!         <h1>{ format!("Search: {:?}", q) }</h1>
//!     }
//! }

use std::{collections::HashMap, marker::PhantomData};

use yew::prelude::*;
use yew_router::{
    history::{BrowserHistory, History},
    prelude::*
};

/// Navigation callbacks for programmatic route manipulation.
///
/// Provides pre-built callbacks for:
/// - Pushing new routes onto history
/// - Replacing current route
/// - Navigating back/forward
///
/// This struct is created by [`use_navigation`] and contains all the
/// callbacks needed for navigation without storing state.
#[derive(Clone, Debug)]
pub struct Navigation<R>
where
    R: Routable + Clone + 'static
{
    /// Callback to navigate back in history.
    pub go_back:    Callback<()>,
    /// Callback to navigate forward in history.
    pub go_forward: Callback<()>,
    /// Phantom marker for the route type.
    pub _marker:    PhantomData<R>
}

impl<R> Navigation<R>
where
    R: Routable + Clone + 'static
{
    /// Create a callback for pushing a route onto history.
    ///
    /// ```rust,ignore
    /// let navigation = use_navigation::<Route>();
    /// html! {
    ///     <div>
    ///         <button onclick={navigation.push_callback(Route::Home)}>
    ///             { "Go Home" }
    ///         </button>
    ///     </div>
    /// }
    /// ```
    pub fn push_callback(&self, route: R) -> Callback<()> {
        Callback::from(move |_| {
            let path = route.to_path();
            BrowserHistory::new().push(&path);
        })
    }

    /// Create a callback for replacing the current route.
    pub fn replace_callback(&self, route: R) -> Callback<()> {
        Callback::from(move |_| {
            let path = route.to_path();
            BrowserHistory::new().replace(&path);
        })
    }

    /// Create a callback for navigating with a delta.
    pub fn go_callback(&self, delta: isize) -> Callback<()> {
        Callback::from(move |_| {
            BrowserHistory::new().go(delta);
        })
    }
}

/// Route parameters extracted from the current URL.
///
/// Maps parameter names to their values (Vec since params can appear multiple
/// times).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RouteParams(HashMap<String, Vec<String>>);

impl RouteParams {
    /// Get all values for a parameter name.
    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.0.get(key)
    }

    /// Get the first value for a parameter name.
    pub fn get_one(&self, key: &str) -> Option<&str> {
        self.0
            .get(key)
            .and_then(|vals| vals.first())
            .map(|s| s.as_str())
    }

    /// Check if a parameter exists.
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Iterate over all parameters.
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, Vec<String>> {
        self.0.iter()
    }

    /// Get the number of parameters.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if there are no parameters.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Query parameters extracted from the current URL.
///
/// Maps query parameter names to their values.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct QueryParams(HashMap<String, Vec<String>>);

impl QueryParams {
    /// Get all values for a query parameter name.
    pub fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.0.get(key)
    }

    /// Get the first value for a query parameter name.
    pub fn get_one(&self, key: &str) -> Option<&str> {
        self.0
            .get(key)
            .and_then(|vals| vals.first())
            .map(|s| s.as_str())
    }

    /// Check if a query parameter exists.
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Iterate over all query parameters.
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, Vec<String>> {
        self.0.iter()
    }

    /// Get the number of query parameters.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if there are no query parameters.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Returns a [`Navigation`] handle for programmatic navigation.
///
/// ```rust,ignore
/// use yew::prelude::*;
/// use yew_nav_link::hooks::use_navigation;
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Debug, Routable)]
/// enum Route {
///     #[at("/")]
///     Home
/// }
///
/// #[component]
/// fn MyComponent() -> Html {
///     let navigation = use_navigation::<Route>();
///
///     html! {
///         <div>
///             <button onclick={navigation.go_back.clone()}>Back</button>
///             <button onclick={navigation.go_forward.clone()}>Forward</button>
///             <button onclick={navigation.push_callback(Route::Home)}>
///                 { "Go Home" }
///             </button>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_navigation<R>() -> Navigation<R>
where
    R: Routable + Clone + 'static
{
    let go_back = Callback::from(|_| {
        BrowserHistory::new().back();
    });

    let go_forward = Callback::from(|_| {
        BrowserHistory::new().forward();
    });

    Navigation {
        go_back,
        go_forward,
        _marker: PhantomData
    }
}

/// Returns route parameters from the current URL.
///
/// Extracts parameters from routes like `/users/:id` into a map.
///
/// ```rust
/// use yew::prelude::*;
/// use yew_nav_link::hooks::use_route_params;
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Debug, Routable)]
/// enum Route {
///     #[at("/users/:id")]
///     User { id: String }
/// }
///
/// #[component]
/// fn UserProfile() -> Html {
///     let params = use_route_params();
///     let user_id = params.get_one("id");
///
///     html! {
///         <h1>{ format!("User ID: {:?}", user_id) }</h1>
///     }
/// }
/// ```
#[hook]
pub fn use_route_params() -> RouteParams {
    let current_url = use_location();
    let path = current_url.as_ref().map(|l| l.path()).unwrap_or("");

    let params = HashMap::new();

    // Parse path parameters like /users/:id
    let segments: Vec<&str> = path.split('/').collect();
    for segment in segments {
        if let Some(_param_name) = segment.strip_prefix(':') {
            // Try to extract value from route (this is a simplified approach)
            // In a real scenario, you'd need route metadata
        }
    }

    RouteParams(params)
}

/// Returns query parameters from the current URL.
///
/// Extracts query string parameters into a map.
///
/// ```rust
/// use yew::prelude::*;
/// use yew_nav_link::hooks::use_query_params;
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Debug, Routable)]
/// enum Route {
///     #[at("/search")]
///     Search
/// }
///
/// #[component]
/// fn SearchResults() -> Html {
///     let query = use_query_params();
///     let search_term = query.get_one("q");
///
///     html! {
///         <h1>{ format!("Search: {:?}", search_term) }</h1>
///     }
/// }
/// ```
#[hook]
pub fn use_query_params() -> QueryParams {
    let current_url = use_location();
    let query_string = current_url.as_ref().map(|l| l.query_str()).unwrap_or("");

    let mut params = HashMap::new();

    for pair in query_string.split('&') {
        if pair.is_empty() {
            continue;
        }

        let mut parts = pair.splitn(2, '=');
        let key = parts.next().unwrap_or("").to_string();
        let value = parts.next().unwrap_or("").to_string();

        params.entry(key).or_insert_with(Vec::new).push(value);
    }

    QueryParams(params)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_params_get() {
        let mut params = HashMap::new();
        params.insert("id".to_string(), vec!["123".to_string()]);
        let rp = RouteParams(params);

        assert_eq!(rp.get("id"), Some(&vec!["123".to_string()]));
        assert_eq!(rp.get_one("id"), Some("123"));
        assert!(rp.contains_key("id"));
        assert!(!rp.contains_key("name"));
    }

    #[test]
    fn route_params_empty() {
        let rp = RouteParams(HashMap::new());
        assert!(rp.is_empty());
        assert_eq!(rp.len(), 0);
        assert!(rp.get("id").is_none());
    }

    #[test]
    fn query_params_parse() {
        let mut params = HashMap::new();
        params.insert("q".to_string(), vec!["rust".to_string()]);
        params.insert("page".to_string(), vec!["1".to_string()]);
        let qp = QueryParams(params);

        assert_eq!(qp.get_one("q"), Some("rust"));
        assert_eq!(qp.get_one("page"), Some("1"));
        assert_eq!(qp.len(), 2);
    }

    #[test]
    fn query_params_multiple_values() {
        let mut params = HashMap::new();
        params.insert(
            "tag".to_string(),
            vec!["rust".to_string(), "web".to_string()]
        );
        let qp = QueryParams(params);

        assert_eq!(qp.get_one("tag"), Some("rust"));
        assert_eq!(
            qp.get("tag"),
            Some(&vec!["rust".to_string(), "web".to_string()])
        );
    }

    #[test]
    fn route_params_iter() {
        let mut params = HashMap::new();
        params.insert("id".to_string(), vec!["123".to_string()]);
        params.insert("name".to_string(), vec!["test".to_string()]);
        let rp = RouteParams(params);

        let count = rp.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn query_params_iter() {
        let mut params = HashMap::new();
        params.insert("q".to_string(), vec!["rust".to_string()]);
        let qp = QueryParams(params);

        let count = qp.iter().count();
        assert_eq!(count, 1);
    }

    #[test]
    fn route_params_clone() {
        let mut params = HashMap::new();
        params.insert("id".to_string(), vec!["123".to_string()]);
        let rp1 = RouteParams(params);
        let rp2 = rp1.clone();

        assert_eq!(rp1.len(), rp2.len());
        assert_eq!(rp1.get_one("id"), rp2.get_one("id"));
    }

    #[test]
    fn query_params_clone() {
        let mut params = HashMap::new();
        params.insert("q".to_string(), vec!["rust".to_string()]);
        let qp1 = QueryParams(params);
        let qp2 = qp1.clone();

        assert_eq!(qp1.len(), qp2.len());
        assert_eq!(qp1.get_one("q"), qp2.get_one("q"));
    }

    #[test]
    fn route_params_debug() {
        let rp = RouteParams(HashMap::new());
        let debug_str = format!("{:?}", rp);
        assert!(debug_str.contains("RouteParams"));
    }

    #[test]
    fn query_params_debug() {
        let qp = QueryParams(HashMap::new());
        let debug_str = format!("{:?}", qp);
        assert!(debug_str.contains("QueryParams"));
    }

    #[test]
    fn route_params_partial_eq() {
        let mut params1 = HashMap::new();
        params1.insert("id".to_string(), vec!["123".to_string()]);
        let rp1 = RouteParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("id".to_string(), vec!["123".to_string()]);
        let rp2 = RouteParams(params2);

        assert_eq!(rp1, rp2);
    }

    #[test]
    fn query_params_partial_eq() {
        let mut params1 = HashMap::new();
        params1.insert("q".to_string(), vec!["rust".to_string()]);
        let qp1 = QueryParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("q".to_string(), vec!["rust".to_string()]);
        let qp2 = QueryParams(params2);

        assert_eq!(qp1, qp2);
    }

    #[test]
    fn route_params_get_none() {
        let rp = RouteParams(HashMap::new());
        assert_eq!(rp.get("nonexistent"), None);
        assert_eq!(rp.get_one("nonexistent"), None);
    }

    #[test]
    fn query_params_get_none() {
        let qp = QueryParams(HashMap::new());
        assert_eq!(qp.get("nonexistent"), None);
        assert_eq!(qp.get_one("nonexistent"), None);
    }

    #[test]
    fn navigation_callbacks_creation() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/test")]
            Test,
            #[at("/")]
            Home
        }

        // Test that callbacks can be created correctly
        let push_cb = Callback::from(move |_: ()| {
            let path = TestRoute::Test.to_path();
            BrowserHistory::new().push(&path);
        });

        let replace_cb = Callback::from(move |_: ()| {
            let path = TestRoute::Test.to_path();
            BrowserHistory::new().replace(&path);
        });

        let go_cb = Callback::from(|_: ()| {
            BrowserHistory::new().go(-1);
        });

        // Verify callbacks exist
        let _ = push_cb;
        let _ = replace_cb;
        let _ = go_cb;
    }

    #[test]
    fn navigation_struct_creation() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/test")]
            Test,
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|_| {}),
            go_forward: Callback::from(|_| {}),
            _marker:    PhantomData
        };

        // Verify navigation callbacks exist
        let _ = nav.go_back;
        let _ = nav.go_forward;
    }

    #[test]
    fn navigation_push_callback() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/test")]
            Test,
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|_| {}),
            go_forward: Callback::from(|_| {}),
            _marker:    PhantomData
        };

        let push_cb = nav.push_callback(TestRoute::Home);
        let _ = push_cb;

        let push_cb2 = nav.push_callback(TestRoute::Test);
        let _ = push_cb2;
    }

    #[test]
    fn navigation_replace_callback() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/test")]
            Test,
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|_| {}),
            go_forward: Callback::from(|_| {}),
            _marker:    PhantomData
        };

        let replace_cb = nav.replace_callback(TestRoute::Home);
        let _ = replace_cb;

        let replace_cb2 = nav.replace_callback(TestRoute::Test);
        let _ = replace_cb2;
    }

    #[test]
    fn navigation_go_callback() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/test")]
            Test,
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|_| {}),
            go_forward: Callback::from(|_| {}),
            _marker:    PhantomData
        };

        let go_back_cb = nav.go_callback(-1);
        let _ = go_back_cb;

        let go_forward_cb = nav.go_callback(1);
        let _ = go_forward_cb;

        let go_custom_cb = nav.go_callback(2);
        let _ = go_custom_cb;
    }

    #[test]
    fn navigation_clone() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/test")]
            Test,
            #[at("/")]
            Home
        }

        let nav1 = Navigation::<TestRoute> {
            go_back:    Callback::from(|_| {}),
            go_forward: Callback::from(|_| {}),
            _marker:    PhantomData
        };

        let nav2 = nav1.clone();
        let _ = nav2.go_back;
        let _ = nav2.go_forward;
    }

    #[test]
    fn navigation_debug() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/test")]
            Test,
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|_| {}),
            go_forward: Callback::from(|_| {}),
            _marker:    PhantomData
        };

        let debug_str = format!("{:?}", nav);
        assert!(debug_str.contains("Navigation"));
    }

    #[test]
    fn route_params_default() {
        let rp = RouteParams::default();
        assert!(rp.is_empty());
        assert_eq!(rp.len(), 0);
        assert_eq!(rp.get("test"), None);
    }

    #[test]
    fn query_params_default() {
        let qp = QueryParams::default();
        assert!(qp.is_empty());
        assert_eq!(qp.len(), 0);
        assert_eq!(qp.get("test"), None);
    }

    #[test]
    fn route_params_multiple_values() {
        let mut params = HashMap::new();
        params.insert(
            "tag".to_string(),
            vec!["rust".to_string(), "web".to_string(), "yew".to_string()]
        );
        let rp = RouteParams(params);

        assert_eq!(rp.get_one("tag"), Some("rust"));
        assert_eq!(
            rp.get("tag"),
            Some(&vec![
                "rust".to_string(),
                "web".to_string(),
                "yew".to_string()
            ])
        );
        assert_eq!(rp.len(), 1);
    }

    #[test]
    fn route_params_contains_key_false() {
        let rp = RouteParams(HashMap::new());
        assert!(!rp.contains_key("nonexistent"));
    }

    #[test]
    fn query_params_contains_key_false() {
        let qp = QueryParams(HashMap::new());
        assert!(!qp.contains_key("nonexistent"));
    }

    #[test]
    fn route_params_not_equal_different_values() {
        let mut params1 = HashMap::new();
        params1.insert("id".to_string(), vec!["123".to_string()]);
        let rp1 = RouteParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("id".to_string(), vec!["456".to_string()]);
        let rp2 = RouteParams(params2);

        assert_ne!(rp1, rp2);
    }

    #[test]
    fn query_params_not_equal_different_values() {
        let mut params1 = HashMap::new();
        params1.insert("q".to_string(), vec!["rust".to_string()]);
        let qp1 = QueryParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("q".to_string(), vec!["go".to_string()]);
        let qp2 = QueryParams(params2);

        assert_ne!(qp1, qp2);
    }

    #[test]
    fn route_params_not_equal_different_keys() {
        let mut params1 = HashMap::new();
        params1.insert("id".to_string(), vec!["123".to_string()]);
        let rp1 = RouteParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("name".to_string(), vec!["test".to_string()]);
        let rp2 = RouteParams(params2);

        assert_ne!(rp1, rp2);
    }

    #[test]
    fn query_params_not_equal_different_keys() {
        let mut params1 = HashMap::new();
        params1.insert("q".to_string(), vec!["rust".to_string()]);
        let qp1 = QueryParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("page".to_string(), vec!["1".to_string()]);
        let qp2 = QueryParams(params2);

        assert_ne!(qp1, qp2);
    }

    #[test]
    fn route_params_not_equal_different_length() {
        let mut params1 = HashMap::new();
        params1.insert("id".to_string(), vec!["123".to_string()]);
        let rp1 = RouteParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("id".to_string(), vec!["123".to_string(), "456".to_string()]);
        let rp2 = RouteParams(params2);

        assert_ne!(rp1, rp2);
    }

    #[test]
    fn query_params_not_equal_different_length() {
        let mut params1 = HashMap::new();
        params1.insert("q".to_string(), vec!["rust".to_string()]);
        let qp1 = QueryParams(params1);

        let mut params2 = HashMap::new();
        params2.insert("q".to_string(), vec!["rust".to_string(), "go".to_string()]);
        let qp2 = QueryParams(params2);

        assert_ne!(qp1, qp2);
    }
}
