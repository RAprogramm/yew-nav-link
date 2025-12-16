// Import necessary modules from Yew and Yew Router
use std::marker::PhantomData;

use yew::prelude::*;
use yew_router::prelude::*;

/// Props for NavLink component.
/// R: Routable - A trait bound to ensure that this component can only be used
/// with routable types.
#[derive(Properties, PartialEq, Debug)]
pub struct NavLinkProps<R: Routable + PartialEq + Clone + 'static> {
    /// The destination route for the link.
    pub to:             R,
    /// Children of this component (usually text or other elements to be
    /// rendered within the link).
    pub children:       Children,
    /// Marker for the generic type R. It does not hold any value.
    #[prop_or_default]
    pub(crate) _marker: PhantomData<R>
}

/// NavLink component for Yew applications using Yew Router.
///
/// This component creates a navigational link that is aware of its active
/// state, based on the current route in the application.
///
/// The component is generic over `R`, where `R` must implement the `Routable`
/// trait. This allows the NavLink to be used with any set of routable types.
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_nav_link::NavLink;
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Debug, Routable)]
/// enum AppRoute {
///     #[at("/")]
///     Home,
///     #[at("/about")]
///     About
/// }
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <nav>
///             <NavLink<AppRoute> to={AppRoute::Home}>{ "Home" }</NavLink<AppRoute>>
///             <NavLink<AppRoute> to={AppRoute::About}>{ "About" }</NavLink<AppRoute>>
///             // ... other NavLinks
///         </nav>
///     }
/// }
/// ```
#[component]
pub fn NavLink<R: Routable + PartialEq + Clone + 'static>(props: &NavLinkProps<R>) -> Html {
    // use_route hook is used to get the current route of the application.
    let current_route = use_route::<R>();

    let is_active = current_route.is_some_and(|route| route == props.to);

    // CSS class for the NavLink.
    // The 'active' class is conditionally added based on the active state.
    let class = format!("nav-link {}", if is_active { "active" } else { "" });

    // Render the NavLink using the Yew's Link component.
    // The Link component is responsible for handling the navigation.
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
/// #[function_component(Menu)]
/// fn menu() -> Html {
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
