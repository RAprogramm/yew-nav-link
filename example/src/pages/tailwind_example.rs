use yew::prelude::*;
use yew_nav_link::NavLink;

use crate::{code_utils::CopyCode, demo_popup::DemoBox, routes::Route};

// ─── Dashboard Stat Data ───────────────────────────────────────────

#[derive(Clone, PartialEq, Debug)]
struct DashboardStat {
    title: String,
    value: String,
    icon: String,
    color: String,
}

impl DashboardStat {
    fn new(title: &str, value: &str, icon: &str, color: &str) -> Self {
        DashboardStat {
            title: title.to_string(),
            value: value.to_string(),
            icon: icon.to_string(),
            color: color.to_string(),
        }
    }
}

fn default_stats() -> Vec<DashboardStat> {
    vec![
        DashboardStat::new("Total Users", "1,234", "\u{1F465}", "var(--accent)"),
        DashboardStat::new("Revenue", "$12,345", "\u{1F4B0}", "#10b981"),
        DashboardStat::new("Orders", "567", "\u{1F4E6}", "#f59e0b"),
        DashboardStat::new("Conversion", "3.2%", "\u{1F4C8}", "#8b5cf6"),
    ]
}

// ─── Stat Card Component ───────────────────────────────────────────

#[derive(Properties, PartialEq)]
struct StatCardProps {
    stat: DashboardStat,
}

#[function_component]
fn StatCard(props: &StatCardProps) -> Html {
    let stat = props.stat.clone();

    html! {
        <div class="tailwind-grid-card">
            <h4 style={format!("color: {}", stat.color)}>{ stat.title }</h4>
            <p class="tailwind-stat">{ stat.value }</p>
        </div>
    }
}

// ─── Tailwind Examples ─────────────────────────────────────────────

#[derive(Clone, PartialEq)]
enum SidebarTheme {
    Dark,
    Light,
}

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
            <div class="grid grid-cols-4 gap-4">
                <StatCard title="Total Users" value={state.users} />
                <StatCard title="Revenue" value={state.revenue} />
                <StatCard title="Orders" value={state.orders} />
                <StatCard title="Conversion" value={state.conversion} />
            </div>
        },
        // ... other routes
    }
}"#;

#[function_component]
pub fn TailwindExample() -> Html {
    let stats = use_state(default_stats);

    let on_simulate_updates = {
        let stats = stats.clone();
        Callback::from(move |_| {
            // Simulate live data updates (values would come from API in production)
            static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let idx = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % 4;

            let samples: [(u64, u64, u64, f64); 4] = [
                (1234, 12345, 567, 2.8),
                (1456, 15892, 623, 3.5),
                (1678, 18234, 712, 4.1),
                (1892, 21567, 789, 3.8),
            ];
            let sample = samples[idx];
            let current = (*stats).clone();
            let mut updated = current.clone();
            updated[0].value = format!("{}", sample.0);
            updated[1].value = format!("${}", sample.1);
            updated[2].value = format!("{}", sample.2);
            updated[3].value = format!("{:.1}%", sample.3);
            stats.set(updated);
        })
    };

    let stats_clone = (*stats).clone();

    html! {
        <div>
            // ── Header ────────────────────────────────────────
            <div class="card">
                <h2>{ "Tailwind CSS Example" }</h2>
                <p>
                    { "Style NavLink with Tailwind CSS using " }<code>{ "@apply" }</code>
                    { " directives or custom CSS. The " }<code>{ "nav-link" }</code>
                    { " and " }<code>{ "active" }</code>
                    { " classes map directly to Tailwind utility classes." }
                </p>
                <div class="example-meta">
                    <span class="tag tag-yew"><span class="tag-label">{ "source" }</span><span class="tag-value">{ "examples/tailwind/" }</span></span>
                </div>
            </div>

            // ── Sidebar Navigation ────────────────────────────
            <div class="card">
                <h3>{ "1. Sidebar Navigation" }</h3>
                <p>
                    { "A dark sidebar with NavLink items. Active links get a darker background. " }
                    { "The sidebar demonstrates NavLink with dark theme styling." }
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

            // ── CSS Styling ───────────────────────────────────
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

            // ── Dashboard Content ─────────────────────────────
            <div class="card">
                <h3>{ "3. Dashboard Content" }</h3>
                <p>
                    { "Page content uses Tailwind utility classes for cards and grid layouts. " }
                    { "Dashboard stat values are configurable state, not hardcoded strings. " }
                    { "Click the button below to simulate live data updates." }
                </p>
                <CopyCode code={CODE_CONTENT.to_string()} />

                <h3>{ "Live Result" }</h3>
                <div style="margin-bottom: 0.75rem;">
                    <button
                        class="btn-demo btn-accent"
                        onclick={on_simulate_updates}
                    >
                        { "\u{1F504} Simulate Live Data" }
                    </button>
                </div>
                <DemoBox>
                    <div class="tailwind-grid-demo">
                        { for stats_clone.iter().map(|stat| {
                            html! { <StatCard stat={stat.clone()} /> }
                        })}
                    </div>
                </DemoBox>
            </div>
        </div>
    }
}
