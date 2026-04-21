use yew::prelude::*;
use yew_nav_link::{
    NavItem, NavLink, NavList,
    components::{NavDropdown, NavDropdownDivider, NavDropdownItem}
};

use crate::{
    code_utils::{CopyCode, highlight_css},
    demo_popup::DemoBox,
    doc_page::{DemoCard, DocPage, Tip},
    routes::Route
};

const SRC: &str = include_str!("../../../../src/components/dropdown.rs");

const CODE_BASIC: &str = r#"use yew::prelude::*;
use yew_nav_link::{NavItem, NavLink, NavList};
use yew_nav_link::components::{
    NavDropdown, NavDropdownItem, NavDropdownDivider,
};

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/profile")]
    Profile,
    #[at("/settings")]
    Settings,
}

#[component]
fn Nav() -> Html {
    html! {
        <NavList>
            <NavItem>
                <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
            </NavItem>
            <NavDropdown toggle_text="Settings">
                <NavDropdownItem>
                    <NavLink<Route> to={Route::Profile}>{ "Profile" }</NavLink<Route>>
                </NavDropdownItem>
                <NavDropdownItem>
                    <NavLink<Route> to={Route::Settings}>{ "Preferences" }</NavLink<Route>>
                </NavDropdownItem>
                <NavDropdownDivider />
                <NavDropdownItem disabled=true>
                    { "Admin (disabled)" }
                </NavDropdownItem>
            </NavDropdown>
        </NavList>
    }
}"#;

const CODE_HTML: &str = r#"<li class="nav-dropdown" role="presentation">
  <button class="nav-dropdown-toggle" aria-expanded="false">
    Settings
    <span class="nav-dropdown-caret"> ▼</span>
  </button>
  <ul class="nav-dropdown-menu" role="menu">
    <li class="nav-dropdown-item" role="menuitem">
      <a href="/profile" class="nav-link">Profile</a>
    </li>
    <li class="nav-dropdown-divider" role="separator"></li>
    <li class="nav-dropdown-item disabled" role="menuitem">
      <span>Admin</span>
    </li>
  </ul>
</li>"#;

const CODE_CSS: &str = r#".custom-dropdown .nav-dropdown-toggle {
    background: #1e293b;
    color: #e2e8f0;
    border: 1px solid #334155;
}

.custom-dropdown .nav-dropdown-menu {
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
}

.custom-dropdown .nav-dropdown-item a {
    color: #cbd5e1;
}

.custom-dropdown .nav-dropdown-item a:hover {
    background: #1e293b;
    color: #f8fafc;
}"#;

const STATE_CODE: &str = r#"let open = use_state(|| false);

let on_toggle = Callback::from(move |e: MouseEvent| {
    e.stop_propagation();
    open.set(!*open);
});

let menu_class = if *open {
    "nav-dropdown-menu open"
} else {
    "nav-dropdown-menu"
};"#;

#[function_component]
pub fn DropdownDoc() -> Html {
    html! {
        <DocPage source={SRC.to_string()}>

            <DemoCard
                title="Basic Dropdown"
                description={html!{
                    <>
                        { "A dropdown with items, a divider, and a disabled entry. " }
                        { "Click the toggle to open/close the menu." }
                    </>
                }}
                code={CODE_BASIC.to_string()}
                tip={html!{
                    <Tip>
                        { "Use " }<code>{ "<NavDropdownDivider />" }</code>
                        { " between groups. Set " }<code>{ "disabled=true" }</code>
                        { " on items to prevent selection." }
                    </Tip>
                }}
            >
                <NavList>
                    <NavItem>
                        <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                    </NavItem>
                    <NavDropdown toggle_text="Settings">
                        <NavDropdownItem>
                            <NavLink<Route> to={Route::Home}>{ "Profile" }</NavLink<Route>>
                        </NavDropdownItem>
                        <NavDropdownItem>
                            <NavLink<Route> to={Route::NavLinkDoc}>{ "Security" }</NavLink<Route>>
                        </NavDropdownItem>
                        <NavDropdownDivider />
                        <NavDropdownItem disabled=true>
                            { "Admin (disabled)" }
                        </NavDropdownItem>
                    </NavDropdown>
                </NavList>
            </DemoCard>

            // ── Architecture ──────────────────────────────────
            <div class="card">
                <h3>{ "Architecture" }</h3>
                <p>
                    { "NavDropdown is a composite of three sub-components that render a semantic nested list structure." }
                </p>
                <div class="arch-diagram-small">
                    <div class="arch-box arch-box-indigo">{ "NavDropdown" }</div>
                    <div class="arch-arrow-sm">{ "\u{2192}" }</div>
                    <div class="arch-box arch-box-blue">{ "NavDropdownItem" }</div>
                    <div class="arch-arrow-sm">{ "+" }</div>
                    <div class="arch-box arch-box-gray">{ "NavDropdownDivider" }</div>
                    <div class="arch-arrow-sm">{ "+" }</div>
                    <div class="arch-box arch-box-orange">{ "NavLink<R>" }</div>
                </div>
            </div>

            // ── HTML Output ───────────────────────────────────
            <div class="card">
                <h3>{ "HTML Output" }</h3>
                <p>
                    { "Understanding the generated DOM helps with CSS customization and debugging." }
                </p>
                <div class="code-block">
                    <pre><code>{ CODE_HTML }</code></pre>
                </div>
                <table class="doc-table">
                    <thead>
                        <tr>
                            <th>{ "Element" }</th>
                            <th>{ "Class" }</th>
                            <th>{ "Purpose" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>{ "<li>" }</code></td>
                            <td><code>{ "nav-dropdown" }</code></td>
                            <td>{ "Container — maintains list semantics" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "<button>" }</code></td>
                            <td><code>{ "nav-dropdown-toggle" }</code></td>
                            <td>{ "Clickable toggle — triggers menu open/close" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "<span>" }</code></td>
                            <td><code>{ "nav-dropdown-caret" }</code></td>
                            <td>{ "Visual caret indicator (▼)" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "<ul>" }</code></td>
                            <td><code>{ "nav-dropdown-menu" }</code></td>
                            <td>{ "Dropdown menu container" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "<li>" }</code></td>
                            <td><code>{ "nav-dropdown-item" }</code></td>
                            <td>{ "Individual menu item" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "<li>" }</code></td>
                            <td><code>{ "nav-dropdown-divider" }</code></td>
                            <td>{ "Visual separator between item groups" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "<li>" }</code></td>
                            <td><code>{ "nav-dropdown-item disabled" }</code></td>
                            <td>{ "Disabled item — prevents interaction" }</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            // ── Accessibility ─────────────────────────────────
            <div class="card">
                <h3>{ "Accessibility" }</h3>
                <p>
                    { "NavDropdown follows WAI-ARIA Authoring Practices for menu patterns." }
                </p>
                <table class="doc-table">
                    <thead>
                        <tr>
                            <th>{ "Attribute" }</th>
                            <th>{ "Element" }</th>
                            <th>{ "Purpose" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>{ "role=\"presentation\"" }</code></td>
                            <td>{ "Container <li>" }</td>
                            <td>{ "Removes list semantics from outer container" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "role=\"menu\"" }</code></td>
                            <td>{ "Inner <ul>" }</td>
                            <td>{ "Identifies the dropdown as a menu" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "role=\"menuitem\"" }</code></td>
                            <td>{ "Item <li>" }</td>
                            <td>{ "Identifies each item as a menu option" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "role=\"separator\"" }</code></td>
                            <td>{ "Divider <li>" }</td>
                            <td>{ "Identifies visual separator" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "aria-haspopup=\"true\"" }</code></td>
                            <td>{ "Toggle button" }</td>
                            <td>{ "Indicates button opens a popup menu" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "aria-expanded" }</code></td>
                            <td>{ "Toggle button" }</td>
                            <td>{ "Dynamically toggles between \"true\" and \"false\"" }</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            // ── Memory & Performance ──────────────────────────
            <div class="card">
                <h3>{ "Memory & Performance" }</h3>
                <table class="doc-table">
                    <thead>
                        <tr>
                            <th>{ "Metric" }</th>
                            <th>{ "Value" }</th>
                            <th>{ "Details" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>{ "DOM nodes" }</code></td>
                            <td>{ "4 + 2n" }</td>
                            <td>{ "4 base (li, button, span, ul) + 2 per item (li, content)" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "State" }</code></td>
                            <td>{ "1 bool" }</td>
                            <td>{ "use_state for open/closed — no subscriptions" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Re-renders" }</code></td>
                            <td>{ "On toggle click only" }</td>
                            <td>{ "No polling, no intervals" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Heap allocations" }</code></td>
                            <td>{ "~1 per dropdown" }</td>
                            <td>{ "One Classes clone for the container element" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Bundle overhead" }</code></td>
                            <td>{ "~2 KB (gzipped)" }</td>
                            <td>{ "Three components: NavDropdown, NavDropdownItem, NavDropdownDivider" }</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            // ── When to Use ───────────────────────────────────
            <div class="card">
                <h3>{ "When to Use" }</h3>
                <div class="feature-grid">
                    <div class="feature-card">
                        <h4>{ "Grouped Navigation" }</h4>
                        <p>{ "Organize related links under a single toggle — settings, user menu, admin tools" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Space-Constrained Layouts" }</h4>
                        <p>{ "Collapse secondary navigation into a dropdown when horizontal space is limited" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Conditional Items" }</h4>
                        <p>{ "Show/hide menu items based on user roles using the disabled prop" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Nested Menus" }</h4>
                        <p>{ "Create multi-level navigation by nesting dropdowns within dropdown items" }</p>
                    </div>
                </div>
            </div>

            // ── State Management ──────────────────────────────
            <div class="card">
                <h3>{ "State Management" }</h3>
                <p>
                    { "NavDropdown uses " }<code>{ "use_state(|| false)" }</code>
                    { " internally to track open/closed state. Each click on the toggle button flips the state." }
                </p>
                <CopyCode code={STATE_CODE.to_string()} language={"rust".to_string()} />

                <h3>{ "State Flow" }</h3>
                <div class="flow-diagram">
                    <div class="flow-step">
                        <div class="flow-num">{ "1" }</div>
                        <div class="flow-text">{ "Mount: " }<code>{ "open = false" }</code></div>
                    </div>
                    <div class="flow-arrow">{ "\u{2192}" }</div>
                    <div class="flow-step">
                        <div class="flow-num">{ "2" }</div>
                        <div class="flow-text">{ "Click toggle" }</div>
                    </div>
                    <div class="flow-arrow">{ "\u{2192}" }</div>
                    <div class="flow-step">
                        <div class="flow-num">{ "3" }</div>
                        <div class="flow-text">{ "open = " }<code>{ "true" }</code></div>
                    </div>
                    <div class="flow-arrow">{ "\u{2192}" }</div>
                    <div class="flow-step flow-step-active">
                        <div class="flow-num">{ "4" }</div>
                        <div class="flow-text">{ "Menu shown, " }<code>{ "aria-expanded=\"true\"" }</code></div>
                    </div>
                </div>

                <h3>{ "Live State Demo" }</h3>
                <DemoBox>
                    <div class="state-demo">
                        <div class="state-indicator">
                            <span class="state-label">{ "open:" }</span>
                            <span class="state-value state-open">{ "true" }</span>
                        </div>
                        <div class="state-indicator">
                            <span class="state-label">{ "aria-expanded:" }</span>
                            <span class="state-value state-aria">{ "\"true\"" }</span>
                        </div>
                        <div class="state-indicator">
                            <span class="state-label">{ "menu class:" }</span>
                            <span class="state-value state-class">{ "\"nav-dropdown-menu open\"" }</span>
                        </div>
                    </div>
                </DemoBox>
                <p class="tip-text">
                    { "Click the dropdown toggle above to see state values update in real time." }
                </p>

                <div class="info-box">
                    <strong>{ "Note:" }</strong>
                    { " The menu does NOT close on outside click by default. " }
                    { "To add this behavior, listen for clicks on the document body and set open to false." }
                </div>
            </div>

            // ── Custom Styling ────────────────────────────────
            <div class="card">
                <h3>{ "Custom Styling" }</h3>
                <p>
                    { "All dropdown components accept a " }<code>{ "classes" }</code>
                    { " prop for additional CSS customization." }
                </p>
                <div class="code-block">
                    <pre><code>{ highlight_css(CODE_CSS) }</code></pre>
                </div>

                <h3>{ "Default vs Custom" }</h3>
                <div class="style-comparison">
                    <div class="style-col">
                        <div class="style-col-label">{ "Default" }</div>
                        <DemoBox>
                            <NavDropdown toggle_text="Default">
                                <NavDropdownItem>
                                    <NavLink<Route> to={Route::Home}>{ "Item 1" }</NavLink<Route>>
                                </NavDropdownItem>
                                <NavDropdownItem>
                                    <NavLink<Route> to={Route::NavLinkDoc}>{ "Item 2" }</NavLink<Route>>
                                </NavDropdownItem>
                            </NavDropdown>
                        </DemoBox>
                    </div>
                    <div class="style-col">
                        <div class="style-col-label">{ "Custom (Dark)" }</div>
                        <DemoBox>
                            <div class="custom-dropdown">
                                <NavDropdown toggle_text="Custom" classes="custom-dropdown">
                                    <NavDropdownItem classes="custom-dropdown-item">
                                        <NavLink<Route> to={Route::Home}>{ "Item 1" }</NavLink<Route>>
                                    </NavDropdownItem>
                                    <NavDropdownItem classes="custom-dropdown-item">
                                        <NavLink<Route> to={Route::NavLinkDoc}>{ "Item 2" }</NavLink<Route>>
                                    </NavDropdownItem>
                                </NavDropdown>
                            </div>
                        </DemoBox>
                    </div>
                </div>
            </div>

        </DocPage>
    }
}
