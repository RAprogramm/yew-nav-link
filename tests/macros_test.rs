//! Integration tests for yew-nav-link macros

use yew::prelude::*;
use yew_nav_link::{
    Match, breadcrumbs, nav_item, nav_link, nav_links, nav_list, nav_menu, nav_pagination,
    nav_tabs
};
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
    Contact
}

#[derive(Clone, PartialEq, Routable)]
enum ItemRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/docs")]
    Docs,
    #[at("/docs/api")]
    DocsApi,
    #[at("/contact")]
    Contact
}

#[test]
fn test_nav_link_macro() {
    let html = nav_link!(TestRoute::Home, "Home", Match::Exact);
    assert!(matches!(html, Html::VComp(_)));
}

#[test]
fn test_nav_links_macro() {
    let html = nav_links![
        (TestRoute::Home, "Home", Match::Exact),
        (TestRoute::About, "About", Match::Exact),
        (TestRoute::Docs, "Docs", Match::Partial),
    ];
    assert!(!matches!(html, Html::VList(list) if list.is_empty()));
}

#[test]
fn test_nav_links_macro_empty() {
    let html = nav_links![];
    assert!(matches!(html, Html::VList(_)));
}

#[test]
fn test_nav_list_macro() {
    let html = nav_list![
        (TestRoute::Home, "Home", Match::Exact),
        (TestRoute::About, "About", Match::Exact),
    ];
    assert!(!matches!(html, Html::VList(list) if list.is_empty()));
}

#[test]
fn test_nav_list_macro_empty() {
    let html = nav_list![];
    assert!(matches!(html, Html::VList(_)));
}

#[test]
fn test_nav_tabs_macro() {
    let html = nav_tabs![(TestRoute::Home, "Home"), (TestRoute::About, "About"),];
    assert!(!matches!(html, Html::VList(list) if list.is_empty()));
}

#[test]
fn test_nav_tabs_macro_empty() {
    let html = nav_tabs![];
    assert!(matches!(html, Html::VList(_)));
}

#[test]
fn test_breadcrumbs_macro() {
    let html = breadcrumbs!(TestRoute, (Home, "Home"), (Docs, "Docs"), (DocsApi, "API"),);
    assert!(!matches!(html, Html::VList(list) if list.is_empty()));
}

#[test]
fn test_breadcrumbs_macro_empty() {
    let html = breadcrumbs![];
    assert!(matches!(html, Html::VList(_)));
}

#[test]
fn test_nav_menu_macro() {
    let html = nav_menu![
        (TestRoute::Home, "Home", Match::Exact),
        (TestRoute::About, "About", Match::Exact),
    ];
    assert!(!matches!(html, Html::VList(list) if list.is_empty()));
}

#[test]
fn test_nav_menu_macro_empty() {
    let html = nav_menu![];
    assert!(matches!(html, Html::VList(_)));
}

#[test]
fn test_nav_pagination_macro() {
    let html = nav_pagination!(
        current_page = 3u32,
        total_pages = 10u32,
        siblings = 2,
        show_first_last = true,
    );
    assert!(!matches!(html, Html::VList(list) if list.is_empty()));
}

#[test]
fn test_nav_pagination_macro_defaults() {
    let html = nav_pagination!(current_page = 1u32, total_pages = 1u32,);
    assert!(!matches!(html, Html::VList(list) if list.is_empty()));
}

#[test]
fn test_nav_pagination_macro_empty() {
    let html = nav_pagination!();
    assert!(matches!(html, Html::VList(_)));
}

#[test]
fn test_nav_item_macro() {
    nav_item!(ItemRoute,
        Home => "Home",
        About => "About Us",
        Docs => "Documentation",
        DocsApi => "API Reference",
        Contact => "Contact Us",
    );

    let items = nav_items();
    assert_eq!(items.len(), 5);
    assert_eq!(items[0].label, "Home");
    assert_eq!(items[1].label, "About Us");
    assert_eq!(items[2].label, "Documentation");
    assert_eq!(items[3].label, "API Reference");
    assert_eq!(items[4].label, "Contact Us");
}
