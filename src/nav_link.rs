// Import necessary modules from Yew and Yew Router
use std::marker::PhantomData;
use yew::prelude::*;
use yew_router::prelude::*;

/// Props for NavLink component.
/// R: Routable - A trait bound to ensure that this component can only be used with routable types.
#[derive(Properties, PartialEq)]
pub struct NavLinkProps<R: Routable + PartialEq + Clone + 'static> {
    /// The destination route for the link.
    pub to: R,
    /// Children of this component (usually text or other elements to be rendered within the link).
    pub children: Children,
    /// Marker for the generic type R. It does not hold any value.
    #[prop_or_default]
    pub(crate) _marker: PhantomData<R>,
}

/// NavLink component for Yew applications using Yew Router.
///
/// This component creates a navigational link that is aware of its active state,
/// based on the current route in the application.
///
/// The component is generic over `R`, where `R` must implement the `Routable` trait.
/// This allows the NavLink to be used with any set of routable types.
///
/// # Example
///
/// ```rust
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
#[function_component(NavLink)]
pub fn nav_link<R: Routable + PartialEq + Clone + 'static>(props: &NavLinkProps<R>) -> Html {
    // use_route hook is used to get the current route of the application.
    let current_route = use_route::<R>();

    // Determine if the NavLink's route matches the current route.
    // If they match, the link is considered 'active'.
    let is_active = if let Some(route) = current_route {
        route == props.to
    } else {
        false
    };

    // CSS class for the NavLink.
    // The 'active' class is conditionally added based on the active state.
    let class = format!("nav-link {}", if is_active { "active" } else { "" });

    // Render the NavLink using the Yew's Link component.
    // The Link component is responsible for handling the navigation.
    html! {
        <Link<R> to={props.to.clone()} classes={classes!(class)}>
            // Rendering children elements passed to the NavLink.
            { for props.children.iter() }
        </Link<R>>
    }
}
