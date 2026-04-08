//! Pagination macro

use yew::prelude::*;

/// Generates a pagination control component
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
