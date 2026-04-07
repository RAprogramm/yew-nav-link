//! # Attribute Builders
//!
//! Builder types for constructing consistent HTML attributes on navigation
//! components. Each builder follows a fluent API with `with_*` methods.
//!
//! # Example
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_nav_link::NavLinkAttrs;
//!
//! let attrs = NavLinkAttrs::new()
//!     .with_class("custom-link")
//!     .with_id("home-link")
//!     .with_aria_current("page");
//! ```
//!
//! # Types
//!
//! | Type | Description |
//! |------|-------------|
//! | [`NavLinkAttrs`] | Attributes for link elements |
//! | [`NavItemAttrs`] | Attributes for list item elements |
//! | [`NavListAttrs`] | Attributes for list container elements |

use yew::prelude::*;

/// HTML attributes for navigation link elements.
///
/// Collects classes, ARIA attributes, and data attributes to apply
/// to [`NavLink`](crate::NavLink) or similar link components.
#[derive(Clone, Debug, Default)]
pub struct NavLinkAttrs {
    /// CSS classes applied to the link element.
    pub class:        Classes,
    /// Optional `id` attribute for the link element.
    pub id:           Option<&'static str>,
    /// Optional `aria-current` attribute, typically `"page"` or `"step"`.
    pub aria_current: Option<&'static str>,
    /// Optional `data-toggle` attribute for framework integration.
    pub data_toggle:  Option<&'static str>,
    /// Optional ARIA `role` attribute for the link element.
    pub role:         Option<&'static str>
}

impl NavLinkAttrs {
    /// Creates a new [`NavLinkAttrs`] with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a CSS class to the link element.
    pub fn with_class(mut self, class: impl Into<Classes>) -> Self {
        self.class.push(class);
        self
    }

    /// Sets the `id` attribute on the link element.
    pub fn with_id(mut self, id: &'static str) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the `aria-current` attribute on the link element.
    ///
    /// Common values: `"page"`, `"step"`, `"location"`.
    pub fn with_aria_current(mut self, value: &'static str) -> Self {
        self.aria_current = Some(value);
        self
    }

    /// Sets the `data-toggle` attribute on the link element.
    pub fn with_data_toggle(mut self, value: &'static str) -> Self {
        self.data_toggle = Some(value);
        self
    }

    /// Sets the ARIA `role` attribute on the link element.
    pub fn with_role(mut self, value: &'static str) -> Self {
        self.role = Some(value);
        self
    }
}

/// HTML attributes for navigation item elements.
///
/// Used to configure the `<li>` wrapper around navigation links
/// within [`NavList`](crate::NavList).
#[derive(Clone, Debug, Default)]
pub struct NavItemAttrs {
    /// CSS classes applied to the item element.
    pub class:    Classes,
    /// Optional `id` attribute for the item element.
    pub id:       Option<&'static str>,
    /// Optional ARIA `role` attribute for the item element.
    pub role:     Option<&'static str>,
    /// Whether the navigation item is disabled.
    pub disabled: bool
}

impl NavItemAttrs {
    /// Creates a new [`NavItemAttrs`] with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a CSS class to the item element.
    pub fn with_class(mut self, class: impl Into<Classes>) -> Self {
        self.class.push(class);
        self
    }

    /// Sets the `id` attribute on the item element.
    pub fn with_id(mut self, id: &'static str) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the ARIA `role` attribute on the item element.
    pub fn with_role(mut self, value: &'static str) -> Self {
        self.role = Some(value);
        self
    }

    /// Marks the navigation item as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// HTML attributes for navigation list containers.
///
/// Used to configure the `<ul>` element rendered by
/// [`NavList`](crate::NavList).
#[derive(Clone, Debug, Default)]
pub struct NavListAttrs {
    /// CSS classes applied to the list element.
    pub class:      Classes,
    /// Optional `id` attribute for the list element.
    pub id:         Option<&'static str>,
    /// Optional `aria-label` for screen reader accessibility.
    pub aria_label: Option<&'static str>
}

impl NavListAttrs {
    /// Creates a new [`NavListAttrs`] with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a CSS class to the list element.
    pub fn with_class(mut self, class: impl Into<Classes>) -> Self {
        self.class.push(class);
        self
    }

    /// Sets the `id` attribute on the list element.
    pub fn with_id(mut self, id: &'static str) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the `aria-label` attribute on the list element.
    pub fn with_aria_label(mut self, label: &'static str) -> Self {
        self.aria_label = Some(label);
        self
    }
}
