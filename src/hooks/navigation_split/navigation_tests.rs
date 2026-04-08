#[cfg(test)]
mod tests {
    use super::*;
    use yew_router::prelude::*;

    #[test]
    fn navigation_callbacks_creation() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/test")]
            Test,
            #[at("/")]
            Home,
        }

        // Test that callbacks can be created correctly
        let push_cb = Callback::from(move |_: ()| {
            let path = TestRoute::Test.to_path();
            BrowserHistory::new().push(&path);
        });

        let replace_cb = Callback::from(move |_: ()| {
            let path = TestRoute::Test.to_path();
            BrowserHistory::new().replace(&path);
        });

        let go_cb = Callback::from(|_: ()| {
            BrowserHistory::new().go(-1);
        });

        // Verify callbacks exist
        let _ = push_cb;
        let _ = replace_cb;
        let _ = go_cb;
    }

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

        // Verify navigation callbacks exist
        let _ = nav.go_back;
        let _ = nav.go_forward;
    }

    #[test]
    fn navigation_push_callback() {
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

        let push_cb = nav.push_callback(TestRoute::Home);
        let _ = push_cb;

        let push_cb2 = nav.push_callback(TestRoute::Test);
        let _ = push_cb2;
    }

    #[test]
    fn navigation_replace_callback() {
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

        let replace_cb = nav.replace_callback(TestRoute::Home);
        let _ = replace_cb;

        let replace_cb2 = nav.replace_callback(TestRoute::Test);
        let _ = replace_cb2;
    }

    #[test]
    fn navigation_go_callback() {
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

        let go_back_cb = nav.go_callback(-1);
        let _ = go_back_cb;

        let go_forward_cb = nav.go_callback(1);
        let _ = go_forward_cb;

        let go_custom_cb = nav.go_callback(2);
        let _ = go_custom_cb;
    }

    #[test]
    fn navigation_clone() {
        #[derive(Clone, PartialEq, Debug, Routable)]
        enum TestRoute {
            #[at("/test")]
            Test,
            #[at("/")]
            Home,
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
            #[at("/")]
            Home,
        }

        let nav = Navigation::<TestRoute> {
            go_back: Callback::from(|_| {}),
            go_forward: Callback::from(|_| {}),
            _marker: PhantomData,
        };

        let debug_str = format!("{:?}", nav);
        assert!(debug_str.contains("Navigation"));
    }
}
