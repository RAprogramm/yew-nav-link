use yew::prelude::*;
use yew_router::prelude::*;

use super::{
    mode::Match,
    props::NavLinkProps,
    utils::{build_class, is_path_prefix}
};

/// Navigation link with automatic active state detection.
#[component]
pub fn NavLink<R: Routable + PartialEq + Clone + 'static>(props: &NavLinkProps<R>) -> Html {
    let current_route = use_route::<R>();
    let is_active = current_route.is_some_and(|route| {
        if props.partial {
            is_path_prefix(&props.to.to_path(), &route.to_path())
        } else {
            route == props.to
        }
    });

    html! {
        <Link<R> to={props.to.clone()} classes={classes!(build_class(is_active, props.class, props.active_class))}>
            { for props.children.iter() }
        </Link<R>>
    }
}

/// Creates a `NavLink` with the specified match mode.
pub fn nav_link<R: Routable + PartialEq + Clone + 'static>(
    to: R,
    children: &str,
    match_mode: Match
) -> Html {
    let partial = match_mode == Match::Partial;
    html! {
        <NavLink<R> to={to} {partial}>{ Html::from(children) }</NavLink<R>>
    }
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
    fn nav_link_exact_returns_html() {
        let html = nav_link(TestRoute::Home, "Home", Match::Exact);
        assert!(matches!(html, Html::VComp(_)));
    }

    #[test]
    fn nav_link_empty_text() {
        let html = nav_link(TestRoute::Home, "", Match::Exact);
        assert!(matches!(html, Html::VComp(_)));
    }

    #[test]
    fn nav_link_partial_match() {
        let html = nav_link(TestRoute::Home, "Home", Match::Partial);
        assert!(matches!(html, Html::VComp(_)));
    }
}
