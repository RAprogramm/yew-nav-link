use yew::prelude::*;
use yew_router::prelude::*;

/// Generates a tabbed navigation interface.
///
/// Each entry is `(Route, "Label")`.
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_nav_link::nav_tabs;
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Routable)]
/// enum Route {
///     #[at("/tab1")]
///     Tab1,
///     #[at("/tab2")]
///     Tab2
/// }
///
/// #[component]
/// fn Tabs() -> Html {
///     html! {
///         { nav_tabs![
///             (Route::Tab1, "Tab 1"),
///             (Route::Tab2, "Tab 2"),
///         ] }
///     }
/// }
/// ```
#[macro_export]
macro_rules! nav_tabs {
    () => {
        yew::html! {}
    };
    ($( ($route:expr, $label:expr) ),+ $(,)?) => {{
        yew::html! {
            <ul class="nav-tabs" role="tablist">
                $(
                    <li class="nav-tab">
                        <a href={$route.to_path()}>
                            { $label }
                        </a>
                    </li>
                )+
            </ul>
        }
    }};
}

/// Generates a breadcrumb trail from route-label pairs.
///
/// First argument is the route type, followed by `(variant, "Label")` pairs.
/// The last entry is marked as active.
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_nav_link::breadcrumbs;
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Routable)]
/// enum Route {
///     #[at("/")]
///     Home,
///     #[at("/docs")]
///     Docs,
///     #[at("/docs/api")]
///     DocsApi
/// }
///
/// #[component]
/// fn Breadcrumb() -> Html {
///     html! {
///         { breadcrumbs!(Route,
///             (Home, "Home"),
///             (Docs, "Docs"),
///             (DocsApi, "API"),
///         ) }
///     }
/// }
/// ```
#[macro_export]
macro_rules! breadcrumbs {
    () => {
        yew::html! {}
    };
    ($route_type:ident, $( ($variant:ident, $label:expr) ),+ $(,)?) => {{
        yew::html! {
            <nav aria-label="breadcrumb">
                <ol class="breadcrumb">
                    $(
                        <li class="breadcrumb-item">
                            <yew_router::prelude::Link<$route_type> to={$route_type::$variant}>
                                { $label }
                            </yew_router::prelude::Link<$route_type>>
                        </li>
                    )+
                </ol>
            </nav>
        }
    }};
}

/// Generates a navigation menu (`<ul>`) with items (`<li>`) containing links.
///
/// Each entry is `(Route, "Label", MatchMode)`.
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_nav_link::{Match, nav_menu};
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Routable)]
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
///         <nav class="menu">
///             { nav_menu![
///                 (Route::Home, "Home", Match::Exact),
///                 (Route::Docs, "Docs", Match::Partial),
///             ] }
///         </nav>
///     }
/// }
/// ```
#[macro_export]
macro_rules! nav_menu {
    () => {
        yew::html! {}
    };
    ($( ($route:expr, $label:expr, $mode:expr) ),+ $(,)?) => {{
        yew::html! {
            <ul class="nav-menu">
                $(
                    <li class="nav-item">
                        { yew_nav_link::nav_link($route, $label, $mode) }
                    </li>
                )+
            </ul>
        }
    }};
}
