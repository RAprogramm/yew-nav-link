#![cfg(test)]

use yew::prelude::*;
use yew_nav_link::components::{NavDropdownDividerProps, NavDropdownItemProps, NavDropdownProps};

#[test]
fn nav_dropdown_props_default_values() {
    // With prop_or_default, need to test manually since Default doesn't work with
    // prop_or
    let props = NavDropdownProps {
        classes:     Classes::default(),
        toggle_text: "dropdown",
        id:          None,
        children:    Children::default()
    };
    assert!(props.classes.is_empty());
    assert_eq!(props.toggle_text, "dropdown");
    assert!(props.id.is_none());
}

#[test]
fn nav_dropdown_props_custom() {
    let props = NavDropdownProps {
        classes:     Classes::from("custom-dropdown"),
        toggle_text: "Menu",
        id:          Some("my-dropdown"),
        children:    Children::new(vec![])
    };
    assert!(props.classes.contains("custom-dropdown"));
    assert_eq!(props.toggle_text, "Menu");
    assert_eq!(props.id, Some("my-dropdown"));
}

#[test]
fn nav_dropdown_props_clone() {
    let props1 = NavDropdownProps {
        classes:     Classes::from("clone"),
        toggle_text: "Test",
        id:          None,
        children:    Children::new(vec![])
    };
    let props2 = props1.clone();
    assert!(props2.classes.contains("clone"));
    assert_eq!(props1.toggle_text, props2.toggle_text);
}

#[test]
fn nav_dropdown_item_props_default_values() {
    let props = NavDropdownItemProps {
        classes:  Classes::default(),
        disabled: false,
        children: Children::default()
    };
    assert!(props.classes.is_empty());
    assert!(!props.disabled);
}

#[test]
fn nav_dropdown_item_props_custom() {
    let props = NavDropdownItemProps {
        classes:  Classes::from("item-class"),
        disabled: true,
        children: Children::new(vec![])
    };
    assert!(props.classes.contains("item-class"));
    assert!(props.disabled);
}

#[test]
fn nav_dropdown_item_props_clone() {
    let props1 = NavDropdownItemProps {
        classes:  Classes::from("test"),
        disabled: false,
        children: Children::new(vec![])
    };
    let props2 = props1.clone();
    assert_eq!(props1.disabled, props2.disabled);
}

#[test]
fn nav_dropdown_divider_props_default_values() {
    let props = NavDropdownDividerProps {
        classes: Classes::default()
    };
    assert!(props.classes.is_empty());
}

#[test]
fn nav_dropdown_divider_props_custom() {
    let props = NavDropdownDividerProps {
        classes: Classes::from("my-divider")
    };
    assert!(props.classes.contains("my-divider"));
}

#[test]
fn nav_dropdown_debug() {
    let props = NavDropdownProps {
        classes:     Classes::default(),
        toggle_text: "dropdown",
        id:          None,
        children:    Children::default()
    };
    let debug_str = format!("{:?}", props);
    assert!(debug_str.contains("NavDropdownProps"));
}

#[test]
fn nav_dropdown_item_debug() {
    let props = NavDropdownItemProps {
        classes:  Classes::default(),
        disabled: false,
        children: Children::default()
    };
    let debug_str = format!("{:?}", props);
    assert!(debug_str.contains("NavDropdownItemProps"));
}

#[test]
fn nav_dropdown_divider_debug() {
    let props = NavDropdownDividerProps {
        classes: Classes::default()
    };
    let debug_str = format!("{:?}", props);
    assert!(debug_str.contains("NavDropdownDividerProps"));
}
