//! Integration tests for nav module.
//!
//! Tests the public API of nav components.

use yew::prelude::*;
use yew_nav_link::nav::NavList;

#[test]
fn nav_list_can_be_created() {
    let props = yew_nav_link::NavListProps {
        classes:    Classes::default(),
        id:         None,
        aria_label: None,
        children:   Children::new(vec![])
    };

    assert!(props.classes.is_empty());
    assert!(props.id.is_none());
}

#[test]
fn nav_list_with_custom_id() {
    let props = yew_nav_link::NavListProps {
        classes:    Classes::default(),
        id:         Some("main-nav"),
        aria_label: None,
        children:   Children::new(vec![])
    };

    assert_eq!(props.id, Some("main-nav"));
}

#[test]
fn nav_list_with_custom_aria_label() {
    let props = yew_nav_link::NavListProps {
        classes:    Classes::default(),
        id:         None,
        aria_label: Some("primary-navigation"),
        children:   Children::new(vec![])
    };

    assert_eq!(props.aria_label, Some("primary-navigation"));
}

#[test]
fn nav_list_with_class() {
    let props = yew_nav_link::NavListProps {
        classes:    Classes::from("nav nav-pills"),
        id:         None,
        aria_label: None,
        children:   Children::new(vec![])
    };

    assert!(props.classes.contains("nav"));
    assert!(props.classes.contains("nav-pills"));
}
