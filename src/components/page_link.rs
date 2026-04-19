//! # `PageLink`
//!
//! Link or span element within a pagination page item.
//! Renders an `<a>` when `href` is set, otherwise a `<span>`.
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
//!                 <PageLink href={Some("/page/1")}>{ "1" }</PageLink>
//!             </PageItem>
//!             <PageItem page={0} disabled=true>
//!                 <PageLink href={None}>{ "..." }</PageLink>
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
//! | `page-link` | Always applied |
//!
//! # Props
//!
//! | Prop | Type | Default | Description |
//! |------|------|---------|-------------|
//! | `href` | `Option<&'static str>` | `None` | Link URL |
//! | `classes` | `Classes` | — | Additional CSS classes |
//! | `children` | `Children` | — | Content |

use yew::prelude::*;

/// Properties for the [`PageLink`] component.
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `href` | `Option<&'static str>` | `None` | Link URL |
/// | `classes` | `Classes` | — | Additional CSS classes |
/// | `children` | `Children` | — | Content |
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct PageLinkProps {
    /// Additional CSS classes applied to the page link.
    #[prop_or_default]
    pub classes: Classes,

    /// Optional URL. When set, renders an `<a>` element; otherwise a `<span>`.
    #[prop_or_default]
    pub href: Option<&'static str>,

    /// Content rendered inside the page link.
    #[prop_or_default]
    pub children: Children
}

/// Link or span element within a pagination page item.
///
/// Renders an `<a>` when `href` is set, otherwise a `<span>`.
///
/// # CSS Classes
///
/// - `page-link` - Always applied
#[function_component]
pub fn PageLink(props: &PageLinkProps) -> Html {
    let mut classes = props.classes.clone();
    classes.push("page-link");

    if let Some(href) = props.href {
        html! {
            <a {href} {classes}>
                { for props.children.iter() }
            </a>
        }
    } else {
        html! {
            <span {classes}>
                { for props.children.iter() }
            </span>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_link_with_href() {
        let props = PageLinkProps {
            classes:  Classes::default(),
            href:     Some("/page/2"),
            children: Children::new(vec![])
        };

        assert_eq!(props.href, Some("/page/2"));
    }

    #[test]
    fn page_link_without_href() {
        let props = PageLinkProps {
            classes:  Classes::default(),
            href:     None,
            children: Children::new(vec![])
        };

        assert!(props.href.is_none());
    }
}
