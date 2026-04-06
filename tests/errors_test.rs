#![cfg(test)]

use yew_nav_link::{NavError, NavResult};

#[test]
fn test_route_not_found() {
    let err = NavError::route_not_found();
    assert_eq!(err, NavError::RouteNotFound);
}

#[test]
fn test_invalid_route() {
    let err = NavError::invalid_route("Invalid path");
    assert_eq!(err, NavError::InvalidRoute("Invalid path".to_string()));
}

#[test]
fn test_navigation_cancelled() {
    let err = NavError::navigation_cancelled();
    assert_eq!(err, NavError::NavigationCancelled);
}

#[test]
fn test_route_not_found_factory() {
    let err = NavError::route_not_found();
    match err {
        NavError::RouteNotFound => {}
        _ => panic!("Expected RouteNotFound")
    }
}

#[test]
fn test_invalid_route_factory() {
    let err = NavError::invalid_route("test error message");
    match err {
        NavError::InvalidRoute(msg) => assert_eq!(msg, "test error message"),
        _ => panic!("Expected InvalidRoute")
    }
}

#[test]
fn test_navigation_cancelled_factory() {
    let err = NavError::navigation_cancelled();
    match err {
        NavError::NavigationCancelled => {}
        _ => panic!("Expected NavigationCancelled")
    }
}

#[test]
fn test_nav_error_clone() {
    let err1 = NavError::invalid_route("test");
    let err2 = err1.clone();
    assert_eq!(err1, err2);
}

#[test]
fn test_nav_error_debug() {
    let err = NavError::RouteNotFound;
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("RouteNotFound"));
}

#[test]
fn test_nav_error_partial_eq() {
    let err1 = NavError::invalid_route("test");
    let err2 = NavError::invalid_route("test");
    let err3 = NavError::invalid_route("other");

    assert_eq!(err1, err2);
    assert_ne!(err1, err3);
}

#[test]
fn test_nav_result_ok() {
    let result: NavResult<i32> = Ok(42);
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_nav_result_err() {
    let result: NavResult<i32> = Err(NavError::route_not_found());
    assert!(result.is_err());
}

#[test]
fn test_nav_result_into() {
    let result: NavResult<i32> = Err(NavError::invalid_route("error"));
    let err = result.unwrap_err();
    assert!(matches!(err, NavError::InvalidRoute(_)));
}
