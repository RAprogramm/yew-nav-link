//! Attribute builder for navigation components.
//!
//! Provides builders for creating consistent HTML attributes.
//!
//! # Example
//!
//! ```ignore
//! use yew::prelude::*;
//! use yew_nav_link::NavLinkAttrs;
//!
//! let attrs = NavLinkAttrs::new()
//!     .with_class("custom-link")
//!     .with_id("home-link");
//! ```

use yew::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct NavLinkAttrs {
    pub class:        Classes,
    pub id:           Option<&'static str>,
    pub aria_current: Option<&'static str>,
    pub data_toggle:  Option<&'static str>,
    pub role:         Option<&'static str>
}

impl NavLinkAttrs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_class(mut self, class: impl Into<Classes>) -> Self {
        self.class.push(class);
        self
    }

    pub fn with_id(mut self, id: &'static str) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_aria_current(mut self, value: &'static str) -> Self {
        self.aria_current = Some(value);
        self
    }

    pub fn with_data_toggle(mut self, value: &'static str) -> Self {
        self.data_toggle = Some(value);
        self
    }

    pub fn with_role(mut self, value: &'static str) -> Self {
        self.role = Some(value);
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NavItemAttrs {
    pub class:    Classes,
    pub id:       Option<&'static str>,
    pub role:     Option<&'static str>,
    pub disabled: bool
}

impl NavItemAttrs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_class(mut self, class: impl Into<Classes>) -> Self {
        self.class.push(class);
        self
    }

    pub fn with_id(mut self, id: &'static str) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_role(mut self, value: &'static str) -> Self {
        self.role = Some(value);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NavListAttrs {
    pub class:      Classes,
    pub id:         Option<&'static str>,
    pub aria_label: Option<&'static str>
}

impl NavListAttrs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_class(mut self, class: impl Into<Classes>) -> Self {
        self.class.push(class);
        self
    }

    pub fn with_id(mut self, id: &'static str) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_aria_label(mut self, label: &'static str) -> Self {
        self.aria_label = Some(label);
        self
    }
}
