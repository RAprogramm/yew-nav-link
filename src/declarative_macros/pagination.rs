use yew::prelude::*;
use yew_router::prelude::*;

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
