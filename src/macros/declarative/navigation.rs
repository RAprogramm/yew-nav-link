//! Navigation macros - nav_tabs, breadcrumbs, nav_menu

use yew::prelude::*;
use yew_router::prelude::*;

/// Generates a tabbed navigation interface
#[macro_export]
macro_rules! nav_tabs {
    () => { yew::html! {} };
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

/// Generates a breadcrumb trail from route-label pairs
#[macro_export]
macro_rules! breadcrumbs {
    () => { yew::html! {} };
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

/// Generates a navigation menu (`<ul>`) with items (`<li>`)
#[macro_export]
macro_rules! nav_menu {
    () => { yew::html! {} };
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
