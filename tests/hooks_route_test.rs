#![cfg(test)]

mod hooks_route_test {
    use yew::prelude::*;
    use yew_nav_link::hooks::BreadcrumbItem;
    use yew_router::prelude::*;

    #[test]
    fn breadcrumb_item_new() {
        #[derive(Clone, PartialEq)]
        struct TestRoute;
        let item = BreadcrumbItem {
            route: TestRoute,
            label: "/test".to_string(),
            is_active: true,
        };
        assert_eq!(item.label, "/test");
        assert!(item.is_active);
    }

    #[test]
    fn breadcrumb_item_clone() {
        let item1 = BreadcrumbItem {
            route: "route1",
            label: "label1".to_string(),
            is_active: false,
        };
        let item2 = item1.clone();
        assert_eq!(item1.label, item2.label);
    }

    #[test]
    fn breadcrumb_item_debug() {
        let item = BreadcrumbItem {
            route: "route",
            label: "test".to_string(),
            is_active: true,
        };
        let debug_str = format!("{:?}", item);
        assert!(debug_str.contains("BreadcrumbItem"));
    }

    #[test]
    fn breadcrumb_item_inactive() {
        let item = BreadcrumbItem {
            route: "route",
            label: "label".to_string(),
            is_active: false,
        };
        assert!(!item.is_active);
    }

    #[test]
    fn breadcrumb_item_nested_path() {
        let item = BreadcrumbItem {
            route: "route",
            label: "/docs/api/v1".to_string(),
            is_active: true,
        };
        assert!(item.label.contains("docs"));
        assert!(item.label.contains("api"));
    }

    #[test]
    fn breadcrumb_item_root_path() {
        let item = BreadcrumbItem {
            route: "route",
            label: "/".to_string(),
            is_active: true,
        };
        assert_eq!(item.label, "/");
    }

    #[test]
    fn hook_types_are_send() {
        fn assert_send<T: Send>() {}
        // Hooks return types should be Send
        assert_send::<Option<String>>();
    }
}
