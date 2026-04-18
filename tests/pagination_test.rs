#![cfg(test)]

mod pagination_test {
    use yew::prelude::*;
    use yew_nav_link::components::PaginationProps;

    #[test]
    fn pagination_props_default() {
        let props = PaginationProps::default();
        assert_eq!(props.current_page, 1);
        assert_eq!(props.total_pages, 10);
        assert_eq!(props.siblings, 1);
        assert!(props.show_prev_next);
        assert!(!props.show_first_last);
        assert!(props.on_page_change.is_none());
        assert!(props.classes.is_empty());
    }

    #[test]
    fn pagination_props_custom_values() {
        let on_change = Callback::from(|_: u32| {});
        let props = PaginationProps {
            classes:         Classes::from("custom-pagination"),
            current_page:    5,
            total_pages:     20,
            siblings:        2,
            show_prev_next:  true,
            show_first_last: true,
            on_page_change:  Some(on_change)
        };

        assert_eq!(props.current_page, 5);
        assert_eq!(props.total_pages, 20);
        assert_eq!(props.siblings, 2);
        assert!(props.show_prev_next);
        assert!(props.show_first_last);
        assert!(props.on_page_change.is_some());
        assert!(props.classes.contains("custom-pagination"));
    }

    #[test]
    fn pagination_props_clone() {
        let props1 = PaginationProps {
            classes:         Classes::from("clone-test"),
            current_page:    3,
            total_pages:     15,
            siblings:        1,
            show_prev_next:  true,
            show_first_last: false,
            on_page_change:  None
        };

        let props2 = props1.clone();
        assert_eq!(props1.current_page, props2.current_page);
        assert_eq!(props1.total_pages, props2.total_pages);
    }

    #[test]
    fn pagination_props_debug() {
        let props = PaginationProps::default();
        let debug_str = format!("{props:?}");
        assert!(debug_str.contains("PaginationProps"));
    }

    #[test]
    fn pagination_props_partial_eq_same() {
        let props1 = PaginationProps::default();
        let props2 = PaginationProps::default();
        assert_eq!(props1, props2);
    }

    #[test]
    fn pagination_props_partial_eq_different() {
        let props1 = PaginationProps {
            current_page: 1,
            ..Default::default()
        };
        let props2 = PaginationProps {
            current_page: 2,
            ..Default::default()
        };
        assert_ne!(props1, props2);
    }

    #[test]
    fn page_to_string_normal() {
        let page_num = 5u32;
        assert_eq!(page_num.to_string(), "5");
    }

    #[test]
    fn page_to_string_ellipsis() {
        let page_num: u32 = 0; // 0 represents ellipsis
        assert_eq!(page_num.to_string(), "0");
    }
}
