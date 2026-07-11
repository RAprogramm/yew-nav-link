// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Type-checked properties for [`crate::NavLink`].

use yew::prelude::*;
use yew_router::prelude::*;

/// Properties for the [`crate::NavLink`] component.
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NavLinkProps<R: Routable + PartialEq + Clone + 'static> {
    /// Target route for navigation.
    pub to: R,

    /// Content rendered inside the link element.
    pub children: Children,

    /// Enable partial (prefix) path matching.
    ///
    /// A root route (`"/"`) matches only the root path even in partial mode,
    /// so a Home link is not highlighted on every page.
    #[prop_or(false)]
    pub partial: bool,

    /// Base CSS class applied to the link.
    #[prop_or(AttrValue::Static("nav-link"))]
    pub class: AttrValue,

    /// CSS class applied when the link is active.
    #[prop_or(AttrValue::Static("active"))]
    pub active_class: AttrValue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Debug, Routable)]
    enum TestRoute {
        #[at("/")]
        Home
    }

    #[test]
    fn props_equality() {
        let props1: NavLinkProps<TestRoute> = NavLinkProps {
            to:           TestRoute::Home,
            children:     Children::default(),
            partial:      false,
            class:        AttrValue::Static("nav-link"),
            active_class: AttrValue::Static("active")
        };
        let props2 = props1.clone();
        assert_eq!(props1, props2);
    }
}
