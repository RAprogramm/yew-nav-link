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
    let class = format!("nav-link {}", if is_active { "active" } else { "" });
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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Clone, PartialEq, Debug, Routable)]
    enum TestRoute {
        #[at("/")]
        Home,
        #[at("/about")]
        About
    }

    #[wasm_bindgen_test]
    fn nav_link_creates_html() {
        let html = nav_link(TestRoute::Home, "Home");
        assert!(matches!(html, Html::VTag(_) | Html::VComp(_)));
    }

    #[wasm_bindgen_test]
    fn nav_link_props_equality() {
        let props1 = NavLinkProps {
            to:       TestRoute::Home,
            children: Default::default(),
            _marker:  PhantomData
        };
        let props2 = NavLinkProps {
            to:       TestRoute::Home,
            children: Default::default(),
            _marker:  PhantomData
        };
        assert_eq!(props1, props2);
    }

    #[wasm_bindgen_test]
    fn nav_link_props_different_routes() {
        let props1 = NavLinkProps {
            to:       TestRoute::Home,
            children: Default::default(),
            _marker:  PhantomData
        };
        let props2 = NavLinkProps {
            to:       TestRoute::About,
            children: Default::default(),
            _marker:  PhantomData
        };
        assert_ne!(props1, props2);
    }
}
