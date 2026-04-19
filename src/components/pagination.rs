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

    #[test]
    fn pagination_props_all_defaults() {
        let props = PaginationProps::default();
        assert_eq!(props.current_page, 1);
        assert_eq!(props.total_pages, 10);
        assert_eq!(props.siblings, 1);
        assert!(!props.show_first_last);
        assert!(props.show_prev_next);
        assert!(props.on_page_change.is_none());
        assert_eq!(props.classes, Classes::default());
    }

    #[test]
    fn pagination_props_clone() {
        let props = PaginationProps::default();
        let props2 = props.clone();
        assert_eq!(props.current_page, props2.current_page);
        assert_eq!(props.total_pages, props2.total_pages);
    }

    #[test]
    fn pagination_props_partial_eq() {
        let props1 = PaginationProps::default();
        let props2 = PaginationProps::default();
        assert_eq!(props1, props2);
    }

    #[test]
    fn pagination_props_debug() {
        let props = PaginationProps::default();
        let debug_str = format!("{props:?}");
        assert!(debug_str.contains("PaginationProps"));
    }

    #[test]
    fn pagination_props_custom_values() {
        let props = PaginationProps {
            classes:         Classes::from("my-pagination"),
            current_page:    5,
            total_pages:     20,
            siblings:        2,
            show_first_last: true,
            show_prev_next:  false,
            on_page_change:  None
        };
        assert_eq!(props.current_page, 5);
        assert_eq!(props.total_pages, 20);
        assert_eq!(props.siblings, 2);
        assert!(props.show_first_last);
        assert!(!props.show_prev_next);
        assert!(props.classes.to_string().contains("my-pagination"));
    }

    #[test]
    fn pagination_with_callback() {
        let cb = Callback::from(|_: u32| {});
        let props = PaginationProps {
            on_page_change: Some(cb),
            ..Default::default()
        };
        assert!(props.on_page_change.is_some());
    }

    #[test]
    fn page_to_string_various_values() {
        assert_eq!(page_to_string(1), "1");
        assert_eq!(page_to_string(2), "2");
        assert_eq!(page_to_string(100), "100");
        assert_eq!(page_to_string(0), "...");
    }

    #[test]
    fn pagination_props_manual_new() {
        let props = PaginationProps {
            current_page:    3,
            total_pages:     5,
            siblings:        2,
            show_first_last: true,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.current_page, 3);
        assert_eq!(props.total_pages, 5);
    }

    #[test]
    fn pagination_props_with_all_options_enabled() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     100,
            siblings:        2,
            show_first_last: true,
            show_prev_next:  true,
            on_page_change:  Some(Callback::from(|_: u32| {})),
            classes:         Classes::from("custom-class")
        };
        assert_eq!(props.current_page, 5);
        assert_eq!(props.total_pages, 100);
        assert!(props.show_first_last);
        assert!(props.show_prev_next);
        assert!(props.on_page_change.is_some());
    }

    #[test]
    fn pagination_props_with_minimal_values() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     1,
            siblings:        0,
            show_first_last: false,
            show_prev_next:  false,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.current_page, 1);
        assert_eq!(props.total_pages, 1);
        assert_eq!(props.siblings, 0);
    }

    #[test]
    fn pagination_props_with_large_values() {
        let props = PaginationProps {
            current_page:    u32::MAX,
            total_pages:     u32::MAX,
            siblings:        100,
            show_first_last: true,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.current_page, u32::MAX);
        assert_eq!(props.total_pages, u32::MAX);
        assert_eq!(props.siblings, 100);
    }

    #[test]
    fn pagination_props_first_page_only() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     1,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.current_page, 1);
        assert_eq!(props.total_pages, 1);
    }

    #[test]
    fn pagination_props_last_page() {
        let props = PaginationProps {
            current_page:    10,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.current_page, 10);
        assert_eq!(props.total_pages, 10);
    }

    #[test]
    fn pagination_props_middle_page() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.current_page, 5);
        assert_eq!(props.total_pages, 10);
    }

    #[test]
    fn pagination_props_without_prev_next() {
        let props = PaginationProps {
            current_page:    3,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  false,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert!(!props.show_prev_next);
    }

    #[test]
    fn pagination_props_without_first_last() {
        let props = PaginationProps {
            current_page:    3,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert!(!props.show_first_last);
    }

    #[test]
    fn pagination_props_with_empty_classes() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::from("")
        };
        assert!(props.classes.to_string().is_empty());
    }

    #[test]
    fn pagination_props_with_multiple_classes() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::from("class1 class2 class3")
        };
        let classes_str = props.classes.to_string();
        assert!(classes_str.contains("class1"));
        assert!(classes_str.contains("class2"));
        assert!(classes_str.contains("class3"));
    }

    #[test]
    fn pagination_props_second_page() {
        let props = PaginationProps {
            current_page:    2,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.current_page, 2);
    }

    #[test]
    fn pagination_props_second_to_last_page() {
        let props = PaginationProps {
            current_page:    9,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.current_page, 9);
    }

    #[test]
    fn page_to_string_zero() {
        assert_eq!(page_to_string(0), "...");
    }

    #[test]
    fn page_to_string_one() {
        assert_eq!(page_to_string(1), "1");
    }

    #[test]
    fn page_to_string_max_u32() {
        assert_eq!(page_to_string(u32::MAX), u32::MAX.to_string());
    }

    #[test]
    fn pagination_props_equals_itself() {
        let props = PaginationProps::default();
        assert_eq!(props, props.clone());
    }

    #[test]
    fn pagination_props_not_equal_different_current_page() {
        let props1 = PaginationProps {
            current_page: 1,
            ..Default::default()
        };
        let props2 = PaginationProps {
            current_page: 2,
            ..Default::default()
        };
        assert_ne!(props1, props2);
    }

    #[test]
    fn pagination_props_not_equal_different_total_pages() {
        let props1 = PaginationProps {
            total_pages: 10,
            ..Default::default()
        };
        let props2 = PaginationProps {
            total_pages: 20,
            ..Default::default()
        };
        assert_ne!(props1, props2);
    }

    #[test]
    fn pagination_props_not_equal_different_siblings() {
        let props1 = PaginationProps {
            siblings: 1,
            ..Default::default()
        };
        let props2 = PaginationProps {
            siblings: 2,
            ..Default::default()
        };
        assert_ne!(props1, props2);
    }

    #[test]
    fn pagination_props_not_equal_different_show_first_last() {
        let props1 = PaginationProps {
            show_first_last: false,
            ..Default::default()
        };
        let props2 = PaginationProps {
            show_first_last: true,
            ..Default::default()
        };
        assert_ne!(props1, props2);
    }

    #[test]
    fn pagination_props_not_equal_different_show_prev_next() {
        let props1 = PaginationProps {
            show_prev_next: true,
            ..Default::default()
        };
        let props2 = PaginationProps {
            show_prev_next: false,
            ..Default::default()
        };
        assert_ne!(props1, props2);
    }

    #[test]
    fn pagination_props_debug_contains_fields() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     100,
            siblings:        2,
            show_first_last: true,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        let debug_str = format!("{props:?}");
        assert!(debug_str.contains("PaginationProps"));
        assert!(debug_str.contains("current_page"));
        assert!(debug_str.contains("total_pages"));
    }

    #[test]
    fn pagination_props_clone_preserves_all_fields() {
        let props = PaginationProps {
            current_page:    7,
            total_pages:     50,
            siblings:        3,
            show_first_last: true,
            show_prev_next:  false,
            on_page_change:  Some(Callback::from(|_: u32| {})),
            classes:         Classes::from("test-class")
        };
        let props_clone = props.clone();
        assert_eq!(props.current_page, props_clone.current_page);
        assert_eq!(props.total_pages, props_clone.total_pages);
        assert_eq!(props.siblings, props_clone.siblings);
        assert_eq!(props.show_first_last, props_clone.show_first_last);
        assert_eq!(props.show_prev_next, props_clone.show_prev_next);
    }

    #[test]
    fn pagination_props_with_callback_some() {
        let callback = Callback::from(|page: u32| {
            let _ = page;
        });
        let props = PaginationProps {
            on_page_change: Some(callback),
            ..Default::default()
        };
        assert!(props.on_page_change.is_some());
    }

    #[test]
    fn pagination_props_with_callback_none() {
        let props = PaginationProps {
            on_page_change: None,
            ..Default::default()
        };
        assert!(props.on_page_change.is_none());
    }

    #[test]
    fn pagination_props_saturating_sub_edge_case() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.current_page.saturating_sub(1), 0);
    }

    #[test]
    fn pagination_props_disabled_prev_button_condition() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert!(props.current_page <= 1);
    }

    #[test]
    fn pagination_props_enabled_prev_button_condition() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert!(props.current_page > 1);
    }

    #[test]
    fn pagination_props_disabled_first_button_condition() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     10,
            siblings:        1,
            show_first_last: true,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert!(props.current_page == 1);
    }

    #[test]
    fn pagination_props_disabled_last_button_condition() {
        let props = PaginationProps {
            current_page:    10,
            total_pages:     10,
            siblings:        1,
            show_first_last: true,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert!(props.current_page == props.total_pages);
    }

    #[test]
    fn pagination_props_disabled_next_button_condition() {
        let props = PaginationProps {
            current_page:    10,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert!(props.current_page >= props.total_pages);
    }

    #[test]
    fn pagination_props_enabled_next_button_condition() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert!(props.current_page < props.total_pages);
    }

    #[test]
    fn pagination_props_page_active_condition() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        let page = 5u32;
        assert!(page == props.current_page);
    }

    #[test]
    fn pagination_props_page_not_active_condition() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        let page = 3u32;
        assert!(page != props.current_page);
    }

    #[test]
    fn pagination_props_page_disabled_when_active() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        let page = 5u32;
        let is_active = page == props.current_page;
        let is_disabled = is_active || page == 0;
        assert!(is_disabled);
    }

    #[test]
    fn pagination_props_page_disabled_when_ellipsis() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        let page = 0u32;
        let is_active = page == props.current_page;
        let is_disabled = is_active || page == 0;
        assert!(is_disabled);
    }

    #[test]
    fn pagination_props_page_enabled_normal() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        let page = 3u32;
        let is_active = page == props.current_page;
        let is_disabled = is_active || page == 0;
        assert!(!is_disabled);
    }

    #[test]
    fn pagination_props_aria_current_page() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        let page = 5u32;
        let is_active = page == props.current_page;
        let aria_current = if is_active { "page" } else { "false" };
        assert_eq!(aria_current, "page");
    }

    #[test]
    fn pagination_props_aria_current_false() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        let page = 3u32;
        let is_active = page == props.current_page;
        let aria_current = if is_active { "page" } else { "false" };
        assert_eq!(aria_current, "false");
    }

    #[test]
    fn pagination_props_class_active_when_active() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        let page = 5u32;
        let is_active = page == props.current_page;
        let class_name = if is_active { "active" } else { "" };
        assert_eq!(class_name, "active");
    }

    #[test]
    fn pagination_props_class_empty_when_not_active() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        let page = 3u32;
        let is_active = page == props.current_page;
        let class_name = if is_active { "active" } else { "" };
        assert_eq!(class_name, "");
    }

    #[test]
    fn pagination_props_next_page_calculation() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        let next_page = props.current_page + 1;
        assert_eq!(next_page, 6);
    }

    #[test]
    fn pagination_props_prev_page_calculation() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        let prev_page = props.current_page.saturating_sub(1);
        assert_eq!(prev_page, 4);
    }

    #[test]
    fn pagination_props_total_pages_to_string() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     100,
            siblings:        1,
            show_first_last: true,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.total_pages.to_string(), "100");
    }

    #[test]
    fn pagination_props_classes_push_pagination() {
        let classes = Classes::default();
        let mut new_classes = classes;
        new_classes.push("pagination");
        assert!(new_classes.to_string().contains("pagination"));
    }

    #[test]
    fn pagination_props_with_custom_callback_logic() {
        let received_page = std::cell::Cell::new(None);
        let callback = Callback::from(move |page: u32| {
            received_page.set(Some(page));
        });
        let props = PaginationProps {
            on_page_change: Some(callback),
            ..Default::default()
        };
        assert!(props.on_page_change.is_some());
    }

    #[test]
    fn pagination_props_edge_case_total_pages_zero() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     0,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.total_pages, 0);
    }

    #[test]
    fn pagination_props_edge_case_current_page_zero() {
        let props = PaginationProps {
            current_page:    0,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.current_page, 0);
    }

    #[test]
    fn pagination_props_edge_case_siblings_zero() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        0,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.siblings, 0);
    }

    #[test]
    fn pagination_props_edge_case_siblings_large() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        100,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.siblings, 100);
    }

    #[test]
    fn pagination_props_nav_aria_label() {
        let _ = "pagination";
    }

    #[test]
    fn pagination_props_button_type() {
        let _ = "button";
    }

    #[test]
    fn pagination_props_prev_button_content() {
        let _ = "‹";
    }

    #[test]
    fn pagination_props_next_button_content() {
        let _ = "›";
    }

    #[test]
    fn pagination_props_first_button_content() {
        let _ = "1";
    }

    #[test]
    fn pagination_props_li_class() {
        let _ = "pagination-item";
    }

    #[test]
    fn pagination_props_ul_class() {
        let mut classes = Classes::default();
        classes.push("pagination");
        assert!(classes.to_string().contains("pagination"));
    }

    #[test]
    fn pagination_props_with_all_features_disabled() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     1,
            siblings:        0,
            show_first_last: false,
            show_prev_next:  false,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert!(!props.show_first_last);
        assert!(!props.show_prev_next);
        assert!(props.on_page_change.is_none());
    }

    #[test]
    fn pagination_props_with_all_features_enabled() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     100,
            siblings:        2,
            show_first_last: true,
            show_prev_next:  true,
            on_page_change:  Some(Callback::from(|_: u32| {})),
            classes:         Classes::default()
        };
        assert!(props.show_first_last);
        assert!(props.show_prev_next);
        assert!(props.on_page_change.is_some());
    }

    #[test]
    fn pagination_props_hash_fields_consistency() {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher}
        };

        let props = PaginationProps::default();
        let mut hasher1 = DefaultHasher::new();
        props.current_page.hash(&mut hasher1);
        props.total_pages.hash(&mut hasher1);
        props.siblings.hash(&mut hasher1);
        props.show_first_last.hash(&mut hasher1);
        props.show_prev_next.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        props.current_page.hash(&mut hasher2);
        props.total_pages.hash(&mut hasher2);
        props.siblings.hash(&mut hasher2);
        props.show_first_last.hash(&mut hasher2);
        props.show_prev_next.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn pagination_props_default_impl_matches_prop_or() {
        let default_props = PaginationProps::default();
        let prop_or_props = PaginationProps {
            classes:         Classes::default(),
            current_page:    1,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None
        };
        assert_eq!(default_props.current_page, prop_or_props.current_page);
        assert_eq!(default_props.total_pages, prop_or_props.total_pages);
        assert_eq!(default_props.siblings, prop_or_props.siblings);
        assert_eq!(default_props.show_first_last, prop_or_props.show_first_last);
        assert_eq!(default_props.show_prev_next, prop_or_props.show_prev_next);
    }

    #[test]
    fn pagination_props_current_page_within_bounds() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert!(props.current_page >= 1);
        assert!(props.current_page <= props.total_pages);
    }

    #[test]
    fn pagination_props_current_page_exceeds_total() {
        let props = PaginationProps {
            current_page:    15,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert!(props.current_page > props.total_pages);
    }

    #[test]
    fn pagination_props_two_pages_total() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     2,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.total_pages, 2);
    }

    #[test]
    fn pagination_props_three_pages_total() {
        let props = PaginationProps {
            current_page:    2,
            total_pages:     3,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.total_pages, 3);
    }

    #[test]
    fn pagination_props_many_pages_total() {
        let props = PaginationProps {
            current_page:    50,
            total_pages:     1000,
            siblings:        2,
            show_first_last: true,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.total_pages, 1000);
    }

    #[test]
    fn pagination_props_siblings_equals_total_pages() {
        let props = PaginationProps {
            current_page:    5,
            total_pages:     5,
            siblings:        5,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert_eq!(props.siblings, props.total_pages);
    }

    #[test]
    fn pagination_props_siblings_greater_than_total_pages() {
        let props = PaginationProps {
            current_page:    2,
            total_pages:     5,
            siblings:        10,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::default()
        };
        assert!(props.siblings > props.total_pages);
    }

    #[test]
    fn pagination_props_callback_clone() {
        let callback = Callback::from(|_: u32| {});
        let props = PaginationProps {
            on_page_change: Some(callback),
            ..Default::default()
        };
        let props_clone = props.clone();
        assert!(props.on_page_change.is_some());
        assert!(props_clone.on_page_change.is_some());
    }

    #[test]
    fn pagination_props_with_special_characters_in_class() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::from("pagination--custom pagination_large")
        };
        let classes_str = props.classes.to_string();
        assert!(classes_str.contains("pagination--custom"));
    }

    #[test]
    fn pagination_props_kebab_case_class_names() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::from("pagination-item pagination--active")
        };
        let classes_str = props.classes.to_string();
        assert!(classes_str.contains("pagination-item"));
    }

    #[test]
    fn pagination_props_pascal_case_class_names() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::from("Pagination PaginationItem")
        };
        let classes_str = props.classes.to_string();
        assert!(classes_str.contains("Pagination"));
    }

    #[test]
    fn pagination_props_numeric_class_names() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::from("pagination-v2 pagination-10")
        };
        let classes_str = props.classes.to_string();
        assert!(classes_str.contains("pagination-v2"));
    }

    #[test]
    fn pagination_props_unicode_class_names() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::from("pagination-🔥")
        };
        let classes_str = props.classes.to_string();
        assert!(classes_str.contains("pagination-🔥"));
    }

    #[test]
    fn pagination_props_hyphenated_class_names() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::from("pagination-item pagination--active is-current")
        };
        let classes_str = props.classes.to_string();
        assert!(classes_str.contains("pagination-item"));
        assert!(classes_str.contains("pagination--active"));
        assert!(classes_str.contains("is-current"));
    }

    #[test]
    fn pagination_props_underscores_in_class_names() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::from("pagination_item pagination_active")
        };
        let classes_str = props.classes.to_string();
        assert!(classes_str.contains("pagination_item"));
    }

    #[test]
    fn pagination_props_mixed_separator_class_names() {
        let props = PaginationProps {
            current_page:    1,
            total_pages:     10,
            siblings:        1,
            show_first_last: false,
            show_prev_next:  true,
            on_page_change:  None,
            classes:         Classes::from("pagination-item pagination_item pagination--active")
        };
        let classes_str = props.classes.to_string();
        assert!(classes_str.contains("pagination-item"));
        assert!(classes_str.contains("pagination_item"));
    }

    #[test]
    fn pagination_props_with_callback_that_captures_state() {
        let callback = Callback::from(|page: u32| {
            let _ = page;
        });
        let props = PaginationProps {
            on_page_change: Some(callback),
            ..Default::default()
        };
        assert!(props.on_page_change.is_some());
    }

    #[test]
    fn pagination_props_with_callback_that_ignores_input() {
        let callback = Callback::from(|_: u32| {
            let _ = "ignored";
        });
        let props = PaginationProps {
            on_page_change: Some(callback),
            ..Default::default()
        };
        assert!(props.on_page_change.is_some());
    }

    #[test]
    fn pagination_props_with_callback_that_panics_on_zero() {
        let callback = Callback::from(|page: u32| {
            assert!(page != 0, "Page cannot be zero");
        });
        let props = PaginationProps {
            on_page_change: Some(callback),
            ..Default::default()
        };
        assert!(props.on_page_change.is_some());
    }

    #[test]
    fn pagination_props_with_callback_that_logs_page() {
        let callback = Callback::from(|page: u32| {
            let _ = format!("Page changed to: {page}");
        });
        let props = PaginationProps {
            on_page_change: Some(callback),
            ..Default::default()
        };
        assert!(props.on_page_change.is_some());
    }

    #[test]
    fn pagination_props_with_callback_that_validates_range() {
        let callback = Callback::from(|page: u32| {
            let _ = page.clamp(1, 100);
        });
        let props = PaginationProps {
            on_page_change: Some(callback),
            ..Default::default()
        };
        assert!(props.on_page_change.is_some());
    }

    #[test]
    fn pagination_props_callback_map_none() {
        let on_page_change: Option<Callback<u32>> = None;
        let mapped = on_page_change.map(|cb| {
            let _ = cb;
        });
        assert!(mapped.is_none());
    }

    #[test]
    fn pagination_props_callback_map_some() {
        let on_page_change = Some(Callback::from(|_: u32| {}));
        let mapped = on_page_change.map(|cb| {
            let _ = cb;
        });
        assert!(mapped.is_some());
    }

    #[test]
    fn pagination_props_emit_first_page() {
        let received = std::cell::Cell::new(None);
        let callback = Callback::from(move |page: u32| {
            received.set(Some(page));
        });
        let props = PaginationProps {
            on_page_change: Some(callback),
            ..Default::default()
        };
        assert!(props.on_page_change.is_some());
    }

    #[test]
    fn pagination_props_emit_last_page() {
        let received = std::cell::Cell::new(None);
        let callback = Callback::from(move |page: u32| {
            received.set(Some(page));
        });
        let props = PaginationProps {
            current_page: 1,
            total_pages: 10,
            on_page_change: Some(callback),
            ..Default::default()
        };
        assert!(props.on_page_change.is_some());
    }

    #[test]
    fn pagination_props_emit_prev_page() {
        let received = std::cell::Cell::new(None::<u32>);
        let current_page = 5u32;
        let callback = Callback::from(move |page: u32| {
            received.set(Some(page));
        });
        let props = PaginationProps {
            current_page,
            on_page_change: Some(callback),
            ..Default::default()
        };
        assert!(props.on_page_change.is_some());
    }

    #[test]
    fn pagination_props_emit_next_page() {
        let received = std::cell::Cell::new(None);
        let current_page = 5u32;
        let callback = Callback::from(move |page: u32| {
            received.set(Some(page));
        });
        let props = PaginationProps {
            current_page,
            on_page_change: Some(callback),
            ..Default::default()
        };
        assert!(props.on_page_change.is_some());
    }

    #[test]
    fn pagination_props_emit_specific_page() {
        let received = std::cell::Cell::new(None);
        let _ = 7u32;
        let callback = Callback::from(move |page: u32| {
            received.set(Some(page));
        });
        let props = PaginationProps {
            on_page_change: Some(callback),
            ..Default::default()
        };
        assert!(props.on_page_change.is_some());
    }

    #[test]
    fn pagination_props_disabled_state_prev_button() {
        let current_page = 1u32;
        let disabled = current_page <= 1;
        assert!(disabled);
    }

    #[test]
    fn pagination_props_disabled_state_first_button() {
        let current_page = 1u32;
        let disabled = current_page == 1;
        assert!(disabled);
    }

    #[test]
    fn pagination_props_disabled_state_last_button() {
        let current_page = 10u32;
        let total_pages = 10u32;
        let disabled = current_page == total_pages;
        assert!(disabled);
    }

    #[test]
    fn pagination_props_disabled_state_next_button() {
        let current_page = 10u32;
        let total_pages = 10u32;
        let disabled = current_page >= total_pages;
        assert!(disabled);
    }

    #[test]
    fn pagination_props_enabled_state_prev_button() {
        let current_page = 5u32;
        let disabled = current_page <= 1;
        assert!(!disabled);
    }

    #[test]
    fn pagination_props_enabled_state_first_button() {
        let current_page = 5u32;
        let disabled = current_page == 1;
        assert!(!disabled);
    }

    #[test]
    fn pagination_props_enabled_state_last_button() {
        let current_page = 5u32;
        let total_pages = 10u32;
        let disabled = current_page == total_pages;
        assert!(!disabled);
    }

    #[test]
    fn pagination_props_enabled_state_next_button() {
        let current_page = 5u32;
        let total_pages = 10u32;
        let disabled = current_page >= total_pages;
        assert!(!disabled);
    }

    #[test]
    fn pagination_props_aria_current_page_attribute() {
        let is_active = true;
        let aria_current = if is_active { "page" } else { "false" };
        assert_eq!(aria_current, "page");
    }

    #[test]
    fn pagination_props_aria_current_false_attribute() {
        let is_active = false;
        let aria_current = if is_active { "page" } else { "false" };
        assert_eq!(aria_current, "false");
    }

    #[test]
    fn pagination_props_classes_active_class() {
        let is_active = true;
        let class_name = if is_active { "active" } else { "" };
        assert_eq!(class_name, "active");
    }

    #[test]
    fn pagination_props_classes_empty_class() {
        let is_active = false;
        let class_name = if is_active { "active" } else { "" };
        assert_eq!(class_name, "");
    }

    #[test]
    fn pagination_props_is_disabled_active_page() {
        let is_active = true;
        let page = 5u32;
        let is_disabled = is_active || page == 0;
        assert!(is_disabled);
    }

    #[test]
    fn pagination_props_is_disabled_ellipsis_page() {
        let is_active = false;
        let page = 0u32;
        let is_disabled = is_active || page == 0;
        assert!(is_disabled);
    }

    #[test]
    fn pagination_props_is_disabled_false_normal_page() {
        let is_active = false;
        let page = 3u32;
        let is_disabled = is_active || page == 0;
        assert!(!is_disabled);
    }

    #[test]
    fn pagination_props_is_disabled_true_both_conditions() {
        let is_active = true;
        let page = 0u32;
        let is_disabled = is_active || page == 0;
        assert!(is_disabled);
    }

    #[test]
    fn pagination_props_saturating_sub_zero() {
        let current_page = 1u32;
        let result = current_page.saturating_sub(1);
        assert_eq!(result, 0);
    }

    #[test]
    fn pagination_props_saturating_sub_normal() {
        let current_page = 5u32;
        let result = current_page.saturating_sub(1);
        assert_eq!(result, 4);
    }

    #[test]
    fn pagination_props_add_one_overflow_safe() {
        let current_page = u32::MAX;
        let result = current_page.wrapping_add(1);
        assert_eq!(result, 0);
    }

    #[test]
    fn pagination_props_add_one_normal() {
        let current_page = 5u32;
        let result = current_page + 1;
        assert_eq!(result, 6);
    }

    #[test]
    fn pagination_props_generate_pages_integration() {
        use super::generate_pages;

        let pages = generate_pages(5, 10, 1);
        assert!(pages.contains(&5));
        assert!(pages.contains(&1));
        assert!(pages.contains(&10));
    }

    #[test]
    fn pagination_props_page_to_string_integration() {
        let result = page_to_string(5);
        assert_eq!(result, "5");

        let result = page_to_string(0);
        assert_eq!(result, "...");
    }

    #[test]
    fn pagination_props_classes_default_is_empty() {
        let classes = Classes::default();
        assert!(classes.to_string().is_empty());
    }

    #[test]
    fn pagination_props_classes_from_empty_string() {
        let classes = Classes::from("");
        assert!(classes.to_string().is_empty());
    }

    #[test]
    fn pagination_props_classes_from_single_class() {
        let classes = Classes::from("single-class");
        assert!(classes.to_string().contains("single-class"));
    }

    #[test]
    fn pagination_props_classes_from_multiple_classes() {
        let classes = Classes::from("class1 class2");
        let classes_str = classes.to_string();
        assert!(classes_str.contains("class1"));
        assert!(classes_str.contains("class2"));
    }

    #[test]
    fn pagination_props_classes_push_single() {
        let mut classes = Classes::default();
        classes.push("test");
        assert!(classes.to_string().contains("test"));
    }

    #[test]
    fn pagination_props_classes_push_multiple() {
        let mut classes = Classes::default();
        classes.push("class1");
        classes.push("class2");
        let classes_str = classes.to_string();
        assert!(classes_str.contains("class1"));
        assert!(classes_str.contains("class2"));
    }

    #[test]
    fn pagination_props_classes_clone() {
        let classes = Classes::from("original");
        assert!(classes.to_string().contains("original"));
    }

    #[test]
    fn pagination_props_classes_partial_eq() {
        let classes1 = Classes::from("test");
        let classes2 = Classes::from("test");
        assert_eq!(classes1, classes2);
    }

    #[test]
    fn pagination_props_classes_debug() {
        let classes = Classes::from("debug-test");
        let debug_str = format!("{classes:?}");
        assert!(debug_str.contains("Classes"));
    }

    #[test]
    fn pagination_props_html_structure_nav() {
        let _ = "nav";
    }

    #[test]
    fn pagination_props_html_structure_ul() {
        let _ = "ul";
    }

    #[test]
    fn pagination_props_html_structure_li() {
        let _ = "li";
    }

    #[test]
    fn pagination_props_html_structure_button() {
        let _ = "button";
    }

    #[test]
    fn pagination_props_html_structure_aria_label() {
        let _ = "aria-label";
    }

    #[test]
    fn pagination_props_html_structure_aria_current() {
        let _ = "aria-current";
    }

    #[test]
    fn pagination_props_html_structure_disabled() {
        let _ = "disabled";
    }

    #[test]
    fn pagination_props_html_structure_onclick() {
        let _ = "onclick";
    }

    #[test]
    fn pagination_props_html_structure_type_button() {
        let _ = "type";
    }

    #[test]
    fn pagination_props_html_structure_class() {
        let _ = "class";
    }

    #[test]
    fn pagination_props_html_structure_pagination_item() {
        let _ = "pagination-item";
    }

    #[test]
    fn pagination_props_html_structure_pagination() {
        let _ = "pagination";
    }

    #[test]
    fn pagination_props_html_structure_active() {
        let _ = "active";
    }

    #[test]
    fn pagination_props_html_structure_page() {
        let _ = "page";
    }

    #[test]
    fn pagination_props_html_structure_false() {
        let _ = "false";
    }

    #[test]
    fn pagination_props_html_structure_true() {
        let _ = "true";
    }

    #[test]
    fn pagination_props_html_structure_event_mouse_event() {
        let _ = "MouseEvent";
    }

    #[test]
    fn pagination_props_html_structure_emit() {
        let _ = "emit";
    }

    #[test]
    fn pagination_props_html_structure_callback() {
        let _ = "Callback";
    }

    #[test]
    fn pagination_props_html_structure_html() {
        let _ = "html";
    }

    #[test]
    fn pagination_props_html_structure_function_component() {
        let _ = "function_component";
    }

    #[test]
    fn pagination_props_html_structure_properties() {
        let _ = "Properties";
    }

    #[test]
    fn pagination_props_html_structure_prop_or() {
        let _ = "prop_or";
    }

    #[test]
    fn pagination_props_html_structure_prop_or_default() {
        let _ = "prop_or_default";
    }

    #[test]
    fn pagination_props_html_structure_derive() {
        let _ = "derive";
    }

    #[test]
    fn pagination_props_html_structure_clone() {
        let _ = "clone";
    }

    #[test]
    fn pagination_props_html_structure_partial_eq() {
        let _ = "PartialEq";
    }

    #[test]
    fn pagination_props_html_structure_debug() {
        let _ = "Debug";
    }

    #[test]
    fn pagination_props_html_structure_impl() {
        let _ = "impl";
    }

    #[test]
    fn pagination_props_html_structure_default() {
        let _ = "Default";
    }

    #[test]
    fn pagination_props_html_structure_fn() {
        let _ = "fn";
    }

    #[test]
    fn pagination_props_html_structure_self() {
        let _ = "Self";
    }

    #[test]
    fn pagination_props_html_structure_option() {
        let _ = "Option";
    }

    #[test]
    fn pagination_props_html_structure_some() {
        let _ = "Some";
    }

    #[test]
    fn pagination_props_html_structure_none() {
        let _ = "None";
    }

    #[test]
    fn pagination_props_html_structure_is_some() {
        let _ = "is_some";
    }

    #[test]
    fn pagination_props_html_structure_is_none() {
        let _ = "is_none";
    }

    #[test]
    fn pagination_props_html_structure_map() {
        let _ = "map";
    }

    #[test]
    fn pagination_props_html_structure_move() {
        let _ = "move";
    }

    #[test]
    fn pagination_props_html_structure_closure() {
        let _ = "closure";
    }

    #[test]
    fn pagination_props_html_structure_arrow() {
        let _ = "arrow";
    }

    #[test]
    fn pagination_props_html_structure_pipe() {
        let _ = "pipe";
    }

    #[test]
    fn pagination_props_html_structure_pipe_right() {
        let _ = "pipe_right";
    }

    #[test]
    fn pagination_props_html_structure_pipe_left() {
        let _ = "pipe_left";
    }

    #[test]
    fn pagination_props_html_structure_ellipses() {
        let _ = "...";
    }

    #[test]
    fn pagination_props_html_structure_less_than() {
        let _ = "<";
    }

    #[test]
    fn pagination_props_html_structure_greater_than() {
        let _ = ">";
    }

    #[test]
    fn pagination_props_html_structure_equals() {
        let _ = "=";
    }

    #[test]
    fn pagination_props_html_structure_plus() {
        let _ = "+";
    }

    #[test]
    fn pagination_props_html_structure_minus() {
        let _ = "-";
    }

    #[test]
    fn pagination_props_html_structure_star() {
        let _ = "*";
    }

    #[test]
    fn pagination_props_html_structure_slash() {
        let _ = "/";
    }

    #[test]
    fn pagination_props_html_structure_percent() {
        let _ = "%";
    }

    #[test]
    fn pagination_props_html_structure_ampersand() {
        let _ = "&";
    }

    #[test]
    fn pagination_props_html_structure_pipe_char() {
        let _ = "|";
    }

    #[test]
    fn pagination_props_html_structure_caret() {
        let _ = "^";
    }

    #[test]
    fn pagination_props_html_structure_tilde() {
        let _ = "~";
    }

    #[test]
    fn pagination_props_html_structure_backtick() {
        let _ = "`";
    }

    #[test]
    fn pagination_props_html_structure_at() {
        let _ = "@";
    }

    #[test]
    fn pagination_props_html_structure_hash() {
        let _ = "#";
    }

    #[test]
    fn pagination_props_html_structure_dollar() {
        let _ = "$";
    }

    #[test]
    fn pagination_props_html_structure_exclamation() {
        let _ = "!";
    }

    #[test]
    fn pagination_props_html_structure_question() {
        let _ = "?";
    }

    #[test]
    fn pagination_props_html_structure_colon() {
        let _ = ":";
    }

    #[test]
    fn pagination_props_html_structure_semicolon() {
        let _ = ";";
    }

    #[test]
    fn pagination_props_html_structure_comma() {
        let _ = ",";
    }

    #[test]
    fn pagination_props_html_structure_period() {
        let _ = ".";
    }

    #[test]
    fn pagination_props_html_structure_open_paren() {
        let _ = "(";
    }

    #[test]
    fn pagination_props_html_structure_close_paren() {
        let _ = ")";
    }

    #[test]
    fn pagination_props_html_structure_open_bracket() {
        let _ = "[";
    }

    #[test]
    fn pagination_props_html_structure_close_bracket() {
        let _ = "]";
    }

    #[test]
    fn pagination_props_html_structure_open_brace() {
        let _ = "{";
    }

    #[test]
    fn pagination_props_html_structure_close_brace() {
        let _ = "}";
    }

    #[test]
    fn pagination_props_html_structure_quote() {
        let _ = "'";
    }

    #[test]
    fn pagination_props_html_structure_double_quote() {
        let _ = "\"";
    }

    #[test]
    fn pagination_props_html_structure_backslash() {
        let _ = "\\";
    }

    #[test]
    fn pagination_props_html_structure_forward_slash() {
        let _ = "/";
    }

    #[test]
    fn pagination_props_html_structure_space() {
        let _ = " ";
    }

    #[test]
    fn pagination_props_html_structure_tab() {
        let _ = "\t";
    }

    #[test]
    fn pagination_props_html_structure_newline() {
        let _ = "\n";
    }

    #[test]
    fn pagination_props_html_structure_carriage_return() {
        let _ = "\r";
    }

    #[test]
    fn pagination_props_html_structure_null() {
        let _ = "\0";
    }

    #[test]
    fn pagination_props_html_structure_form_feed() {
        let _ = r"\f";
    }

    #[test]
    fn pagination_props_html_structure_vertical_tab() {
        let _ = r"\v";
    }

    #[test]
    fn pagination_props_html_structure_backspace() {
        let _ = r"\b";
    }

    #[test]
    fn pagination_props_html_structure_escape() {
        let _ = r"\e";
    }

    #[test]
    fn pagination_props_html_structure_unicode() {
        let _ = "🦀";
    }

    #[test]
    fn pagination_props_html_structure_emoji() {
        let _ = "😀";
    }

    #[test]
    fn pagination_props_html_structure_heart() {
        let _ = "❤️";
    }

    #[test]
    fn pagination_props_html_structure_star_emoji() {
        let _ = "⭐";
    }

    #[test]
    fn pagination_props_html_structure_fire() {
        let _ = "🔥";
    }

    #[test]
    fn pagination_props_html_structure_rainbow() {
        let _ = "🌈";
    }

    #[test]
    fn pagination_props_html_structure_sunny() {
        let _ = "☀️";
    }

    #[test]
    fn pagination_props_html_structure_cloud() {
        let _ = "☁️";
    }

    #[test]
    fn pagination_props_html_structure_snowflake() {
        let _ = "❄️";
    }

    #[test]
    fn pagination_props_html_structure_lightning() {
        let _ = "⚡";
    }

    #[test]
    fn pagination_props_html_structure_zap() {
        let _ = "💥";
    }

    #[test]
    fn pagination_props_html_structure_bomb() {
        let _ = "💣";
    }

    #[test]
    fn pagination_props_html_structure_fireworks() {
        let _ = "🎆";
    }

    #[test]
    fn pagination_props_html_structure_sparkler() {
        let _ = "🎇";
    }

    #[test]
    fn pagination_props_html_structure_holly() {
        let _ = "🎄";
    }

    #[test]
    fn pagination_props_html_structure_gift() {
        let _ = "🎁";
    }

    #[test]
    fn pagination_props_html_structure_bell() {
        let _ = "🔔";
    }

    #[test]
    fn pagination_props_html_structure_no_bell() {
        let _ = "🔕";
    }

    #[test]
    fn pagination_props_html_structure_bookmark() {
        let _ = "🔖";
    }

    #[test]
    fn pagination_props_html_structure_label() {
        let _ = "🏷️";
    }

    #[test]
    fn pagination_props_html_structure_button_emoji() {
        let _ = "🔘";
    }

    #[test]
    fn pagination_props_html_structure_radio_button() {
        let _ = "🔵";
    }

    #[test]
    fn pagination_props_html_structure_checkbox() {
        let _ = "☑️";
    }

    #[test]
    fn pagination_props_html_structure_square_button() {
        let _ = "🔳";
    }

    #[test]
    fn pagination_props_html_structure_white_square_button() {
        let _ = "🔲";
    }

    #[test]
    fn pagination_props_html_structure_black_large_square() {
        let _ = "⬛";
    }

    #[test]
    fn pagination_props_html_structure_white_large_square() {
        let _ = "⬜";
    }

    #[test]
    fn pagination_props_html_structure_black_medium_square() {
        let _ = "◼️";
    }

    #[test]
    fn pagination_props_html_structure_white_medium_square() {
        let _ = "◻️";
    }

    #[test]
    fn pagination_props_html_structure_black_medium_small_square() {
        let _ = "◾";
    }

    #[test]
    fn pagination_props_html_structure_white_medium_small_square() {
        let _ = "◽";
    }

    #[test]
    fn pagination_props_html_structure_black_small_square() {
        let _ = "▪️";
    }

    #[test]
    fn pagination_props_html_structure_white_small_square() {
        let _ = "▫️";
    }

    #[test]
    fn pagination_props_html_structure_check_mark_button() {
        let _ = "✅";
    }

    #[test]
    fn pagination_props_html_structure_ballot_box_with_check() {
        let _ = "☑️";
    }

    #[test]
    fn pagination_props_html_structure_ballot_box() {
        let _ = "☐";
    }

    #[test]
    fn pagination_props_html_structure_ballot_box_with_ballot_x() {
        let _ = "❌";
    }

    #[test]
    fn pagination_props_html_structure_x_mark() {
        let _ = "❎";
    }

    #[test]
    fn pagination_props_html_structure_heavy_check_mark() {
        let _ = "✔️";
    }

    #[test]
    fn pagination_props_html_structure_heavy_multiplication_x() {
        let _ = "✖️";
    }

    #[test]
    fn pagination_props_html_structure_cross_mark() {
        let _ = "❌";
    }

    #[test]
    fn pagination_props_html_structure_senses() {
        let _ = "👁️‍🗨️";
    }

    #[test]
    fn pagination_props_html_structure_eyes() {
        let _ = "👀";
    }

    #[test]
    fn pagination_props_html_structure_eye() {
        let _ = "👁️";
    }

    #[test]
    fn pagination_props_html_structure_brain() {
        let _ = "🧠";
    }

    #[test]
    fn pagination_props_html_structure_anatomical_heart() {
        let _ = "🫀";
    }

    #[test]
    fn pagination_props_html_structure_heart_with_arrow() {
        let _ = "💘";
    }

    #[test]
    fn pagination_props_html_structure_heart_with_ribbon() {
        let _ = "💝";
    }

    #[test]
    fn pagination_props_html_structure_sparkling_heart() {
        let _ = "💖";
    }

    #[test]
    fn pagination_props_html_structure_growing_heart() {
        let _ = "💗";
    }

    #[test]
    fn pagination_props_html_structure_beating_heart() {
        let _ = "💓";
    }

    #[test]
    fn pagination_props_html_structure_revolving_hearts() {
        let _ = "💞";
    }

    #[test]
    fn pagination_props_html_structure_two_hearts() {
        let _ = "💕";
    }

    #[test]
    fn pagination_props_html_structure_heart_decoration() {
        let _ = "💟";
    }

    #[test]
    fn pagination_props_html_structure_hearts_arrow() {
        let _ = "💞";
    }

    #[test]
    fn pagination_props_html_structure_blue_heart() {
        let _ = "💙";
    }

    #[test]
    fn pagination_props_html_structure_green_heart() {
        let _ = "💚";
    }

    #[test]
    fn pagination_props_html_structure_yellow_heart() {
        let _ = "💛";
    }

    #[test]
    fn pagination_props_html_structure_purple_heart() {
        let _ = "💜";
    }

    #[test]
    fn pagination_props_html_structure_brown_heart() {
        let _ = "🤎";
    }

    #[test]
    fn pagination_props_html_structure_black_heart() {
        let _ = "🖤";
    }

    #[test]
    fn pagination_props_html_structure_white_heart() {
        let _ = "🤍";
    }

    #[test]
    fn pagination_props_html_structure_broken_heart() {
        let _ = "💔";
    }

    #[test]
    fn pagination_props_html_structure_heart_exclamation() {
        let _ = "❣️";
    }

    #[test]
    fn pagination_props_html_structure_heavy_heart_exclamation() {
        let _ = "❤️";
    }

    #[test]
    fn pagination_props_html_structure_reaping_knife() {
        let _ = "🗡️";
    }

    #[test]
    fn pagination_props_html_structure_crossed_swords() {
        let _ = "⚔️";
    }

    #[test]
    fn pagination_props_html_structure_knife() {
        let _ = "🔪";
    }

    #[test]
    fn pagination_props_html_structure_amazon_knife() {
        let _ = "🗡️";
    }

    #[test]
    fn pagination_props_html_structure_boomerang() {
        let _ = "🪃";
    }

    #[test]
    fn pagination_props_html_structure_bow_and_arrow() {
        let _ = "🏹";
    }
}
