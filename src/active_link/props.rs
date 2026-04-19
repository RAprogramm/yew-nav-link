use std::marker::PhantomData;

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
    #[prop_or(false)]
    pub partial: bool,

    /// Base CSS class applied to the link.
    #[prop_or("nav-link")]
    pub class: &'static str,

    /// CSS class applied when the link is active.
    #[prop_or("active")]
    pub active_class: &'static str,

    #[prop_or_default]
    pub(crate) _marker: PhantomData<R>
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
            class:        "nav-link",
            active_class: "active",
            _marker:      PhantomData
        };
        let props2 = props1.clone();
        assert_eq!(props1, props2);
    }
}
