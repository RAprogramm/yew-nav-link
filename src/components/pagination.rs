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
//! | `active` | Applied to the current page button |
//!
//! # Props
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `current_page` | `u32` | `1` | Currently active page (1-indexed) |
//! | `total_pages` | `u32` | `10` | Total number of pages |
//! | `siblings` | `u32` | `1` | Pages shown on each side of current |
//! | `show_prev_next` | `bool` | `true` | Show prev/next buttons |
//! | `show_first_last` | `bool` | `false` | Show first/last page buttons |
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
/// | `show_prev_next` | `bool` | `true` | Show prev/next buttons |
/// | `show_first_last` | `bool` | `false` | Show first/last page buttons |
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

    /// Whether to show first and last page buttons.
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
/// first/last navigation controls.
///
/// # CSS Classes
///
/// - `pagination` - Container `<ul>` element
/// - `pagination-item` - Each `<li>` page button wrapper
/// - `active` - Applied to the current page button
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
            <ul {classes}>
                if show_prev_next {
                    <li class="pagination-item">
                        <button
                            type="button"
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
                            disabled={current_page == 1}
                            onclick={on_page_change.clone().map(move |cb| {
                                let cb = cb.clone();
                                move |_: MouseEvent| cb.emit(1)
                            })}
                        >
                            {"1"}
                        </button>
                    </li>
                }

                { for pages.iter().map(|page| {
                    let onclick = on_page_change.clone().map(move |cb| {
                        let cb = cb.clone();
                        let page_num = *page;
                        move |_: MouseEvent| cb.emit(page_num)
                    });

                    let is_active = *page == current_page;
                    let is_disabled = is_active || *page == 0;

                    html! {
                        <li class={classes!("pagination-item", if is_active { "active" } else { "" })}>
                            <button
                                type="button"
                                disabled={is_disabled}
                                aria-current={if is_active { "page" } else { "false" }}
                                {onclick}
                            >
                                { page_to_string(*page) }
                            </button>
                        </li>
                    }
                }) }

                if show_first_last {
                    <li class="pagination-item">
                        <button
                            type="button"
                            disabled={current_page == total_pages}
                            onclick={on_page_change.clone().map(move |cb| {
                                let cb = cb.clone();
                                move |_: MouseEvent| cb.emit(total_pages)
                            })}
                        >
                            { total_pages.to_string() }
                        </button>
                    </li>
                }

                if show_prev_next {
                    <li class="pagination-item">
                        <button
                            type="button"
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

fn page_to_string(page: u32) -> String {
    if page == 0 {
        "...".to_string()
    } else {
        page.to_string()
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
        assert!(props.show_prev_next);
    }

    #[test]
    fn page_to_string_normal() {
        assert_eq!(page_to_string(1), "1");
        assert_eq!(page_to_string(10), "10");
    }

    #[test]
    fn page_to_string_ellipsis() {
        assert_eq!(page_to_string(0), "...");
    }
}
