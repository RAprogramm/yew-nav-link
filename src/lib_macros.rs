//! Macro implementations for yew-nav-link
//!
//! This module provides declarative macros for reducing boilerplate
//! when building navigation interfaces.
//!
//! Enable the `macros` feature in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! yew-nav-link = { version = "0.7", features = ["macros"] }
//! ```

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

/// Generates a pagination control component.
///
/// # Parameters
///
/// - `current_page` — current page number (required)
/// - `total_pages` — total number of pages (required)
/// - `siblings` — number of page links around current page (default: 1)
/// - `show_first_last` — show first/last page buttons (default: false)
/// - `show_prev_next` — show prev/next buttons (default: true)
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_nav_link::nav_pagination;
///
/// #[component]
/// fn Paginator() -> Html {
///     let current_page = 3u32;
///     let total_pages = 20u32;
///     html! {
///         { nav_pagination!(
///             current_page = current_page,
///             total_pages = total_pages,
///             siblings = 2,
///             show_first_last = true,
///         ) }
///     }
/// }
/// ```
#[macro_export]
macro_rules! nav_pagination {
    () => {
        yew::html! {}
    };
    (current_page = $current_page:expr, total_pages = $total_pages:expr $(,)?) => {{
        let __cp: u32 = $current_page;
        let __tp: u32 = $total_pages;
        let __siblings: usize = 1;
        let __sfl: bool = false;
        let __spn: bool = true;
        $crate::__pagination_render!(__cp, __tp, __siblings, __sfl, __spn)
    }};
    (current_page = $current_page:expr, total_pages = $total_pages:expr, siblings = $siblings:expr $(,)?) => {{
        let __cp: u32 = $current_page;
        let __tp: u32 = $total_pages;
        let __siblings: usize = $siblings;
        let __sfl: bool = false;
        let __spn: bool = true;
        $crate::__pagination_render!(__cp, __tp, __siblings, __sfl, __spn)
    }};
    (current_page = $current_page:expr, total_pages = $total_pages:expr, show_first_last = $sfl:expr $(,)?) => {{
        let __cp: u32 = $current_page;
        let __tp: u32 = $total_pages;
        let __siblings: usize = 1;
        let __sfl: bool = $sfl;
        let __spn: bool = true;
        $crate::__pagination_render!(__cp, __tp, __siblings, __sfl, __spn)
    }};
    (current_page = $current_page:expr, total_pages = $total_pages:expr, show_prev_next = $spn:expr $(,)?) => {{
        let __cp: u32 = $current_page;
        let __tp: u32 = $total_pages;
        let __siblings: usize = 1;
        let __sfl: bool = false;
        let __spn: bool = $spn;
        $crate::__pagination_render!(__cp, __tp, __siblings, __sfl, __spn)
    }};
    (current_page = $current_page:expr, total_pages = $total_pages:expr, siblings = $siblings:expr, show_first_last = $sfl:expr $(,)?) => {{
        let __cp: u32 = $current_page;
        let __tp: u32 = $total_pages;
        let __siblings: usize = $siblings;
        let __sfl: bool = $sfl;
        let __spn: bool = true;
        $crate::__pagination_render!(__cp, __tp, __siblings, __sfl, __spn)
    }};
    (current_page = $current_page:expr, total_pages = $total_pages:expr, siblings = $siblings:expr, show_first_last = $sfl:expr, show_prev_next = $spn:expr $(,)?) => {{
        let __cp: u32 = $current_page;
        let __tp: u32 = $total_pages;
        let __siblings: usize = $siblings;
        let __sfl: bool = $sfl;
        let __spn: bool = $spn;
        $crate::__pagination_render!(__cp, __tp, __siblings, __sfl, __spn)
    }};
    (current_page = $current_page:expr, total_pages = $total_pages:expr, siblings = $siblings:expr, show_prev_next = $spn:expr $(,)?) => {{
        let __cp: u32 = $current_page;
        let __tp: u32 = $total_pages;
        let __siblings: usize = $siblings;
        let __sfl: bool = false;
        let __spn: bool = $spn;
        $crate::__pagination_render!(__cp, __tp, __siblings, __sfl, __spn)
    }};
    (current_page = $current_page:expr, total_pages = $total_pages:expr, show_first_last = $sfl:expr, show_prev_next = $spn:expr $(,)?) => {{
        let __cp: u32 = $current_page;
        let __tp: u32 = $total_pages;
        let __siblings: usize = 1;
        let __sfl: bool = $sfl;
        let __spn: bool = $spn;
        $crate::__pagination_render!(__cp, __tp, __siblings, __sfl, __spn)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __pagination_render {
    ($cp:ident, $tp:ident, $sib:ident, $sfl:ident, $spn:ident) => {
        yew::html! {
            <nav aria-label="pagination">
                <ul class="pagination">
                    if $spn {
                        <li class="page-item">
                            <button class="page-link" disabled={$cp <= 1}>
                                {"Previous"}
                            </button>
                        </li>
                    }

                    { for (1..=$tp).map(|i| {
                        yew::html! {
                            <li class={yew::classes!("page-item", if i == $cp { "active" } else { "" })}>
                                <button class="page-link">
                                    { i.to_string() }
                                </button>
                            </li>
                        }
                    }) }

                    if $spn {
                        <li class="page-item">
                            <button class="page-link" disabled={$cp >= $tp}>
                                {"Next"}
                            </button>
                        </li>
                    }
                </ul>
            </nav>
        }
    };
}

/// Extends a `Routable` enum with navigation helper methods.
///
/// Generates the following methods:
/// - `fn nav_label(&self) -> &'static str` — human-readable label for the route
/// - `fn nav_icon(&self) -> Option<&'static str>` — optional icon class
/// - `fn is_root(&self) -> bool` — whether the route path is `/`
///
/// # Example
///
/// ```rust
/// use yew_router::prelude::*;
/// use yew_nav_link::routable_ext;
///
/// #[derive(Clone, PartialEq, Routable)]
/// enum Route {
///     #[at("/")]
///     Home,
///     #[at("/about")]
///     About,
///     #[at("/docs")]
///     Docs,
/// }
///
/// routable_ext!(Route,
///     labels: {
///         Home => "Home",
///         About => "About Us",
///         Docs => "Documentation",
///     }
/// );
/// ```
#[macro_export]
macro_rules! routable_ext {
    ($name:ident, labels: { $( $variant:ident => $label:expr ),+ $(,)? }, icons: { $( $icon_variant:ident => $icon:expr ),+ $(,)? }) => {
        impl $name {
            /// Returns a human-readable label for this route.
            pub fn nav_label(&self) -> &'static str {
                match self {
                    $( $name::$variant => $label, )+
                }
            }

            /// Returns an optional icon class for this route.
            pub fn nav_icon(&self) -> Option<&'static str> {
                match self {
                    $( $name::$icon_variant => Some($icon), )+
                    _ => None
                }
            }

            /// Returns `true` if this route's path is `/`.
            pub fn is_root(&self) -> bool {
                self.to_path() == "/"
            }
        }
    };
    ($name:ident, labels: { $( $variant:ident => $label:expr ),+ $(,)? }) => {
        impl $name {
            /// Returns a human-readable label for this route.
            pub fn nav_label(&self) -> &'static str {
                match self {
                    $( $name::$variant => $label, )+
                }
            }

            /// Returns `true` if this route's path is `/`.
            pub fn is_root(&self) -> bool {
                self.to_path() == "/"
            }
        }
    };
    ($name:ident) => {
        impl $name {
            /// Returns `true` if this route's path is `/`.
            pub fn is_root(&self) -> bool {
                self.to_path() == "/"
            }
        }
    };
}

/// Generates a navigation item struct with path and label for a route.
///
/// Creates a struct that pairs a route with a display label, useful for
/// building navigation menus programmatically.
///
/// # Example
///
/// ```rust
/// use yew_nav_link::nav_item;
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Routable)]
/// enum Route {
///     #[at("/")]
///     Home,
///     #[at("/about")]
///     About,
/// }
///
/// nav_item!(Route,
///     Home => "Home",
///     About => "About Us",
/// );
///
/// let items: Vec<NavItem<Route>> = nav_items();
/// ```
#[macro_export]
macro_rules! nav_item {
    ($name:ident, $( $variant:ident => $label:expr ),+ $(,)?) => {
        /// A navigation item pairing a route with a display label.
        #[derive(Clone, Debug)]
        pub struct NavItem<R> {
            /// The route this item navigates to.
            pub route: R,
            /// Human-readable label for display.
            pub label: &'static str,
        }

        impl<R> NavItem<R> {
            /// Creates a new navigation item.
            pub fn new(route: R, label: &'static str) -> Self {
                Self { route, label }
            }
        }

        /// Returns all navigation items for this route enum.
        pub fn nav_items() -> Vec<NavItem<$name>> {
            vec![
                $( NavItem::new($name::$variant, $label), )+
            ]
        }
    };
}
