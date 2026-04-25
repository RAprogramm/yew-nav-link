use yew::prelude::*;
use yew_nav_link::hooks::use_query_params;
use yew_router::prelude::*;

use crate::{
    doc_page::{DemoCard, DocPage, Tip},
    routes::Route
};

const NAVIGATION_HOOKS_SRC: &str = include_str!("../../../src/hooks/navigation/use_navigation.rs");

const CODE_NAVIGATION: &str = r#"
use yew_nav_link::hooks::use_navigation;

#[component]
fn MyComponent() -> Html {
    let navigation = use_navigation::<Route>();
    
    html! {
        <>
            <button onclick={navigation.push_callback(Route::About)}>
                { "Go to About" }
            </button>
            <button onclick={navigation.replace_callback(Route::Home)}>
                { "Replace with Home" }
            </button>
            <button onclick={navigation.go_back.clone()}>
                { "Back" }
            </button>
            <button onclick={navigation.go_forward.clone()}>
                { "Forward" }
            </button>
        </>
    }
}
"#;

const CODE_QUERY_PARAMS: &str = r#"
use yew_nav_link::hooks::use_query_params;

#[component]
fn SearchResults() -> Html {
    let query_params = use_query_params();
    let search_query = query_params.get_one("q").unwrap_or("");
    let page = query_params.get_one("page").and_then(|p| p.parse::<i32>().ok()).unwrap_or(1);
    
    html! {
        <div>
            <h1>{ format!("Search Results for: {}", search_query) }</h1>
            <p>{ format!("Page: {}", page) }</p>
        </div>
    }
}
"#;

#[function_component]
fn NavigationDemo() -> Html {
    let route = use_route::<Route>();

    let current_route_name = match route.as_ref() {
        Some(r) => format!("{:?}", r),
        None => "Unknown".to_string()
    };

    html! {
        <div class="navigation-demo">
            <div class="demo-info">
                <h5>{ "Current Route:" }</h5>
                <code>{ current_route_name }</code>
            </div>

            <div class="demo-buttons">
                <Link<Route> to={Route::Home} classes="demo-btn">
                    { "Go to Home" }
                </Link<Route>>
                <Link<Route> to={Route::NavigationHooks} classes="demo-btn">
                    { "Go to Navigation Hooks" }
                </Link<Route>>
            </div>
            <p style="margin-top: 1rem; font-size: 0.75rem; color: var(--text-muted);">
                { "Use browser back/forward buttons to test history navigation." }
            </p>
        </div>
    }
}

#[function_component]
fn QueryParamsDemo() -> Html {
    let _query_params = use_query_params();
    let route = use_route::<Route>();

    let params_display: Html = if let Some(rp) = route.as_ref() {
        let path = rp.to_path();
        if let Some(query_start) = path.find('?') {
            let query_string = &path[query_start + 1..];
            html! {
                <div class="query-params-display">
                    <h6>{ "Query Parameters:" }</h6>
                    { for query_string.split('&').filter(|s| !s.is_empty()).map(|pair| {
                        let mut parts = pair.splitn(2, '=');
                        let key = parts.next().unwrap_or("");
                        let value = parts.next().unwrap_or("");
                        html! {
                            <span class="query-param-pill">
                                <strong>{ format!("{}:", key) }</strong> { value }
                            </span>
                        }
                    })}
                </div>
            }
        } else {
            html! { <p>{ "No query parameters" }</p> }
        }
    } else {
        html! { <p>{ "No route" }</p> }
    };

    html! {
        <div>
            { params_display }
            <div class="demo-query-links" style="margin-top: 1rem;">
                <Link<Route> to={Route::NavigationHooks} classes="query-link">
                    { "Navigate with ?q=test&page=1" }
                </Link<Route>>
            </div>
        </div>
    }
}

#[function_component]
pub fn NavigationHooks() -> Html {
    html! {
        <DocPage source={NAVIGATION_HOOKS_SRC.to_string()}>

            <div class="alert alert-info" style="margin-bottom: 2rem;">
                <strong>{ "New in v0.8.0" }</strong>
                { " — Programmatic navigation and URL parameter access." }
            </div>

            <DemoCard
                title="use_navigation"
                description={html!{
                    <>
                        { "Navigate programmatically without clicking links:" }
                    </>
                }}
                code={CODE_NAVIGATION.to_string()}
                tip={html!{
                    <Tip>
                        { "Use " }<code>{ "push_callback()" }</code>
                        { " to add to history, " }<code>{ "replace_callback()" }</code>
                        { " to replace current entry." }
                    </Tip>
                }}
            >
                <NavigationDemo />
            </DemoCard>

            <DemoCard
                title="use_query_params"
                description={html!{
                    <>
                        { "Access query string parameters from the URL:" }
                    </>
                }}
                code={CODE_QUERY_PARAMS.to_string()}
                tip={html!{
                    <Tip>
                        { "Parses " }<code>{ "?key=value&foo=bar" }</code>
                        { " into accessible key-value pairs." }
                    </Tip>
                }}
            >
                <QueryParamsDemo />
            </DemoCard>

            <DemoCard
                title="Navigation Methods"
                description={html!{
                    <>
                        { "Available navigation methods:" }
                    </>
                }}
                code={"// Push a new route onto history\nnavigation.push_callback(Route::About)\n\n// Replace current route\nnavigation.replace_callback(Route::Home)\n\n// Navigate back in history\nnavigation.go_back.clone()\n\n// Navigate forward in history\nnavigation.go_forward.clone()".to_string()}
                tip={html! { <></> }}
            >
                <p>{ "The " }<code>{ "Navigation" }</code>{ " struct provides pre-built callbacks for common navigation actions." }</p>
            </DemoCard>

        </DocPage>
    }
}
