// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! # Pagination
//!
//! Page navigation component with prev/next buttons, first/last shortcuts,
//! and configurable sibling pages with ellipsis gaps.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::components::Pagination;
//!
//! #[component]
//! fn Paginator() -> Html {
//!     let page = use_state(|| 1u32);
//!     let on_change = {
//!         let page = page.clone();
//!         Callback::from(move |p: u32| page.set(p))
//!     };
//!
//!     html! {
//!         <Pagination
//!             current_page={*page}
//!             total_pages={20}
//!             siblings={2}
//!             show_prev_next={true}
//!             on_page_change={Some(on_change)}
//!         />
//!     }
//! }
//! ```
//!
//! # CSS Classes
//!
//! | Class | Condition |
//! |-------|-----------|
//! | `pagination` | Container `<ul>` element |
//! | `pagination-item` | Each `<li>` page button wrapper |
//! | `pagination-ellipsis` | `<li>` holding a non-interactive `…` gap |
//! | `active` | Applied to the current page item |
//!
//! # Props
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `current_page` | `u32` | `1` | Currently active page (1-indexed) |
//! | `total_pages` | `u32` | `10` | Total number of pages |
//! | `siblings` | `u32` | `1` | Pages shown on each side of current |
//! | `show_prev_next` | `bool` | `true` | Show prev/next (`‹`/`›`) buttons |
//! | `show_first_last` | `bool` | `false` | Show first/last (`«`/`»`) jump buttons |
//! | `on_page_change` | `Option<Callback<u32>>` | `None` | Page change callback |
//! | `classes` | `Classes` | — | Additional CSS classes |

use yew::prelude::*;

use super::pagination_page::generate_pages;

/// Properties for the [`Pagination`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `current_page` | `u32` | `1` | Currently active page (1-indexed) |
/// | `total_pages` | `u32` | `10` | Total number of pages |
/// | `siblings` | `u32` | `1` | Pages shown on each side of current |
/// | `show_prev_next` | `bool` | `true` | Show prev/next (`‹`/`›`) buttons |
/// | `show_first_last` | `bool` | `false` | Show first/last (`«`/`»`) jump buttons |
/// | `on_page_change` | `Option<Callback<u32>>` | `None` | Page change callback |
/// | `classes` | `Classes` | — | Additional CSS classes |
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct PaginationProps {
    /// Additional CSS classes applied to the pagination container.
    #[prop_or_default]
    pub classes: Classes,

    /// The currently active page number (1-indexed).
    #[prop_or(1)]
    pub current_page: u32,

    /// Total number of pages available.
    #[prop_or(10)]
    pub total_pages: u32,

    /// Number of sibling pages to show on each side of the current page.
    #[prop_or(1)]
    pub siblings: u32,

    /// Whether to show first/last (`«`/`»`) jump buttons.
    ///
    /// Rendered as glyph buttons with `aria-label`s, since the numbered
    /// window from page generation already contains the first and last page
    /// numbers.
    #[prop_or(false)]
    pub show_first_last: bool,

    /// Whether to show previous and next navigation buttons.
    #[prop_or(true)]
    pub show_prev_next: bool,

    /// Callback invoked with the new page number when a page is selected.
    #[prop_or_default]
    pub on_page_change: Option<Callback<u32>>
}

impl Default for PaginationProps {
    fn default() -> Self {
        Self {
            classes:         Classes::default(),
            current_page:    1,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None
        }
    }
}

/// Pagination component for navigating between pages of content.
///
/// Renders a `<nav>` with page buttons and optional prev/next and
/// first/last navigation controls. The current page stays focusable and is
/// marked with `aria-current="page"`; ellipsis gaps render as
/// non-interactive text, not buttons.
///
/// # CSS Classes
///
/// - `pagination` - Container `<ul>` element
/// - `pagination-item` - Each `<li>` page button wrapper
/// - `pagination-ellipsis` - `<li>` holding a non-interactive `…` gap
/// - `active` - Applied to the current page item
#[function_component]
pub fn Pagination(props: &PaginationProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("pagination");

    let pages = generate_pages(props.current_page, props.total_pages, props.siblings);
    let on_page_change = props.on_page_change.clone();
    let current_page = props.current_page;
    let total_pages = props.total_pages;
    let show_prev_next = props.show_prev_next;
    let show_first_last = props.show_first_last;

    html! {
        <nav aria-label="pagination">
            <ul class={classes}>
                if show_prev_next {
                    <li class="pagination-item">
                        <button
                            type="button"
                            aria-label="Previous page"
                            disabled={current_page <= 1}
                            onclick={on_page_change.clone().map(move |cb| {
                                let cb = cb.clone();
                                move |_: MouseEvent| cb.emit(current_page.saturating_sub(1))
                            })}
                        >
                            {"‹"}
                        </button>
                    </li>
                }

                if show_first_last {
                    <li class="pagination-item">
                        <button
                            type="button"
                            aria-label="First page"
                            disabled={current_page == 1}
                            onclick={on_page_change.clone().map(move |cb| {
                                let cb = cb.clone();
                                move |_: MouseEvent| cb.emit(1)
                            })}
                        >
                            {"«"}
                        </button>
                    </li>
                }

                { for pages.iter().map(|page| {
                    if *page == 0 {
                        return html! {
                            <li class="pagination-item pagination-ellipsis">
                                <span>{"…"}</span>
                            </li>
                        };
                    }

                    let onclick = on_page_change.clone().map(move |cb| {
                        let cb = cb.clone();
                        let page_num = *page;
                        move |_: MouseEvent| cb.emit(page_num)
                    });

                    let is_active = *page == current_page;
                    let aria_current = is_active.then_some("page");

                    html! {
                        <li class={classes!("pagination-item", if is_active { "active" } else { "" })}>
                            <button
                                type="button"
                                aria-current={aria_current}
                                {onclick}
                            >
                                { page.to_string() }
                            </button>
                        </li>
                    }
                }) }

                if show_first_last {
                    <li class="pagination-item">
                        <button
                            type="button"
                            aria-label="Last page"
                            disabled={current_page == total_pages}
                            onclick={on_page_change.clone().map(move |cb| {
                                let cb = cb.clone();
                                move |_: MouseEvent| cb.emit(total_pages)
                            })}
                        >
                            {"»"}
                        </button>
                    </li>
                }

                if show_prev_next {
                    <li class="pagination-item">
                        <button
                            type="button"
                            aria-label="Next page"
                            disabled={current_page >= total_pages}
                            onclick={on_page_change.clone().map(move |cb| {
                                let cb = cb.clone();
                                move |_: MouseEvent| cb.emit(current_page + 1)
                            })}
                        >
                            {"›"}
                        </button>
                    </li>
                }
            </ul>
        </nav>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pagination_props_default() {
        let props = PaginationProps::default();
        assert_eq!(props.current_page, 1);
        assert_eq!(props.total_pages, 10);
        assert_eq!(props.siblings, 1);
        assert!(!props.show_first_last);
        assert!(props.show_prev_next);
        assert!(props.on_page_change.is_none());
    }

    #[test]
    fn pagination_props_custom() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     20,
            siblings:        2,
            show_first_last: true,
            show_prev_next:  false,
            on_page_change:  Some(Callback::from(|_: u32| {})),
            classes:         Classes::from("my-pagination")
        };
        assert_eq!(props.current_page, 5);
        assert_eq!(props.total_pages, 20);
        assert!(props.show_first_last);
        assert!(!props.show_prev_next);
    }

    #[test]
    fn pagination_props_clone() {
        let props = PaginationProps::default();
        let cloned = props.clone();
        assert_eq!(props, cloned);
    }

    #[test]
    fn pagination_props_neq() {
        let p1 = PaginationProps {
            current_page: 1,
            ..Default::default()
        };
        let p2 = PaginationProps {
            current_page: 2,
            ..Default::default()
        };
        assert_ne!(p1, p2);
    }

    #[test]
    fn pagination_props_callback_invoke() {
        use std::sync::{
            Arc,
            atomic::{AtomicU32, Ordering}
        };

        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();
        let cb = Callback::from(move |v: u32| {
            counter_clone.store(v, Ordering::SeqCst);
        });
        let props = PaginationProps {
            on_page_change: Some(cb),
            ..Default::default()
        };
        props.on_page_change.as_ref().unwrap().emit(5);
        assert_eq!(counter.load(Ordering::SeqCst), 5);
    }
}
