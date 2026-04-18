use std::marker::PhantomData;

use yew::prelude::*;
use yew_router::prelude::*;

#[cfg(target_arch = "wasm32")]
use yew_router::history::{BrowserHistory, History};

#[cfg(target_arch = "wasm32")]
fn history_push(path: &str) {
    BrowserHistory::new().push(path);
}

#[cfg(not(target_arch = "wasm32"))]
fn history_push(_path: &str) {}

#[cfg(target_arch = "wasm32")]
fn history_replace(path: &str) {
    BrowserHistory::new().replace(path);
}

#[cfg(not(target_arch = "wasm32"))]
fn history_replace(_path: &str) {}

#[cfg(target_arch = "wasm32")]
fn history_go(delta: isize) {
    BrowserHistory::new().go(delta);
}

#[cfg(not(target_arch = "wasm32"))]
fn history_go(_delta: isize) {}

#[cfg(target_arch = "wasm32")]
fn history_back() {
    BrowserHistory::new().back();
}

#[cfg(not(target_arch = "wasm32"))]
fn history_back() {}

#[cfg(target_arch = "wasm32")]
fn history_forward() {
    BrowserHistory::new().forward();
}

#[cfg(not(target_arch = "wasm32"))]
fn history_forward() {}

fn route_path<R>(route: &R) -> String
where
    R: Routable + Clone + 'static,
{
    route.to_path()
}

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
    R: Routable + Clone + 'static,
{
    /// Callback to navigate back in history.
    pub go_back: Callback<()>,
    /// Callback to navigate forward in history.
    pub go_forward: Callback<()>,
    /// Phantom marker for the route type.
    pub _marker: PhantomData<R>,
}

impl<R> Navigation<R>
where
    R: Routable + Clone + 'static,
{
    /// Create a callback for pushing a route onto history.
    pub fn push_callback(&self, route: R) -> Callback<()> {
        Callback::from(move |_| {
            let path = route_path(&route);
            history_push(&path);
        })
    }

    /// Create a callback for replacing the current route.
    pub fn replace_callback(&self, route: R) -> Callback<()> {
        Callback::from(move |_| {
            let path = route_path(&route);
            history_replace(&path);
        })
    }

    /// Create a callback for navigating with a delta.
    pub fn go_callback(&self, delta: isize) -> Callback<()> {
        Callback::from(move |_| {
            history_go(delta);
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
    R: Routable + Clone + 'static,
{
    let go_back = Callback::from(|_| {
        history_back();
    });

    let go_forward = Callback::from(|_| {
        history_forward();
    });

    Navigation {
        go_back,
        go_forward,
        _marker: PhantomData,
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
            Home,
        }

        let nav = Navigation::<TestRoute> {
            go_back: Callback::from(|_| {}),
            go_forward: Callback::from(|_| {}),
            _marker: PhantomData,
        };

        let _ = nav.go_back;
        let _ = nav.go_forward;
    }

    #[test]
    fn navigation_clone() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/test")]
            Test,
        }

        let nav1 = Navigation::<TestRoute> {
            go_back: Callback::from(|_| {}),
            go_forward: Callback::from(|_| {}),
            _marker: PhantomData,
        };

        let nav2 = nav1.clone();
        let _ = nav2.go_back;
        let _ = nav2.go_forward;
    }

    #[test]
    fn navigation_debug() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/test")]
            Test,
        }

        let nav = Navigation::<TestRoute> {
            go_back: Callback::from(|_| {}),
            go_forward: Callback::from(|_| {}),
            _marker: PhantomData,
        };

        let debug_str = format!("{:?}", nav);
        assert!(debug_str.contains("Navigation"));
    }

    #[test]
    fn route_path_uses_routable_path() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home,
        }

        assert_eq!(super::route_path(&TestRoute::Home), "/");
    }

    #[test]
    fn navigation_callbacks_invoke_without_panics() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/")]
            Home,
        }

        let nav = Navigation::<TestRoute> {
            go_back: Callback::from(|_| {}),
            go_forward: Callback::from(|_| {}),
            _marker: PhantomData,
        };

        nav.push_callback(TestRoute::Home).emit(());
        nav.replace_callback(TestRoute::Home).emit(());
        nav.go_callback(1).emit(());
    }

    #[test]
    fn history_helpers_are_callable() {
        super::history_back();
        super::history_forward();
        super::history_go(-1);
        super::history_push("/");
        super::history_replace("/");
    }
}
