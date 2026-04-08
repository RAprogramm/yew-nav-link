//! Routable extension macros

use yew_router::prelude::*;

/// Extends a `Routable` enum with navigation helper methods
#[macro_export]
macro_rules! routable_ext {
    ($name:ident, labels: { $( $variant:ident => $label:expr ),+ $(,)? }, icons: { $( $icon_variant:ident => $icon:expr ),+ $(,)? }) => {
        impl $name {
            pub fn nav_label(&self) -> &'static str {
                match self {
                    $( $name::$variant => $label, )+
                }
            }

            pub fn nav_icon(&self) -> Option<&'static str> {
                match self {
                    $( $name::$icon_variant => Some($icon), )+
                    _ => None
                }
            }

            pub fn is_root(&self) -> bool {
                self.to_path() == "/"
            }
        }
    };
    ($name:ident, labels: { $( $variant:ident => $label:expr ),+ $(,)? }) => {
        impl $name {
            pub fn nav_label(&self) -> &'static str {
                match self {
                    $( $name::$variant => $label, )+
                }
            }

            pub fn is_root(&self) -> bool {
                self.to_path() == "/"
            }
        }
    };
    ($name:ident) => {
        impl $name {
            pub fn is_root(&self) -> bool {
                self.to_path() == "/"
            }
        }
    };
}

/// Generates a navigation item struct with path and label for a route
#[macro_export]
macro_rules! nav_item {
    ($name:ident, $( $variant:ident => $label:expr ),+ $(,)?) => {
        #[derive(Clone, Debug)]
        pub struct NavItem<R> {
            pub route: R,
            pub label: &'static str,
        }

        impl<R> NavItem<R> {
            pub fn new(route: R, label: &'static str) -> Self {
                Self { route, label }
            }
        }

        pub fn nav_items() -> Vec<NavItem<$name>> {
            vec![
                $( NavItem::new($name::$variant, $label), )+
            ]
        }
    };
}
