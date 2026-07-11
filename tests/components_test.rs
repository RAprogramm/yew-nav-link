// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Integration tests for components module.

#![cfg(not(target_arch = "wasm32"))]

use yew::prelude::*;

#[test]
fn nav_badge_default_props() {
    let props = yew_nav_link::NavBadgeProps {
        classes:  Classes::default(),
        variant:  AttrValue::Static("primary"),
        pill:     false,
        children: Children::new(vec![])
    };

    assert_eq!(props.variant, "primary");
    assert!(!props.pill);
}

#[test]
fn nav_badge_with_variant() {
    let props = yew_nav_link::NavBadgeProps {
        classes:  Classes::default(),
        variant:  AttrValue::Static("success"),
        pill:     false,
        children: Children::new(vec![])
    };

    assert_eq!(props.variant, "success");
}

#[test]
fn nav_badge_pill_style() {
    let props = yew_nav_link::NavBadgeProps {
        classes:  Classes::default(),
        variant:  AttrValue::Static("danger"),
        pill:     true,
        children: Children::new(vec![])
    };

    assert!(props.pill);
}

#[test]
fn nav_badge_clone() {
    let props1 = yew_nav_link::NavBadgeProps {
        classes:  Classes::from("custom"),
        variant:  AttrValue::Static("warning"),
        pill:     true,
        children: Children::new(vec![])
    };

    let props2 = props1.clone();
    assert_eq!(props1.variant, props2.variant);
    assert_eq!(props1.pill, props2.pill);
}

#[test]
fn nav_header_with_text() {
    let props = yew_nav_link::NavHeaderProps {
        classes:  Classes::default(),
        text:     Some(AttrValue::Static("Settings")),
        children: Children::new(vec![])
    };

    assert_eq!(props.text.as_deref(), Some("Settings"));
}

#[test]
fn nav_header_without_text() {
    let props = yew_nav_link::NavHeaderProps {
        classes:  Classes::default(),
        text:     None,
        children: Children::new(vec![])
    };

    assert!(props.text.is_none());
}

#[test]
fn nav_header_clone() {
    let props1 = yew_nav_link::NavHeaderProps {
        classes:  Classes::from("nav-header-primary"),
        text:     Some(AttrValue::Static("Menu")),
        children: Children::new(vec![])
    };

    let props2 = props1.clone();
    assert_eq!(props1.text, props2.text);
    assert_eq!(props1.classes, props2.classes);
}

#[test]
fn nav_text_required_props() {
    let props = yew_nav_link::NavTextProps {
        classes: Classes::default(),
        text:    AttrValue::Static("Hello World")
    };

    assert_eq!(props.text, "Hello World");
}

#[test]
fn nav_text_with_class() {
    let props = yew_nav_link::NavTextProps {
        classes: Classes::from("nav-text-muted"),
        text:    AttrValue::Static("Copyright")
    };

    assert!(props.classes.contains("nav-text-muted"));
}
