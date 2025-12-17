//! Navigation link component with automatic active state detection.
//!
//! This module provides the [`NavLink`] component and [`nav_link`] helper
//! function for building navigation menus in Yew applications. The component
//! automatically detects when its target route matches the current URL and
//! applies an `active` CSS class accordingly.
//!
//! # Features
//!
//! - **Automatic Active State**: Compares the target route against the current
//!   route and applies the `active` class when they match.
//! - **Type-Safe Routing**: Leverages Yew Router's [`Routable`] trait for
//!   compile-time route validation.
//! - **Flexible Children**: Accepts any valid Yew children, including text,
//!   HTML elements, or other components.
//! - **CSS Integration**: Renders with `nav-link` base class, compatible with
//!   Bootstrap and similar CSS frameworks.
//!
//! # CSS Classes
//!
//! | Class | Condition |
//! |-------|-----------|
//! | `nav-link` | Always applied |
//! | `active` | Applied when the target route matches the current route |
//!
//! # Match Modes
//!
//! NavLink supports two matching modes via the `partial` prop:
//!
//! - **Exact** (default): Link is active only when paths match exactly
//! - **Partial**: Link is active when current path starts with target path
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::NavLink;
//! use yew_router::prelude::*;
//!
//! #[derive(Clone, PartialEq, Routable)]
//! enum Route {
//!     #[at("/docs")]
//!     Docs,
//!     #[at("/docs/api")]
//!     DocsApi
//! }
//!
//! #[component]
//! fn Navigation() -> Html {
//!     html! {
//!         <nav>
//!             // Exact: active only on /docs
//!             <NavLink<Route> to={Route::Docs}>{ "Docs" }</NavLink<Route>>
//!             // Partial: active on /docs, /docs/api, /docs/*
//!             <NavLink<Route> to={Route::Docs} partial=true>{ "Docs" }</NavLink<Route>>
//!         </nav>
//!     }
//! }
//! ```
//!
//! # Function Syntax
//!
//! For text-only links, use [`nav_link`] with explicit [`Match`] mode:
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::{Match, nav_link};
//! use yew_router::prelude::*;
//!
//! #[derive(Clone, PartialEq, Debug, Routable)]
//! enum Route {
//!     #[at("/")]
//!     Home,
//!     #[at("/docs")]
//!     Docs
//! }
//!
//! #[component]
//! fn Navigation() -> Html {
//!     html! {
//!         <ul class="nav">
//!             <li>{ nav_link(Route::Home, "Home", Match::Exact) }</li>
//!             <li>{ nav_link(Route::Docs, "Docs", Match::Partial) }</li>
//!         </ul>
//!     }
//! }
//! ```

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
    Partial
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

    #[prop_or_default]
    pub(crate) _marker: PhantomData<R>
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
        <Link<R> to={props.to.clone()} classes={classes!(build_class(is_active))}>
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
    match_mode: Match
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
fn is_path_prefix(target: &str, current: &str) -> bool {
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
fn build_class(is_active: bool) -> &'static str {
    if is_active {
        "nav-link active"
    } else {
        "nav-link"
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
        DocsApi
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
        assert_eq!(build_class(true), "nav-link active");
    }

    #[test]
    fn build_class_inactive() {
        assert_eq!(build_class(false), "nav-link");
    }

    // NavLinkProps tests
    #[test]
    fn props_equality_same() {
        let props1: NavLinkProps<TestRoute> = NavLinkProps {
            to:       TestRoute::Home,
            children: Default::default(),
            partial:  false,
            _marker:  PhantomData
        };
        let props2: NavLinkProps<TestRoute> = NavLinkProps {
            to:       TestRoute::Home,
            children: Default::default(),
            partial:  false,
            _marker:  PhantomData
        };
        assert_eq!(props1, props2);
    }

    #[test]
    fn props_equality_different_route() {
        let props1: NavLinkProps<TestRoute> = NavLinkProps {
            to:       TestRoute::Home,
            children: Default::default(),
            partial:  false,
            _marker:  PhantomData
        };
        let props2: NavLinkProps<TestRoute> = NavLinkProps {
            to:       TestRoute::About,
            children: Default::default(),
            partial:  false,
            _marker:  PhantomData
        };
        assert_ne!(props1, props2);
    }

    #[test]
    fn props_equality_different_partial() {
        let props1: NavLinkProps<TestRoute> = NavLinkProps {
            to:       TestRoute::Home,
            children: Default::default(),
            partial:  false,
            _marker:  PhantomData
        };
        let props2: NavLinkProps<TestRoute> = NavLinkProps {
            to:       TestRoute::Home,
            children: Default::default(),
            partial:  true,
            _marker:  PhantomData
        };
        assert_ne!(props1, props2);
    }

    #[test]
    fn props_debug() {
        let props: NavLinkProps<TestRoute> = NavLinkProps {
            to:       TestRoute::Home,
            children: Default::default(),
            partial:  false,
            _marker:  PhantomData
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
