//! Integration tests for components module.

use yew::prelude::*;

#[test]
fn nav_badge_default_props() {
    let props = yew_nav_link::NavBadgeProps {
        classes:  Classes::default(),
        variant:  "primary",
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
        variant:  "success",
        pill:     false,
        children: Children::new(vec![])
    };

    assert_eq!(props.variant, "success");
}

#[test]
fn nav_badge_pill_style() {
    let props = yew_nav_link::NavBadgeProps {
        classes:  Classes::default(),
        variant:  "danger",
        pill:     true,
        children: Children::new(vec![])
    };

    assert!(props.pill);
}

#[test]
fn nav_badge_clone() {
    let props1 = yew_nav_link::NavBadgeProps {
        classes:  Classes::from("custom"),
        variant:  "warning",
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
        text:     Some("Settings"),
        children: Children::new(vec![])
    };

    assert_eq!(props.text, Some("Settings"));
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
        text:     Some("Menu"),
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
        text:    "Hello World"
    };

    assert_eq!(props.text, "Hello World");
}

#[test]
fn nav_text_with_class() {
    let props = yew_nav_link::NavTextProps {
        classes: Classes::from("nav-text-muted"),
        text:    "Copyright"
    };

    assert!(props.classes.contains("nav-text-muted"));
}
