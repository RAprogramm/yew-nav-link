#![cfg(test)]

use yew_nav_link::{NavItemAttrs, NavLinkAttrs, NavListAttrs};

#[test]
fn test_nav_link_attrs_new() {
    let attrs = NavLinkAttrs::new();
    assert!(attrs.class.is_empty());
    assert!(attrs.id.is_none());
    assert!(attrs.aria_current.is_none());
}

#[test]
fn test_nav_link_attrs_with_class() {
    let attrs = NavLinkAttrs::new().with_class("custom-class");
    assert!(attrs.class.contains("custom-class"));
}

#[test]
fn test_nav_link_attrs_with_id() {
    let attrs = NavLinkAttrs::new().with_id("my-id");
    assert_eq!(attrs.id, Some("my-id"));
}

#[test]
fn test_nav_link_attrs_with_aria_current() {
    let attrs = NavLinkAttrs::new().with_aria_current("page");
    assert_eq!(attrs.aria_current, Some("page"));
}

#[test]
fn test_nav_link_attrs_with_data_toggle() {
    let attrs = NavLinkAttrs::new().with_data_toggle("dropdown");
    assert_eq!(attrs.data_toggle, Some("dropdown"));
}

#[test]
fn test_nav_link_attrs_with_role() {
    let attrs = NavLinkAttrs::new().with_role("menuitem");
    assert_eq!(attrs.role, Some("menuitem"));
}

#[test]
fn test_nav_link_attrs_chained() {
    let attrs = NavLinkAttrs::new()
        .with_class("class1")
        .with_class("class2")
        .with_id("test-id")
        .with_aria_current("page")
        .with_data_toggle("dropdown")
        .with_role("link");

    assert!(attrs.class.contains("class1"));
    assert!(attrs.class.contains("class2"));
    assert_eq!(attrs.id, Some("test-id"));
    assert_eq!(attrs.aria_current, Some("page"));
    assert_eq!(attrs.data_toggle, Some("dropdown"));
    assert_eq!(attrs.role, Some("link"));
}

#[test]
fn test_nav_link_attrs_clone() {
    let attrs1 = NavLinkAttrs::new().with_class("test");
    let attrs2 = attrs1;
    assert!(attrs2.class.contains("test"));
}

#[test]
fn test_nav_link_attrs_debug() {
    let attrs = NavLinkAttrs::new().with_class("test");
    let debug_str = format!("{attrs:?}");
    assert!(debug_str.contains("NavLinkAttrs"));
}

#[test]
fn test_nav_item_attrs_new() {
    let attrs = NavItemAttrs::new();
    assert!(attrs.class.is_empty());
    assert!(attrs.id.is_none());
    assert!(attrs.role.is_none());
    assert!(!attrs.disabled);
}

#[test]
fn test_nav_item_attrs_with_class() {
    let attrs = NavItemAttrs::new().with_class("nav-item");
    assert!(attrs.class.contains("nav-item"));
}

#[test]
fn test_nav_item_attrs_with_id() {
    let attrs = NavItemAttrs::new().with_id("item-1");
    assert_eq!(attrs.id, Some("item-1"));
}

#[test]
fn test_nav_item_attrs_with_role() {
    let attrs = NavItemAttrs::new().with_role("presentation");
    assert_eq!(attrs.role, Some("presentation"));
}

#[test]
fn test_nav_item_attrs_disabled() {
    let attrs = NavItemAttrs::new().disabled();
    assert!(attrs.disabled);
}

#[test]
fn test_nav_item_attrs_chained() {
    let attrs = NavItemAttrs::new()
        .with_class("item")
        .with_id("test")
        .with_role("item")
        .disabled();

    assert!(attrs.class.contains("item"));
    assert_eq!(attrs.id, Some("test"));
    assert_eq!(attrs.role, Some("item"));
    assert!(attrs.disabled);
}

#[test]
fn test_nav_item_attrs_clone() {
    let attrs1 = NavItemAttrs::new().with_class("clone-test");
    let attrs2 = attrs1;
    assert!(attrs2.class.contains("clone-test"));
}

#[test]
fn test_nav_item_attrs_debug() {
    let attrs = NavItemAttrs::new().with_class("test");
    let debug_str = format!("{attrs:?}");
    assert!(debug_str.contains("NavItemAttrs"));
}

#[test]
fn test_nav_list_attrs_new() {
    let attrs = NavListAttrs::new();
    assert!(attrs.class.is_empty());
    assert!(attrs.id.is_none());
    assert!(attrs.aria_label.is_none());
}

#[test]
fn test_nav_list_attrs_with_class() {
    let attrs = NavListAttrs::new().with_class("nav");
    assert!(attrs.class.contains("nav"));
}

#[test]
fn test_nav_list_attrs_with_id() {
    let attrs = NavListAttrs::new().with_id("nav-list");
    assert_eq!(attrs.id, Some("nav-list"));
}

#[test]
fn test_nav_list_attrs_with_aria_label() {
    let attrs = NavListAttrs::new().with_aria_label("Main navigation");
    assert_eq!(attrs.aria_label, Some("Main navigation"));
}

#[test]
fn test_nav_list_attrs_chained() {
    let attrs = NavListAttrs::new()
        .with_class("nav-list")
        .with_id("primary")
        .with_aria_label("Primary");

    assert!(attrs.class.contains("nav-list"));
    assert_eq!(attrs.id, Some("primary"));
    assert_eq!(attrs.aria_label, Some("Primary"));
}

#[test]
fn test_nav_list_attrs_clone() {
    let attrs1 = NavListAttrs::new().with_class("clone");
    let attrs2 = attrs1;
    assert!(attrs2.class.contains("clone"));
}

#[test]
fn test_nav_list_attrs_debug() {
    let attrs = NavListAttrs::new().with_class("test");
    let debug_str = format!("{attrs:?}");
    assert!(debug_str.contains("NavListAttrs"));
}
