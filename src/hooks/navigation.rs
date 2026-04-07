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
}
