// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Programmatic navigation built on top of yew-router's `Navigator`.
//!
//! [`use_navigation`] returns a [`Navigation<R>`] handle. `go_back` and
//! `go_forward` are ready-made `Callback<()>` values; `push_callback`,
//! `replace_callback`, and `go_callback` build a `Callback<()>` from an
//! argument. Every one routes through the router's [`Navigator`], so the
//! configured basename is honored — the same path [`NavLink`](crate::NavLink)
//! takes.

use std::marker::PhantomData;

use yew::prelude::*;
use yew_router::prelude::*;

/// Handle for programmatic route manipulation, created by [`use_navigation`].
///
/// `go_back` and `go_forward` are ready-made callbacks. `push_callback`,
/// `replace_callback`, and `go_callback` build a callback from an argument.
/// All of them route through the captured [`Navigator`], so the router
/// basename is honored.
#[derive(Clone, Debug)]
pub struct Navigation<R>
where
    R: Routable + Clone + 'static
{
    navigator:      Option<Navigator>,
    /// Callback to navigate back in history.
    pub go_back:    Callback<()>,
    /// Callback to navigate forward in history.
    pub go_forward: Callback<()>,
    marker:         PhantomData<R>
}

impl<R> Navigation<R>
where
    R: Routable + Clone + 'static
{
    /// Create a callback that pushes `route` onto the history stack.
    ///
    /// The push goes through the router's [`Navigator`], so the configured
    /// basename is prepended.
    pub fn push_callback(&self, route: R) -> Callback<()> {
        let navigator = self.navigator.clone();
        Callback::from(move |()| {
            if let Some(navigator) = &navigator {
                navigator.push(&route);
            }
        })
    }

    /// Create a callback that replaces the current history entry with `route`.
    ///
    /// The replace goes through the router's [`Navigator`], so the configured
    /// basename is prepended.
    pub fn replace_callback(&self, route: R) -> Callback<()> {
        let navigator = self.navigator.clone();
        Callback::from(move |()| {
            if let Some(navigator) = &navigator {
                navigator.replace(&route);
            }
        })
    }

    /// Create a callback that moves `delta` entries through history.
    #[must_use]
    pub fn go_callback(&self, delta: isize) -> Callback<()> {
        let navigator = self.navigator.clone();
        Callback::from(move |()| {
            if let Some(navigator) = &navigator {
                navigator.go(delta);
            }
        })
    }
}

/// Returns a [`Navigation`] handle for programmatic navigation.
///
/// ```rust,ignore
/// use yew::prelude::*;
/// use yew_nav_link::hooks::use_navigation;
/// use yew_router::prelude::*;
///
/// #[derive(Clone, PartialEq, Debug, Routable)]
/// enum Route {
///     #[at("/")]
///     Home
/// }
///
/// #[component]
/// fn MyComponent() -> Html {
///     let navigation = use_navigation::<Route>();
///
///     html! {
///         <div>
///             <button onclick={navigation.go_back.clone()}>Back</button>
///             <button onclick={navigation.go_forward.clone()}>Forward</button>
///             <button onclick={navigation.push_callback(Route::Home)}>
///                 { "Go Home" }
///             </button>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_navigation<R>() -> Navigation<R>
where
    R: Routable + Clone + 'static
{
    let navigator = use_navigator();

    let go_back = {
        let navigator = navigator.clone();
        Callback::from(move |()| {
            if let Some(navigator) = &navigator {
                navigator.back();
            }
        })
    };

    let go_forward = {
        let navigator = navigator.clone();
        Callback::from(move |()| {
            if let Some(navigator) = &navigator {
                navigator.forward();
            }
        })
    };

    Navigation {
        navigator,
        go_back,
        go_forward,
        marker: PhantomData
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use super::*;

    #[test]
    fn navigation_struct_creation() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/test")]
            Test,
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let _ = nav.go_back;
        let _ = nav.go_forward;
    }

    #[test]
    fn navigation_clone() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/test")]
            Test
        }

        let nav1 = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let nav2 = nav1;
        let _ = nav2.go_back;
        let _ = nav2.go_forward;
    }

    #[test]
    fn navigation_debug() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/test")]
            Test
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let debug_str = format!("{nav:?}");
        assert!(debug_str.contains("Navigation"));
    }

    #[test]
    fn navigation_partial_eq() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/test")]
            Test
        }

        let nav1 = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };
        let nav2 = nav1;
        let _ = nav1.go_back;
        let _ = nav2.go_back;
    }

    #[test]
    fn navigation_push_callback() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };
        let _ = nav.push_callback(TestRoute::Home);
    }

    #[test]
    fn navigation_replace_callback() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };
        let _ = nav.replace_callback(TestRoute::Home);
    }

    #[test]
    fn navigation_go_callback() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };
        let _ = nav.go_callback(-1);
    }

    #[test]
    fn navigation_go_callback_with_positive_delta() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };
        let callback = nav.go_callback(1);
        let _ = callback;
    }

    #[test]
    fn navigation_go_callback_with_zero_delta() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };
        let callback = nav.go_callback(0);
        let _ = callback;
    }

    #[test]
    fn navigation_go_callback_with_large_negative_delta() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };
        let callback = nav.go_callback(-10);
        let _ = callback;
    }

    #[test]
    fn navigation_go_callback_with_large_positive_delta() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };
        let callback = nav.go_callback(10);
        let _ = callback;
    }

    #[test]
    fn navigation_push_callback_with_struct_route() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/users/:id")]
            User { id: String }
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let route = TestRoute::User {
            id: "123".to_string()
        };
        let callback = nav.push_callback(route);
        let _ = callback;
    }

    #[test]
    fn navigation_push_callback_with_tuple_route() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/posts/:year")]
            Post { year: u32 }
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let route = TestRoute::Post {
            year: 2024
        };
        let callback = nav.push_callback(route);
        let _ = callback;
    }

    #[test]
    fn navigation_replace_callback_with_struct_route() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/settings/:section")]
            Settings { section: String }
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let route = TestRoute::Settings {
            section: "profile".to_string()
        };
        let callback = nav.replace_callback(route);
        let _ = callback;
    }

    #[test]
    fn navigation_replace_callback_with_tuple_route() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/api/:version")]
            Api { version: u32 }
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let route = TestRoute::Api {
            version: 1
        };
        let callback = nav.replace_callback(route);
        let _ = callback;
    }

    #[test]
    fn navigation_callbacks_are_distinct() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home,
            #[at("/about")]
            About
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let push_callback = nav.push_callback(TestRoute::Home);
        let replace_callback = nav.replace_callback(TestRoute::About);
        let go_callback = nav.go_callback(-1);

        let _ = push_callback;
        let _ = replace_callback;
        let _ = go_callback;
    }

    #[test]
    fn navigation_multiple_callbacks_same_route() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let callback1 = nav.push_callback(TestRoute::Home);
        let callback2 = nav.push_callback(TestRoute::Home);
        let callback3 = nav.push_callback(TestRoute::Home);

        let _ = callback1;
        let _ = callback2;
        let _ = callback3;
    }

    #[test]
    fn navigation_callbacks_with_complex_route() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum ComplexRoute {
            #[at("/")]
            Home,
            #[at("/users")]
            Users,
            #[at("/admin")]
            Admin,
            #[at("/date")]
            Date
        }

        let nav = Navigation::<ComplexRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let _ = nav.push_callback(ComplexRoute::Home);
        let _ = nav.push_callback(ComplexRoute::Users);
        let _ = nav.push_callback(ComplexRoute::Admin);
        let _ = nav.push_callback(ComplexRoute::Date);
    }

    #[test]
    fn navigation_callbacks_preserve_route_data() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/search/:query")]
            Search { query: String }
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let query = "rust programming".to_string();
        let route = TestRoute::Search {
            query: query.clone()
        };
        let _callback = nav.push_callback(route);

        assert_eq!(query, "rust programming");
    }

    #[test]
    fn navigation_go_back_callback_exists() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let _ = nav.go_back;
    }

    #[test]
    fn navigation_go_forward_callback_exists() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let _ = nav.go_forward;
    }

    #[test]
    fn navigation_struct_with_multiple_variants() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum MultiVariantRoute {
            #[at("/")]
            Home,
            #[at("/about")]
            About,
            #[at("/contact")]
            Contact,
            #[at("/products")]
            Products,
            #[at("/services")]
            Services,
            #[at("/blog")]
            Blog,
            #[at("/faq")]
            Faq
        }

        let nav = Navigation::<MultiVariantRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let _ = nav.push_callback(MultiVariantRoute::Home);
        let _ = nav.push_callback(MultiVariantRoute::About);
        let _ = nav.push_callback(MultiVariantRoute::Contact);
        let _ = nav.push_callback(MultiVariantRoute::Products);
        let _ = nav.push_callback(MultiVariantRoute::Services);
        let _ = nav.push_callback(MultiVariantRoute::Blog);
        let _ = nav.push_callback(MultiVariantRoute::Faq);
    }

    #[test]
    fn navigation_go_callback_edge_cases() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let min_delta = nav.go_callback(isize::MIN);
        let max_delta = nav.go_callback(isize::MAX);
        let neg_one = nav.go_callback(-1);
        let pos_one = nav.go_callback(1);

        let _ = min_delta;
        let _ = max_delta;
        let _ = neg_one;
        let _ = pos_one;
    }

    #[test]
    fn navigation_callbacks_with_empty_string_route() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/item/:name")]
            Item { name: String }
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let route = TestRoute::Item {
            name: String::new()
        };
        let callback = nav.push_callback(route);
        let _ = callback;
    }

    #[test]
    fn navigation_callbacks_with_special_characters() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/tag/:name")]
            Tag { name: String }
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let route = TestRoute::Tag {
            name: "rust-lang".to_string()
        };
        let callback = nav.push_callback(route);
        let _ = callback;
    }

    #[test]
    fn navigation_callbacks_with_unicode() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/emoji/:emoji")]
            Emoji { emoji: String }
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let route = TestRoute::Emoji {
            emoji: "🦀".to_string()
        };
        let callback = nav.push_callback(route);
        let _ = callback;
    }

    #[test]
    fn navigation_callbacks_with_numbers() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/number/:num")]
            Number { num: u64 }
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let route = TestRoute::Number {
            num: 0
        };
        let callback = nav.push_callback(route);
        let _ = callback;

        let route = TestRoute::Number {
            num: u64::MAX
        };
        let callback = nav.push_callback(route);
        let _ = callback;
    }

    #[test]
    fn navigation_struct_debug_format() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let debug_str = format!("{nav:?}");
        assert!(debug_str.contains("Navigation"));
        assert!(debug_str.contains("go_back"));
        assert!(debug_str.contains("go_forward"));
    }

    #[test]
    fn navigation_callbacks_chainability() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home,
            #[at("/step1")]
            Step1,
            #[at("/step2")]
            Step2,
            #[at("/step3")]
            Step3
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let step1_callback = nav.push_callback(TestRoute::Step1);
        let step2_callback = nav.push_callback(TestRoute::Step2);
        let step3_callback = nav.push_callback(TestRoute::Step3);
        let back_callback = nav.go_callback(-1);

        let _ = step1_callback;
        let _ = step2_callback;
        let _ = step3_callback;
        let _ = back_callback;
    }

    #[test]
    fn navigation_replace_vs_push_same_route() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let push_cb = nav.push_callback(TestRoute::Home);
        let replace_cb = nav.replace_callback(TestRoute::Home);

        let _ = push_cb;
        let _ = replace_cb;
    }

    #[test]
    fn navigation_callbacks_with_nested_route_params() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/org/:org_id/team/:team_id/member/:member_id")]
            Member {
                org_id:    String,
                team_id:   String,
                member_id: String
            }
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let route = TestRoute::Member {
            org_id:    "org1".to_string(),
            team_id:   "team1".to_string(),
            member_id: "member1".to_string()
        };
        let callback = nav.push_callback(route);
        let _ = callback;
    }

    #[test]
    fn navigation_callbacks_with_optional_like_params() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/filter/:min/:max")]
            Filter { min: u32, max: u32 }
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let route = TestRoute::Filter {
            min: 0, max: 100
        };
        let callback = nav.push_callback(route);
        let _ = callback;
    }

    #[test]
    fn navigation_go_callback_returns_valid_callback() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let callback = nav.go_callback(-5);
        let _ = callback;
    }

    #[test]
    fn navigation_struct_fields_accessible() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let _nav_back = &nav.go_back;
        let _nav_forward = &nav.go_forward;
        let _ = nav.marker;
    }

    #[test]
    fn navigation_with_multiple_routable_types() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum Route1 {
            #[at("/")]
            Home
        }

        #[derive(Clone, PartialEq, Debug, Routable)]
        enum Route2 {
            #[at("/")]
            Page
        }

        let nav1 = Navigation::<Route1> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let nav2 = Navigation::<Route2> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let _ = nav1.push_callback(Route1::Home);
        let _ = nav2.push_callback(Route2::Page);
    }

    #[test]
    fn navigation_push_callback_multiple_times() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let _ = nav.push_callback(TestRoute::Home);
        let _ = nav.push_callback(TestRoute::Home);
        let _ = nav.push_callback(TestRoute::Home);
    }

    #[test]
    fn navigation_replace_callback_multiple_times() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home
        }

        let nav = Navigation::<TestRoute> {
            go_back:    Callback::from(|()| {}),
            go_forward: Callback::from(|()| {}),
            navigator:  None,
            marker:     PhantomData
        };

        let _ = nav.replace_callback(TestRoute::Home);
        let _ = nav.replace_callback(TestRoute::Home);
        let _ = nav.replace_callback(TestRoute::Home);
    }
}
