//! Basic navigation macros - nav_link, nav_links, nav_list

use yew::prelude::*;
use yew_router::prelude::*;

/// Convenience wrapper around [`nav_link`](yew_nav_link::nav_link)
#[macro_export]
macro_rules! nav_link {
    ($route:expr, $label:expr, $mode:expr) => {
        yew_nav_link::nav_link($route, $label, $mode)
    };
}

/// Generates multiple navigation links as a single [`Html`] collection
#[macro_export]
macro_rules! nav_links {
    () => { yew::html! {} };
    ($( ($route:expr, $label:expr, $mode:expr) ),+ $(,)?) => {{
        let links: ::yew::Html = vec![
            $( yew_nav_link::nav_link($route, $label, $mode) ),+
        ].into_iter().collect();
        links
    }};
}

/// Generates a navigation list (`<ul>`) with items (`<li>`)
#[macro_export]
macro_rules! nav_list {
    () => { yew::html! {} };
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
