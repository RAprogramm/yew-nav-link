use yew::prelude::*;
use yew_router::prelude::*;

/// Convenience wrapper around [`nav_link`](yew_nav_link::nav_link) for
/// declarative macro usage.
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_nav_link::{Match, nav_link};
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
/// fn Nav() -> Html {
///     html! {
///         <nav>
///             { nav_link!(Route::Home, "Home", Match::Exact) }
///             { nav_link!(Route::About, "About", Match::Exact) }
///         </nav>
///     }
/// }
/// ```
#[macro_export]
macro_rules! nav_link {
    ($route:expr, $label:expr, $mode:expr) => {
        yew_nav_link::nav_link($route, $label, $mode)
    };
}

/// Generates multiple navigation links as a single [`Html`] collection.
///
/// Each entry is `(Route, "Label", MatchMode)`.
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_nav_link::{Match, nav_links};
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Routable)]
/// enum Route {
///     #[at("/")]
///     Home,
///     #[at("/about")]
///     About,
///     #[at("/docs")]
///     Docs
/// }
///
/// #[component]
/// fn Nav() -> Html {
///     html! {
///         <nav class="nav">
///             { nav_links![
///                 (Route::Home, "Home", Match::Exact),
///                 (Route::About, "About", Match::Exact),
///                 (Route::Docs, "Docs", Match::Partial),
///             ] }
///         </nav>
///     }
/// }
/// ```
#[macro_export]
macro_rules! nav_links {
    () => {
        yew::html! {}
    };
    ($( ($route:expr, $label:expr, $mode:expr) ),+ $(,)?) => {{
        let links: ::yew::Html = vec![
            $( yew_nav_link::nav_link($route, $label, $mode) ),+
        ].into_iter().collect();
        links
    }};
}

/// Generates a navigation list (`<ul>`) with items (`<li>`) containing links.
///
/// Each entry is `(Route, "Label", MatchMode)`.
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_nav_link::{Match, nav_list};
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
/// fn Nav() -> Html {
///     html! {
///         { nav_list![
///             (Route::Home, "Home", Match::Exact),
///             (Route::About, "About", Match::Exact),
///         ] }
///     }
/// }
/// ```
#[macro_export]
macro_rules! nav_list {
    () => {
        yew::html! {}
    };
    ($( ($route:expr, $label:expr, $mode:expr) ),+ $(,)?) => {{
        yew::html! {
            <ul class="nav-list">
                $(
                    <li class="nav-item">
                        { yew_nav_link::nav_link($route, $label, $mode) }
                    </li>
                )+
            </ul>
        }
    }};
}
