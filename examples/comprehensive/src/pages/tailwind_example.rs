use crate::demo_popup::DemoBox;
use crate::doc_page::CopyCode;
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::NavLink;

const CODE_SIDEBAR: &str = r#"use yew::prelude::*;
use yew_nav_link::NavLink;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Dashboard,
    #[at("/analytics")]
    Analytics,
    #[at("/settings")]
    Settings,
}

#[component]
fn Sidebar() -> Html {
    html! {
        <aside class="sidebar-dark">
            <div class="sidebar-brand">{ "Dashboard" }</div>
            <nav>
                <NavLink<Route> to={Route::Dashboard}>{ "Dashboard" }</NavLink<Route>>
                <NavLink<Route> to={Route::Analytics}>{ "Analytics" }</NavLink<Route>>
                <NavLink<Route> to={Route::Settings}>{ "Settings" }</NavLink<Route>>
            </nav>
        </aside>
    }
}"#;

const CODE_CSS: &str = r#"/* Tailwind CSS with @apply */
.nav-link {
    @apply px-4 py-2 text-gray-600 hover:text-gray-900;
}
.nav-link.active {
    @apply text-blue-600 font-semibold;
}

/* Or with inline styles */
.sidebar-dark {
    @apply bg-gray-800 text-white w-64 min-h-screen p-4;
}
.sidebar-dark .nav-link {
    @apply block rounded-md text-gray-300 hover:bg-gray-700;
}
.sidebar-dark .nav-link.active {
    @apply bg-gray-900 text-white;
}"#;

const CODE_CONTENT: &str = r#"fn switch(route: Route) -> Html {
    match route {
        Route::Dashboard => html! {
            <div class="grid grid-cols-3 gap-4">
                <div class="card">
                    <h3>{ "Total Users" }</h3>
                    <p class="stat">{ "1,234" }</p>
                </div>
                <div class="card">
                    <h3>{ "Revenue" }</h3>
                    <p class="stat">{ "$12,345" }</p>
                </div>
                <div class="card">
                    <h3>{ "Orders" }</h3>
                    <p class="stat">{ "567" }</p>
                </div>
            </div>
        },
        // ... other routes
    }
}"#;

#[function_component]
pub fn TailwindExample() -> Html {
    html! {
        <div>
            <div class="card">
                <h2>{ "Tailwind CSS Example" }</h2>
                <p>
                    { "Style NavLink with Tailwind CSS using " }<code>{ "@apply" }</code>
                    { " directives or custom CSS. The " }<code>{ "nav-link" }</code>
                    { " and " }<code>{ "active" }</code>
                    { " classes map directly to Tailwind utilities." }
                </p>
                <div class="example-meta">
                    <span class="tag tag-yew"><span class="tag-label">{ "source" }</span><span class="tag-value">{ "examples/tailwind/" }</span></span>
                </div>
            </div>

            <div class="card">
                <h3>{ "1. Sidebar Navigation" }</h3>
                <p>
                    { "A dark sidebar with NavLink items. Active links get a darker background." }
                </p>
                <CopyCode code={CODE_SIDEBAR.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <div class="tailwind-sidebar-demo">
                        <div class="tailwind-sidebar">
                            <div class="tailwind-sidebar-brand">{ "Dashboard" }</div>
                            <nav>
                                <NavLink<Route> to={Route::Home}>{ "Dashboard" }</NavLink<Route>>
                                <NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>>
                                <NavLink<Route> to={Route::TailwindExample}>{ "Tailwind" }</NavLink<Route>>
                            </nav>
                        </div>
                    </div>
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "2. CSS Styling" }</h3>
                <p>
                    { "Define " }<code>{ ".nav-link" }</code>
                    { " and " }<code>{ ".nav-link.active" }</code>
                    { " with Tailwind " }<code>{ "@apply" }</code>
                    { " directives." }
                </p>
                <CopyCode code={CODE_CSS.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <div class="tailwind-css-demo">
                        <div class="tailwind-css-preview">
                            <div class="tailwind-css-link">{ "Default: px-4 py-2 text-gray-600" }</div>
                            <div class="tailwind-css-link tailwind-css-link-active">{ "Active: text-blue-600 font-semibold" }</div>
                            <div class="tailwind-css-link">{ "Hover: hover:text-gray-900" }</div>
                        </div>
                    </div>
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "3. Dashboard Content" }</h3>
                <p>
                    { "Page content uses Tailwind utility classes for cards and grid layouts." }
                </p>
                <CopyCode code={CODE_CONTENT.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <div class="tailwind-grid-demo">
                        <div class="tailwind-grid-card">
                            <h4>{ "Total Users" }</h4>
                            <p class="tailwind-stat">{ "1,234" }</p>
                        </div>
                        <div class="tailwind-grid-card">
                            <h4>{ "Revenue" }</h4>
                            <p class="tailwind-stat">{ "$12,345" }</p>
                        </div>
                        <div class="tailwind-grid-card">
                            <h4>{ "Orders" }</h4>
                            <p class="tailwind-stat">{ "567" }</p>
                        </div>
                    </div>
                </DemoBox>
            </div>
        </div>
    }
}
