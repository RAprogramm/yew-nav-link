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
//! The component applies the following CSS classes to the rendered `<a>`
//! element:
//!
//! | Class | Condition |
//! |-------|-----------|
//! | `nav-link` | Always applied |
//! | `active` | Applied when the target route matches the current route |
//!
//! # Usage
//!
//! ## Component Syntax
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::NavLink;
//! use yew_router::prelude::*;
//!
//! #[derive(Clone, PartialEq, Debug, Routable)]
//! enum Route {
//!     #[at("/")]
//!     Home,
//!     #[at("/about")]
//!     About
//! }
//!
//! #[component]
//! fn Navigation() -> Html {
//!     html! {
//!         <nav>
//!             <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
//!             <NavLink<Route> to={Route::About}>{ "About" }</NavLink<Route>>
//!         </nav>
//!     }
//! }
//! ```
//!
//! ## Function Syntax
//!
//! For simpler cases with text-only children:
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::nav_link;
//! use yew_router::prelude::*;
//!
//! #[derive(Clone, PartialEq, Debug, Routable)]
//! enum Route {
//!     #[at("/")]
//!     Home,
//!     #[at("/about")]
//!     About
//! }
//!
//! #[component]
//! fn Navigation() -> Html {
//!     html! {
//!         <ul class="nav">
//!             <li>{ nav_link(Route::Home, "Home") }</li>
//!             <li>{ nav_link(Route::About, "About") }</li>
//!         </ul>
//!     }
//! }
//! ```
//!
//! # Integration with CSS Frameworks
//!
//! The component works seamlessly with Bootstrap, Tailwind, and other CSS
//! frameworks that use the `.nav-link` and `.active` class conventions:
//!
//! ```html
//! <!-- Bootstrap Navigation -->
//! <ul class="nav nav-pills">
//!     <li class="nav-item">
//!         <!-- NavLink renders: <a class="nav-link active" href="/"> -->
//!     </li>
//! </ul>
//! ```

use std::marker::PhantomData;

use yew::prelude::*;
use yew_router::prelude::*;

/// Properties for the [`NavLink`] component.
///
/// # Type Parameters
///
/// * `R` - A type implementing [`Routable`] that defines the target route.
#[derive(Properties, PartialEq, Debug)]
pub struct NavLinkProps<R: Routable + PartialEq + Clone + 'static> {
    /// Target route for navigation.
    ///
    /// When clicked, the application navigates to this route.
    /// The component compares this value against the current route
    /// to determine active state.
    pub to: R,

    /// Content rendered inside the link element.
    ///
    /// Accepts any valid Yew children: text, HTML elements, or components.
    pub children: Children,

    #[prop_or_default]
    pub(crate) _marker: PhantomData<R>
}

/// Navigation link with automatic active state detection.
///
/// Wraps Yew Router's [`Link`] component and automatically applies the `active`
/// CSS class when the target route matches the current URL.
///
/// # CSS Classes
///
/// - `nav-link` - Always applied
/// - `active` - Applied when route matches current URL
///
/// # Type Parameters
///
/// * `R` - Route type implementing [`Routable`]
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
    let is_active = current_route.is_some_and(|route| route == props.to);
    let class = build_class(is_active);
    html! {
        <Link<R> to={props.to.clone()} classes={classes!(class)}>
            for child in props.children.iter() {
                { child }
            }
        </Link<R>>
    }
}

/// Creates a NavLink component for the specified route with the provided
/// children.
///
/// This function creates a NavLink component for Yew applications using Yew
/// Router. It takes a route (`R`) and children text, and returns a NavLink
/// component.
///
/// # Arguments
///
/// * `to` - The destination route for the link.
/// * `children` - The text or other elements to be rendered within the link.
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_nav_link::{NavLink, nav_link};
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Debug, Routable)]
/// enum HomeRoute {
///     #[at("/")]
///     IntroPage,
///     #[at("/about")]
///     About
/// }
///
/// #[component]
/// fn Menu() -> Html {
///     html! {
///         <ul class="nav">
///             // Creating a NavLink for the Home route with the text "Home Page"
///             <li class="nav-item">
///                 { nav_link(HomeRoute::IntroPage, "Home Page") }
///             </li>
///             <li class="nav-item">
///                 { nav_link(HomeRoute::About, "About") }
///             </li>
///         </ul>
///     }
/// }
/// ```
///
/// # Generic Type
///
/// * `R` - The route type that implements the `Routable` trait.
///
/// # Returns
///
/// An HTML representation of the NavLink component.
///
/// # Note
///
/// The `to` parameter must be of a type that implements the `Routable` trait.
pub fn nav_link<R: Routable + PartialEq + Clone + 'static>(to: R, children: &str) -> Html {
    html! {
        <NavLink<R> to={to}>{ Html::from(children) }</NavLink<R>>
    }
}

/// Generates CSS class string based on active state.
#[inline]
fn build_class(is_active: bool) -> String {
    if is_active {
        "nav-link active".to_string()
    } else {
        "nav-link".to_string()
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
        About
    }

    #[test]
    fn build_class_active() {
        assert_eq!(build_class(true), "nav-link active");
    }

    #[test]
    fn build_class_inactive() {
        assert_eq!(build_class(false), "nav-link");
    }

    #[test]
    fn props_equality_same_route() {
        let props1: NavLinkProps<TestRoute> = NavLinkProps {
            to:       TestRoute::Home,
            children: Default::default(),
            _marker:  PhantomData
        };
        let props2: NavLinkProps<TestRoute> = NavLinkProps {
            to:       TestRoute::Home,
            children: Default::default(),
            _marker:  PhantomData
        };
        assert_eq!(props1, props2);
    }

    #[test]
    fn props_equality_different_routes() {
        let props1: NavLinkProps<TestRoute> = NavLinkProps {
            to:       TestRoute::Home,
            children: Default::default(),
            _marker:  PhantomData
        };
        let props2: NavLinkProps<TestRoute> = NavLinkProps {
            to:       TestRoute::About,
            children: Default::default(),
            _marker:  PhantomData
        };
        assert_ne!(props1, props2);
    }

    #[test]
    fn props_debug_impl() {
        let props: NavLinkProps<TestRoute> = NavLinkProps {
            to:       TestRoute::Home,
            children: Default::default(),
            _marker:  PhantomData
        };
        let debug_str = format!("{:?}", props);
        assert!(debug_str.contains("NavLinkProps"));
        assert!(debug_str.contains("Home"));
    }

    #[test]
    fn nav_link_fn_returns_html() {
        let html = nav_link(TestRoute::Home, "Home");
        assert!(matches!(html, Html::VComp(_)));
    }

    #[test]
    fn nav_link_fn_different_routes() {
        let html1 = nav_link(TestRoute::Home, "Home");
        let html2 = nav_link(TestRoute::About, "About");
        assert!(matches!(html1, Html::VComp(_)));
        assert!(matches!(html2, Html::VComp(_)));
    }

    #[test]
    fn nav_link_fn_empty_text() {
        let html = nav_link(TestRoute::Home, "");
        assert!(matches!(html, Html::VComp(_)));
    }

    #[test]
    fn nav_link_fn_long_text() {
        let html = nav_link(TestRoute::Home, "This is a very long navigation link text");
        assert!(matches!(html, Html::VComp(_)));
    }

    #[test]
    fn route_equality() {
        assert_eq!(TestRoute::Home, TestRoute::Home);
        assert_ne!(TestRoute::Home, TestRoute::About);
    }
}
