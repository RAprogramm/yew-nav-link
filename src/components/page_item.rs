//! # PageItem
//!
//! A single page button within a [`Pagination`](super::Pagination) component.
//! Renders an `<li>` with the `page-item` class and active/disabled state.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::components::{PageItem, PageLink};
//!
//! #[component]
//! fn PaginationNav() -> Html {
//!     html! {
//!         <nav><ul class="pagination">
//!             <PageItem page={1} active=true>
//!                 <PageLink href={None}>{ "1" }</PageLink>
//!             </PageItem>
//!             <PageItem page={2}>
//!                 <PageLink href={Some("/page/2")}>{ "2" }</PageLink>
//!             </PageItem>
//!         </ul></nav>
//!     }
//! }
//! ```
//!
//! # CSS Classes
//!
//! | Class | Condition |
//! |-------|-----------|
//! | `page-item` | Always applied |
//! | `active` | Applied when `active` is `true` |
//! | `disabled` | Applied when `disabled` is `true` |
//!
//! # Props
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `page` | `u32` | — | Page number (required) |
//! | `active` | `bool` | `false` | Currently active page |
//! | `disabled` | `bool` | `false` | Disabled state |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Content |

use yew::prelude::*;

/// Properties for the [`PageItem`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `page` | `u32` | — | Page number (required) |
/// | `active` | `bool` | `false` | Currently active page |
/// | `disabled` | `bool` | `false` | Disabled state |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Content |
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct PageItemProps {
    /// Additional CSS classes applied to the page item.
    #[prop_or_default]
    pub classes: Classes,

    /// Page number this item represents.
    pub page: u32,

    /// Whether this page item is the currently active page.
    #[prop_or(false)]
    pub active: bool,

    /// Whether this page item is disabled (e.g. an ellipsis).
    #[prop_or(false)]
    pub disabled: bool,

    /// Content rendered inside the page item.
    #[prop_or_default]
    pub children: Children,
}

/// A single page button within a [`Pagination`](super::Pagination) component.
///
/// # CSS Classes
///
/// - `page-item` - Always applied
/// - `active` - Applied when `active` is `true`
/// - `disabled` - Applied when `disabled` is `true`
#[function_component]
pub fn PageItem(props: &PageItemProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("page-item");

    if props.active {
        classes.push("active");
    }

    if props.disabled {
        classes.push("disabled");
    }

    html! {
        <li {classes}>
            <span class="page-link">
                { for props.children.iter() }
            </span>
        </li>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_item_props() {
        let props = PageItemProps {
            classes: Classes::default(),
            page: 1,
            active: false,
            disabled: false,
            children: Children::new(vec![]),
        };

        assert_eq!(props.page, 1);
        assert!(!props.active);
        assert!(!props.disabled);
    }
}
