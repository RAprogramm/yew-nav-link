//! Integration tests for nav components.

use yew::prelude::*;
use yew_nav_link::nav::{NavDivider, NavItem};

#[test]
fn nav_item_default_props() {
    let props = yew_nav_link::NavItemProps {
        classes:  Classes::default(),
        disabled: false,
        children: Children::new(vec![])
    };

    assert!(!props.disabled);
    assert!(props.classes.is_empty());
}

#[test]
fn nav_item_disabled_state() {
    let props = yew_nav_link::NavItemProps {
        classes:  Classes::default(),
        disabled: true,
        children: Children::new(vec![])
    };

    assert!(props.disabled);
}

#[test]
fn nav_item_with_custom_class() {
    let props = yew_nav_link::NavItemProps {
        classes:  Classes::from("nav-item-active"),
        disabled: false,
        children: Children::new(vec![])
    };

    assert!(props.classes.contains("nav-item-active"));
}

#[test]
fn nav_item_clone() {
    let props1 = yew_nav_link::NavItemProps {
        classes:  Classes::from("test"),
        disabled: true,
        children: Children::new(vec![])
    };

    let props2 = props1.clone();
    assert_eq!(props1.disabled, props2.disabled);
    assert_eq!(props1.classes, props2.classes);
}

#[test]
fn nav_divider_default_props() {
    let props = yew_nav_link::NavDividerProps {
        classes:  Classes::default(),
        vertical: false,
        text:     None
    };

    assert!(!props.vertical);
    assert!(props.text.is_none());
}

#[test]
fn nav_divider_vertical() {
    let props = yew_nav_link::NavDividerProps {
        classes:  Classes::default(),
        vertical: true,
        text:     None
    };

    assert!(props.vertical);
}

#[test]
fn nav_divider_with_text() {
    let props = yew_nav_link::NavDividerProps {
        classes:  Classes::default(),
        vertical: false,
        text:     Some("Or")
    };

    assert_eq!(props.text, Some("Or"));
}

#[test]
fn nav_divider_clone() {
    let props1 = yew_nav_link::NavDividerProps {
        classes:  Classes::from("custom"),
        vertical: true,
        text:     Some("Divider")
    };

    let props2 = props1.clone();
    assert_eq!(props1.vertical, props2.vertical);
    assert_eq!(props1.text, props2.text);
}
