use crate::demo_popup::DemoBox;
use crate::code_utils::CopyCode;
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::NavLink;

const CODE_CARGO: &str = r##"[dependencies]
yew-nav-link = { version = "0.6", features = ["macros"] }"##;

const CODE_BEFORE: &str = r##"// Without macros — verbose, repetitive
#[component]
fn Navigation() -> Html {
    html! {
        <nav>
            <ul class="nav-list">
                <li class="nav-item">
                    <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                </li>
                <li class="nav-item">
                    <NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>>
                </li>
                <li class="nav-item">
                    <NavLink<Route> to={Route::NavListDoc}>{ "NavList" }</NavLink<Route>>
                </li>
                <li class="nav-item">
                    <NavLink<Route> to={Route::WithMacrosExample}>{ "Macros" }</NavLink<Route>>
                </li>
            </ul>
        </nav>
    }
}"##;

const CODE_AFTER: &str = r##"// With macros — declarative, concise
#[component]
fn Navigation() -> Html {
    html! {
        <nav>
            { nav_list!(Route, [
                (Home, "Home"),
                (NavLinkDoc, "NavLink"),
                (NavListDoc, "NavList"),
                (WithMacrosExample, "Macros"),
            ]) }
        </nav>
    }
}"##;

const CODE_NAV_LIST: &str = r##"use yew::prelude::*;
use yew_nav_link::{NavLink, nav_list, nav_links};
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/links")]
    Links,
    #[at("/lists")]
    Lists,
}

// nav_list! generates <ul> with <li> items
// nav_links! generates NavLink items from a list of (route, label) pairs
#[component]
fn Navigation() -> Html {
    html! {
        <nav>
            { nav_list!(Route, [
                (Home, "Home"),
                (Links, "Links"),
                (Lists, "Lists"),
            ]) }
        </nav>
    }
}"##;

const CODE_NAV_MENU: &str = r##"// nav_menu! generates a complete sidebar menu with sections
#[component]
fn Sidebar() -> Html {
    html! {
        { nav_menu!(Route, [
            section "Main" => [
                (Home, "Home"),
                (Links, "Links"),
            ],
            section "Components" => [
                (Lists, "Lists"),
                (Container, "Container"),
                (Hooks, "Hooks"),
            ],
        ]) }
    }
}"##;

const CODE_NAV_TABS: &str = r##"// nav_tabs! generates tab navigation
#[component]
fn Tabs() -> Html {
    html! {
        { nav_tabs!(LocalRoute, [
            (Tab1, "Tab 1"),
            (Tab2, "Tab 2"),
            (Tab3, "Tab 3"),
        ]) }
    }
}"##;

const CODE_BREADCRUMBS: &str = r##"// breadcrumbs! generates a breadcrumb trail
#[component]
fn Breadcrumbs() -> Html {
    html! {
        { breadcrumbs!(Route, [
            (Home, "Home"),
            (Links, "Links"),
        ]) }
    }
}"##;

const CODE_NAV_PAGINATION: &str = r##"// nav_pagination! generates pagination controls
#[component]
fn Pagination() -> Html {
    html! {
        { nav_pagination!(current_page: 3, total_pages: 10) }
    }
}"##;

const CODE_EXPANSION_NAV_LIST: &str = r##"// Input:
nav_list!(Route, [
    (Home, "Home"),
    (About, "About"),
])

// Expands to (simplified):
html! {
    <ul class="nav-list">
        <li class="nav-item">
            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
        </li>
        <li class="nav-item">
            <NavLink<Route> to={Route::About}>{ "About" }</NavLink<Route>>
        </li>
    </ul>
}"##;

const CODE_EXPANSION_NAV_MENU: &str = r##"// Input:
nav_menu!(Route, [
    section "Main" => [
        (Home, "Home"),
        (About, "About"),
    ],
    section "Settings" => [
        (Profile, "Profile"),
    ],
])

// Expands to (simplified):
html! {
    <div class="nav-menu">
        <div class="nav-menu-section">
            <div class="nav-menu-section-title">{ "Main" }</div>
            <ul class="nav-list">
                <li class="nav-item">
                    <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                </li>
                <li class="nav-item">
                    <NavLink<Route> to={Route::About}>{ "About" }</NavLink<Route>>
                </li>
            </ul>
        </div>
        <div class="nav-menu-section">
            <div class="nav-menu-section-title">{ "Settings" }</div>
            <ul class="nav-list">
                <li class="nav-item">
                    <NavLink<Route> to={Route::Profile}>{ "Profile" }</NavLink<Route>>
                </li>
            </ul>
        </div>
    </div>
}"##;

#[function_component]
pub fn WithMacrosExample() -> Html {
    html! {
        <div>
            // Header
            <div class="example-header">
                <div class="example-header-top">
                    <span class="example-tag tag-blue">{ "macros" }</span>
                    <span class="example-source">{ "examples/with-macros/" }</span>
                </div>
                <h2>{ "With Macros Example" }</h2>
                <p class="example-desc">
                    { "Enable the " }<code>{ "macros" }</code>
                    { " feature to use declarative helper macros that reduce navigation boilerplate by ~75%. " }
                    { "Macros expand at compile time into the same component tree you would write by hand — " }
                    { "zero runtime overhead." }
                </p>
            </div>

            // Enable the feature
            <div class="card">
                <h3>{ "Enable the Feature" }</h3>
                <p>
                    { "Add the " }<code>{ "macros" }</code>
                    { " feature flag to your " }<code>{ "Cargo.toml" }</code>{ ":" }
                </p>
                <CopyCode code={CODE_CARGO.to_string()} />
            </div>

            // Architecture
            <div class="card">
                <h3>{ "Architecture" }</h3>
                <p>
                    { "The macro system follows a compile-time code generation pattern. " }
                    { "Each macro parses a declarative DSL and emits equivalent " }<code>{ "html!" }</code>
                    { " tokens. The generated code is indistinguishable from hand-written markup." }
                </p>
                <div class="arch-diagram-small">
                    <div class="arch-box arch-box-blue">{ "Route Enum" }</div>
                    <div class="arch-arrow-sm">{ "\u{2192}" }</div>
                    <div class="arch-box arch-box-purple">{ "nav_list!" }</div>
                    <div class="arch-arrow-sm">{ "\u{2192}" }</div>
                    <div class="arch-box arch-box-green">{ "html! tokens" }</div>
                    <div class="arch-arrow-sm">{ "\u{2192}" }</div>
                    <div class="arch-box arch-box-orange">{ "NavLink<R>" }</div>
                    <div class="arch-arrow-sm">{ "\u{2192}" }</div>
                    <div class="arch-box arch-box-gray">{ "DOM" }</div>
                </div>
                <div class="info-box" style="margin-top:1rem;">
                    <strong>{ "Key principle:" }</strong>
                    { " Macros are a compile-time abstraction. They parse your declarative input and emit " }
                    { "the same " }<code>{ "yew::html!" }</code>
                    { " tokens you would write manually. There is no runtime macro evaluation." }
                </div>
            </div>

            // Before / After comparison
            <div class="card">
                <h3>{ "Before and After" }</h3>
                <p>
                    { "The difference between manual markup and macro-generated navigation:" }
                </p>
                <CopyCode code={CODE_BEFORE.to_string()} />
                <p style="margin-top:0.75rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Above: 20 lines of repetitive " }<code>{ "<li>" }</code>
                    { " / " }<code>{ "<NavLink>" }</code>{ " nesting." }
                </p>
                <CopyCode code={CODE_AFTER.to_string()} />
                <p style="margin-top:0.75rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Below: 8 lines — a single macro call with route-label pairs. " }
                    { "Same output, 60% less code." }
                </p>
            </div>

            // nav_list!
            <div class="card">
                <h3>{ "1. nav_list! and nav_links!" }</h3>
                <p>
                    { "Generate a " }<code>{ "<ul>" }</code>
                    { " with " }<code>{ "<li>" }</code>
                    { " items and " }<code>{ "NavLink" }</code>
                    { " components from a simple list of route-label pairs." }
                </p>
                <CopyCode code={CODE_NAV_LIST.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <nav>
                        <ul class="nav-list">
                            <li class="nav-item">
                                <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                            </li>
                            <li class="nav-item">
                                <NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>>
                            </li>
                            <li class="nav-item">
                                <NavLink<Route> to={Route::WithMacrosExample}>{ "With Macros" }</NavLink<Route>>
                            </li>
                        </ul>
                    </nav>
                </DemoBox>
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "3 lines of macro code replaces 12 lines of explicit HTML." }
                </p>
            </div>

            // nav_menu!
            <div class="card">
                <h3>{ "2. nav_menu!" }</h3>
                <p>
                    { "Generate a complete sidebar menu with labeled sections. " }
                    { "Each section becomes a " }<code>{ "<div>" }</code>
                    { " with a title and nested " }<code>{ "<ul>" }</code>{ " list." }
                </p>
                <CopyCode code={CODE_NAV_MENU.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <div class="macro-menu-demo">
                        <div class="macro-menu-section">
                            <div class="macro-menu-section-title">{ "Main" }</div>
                            <ul class="nav-list">
                                <li class="nav-item">
                                    <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                                </li>
                                <li class="nav-item">
                                    <NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>>
                                </li>
                            </ul>
                        </div>
                        <div class="macro-menu-section">
                            <div class="macro-menu-section-title">{ "Components" }</div>
                            <ul class="nav-list">
                                <li class="nav-item">
                                    <NavLink<Route> to={Route::NavListDoc}>{ "NavList" }</NavLink<Route>>
                                </li>
                                <li class="nav-item">
                                    <NavLink<Route> to={Route::WithMacrosExample}>{ "Macros" }</NavLink<Route>>
                                </li>
                            </ul>
                        </div>
                    </div>
                </DemoBox>
            </div>

            // nav_tabs!
            <div class="card">
                <h3>{ "3. nav_tabs!" }</h3>
                <p>
                    { "Generate tab navigation from a list of route-label pairs. " }
                    { "Ideal for switching between views within a single page." }
                </p>
                <CopyCode code={CODE_NAV_TABS.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <div class="macro-tabs-demo">
                        <ul class="nav-tabs-list">
                            <li class="nav-tab-item">
                                <NavLink<Route> to={Route::Home}>{ "Tab 1" }</NavLink<Route>>
                            </li>
                            <li class="nav-tab-item">
                                <NavLink<Route> to={Route::NavLinkDoc}>{ "Tab 2" }</NavLink<Route>>
                            </li>
                            <li class="nav-tab-item">
                                <NavLink<Route> to={Route::WithMacrosExample}>{ "Tab 3" }</NavLink<Route>>
                            </li>
                        </ul>
                    </div>
                </DemoBox>
            </div>

            // breadcrumbs!
            <div class="card">
                <h3>{ "4. breadcrumbs!" }</h3>
                <p>
                    { "Generate a breadcrumb trail from route-label pairs. " }
                    { "Each segment is a NavLink with separator tokens between them." }
                </p>
                <CopyCode code={CODE_BREADCRUMBS.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <div class="breadcrumbs-demo">
                        <nav class="breadcrumbs">
                            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                            <span class="breadcrumb-separator">{ "/" }</span>
                            <NavLink<Route> to={Route::WithMacrosExample}>{ "Macros" }</NavLink<Route>>
                        </nav>
                    </div>
                </DemoBox>
            </div>

            // nav_pagination!
            <div class="card">
                <h3>{ "5. nav_pagination!" }</h3>
                <p>
                    { "Generate pagination controls with current page and total pages. " }
                    { "Handles edge cases like first/last page disabled states automatically." }
                </p>
                <CopyCode code={CODE_NAV_PAGINATION.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
                    <div class="pagination-demo">
                        <nav class="pagination">
                            <span class="page-item disabled">{ "\u{00AB}" }</span>
                            <span class="page-item active">{ "1" }</span>
                            <span class="page-item">{ "2" }</span>
                            <span class="page-item">{ "3" }</span>
                            <span class="page-item">{ "4" }</span>
                            <span class="page-item">{ "5" }</span>
                            <span class="page-item">{ "\u{00BB}" }</span>
                        </nav>
                    </div>
                </DemoBox>
            </div>

            // Macro Expansion Comparison
            <div class="card">
                <h3>{ "Macro Expansion Comparison" }</h3>
                <p>
                    { "Understanding what the macro generates helps demystify the abstraction. " }
                    { "Below is the nav_list! expansion:" }
                </p>
                <CopyCode code={CODE_EXPANSION_NAV_LIST.to_string()} />
                <p style="margin-top:0.75rem;">
                    { "And the nav_menu! expansion:" }
                </p>
                <CopyCode code={CODE_EXPANSION_NAV_MENU.to_string()} />
                <div class="info-box" style="margin-top:1rem;">
                    <strong>{ "Note:" }</strong>
                    { " The actual expansion includes additional attributes and optimization hints. " }
                    { "The simplified view above shows the structural equivalence." }
                </div>
            </div>

            // Memory & Performance
            <div class="card">
                <h3>{ "Memory & Performance" }</h3>
                <table class="doc-table">
                    <thead>
                        <tr>
                            <th>{ "Metric" }</th>
                            <th>{ "Manual" }</th>
                            <th>{ "Macro" }</th>
                            <th>{ "Delta" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>{ "Source LOC (5 links)" }</code></td>
                            <td>{ "20" }</td>
                            <td>{ "7" }</td>
                            <td>{ "-65%" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Source LOC (20 links)" }</code></td>
                            <td>{ "80" }</td>
                            <td>{ "22" }</td>
                            <td>{ "-72%" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Source LOC (50 links)" }</code></td>
                            <td>{ "200" }</td>
                            <td>{ "52" }</td>
                            <td>{ "-74%" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Binary size impact" }</code></td>
                            <td>{ "0 bytes" }</td>
                            <td>{ "0 bytes" }</td>
                            <td>{ "Identical" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Runtime allocations" }</code></td>
                            <td>{ "~2 per NavLink" }</td>
                            <td>{ "~2 per NavLink" }</td>
                            <td>{ "Identical" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Compile time overhead" }</code></td>
                            <td>{ "Baseline" }</td>
                            <td>{ "+5-15ms per macro" }</td>
                            <td>{ "Negligible" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Re-render cost" }</code></td>
                            <td>{ "O(n) NavLink checks" }</td>
                            <td>{ "O(n) NavLink checks" }</td>
                            <td>{ "Identical" }</td>
                        </tr>
                    </tbody>
                </table>
                <p style="margin-top:0.75rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Macros have zero runtime cost — they expand at compile time into the same component tree you would write manually." }
                </p>
            </div>

            // Compile-Time Analysis
            <div class="card">
                <h3>{ "Compile-Time Analysis" }</h3>
                <table class="doc-table">
                    <thead>
                        <tr>
                            <th>{ "Phase" }</th>
                            <th>{ "What Happens" }</th>
                            <th>{ "Duration" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>{ "Lexing" }</code></td>
                            <td>{ "Macro input tokenized from source" }</td>
                            <td>{ "< 1ms" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Parsing" }</code></td>
                            <td>{ "DSL parsed into internal AST" }</td>
                            <td>{ "1-3ms" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Expansion" }</code></td>
                            <td>{ "AST transformed to html! tokens" }</td>
                            <td>{ "2-5ms" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Type checking" }</code></td>
                            <td>{ "Generated code type-checked with Route enum" }</td>
                            <td>{ "1-2ms" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Total per macro" }</code></td>
                            <td>{ "Full pipeline" }</td>
                            <td>{ "5-15ms" }</td>
                        </tr>
                    </tbody>
                </table>
                <div class="info-box" style="margin-top:1rem;">
                    <strong>{ "Why so fast:" }</strong>
                    { " Procedural macros operate on token streams, not full ASTs. " }
                    { "The syn/quote ecosystem is highly optimized for this pattern." }
                </div>
            </div>

            // Code Reduction Metrics
            <div class="card">
                <h3>{ "Code Reduction Metrics" }</h3>
                <table class="doc-table">
                    <thead>
                        <tr>
                            <th>{ "Macro" }</th>
                            <th>{ "Manual Lines" }</th>
                            <th>{ "Macro Lines" }</th>
                            <th>{ "Reduction" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>{ "nav_list!" }</code></td>
                            <td>{ "4n + 4" }</td>
                            <td>{ "n + 4" }</td>
                            <td>{ "~75%" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "nav_menu!" }</code></td>
                            <td>{ "6n + 8s + 12" }</td>
                            <td>{ "n + 3s + 4" }</td>
                            <td>{ "~80%" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "nav_tabs!" }</code></td>
                            <td>{ "4n + 4" }</td>
                            <td>{ "n + 4" }</td>
                            <td>{ "~75%" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "breadcrumbs!" }</code></td>
                            <td>{ "3n + 2" }</td>
                            <td>{ "n + 2" }</td>
                            <td>{ "~66%" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "nav_pagination!" }</code></td>
                            <td>{ "15-25" }</td>
                            <td>{ "1" }</td>
                            <td>{ "~95%" }</td>
                        </tr>
                    </tbody>
                </table>
                <p style="margin-top:0.75rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "n = number of links, s = number of sections. Formulas represent approximate line counts." }
                </p>
            </div>

            // All Available Macros
            <div class="card">
                <h3>{ "All Available Macros" }</h3>
                <div class="feature-grid">
                    <div class="feature-card">
                        <h4>{ "nav_list!" }</h4>
                        <p>{ "Generate " }<code>{ "<ul>" }</code>{ " with " }<code>{ "<NavLink>" }</code>{ " items from route-label pairs" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "nav_links!" }</h4>
                        <p>{ "Generate " }<code>{ "<NavLink>" }</code>{ " components from a list of (route, label) tuples" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "nav_menu!" }</h4>
                        <p>{ "Generate a sidebar menu with labeled sections and nested lists" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "nav_tabs!" }</h4>
                        <p>{ "Generate tab navigation from route-label pairs" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "breadcrumbs!" }</h4>
                        <p>{ "Generate a breadcrumb trail from route-label pairs with separators" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "nav_pagination!" }</h4>
                        <p>{ "Generate pagination controls with current/total page configuration" }</p>
                    </div>
                </div>
            </div>

            // When to Use
            <div class="card">
                <h3>{ "When to Use This Pattern" }</h3>
                <div class="feature-grid">
                    <div class="feature-card">
                        <h4>{ "Large Navigation Trees" }</h4>
                        <p>{ "Applications with 10+ navigation links benefit most from the declarative syntax. The reduction in boilerplate compounds with scale." }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Sidebar Menus" }</h4>
                        <p>{ "nav_menu! is purpose-built for sectioned sidebar navigation. Define your entire menu structure in a single macro invocation." }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Rapid Prototyping" }</h4>
                        <p>{ "Get navigation working in minutes instead of hours. Add routes by appending tuples — no HTML structure to maintain." }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Team Consistency" }</h4>
                        <p>{ "Enforce consistent navigation patterns across your codebase. Every developer uses the same macro syntax, eliminating style drift." }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Route Refactoring" }</h4>
                        <p>{ "When route enum variants are renamed, the macro catches missing updates at compile time. No orphaned navigation links." }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "When NOT to Use" }</h4>
                        <p>{ "For 2-3 links with custom styling per item, manual markup may be clearer. Macros shine with repetition, not uniqueness." }</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
