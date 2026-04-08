use yew_router::prelude::*;

/// Extends a `Routable` enum with navigation helper methods.
///
/// Generates the following methods:
/// - `fn nav_label(&self) -> &'static str` — human-readable label for the route
/// - `fn nav_icon(&self) -> Option<&'static str>` — optional icon class
/// - `fn is_root(&self) -> bool` — whether the route path is `/`
///
/// # Example
///
/// ```rust
/// use yew_router::prelude::*;
/// use yew_nav_link::routable_ext;
///
/// #[derive(Clone, PartialEq, Routable)]
/// enum Route {
///     #[at("/")]
///     Home,
///     #[at("/about")]
///     About,
///     #[at("/docs")]
///     Docs,
/// }
///
/// routable_ext!(Route,
///     labels: {
///         Home => "Home",
///         About => "About Us",
///         Docs => "Documentation",
///     }
/// );
/// ```
#[macro_export]
macro_rules! routable_ext {
    ($name:ident, labels: { $( $variant:ident => $label:expr ),+ $(,)? }, icons: { $( $icon_variant:ident => $icon:expr ),+ $(,)? }) => {
        impl $name {
            /// Returns a human-readable label for this route.
            pub fn nav_label(&self) -> &'static str {
                match self {
                    $( $name::$variant => $label, )+
                }
            }

            /// Returns an optional icon class for this route.
            pub fn nav_icon(&self) -> Option<&'static str> {
                match self {
                    $( $name::$icon_variant => Some($icon), )+
                    _ => None
                }
            }

            /// Returns `true` if this route's path is `/`.
            pub fn is_root(&self) -> bool {
                self.to_path() == "/"
            }
        }
    };
    ($name:ident, labels: { $( $variant:ident => $label:expr ),+ $(,)? }) => {
        impl $name {
            /// Returns a human-readable label for this route.
            pub fn nav_label(&self) -> &'static str {
                match self {
                    $( $name::$variant => $label, )+
                }
            }

            /// Returns `true` if this route's path is `/`.
            pub fn is_root(&self) -> bool {
                self.to_path() == "/"
            }
        }
    };
    ($name:ident) => {
        impl $name {
            /// Returns `true` if this route's path is `/`.
            pub fn is_root(&self) -> bool {
                self.to_path() == "/"
            }
        }
    };
}

/// Generates a navigation item struct with path and label for a route.
///
/// Creates a struct that pairs a route with a display label, useful for
/// building navigation menus programmatically.
///
/// # Example
///
/// ```rust
/// use yew_nav_link::nav_item;
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Routable)]
/// enum Route {
///     #[at("/")]
///     Home,
///     #[at("/about")]
///     About,
///     #[at("/docs")]
///     Docs
/// }
///
/// nav_item!(Route,
///     Home => "Home",
///     About => "About Us",
///     Docs => "Documentation"
/// );
///
/// let items: Vec<NavItem<Route>> = nav_items();
/// ```
#[macro_export]
macro_rules! nav_item {
    ($name:ident, $( $variant:ident => $label:expr ),+ $(,)?) => {
        /// A navigation item pairing a route with a display label.
        #[derive(Clone, Debug)]
        pub struct NavItem<R> {
            /// The route this item navigates to.
            pub route: R,
            /// Human-readable label for display.
            pub label: &'static str,
        }

        impl<R> NavItem<R> {
            /// Creates a new navigation item.
            pub fn new(route: R, label: &'static str) -> Self {
                Self { route, label }
            }
        }

        /// Returns all navigation items for this route enum.
        pub fn nav_items() -> Vec<NavItem<$name>> {
            vec![
                $( NavItem::new($name::$variant, $label), )+
            ]
        }
    };
}
