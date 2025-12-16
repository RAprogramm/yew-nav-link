//! Tailwind CSS integration example for yew-nav-link.
//!
//! Shows how to style NavLink using Tailwind CSS classes.
//! The `nav-link` and `active` classes can be styled with @apply.
//!
//! Run with: `trunk serve` from the examples/tailwind directory.

use yew::prelude::*;
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
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <div class="min-h-screen flex">
                <Sidebar />
                <main class="flex-1 p-8">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

/// Sidebar navigation styled with Tailwind.
#[component]
fn Sidebar() -> Html {
    html! {
        <aside class="w-64 bg-gray-800 min-h-screen p-4">
            <div class="text-white text-xl font-bold mb-8 px-4">{ "Dashboard" }</div>
            <nav class="space-y-2">
                <NavLink<Route> to={Route::Dashboard}>{ "Dashboard" }</NavLink<Route>>
                <NavLink<Route> to={Route::Analytics}>{ "Analytics" }</NavLink<Route>>
                <NavLink<Route> to={Route::Settings}>{ "Settings" }</NavLink<Route>>
            </nav>
            <div class="mt-8 px-4 text-gray-500 text-sm">
                <p>{ "NavLink outputs:" }</p>
                <code class="text-gray-400">{ ".nav-link" }</code>
                <p class="mt-1">{ "When active:" }</p>
                <code class="text-gray-400">{ ".nav-link.active" }</code>
            </div>
        </aside>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Dashboard => html! {
            <div>
                <h1 class="text-3xl font-bold text-gray-800 mb-4">{ "Dashboard" }</h1>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div class="bg-white rounded-xl shadow p-6">
                        <div class="text-gray-500">{ "Users" }</div>
                        <div class="text-3xl font-bold">{ "1,234" }</div>
                    </div>
                    <div class="bg-white rounded-xl shadow p-6">
                        <div class="text-gray-500">{ "Revenue" }</div>
                        <div class="text-3xl font-bold">{ "$12,345" }</div>
                    </div>
                    <div class="bg-white rounded-xl shadow p-6">
                        <div class="text-gray-500">{ "Orders" }</div>
                        <div class="text-3xl font-bold">{ "567" }</div>
                    </div>
                </div>
            </div>
        },
        Route::Analytics => html! {
            <div>
                <h1 class="text-3xl font-bold text-gray-800 mb-4">{ "Analytics" }</h1>
                <div class="bg-white rounded-xl shadow p-6">
                    <p class="text-gray-600">{ "Analytics dashboard content goes here." }</p>
                </div>
            </div>
        },
        Route::Settings => html! {
            <div>
                <h1 class="text-3xl font-bold text-gray-800 mb-4">{ "Settings" }</h1>
                <div class="bg-white rounded-xl shadow p-6">
                    <p class="text-gray-600">{ "Settings page content goes here." }</p>
                </div>
            </div>
        },
        Route::NotFound => html! {
            <div class="text-center py-12">
                <h1 class="text-4xl font-bold text-gray-800">{ "404" }</h1>
                <p class="text-gray-600 mt-2">{ "Page not found" }</p>
            </div>
        },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
