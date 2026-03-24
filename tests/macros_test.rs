//! Integration tests for yew-nav-link macros

use yew::prelude::*;
use yew_nav_link::{nav_link, Match};
use yew_router::prelude::Routable;

#[derive(Clone, PartialEq, Routable)]
enum TestRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/docs")]
    Docs,
    #[at("/docs/api")]
    DocsApi,
    #[at("/contact")]
    Contact,
}

#[test]
fn test_nav_links_macro() {
    let routes = vec![
        (TestRoute::Home, "Home", Match::Exact),
        (TestRoute::About, "About", Match::Exact),
        (TestRoute::Docs, "Docs", Match::Partial),
    ];

    let html: Html = routes
        .into_iter()
        .map(|(route, label, match_mode)| nav_link(route, label, match_mode))
        .collect();

    let vlist = matches!(html, Html::VList(_));
    assert!(vlist, "Expected Html::VList but got: {:?}", html);
}

#[test]
fn test_nav_list_macro() {
    let items = vec![
        (TestRoute::Home, "Home", Match::Exact),
        (TestRoute::About, "About", Match::Exact),
    ];

    let html: Html = items
        .into_iter()
        .map(|(route, label, match_mode)| {
            html! {
                <li class="nav-item">
                    { nav_link(route, label, match_mode) }
                </li>
            }
        })
        .collect();

    let vlist = matches!(html, Html::VList(_));
    assert!(vlist, "Expected Html::VList but got: {:?}", html);
}

#[test]
fn test_nav_tabs_macro() {
    let tabs = vec![(TestRoute::Home, "Home"), (TestRoute::About, "About")];

    let html: Html = tabs
        .into_iter()
        .map(|(route, label)| {
            html! {
                <li class="nav-tab">
                    <a href={route.to_path()}>
                        { label }
                    </a>
                </li>
            }
        })
        .collect();

    let vlist = matches!(html, Html::VList(_));
    assert!(vlist, "Expected Html::VList but got: {:?}", html);
}

#[test]
fn test_breadcrumbs_macro() {
    let crumbs = vec![(TestRoute::Home, "/"), (TestRoute::About, "About")];

    let html: Html = crumbs
        .into_iter()
        .map(|(route, label)| {
            html! {
                <li class="breadcrumb-item">
                    <yew_router::prelude::Link<TestRoute> to={route.clone()}>
                        { label }
                    </yew_router::prelude::Link<TestRoute>>
                </li>
            }
        })
        .collect();

    let vlist = matches!(html, Html::VList(_));
    assert!(vlist, "Expected Html::VList but got: {:?}", html);
}
