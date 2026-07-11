// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! The [`NavLink`] component and the [`nav_link`] function-syntax helper.
//!
//! Both build on the same active-state algorithm: compare the current route
//! to the target on every render and toggle the `active` class plus
//! `aria-current="page"` accordingly.

use yew::prelude::*;
use yew_router::prelude::*;

use super::{
    mode::Match,
    props::NavLinkProps,
    utils::{build_class, is_path_prefix}
};

/// Navigation link with automatic active-state detection.
///
/// Renders an `<a>` tag and intercepts left-clicks to push the target route
/// through `yew_router`'s [`Navigator`]. Modifier-clicks (Cmd/Ctrl/Shift/Alt
/// or middle-click) fall through to the browser, preserving the standard
/// "open in new tab" affordance.
///
/// When the route matches, the rendered anchor gains:
/// - the `active` class (or whatever `active_class` overrides it with), and
/// - `aria-current="page"` so screen readers announce the current location.
///
/// Outside a router context (no [`Navigator`] in scope) the component
/// degrades to a plain anchor: the href is the bare route path and clicks
/// follow default browser navigation instead of being suppressed.
#[component]
pub fn NavLink<R: Routable + PartialEq + Clone + 'static>(props: &NavLinkProps<R>) -> Html {
    let current_route = use_route::<R>();
    let navigator = use_navigator();
    let is_active = current_route.is_some_and(|route| {
        if props.partial {
            is_path_prefix(&props.to.to_path(), &route.to_path())
        } else {
            route == props.to
        }
    });

    let href = resolved_href(navigator.as_ref(), props.to.to_path());

    let onclick = {
        let to = props.to.clone();
        Callback::from(move |event: MouseEvent| {
            if is_modified_click(&event) {
                return;
            }
            if let Some(nav) = &navigator {
                event.prevent_default();
                nav.push(&to);
            }
        })
    };

    let class = build_class(is_active, &props.class, &props.active_class);
    let aria_current = if is_active { Some("page") } else { None };

    html! {
        <a class={class} href={href} onclick={onclick} aria-current={aria_current}>
            { for props.children.iter() }
        </a>
    }
}

/// Builds the displayed href, prefixing the router basename when one is set
/// (e.g. `/yew-nav-link` on GitHub Pages).
///
/// Without a `Navigator` in scope the bare route path is returned; it stays a
/// valid relative anchor, and the click handler then leaves default browser
/// navigation intact so the link keeps working outside a router context.
fn resolved_href(navigator: Option<&Navigator>, path: String) -> String {
    match navigator.and_then(Navigator::basename) {
        Some(base) if !base.is_empty() => format!("{base}{path}"),
        _ => path
    }
}

/// Returns `true` for clicks carrying a modifier key, which must fall through
/// to the browser to preserve "open in new tab / window" affordances.
fn is_modified_click(event: &MouseEvent) -> bool {
    event.meta_key() || event.ctrl_key() || event.shift_key() || event.alt_key()
}

/// Creates a `NavLink` with the specified match mode for plain-text labels.
///
/// `match_mode` selects between exact and prefix matching:
///
/// - [`Match::Exact`]: the link is active only when the current route equals
///   `to`.
/// - [`Match::Partial`]: the link stays active for any nested route whose path
///   begins with `to.to_path()`.
///
/// # Examples
///
/// ```rust
/// use yew::prelude::*;
/// use yew_nav_link::{Match, nav_link};
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Debug, Routable)]
/// enum Route {
///     #[at("/")]
///     Home,
///     #[at("/docs")]
///     Docs,
///     #[at("/docs/api")]
///     DocsApi
/// }
///
/// #[function_component]
/// fn Menu() -> Html {
///     html! {
///         <nav>
///             { nav_link(Route::Home, "Home", Match::Exact) }
///             { nav_link(Route::Docs, "Docs", Match::Partial) }
///         </nav>
///     }
/// }
/// ```
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
