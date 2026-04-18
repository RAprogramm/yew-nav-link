use std::marker::PhantomData;

use yew::prelude::*;
use yew_router::{
    history::{BrowserHistory, History},
    prelude::*
};

/// Navigation callbacks for programmatic route manipulation.
///
/// Provides pre-built callbacks for:
/// - Pushing new routes onto history
/// - Replacing current route
/// - Navigating back/forward
///
/// This struct is created by [`use_navigation`] and contains all the
/// callbacks needed for navigation without storing state.
#[derive(Clone, Debug)]
pub struct Navigation<R>
where
    R: Routable + Clone + 'static
{
    /// Callback to navigate back in history.
    pub go_back:    Callback<()>,
    /// Callback to navigate forward in history.
    pub go_forward: Callback<()>,
    /// Phantom marker for the route type.
    pub _marker:    PhantomData<R>
}

impl<R> Navigation<R>
where
    R: Routable + Clone + 'static
{
    /// Create a callback for pushing a route onto history.
    pub fn push_callback(&self, route: R) -> Callback<()> {
        Callback::from(move |()| {
            let path = route.to_path();
            BrowserHistory::new().push(&path);
        })
    }

    /// Create a callback for replacing the current route.
    pub fn replace_callback(&self, route: R) -> Callback<()> {
        Callback::from(move |()| {
            let path = route.to_path();
            BrowserHistory::new().replace(&path);
        })
    }

    #[must_use]
    /// Create a callback for navigating with a delta.
    pub fn go_callback(&self, delta: isize) -> Callback<()> {
        Callback::from(move |()| {
            BrowserHistory::new().go(delta);
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
    let go_back = Callback::from(|()| {
        BrowserHistory::new().back();
    });

    let go_forward = Callback::from(|()| {
        BrowserHistory::new().forward();
    });

    Navigation {
        go_back,
        go_forward,
        _marker: PhantomData
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
            _marker:    PhantomData
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
            _marker:    PhantomData
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
            _marker:    PhantomData
        };

        let debug_str = format!("{nav:?}");
        assert!(debug_str.contains("Navigation"));
    }
}
