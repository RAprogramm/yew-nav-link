//! # NavLink
//!
//! Drop-in replacement for Yew Router's `<Link>` that automatically adds an
//! `active` CSS class when the current URL matches the target route. This
//! means you never have to manually track which page the user is on —
//! `NavLink` does it for you.
//!
//! # Quick Start
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::NavLink;
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Routable)]
//! # enum Route {
//! #     #[at("/")]
//! #     Home,
//! #     #[at("/about")]
//! #     About,
//! # }
//! #[component]
//! fn Nav() -> Html {
//!     html! {
//!         <nav>
//!             <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
//!             <NavLink<Route> to={Route::About}>{ "About" }</NavLink<Route>>
//!         </nav>
//!     }
//! }
//! ```
//!
//! When the user visits `/about`, the second link automatically gets
//! `class="nav-link active"`. All other links get `class="nav-link"`.
//!
//! # CSS Classes
//!
//! | Class | When Applied |
//! |-------|--------------|
//! | `nav-link` | Always |
//! | `active` | When the target route matches the current URL |
//!
//! This works out of the box with Bootstrap (`nav-link active`), Tailwind
//! (use `class:` bindings), or any CSS framework.
//!
//! # Exact vs Partial Matching
//!
//! By default (`partial=false`), NavLink uses **exact** matching: the link
//! is active only when the URL matches the route path exactly.
//!
//! With `partial=true`, NavLink uses **prefix** matching: the link stays
//! active on any nested route. This is useful for sidebar navigation where
//! a parent section should highlight even when the user is on a sub-page.
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::NavLink;
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Routable)]
//! # enum Route {
//! #     #[at("/docs")]
//! #     Docs,
//! #     #[at("/docs/api")]
//! #     DocsApi,
//! # }
//! #[component]
//! fn Nav() -> Html {
//!     html! {
//!         <nav>
//!             // Exact: active ONLY on /docs
//!             <NavLink<Route> to={Route::Docs}>{ "Docs (exact)" }</NavLink<Route>>
//!
//!             // Partial: active on /docs, /docs/api, /docs/anything
//!             <NavLink<Route> to={Route::Docs} partial=true>
//!                 { "Docs (partial)" }
//!             </NavLink<Route>>
//!         </nav>
//!     }
//! }
//! ```
//!
//! Partial matching compares path segments, so `/docs` won't accidentally
//! match `/documentation`.
//!
//! # Function Syntax
//!
//! If you prefer a function call instead of JSX, use `nav_link()`:
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{Match, nav_link};
//! use yew_router::prelude::*;
//!
//! # #[derive(Clone, PartialEq, Debug, Routable)]
//! # enum Route {
//! #     #[at("/")]
//! #     Home,
//! #     #[at("/docs")]
//! #     Docs,
//! # }
//! #[component]
//! fn Menu() -> Html {
//!     html! {
//!         <ul class="nav">
//!             <li>{ nav_link(Route::Home, "Home", Match::Exact) }</li>
//!             <li>{ nav_link(Route::Docs, "Docs", Match::Partial) }</li>
//!         </ul>
//!     }
//! }
//! ```
//!
//! # Props
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `to` | `R` | — | Target route (required) |
//! | `children` | `Children` | — | Link content (required) |
//! | `partial` | `bool` | `false` | Enable prefix matching |
//!
//! # How It Works
//!
//! On every render, NavLink calls `use_route::<R>()` to get the current route,
//! then compares it with the `to` prop using Yew Router's `Routable::eq()`.
//! If they match (exactly or by prefix), the `active` class is added.
//!
//! Partial matching compares path segments: `/docs` matches `/docs/api` and
//! `/docs/api/v1`, but NOT `/documentation`. This prevents false positives
//! on similar-looking paths.
//!
//! # Memory & Performance
//!
//! NavLink is lightweight — it only clones the route `R` once per render and
//! does a single `==` comparison (or one `starts_with` for partial). No heap
//! allocations beyond what Yew Router's `use_route()` already does.
//!
//! If you have many links (100+), each one independently calls `use_route()`.
//! Yew batches these into a single subscription, so there's no extra cost
//! per link beyond the comparison itself.

use std::marker::PhantomData;

use yew::prelude::*;
use yew_router::prelude::*;

/// Path matching strategy for NavLink active state detection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Match {
    /// Link is active only when paths match exactly.
    #[default]
    Exact,
    /// Link is active when current path starts with target path (segment-wise).
    Partial,
}

/// Properties for the [`NavLink`] component.
#[derive(Properties, PartialEq, Debug)]
pub struct NavLinkProps<R: Routable + PartialEq + Clone + 'static> {
    /// Target route for navigation.
    pub to: R,

    /// Content rendered inside the link element.
    pub children: Children,

    /// Enable partial (prefix) path matching.
    ///
    /// When `false` (default), the link is active only on exact path match.
    /// When `true`, the link is active if current path starts with target path.
    #[prop_or(false)]
    pub partial: bool,

    /// Base CSS class applied to the link.
    #[prop_or("nav-link")]
    pub class: &'static str,

    /// CSS class applied when the link is active.
    #[prop_or("active")]
    pub active_class: &'static str,

    #[prop_or_default]
    pub(crate) _marker: PhantomData<R>,
}

/// Navigation link with automatic active state detection.
///
/// # CSS Classes
///
/// - `nav-link` - Always applied
/// - `active` - Applied when route matches current URL
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_nav_link::NavLink;
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Routable)]
/// enum Route {
///     #[at("/")]
///     Home,
///     #[at("/about")]
///     About
/// }
///
/// #[component]
/// fn Navigation() -> Html {
///     html! {
///         <nav>
///             <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
///             <NavLink<Route> to={Route::About}>{ "About" }</NavLink<Route>>
///         </nav>
///     }
/// }
/// ```
#[component]
pub fn NavLink<R: Routable + PartialEq + Clone + 'static>(props: &NavLinkProps<R>) -> Html {
    let current_route = use_route::<R>();
    let is_active = current_route.is_some_and(|route| {
        if props.partial {
            is_path_prefix(&props.to.to_path(), &route.to_path())
        } else {
            route == props.to
        }
    });

    html! {
        <Link<R> to={props.to.clone()} classes={classes!(build_class(is_active, props.class, props.active_class))}>
            { for props.children.iter() }
        </Link<R>>
    }
}

/// Creates a NavLink with the specified match mode.
///
/// # Arguments
///
/// * `to` - Target route
/// * `children` - Link text
/// * `match_mode` - [`Match::Exact`] or [`Match::Partial`]
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_nav_link::{Match, nav_link};
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Debug, Routable)]
/// enum Route {
///     #[at("/")]
///     Home,
///     #[at("/docs")]
///     Docs
/// }
///
/// #[component]
/// fn Menu() -> Html {
///     html! {
///         <nav>
///             { nav_link(Route::Home, "Home", Match::Exact) }
///             { nav_link(Route::Docs, "Docs", Match::Partial) }
///         </nav>
///     }
/// }
/// ```
pub fn nav_link<R: Routable + PartialEq + Clone + 'static>(
    to: R,
    children: &str,
    match_mode: Match,
) -> Html {
    let partial = match_mode == Match::Partial;
    html! {
        <NavLink<R> to={to} {partial}>{ Html::from(children) }</NavLink<R>>
    }
}

/// Checks if `target` path is a segment-wise prefix of `current` path.
///
/// Uses iterators without heap allocation for efficiency during renders.
///
/// # Examples
///
/// ```text
/// is_path_prefix("/docs", "/docs/api")  -> true
/// is_path_prefix("/docs", "/docs")      -> true
/// is_path_prefix("/doc", "/documents")  -> false (segment boundary)
/// is_path_prefix("/", "/anything")      -> true
/// ```
#[inline]
pub(crate) fn is_path_prefix(target: &str, current: &str) -> bool {
    let mut target_iter = target.split('/').filter(|s| !s.is_empty());
    let mut current_iter = current.split('/').filter(|s| !s.is_empty());

    loop {
        match (target_iter.next(), current_iter.next()) {
            (Some(t), Some(c)) if t == c => continue,
            (Some(_), Some(_)) => return false,
            (Some(_), None) => return false,
            (None, _) => return true,
        }
    }
}

#[inline]
fn build_class(is_active: bool, base_class: &str, active_class: &str) -> String {
    if is_active {
        format!("{} {}", base_class, active_class)
    } else {
        base_class.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Debug, Routable)]
    enum TestRoute {
        #[at("/")]
        Home,
        #[at("/about")]
        About,
        #[at("/docs")]
        Docs,
        #[at("/docs/api")]
        DocsApi,
    }

    // Match enum tests
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
        assert_eq!(format!("{:?}", Match::Partial), "Partial");
    }

    #[test]
    fn match_clone() {
        let m = Match::Partial;
        let cloned = m;
        assert_eq!(m, cloned);
    }

    // build_class tests
    #[test]
    fn build_class_active() {
        assert_eq!(build_class(true, "nav-link", "active"), "nav-link active");
    }

    #[test]
    fn build_class_inactive() {
        assert_eq!(build_class(false, "nav-link", "active"), "nav-link");
    }

    #[test]
    fn build_class_custom_classes() {
        assert_eq!(
            build_class(true, "custom-link", "is-active"),
            "custom-link is-active"
        );
        assert_eq!(
            build_class(false, "custom-link", "is-active"),
            "custom-link"
        );
    }

    // NavLinkProps tests
    #[test]
    fn props_equality_same() {
        let props1: NavLinkProps<TestRoute> = NavLinkProps {
            to: TestRoute::Home,
            children: Default::default(),
            partial: false,
            class: "nav-link",
            active_class: "active",
            _marker: PhantomData,
        };
        let props2: NavLinkProps<TestRoute> = NavLinkProps {
            to: TestRoute::Home,
            children: Default::default(),
            partial: false,
            class: "nav-link",
            active_class: "active",
            _marker: PhantomData,
        };
        assert_eq!(props1, props2);
    }

    #[test]
    fn props_equality_different_route() {
        let props1: NavLinkProps<TestRoute> = NavLinkProps {
            to: TestRoute::Home,
            children: Default::default(),
            partial: false,
            class: "nav-link",
            active_class: "active",
            _marker: PhantomData,
        };
        let props2: NavLinkProps<TestRoute> = NavLinkProps {
            to: TestRoute::About,
            children: Default::default(),
            partial: false,
            class: "nav-link",
            active_class: "active",
            _marker: PhantomData,
        };
        assert_ne!(props1, props2);
    }

    #[test]
    fn props_equality_different_partial() {
        let props1: NavLinkProps<TestRoute> = NavLinkProps {
            to: TestRoute::Home,
            children: Default::default(),
            partial: false,
            class: "nav-link",
            active_class: "active",
            _marker: PhantomData,
        };
        let props2: NavLinkProps<TestRoute> = NavLinkProps {
            to: TestRoute::Home,
            children: Default::default(),
            partial: true,
            class: "nav-link",
            active_class: "active",
            _marker: PhantomData,
        };
        assert_ne!(props1, props2);
    }

    #[test]
    fn props_debug() {
        let props: NavLinkProps<TestRoute> = NavLinkProps {
            to: TestRoute::Home,
            children: Default::default(),
            partial: false,
            class: "nav-link",
            active_class: "active",
            _marker: PhantomData,
        };
        let debug = format!("{:?}", props);
        assert!(debug.contains("NavLinkProps"));
        assert!(debug.contains("Home"));
    }

    // nav_link function tests
    #[test]
    fn nav_link_exact_returns_html() {
        let html = nav_link(TestRoute::Home, "Home", Match::Exact);
        assert!(matches!(html, Html::VComp(_)));
    }

    #[test]
    fn nav_link_partial_returns_html() {
        let html = nav_link(TestRoute::Docs, "Docs", Match::Partial);
        assert!(matches!(html, Html::VComp(_)));
    }

    #[test]
    fn nav_link_different_routes() {
        let h1 = nav_link(TestRoute::Home, "Home", Match::Exact);
        let h2 = nav_link(TestRoute::About, "About", Match::Exact);
        assert!(matches!(h1, Html::VComp(_)));
        assert!(matches!(h2, Html::VComp(_)));
    }

    #[test]
    fn nav_link_empty_text() {
        let html = nav_link(TestRoute::Home, "", Match::Exact);
        assert!(matches!(html, Html::VComp(_)));
    }

    // is_path_prefix tests - exact matches
    #[test]
    fn prefix_exact_match() {
        assert!(is_path_prefix("/", "/"));
        assert!(is_path_prefix("/docs", "/docs"));
        assert!(is_path_prefix("/docs/api", "/docs/api"));
    }

    // is_path_prefix tests - valid prefixes
    #[test]
    fn prefix_valid() {
        assert!(is_path_prefix("/docs", "/docs/api"));
        assert!(is_path_prefix("/docs", "/docs/api/ref"));
        assert!(is_path_prefix("/a", "/a/b/c/d"));
    }

    #[test]
    fn prefix_root_matches_all() {
        assert!(is_path_prefix("/", "/docs"));
        assert!(is_path_prefix("/", "/docs/api"));
        assert!(is_path_prefix("/", "/any/path/here"));
    }

    // is_path_prefix tests - not prefixes
    #[test]
    fn prefix_not_prefix() {
        assert!(!is_path_prefix("/docs/api", "/docs"));
        assert!(!is_path_prefix("/about", "/docs"));
        assert!(!is_path_prefix("/a/b/c", "/a/b"));
    }

    #[test]
    fn prefix_segment_boundary() {
        assert!(!is_path_prefix("/doc", "/documents"));
        assert!(!is_path_prefix("/api", "/api-v2"));
        assert!(!is_path_prefix("/user", "/users"));
    }

    // is_path_prefix tests - edge cases
    #[test]
    fn prefix_trailing_slashes() {
        assert!(is_path_prefix("/docs/", "/docs/api"));
        assert!(is_path_prefix("/docs", "/docs/api/"));
        assert!(is_path_prefix("/docs/", "/docs/"));
    }

    #[test]
    fn prefix_multiple_slashes() {
        assert!(is_path_prefix("/docs//", "/docs/api"));
        assert!(is_path_prefix("//docs", "/docs//api"));
    }

    #[test]
    fn prefix_empty_paths() {
        assert!(is_path_prefix("", "/docs"));
        assert!(is_path_prefix("", ""));
        assert!(!is_path_prefix("/docs", ""));
    }

    // Route tests
    #[test]
    fn route_equality() {
        assert_eq!(TestRoute::Home, TestRoute::Home);
        assert_ne!(TestRoute::Home, TestRoute::About);
    }

    #[test]
    fn route_to_path() {
        assert_eq!(TestRoute::Home.to_path(), "/");
        assert_eq!(TestRoute::About.to_path(), "/about");
        assert_eq!(TestRoute::Docs.to_path(), "/docs");
        assert_eq!(TestRoute::DocsApi.to_path(), "/docs/api");
    }
}
