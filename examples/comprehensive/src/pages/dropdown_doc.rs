use crate::code_utils::CopyCode;
use crate::demo_popup::DemoBox;
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{
    components::{NavDropdown, NavDropdownDivider, NavDropdownItem},
    NavItem, NavLink, NavList,
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

const CODE_HTML_STRUCTURE: &str = r##"<li class="nav-dropdown" role="presentation">
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
      <a href="#">Admin</a>
    </li>
  </ul>
</li>"##;

const CODE_CUSTOM: &str = r#"<NavDropdown toggle_text="More" classes="custom-dropdown">
    <NavDropdownItem classes="custom-item">
        <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
    </NavDropdownItem>
</NavDropdown>"#;

#[function_component]
pub fn DropdownDoc() -> Html {
    html! {
        <div>
            // ── Header ────────────────────────────────────────
            <div class="example-header">
                <div class="example-header-top">
                    <span class="example-tag tag-indigo">{ "component" }</span>
                    <span class="example-source">{ "src/components/dropdown.rs" }</span>
                </div>
                <h2>{ "NavDropdown" }</h2>
                <p class="example-desc">
                    { "Collapsible dropdown menu for grouping related navigation items. " }
                    { "Renders a " }<code>{ "<li>" }</code>
                    { " with a toggle button and a nested " }<code>{ "<ul>" }</code>
                    { " menu. Supports dividers, disabled items, and custom CSS classes." }
                </p>
            </div>

            // ── Architecture ──────────────────────────────────
            <div class="card">
                <h3>{ "Architecture" }</h3>
                <p>
                    { "NavDropdown is a composite component built from three sub-components that work together to create a hierarchical navigation structure." }
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
                <div class="info-box">
                    <strong>{ "DOM structure:" }</strong>
                    { " The dropdown renders as " }<code>{ "<li>" }</code>
                    { " (not " }<code>{ "<div>" }</code>{ ") to maintain semantic " }<code>{ "<ul>" }</code>
                    { " list structure. The toggle is a " }<code>{ "<button>" }</code>
                    { " (not " }<code>{ "<a>" }</code>{ ") for proper accessibility." }
                </div>
            </div>

            // ── Basic Usage ───────────────────────────────────
            <div class="card">
                <h3>{ "Basic Dropdown" }</h3>
                <p>
                    { "A dropdown with items, a divider, and a disabled entry. " }
                    { "Click the toggle to open/close the menu." }
                </p>
                <CopyCode code={CODE_BASIC.to_string()} />

                <h3>{ "Live Result" }</h3>
                <DemoBox>
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
                </DemoBox>
            </div>

            // ── HTML Output ───────────────────────────────────
            <div class="card">
                <h3>{ "HTML Output" }</h3>
                <p>
                    { "Understanding the generated DOM helps with CSS customization and debugging." }
                </p>
                <CopyCode code={CODE_HTML_STRUCTURE.to_string()} />

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

            // ── Custom Classes ────────────────────────────────
            <div class="card">
                <h3>{ "Custom CSS Classes" }</h3>
                <p>
                    { "All dropdown components accept a " }<code>{ "classes" }</code>
                    { " prop for additional styling." }
                </p>
                <CopyCode code={CODE_CUSTOM.to_string()} />

                <h3>{ "CSS Customization Example" }</h3>
                <div class="code-block">
                    <pre><code class="language-css">{ r#".custom-dropdown .nav-dropdown-toggle {
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
}"# }</code></pre>
                </div>
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
                            <td><code>{ "aria-expanded=\"false\"" }</code></td>
                            <td>{ "Toggle button" }</td>
                            <td>{ "Indicates initial collapsed state" }</td>
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
                            <td>{ "4 base nodes (li, button, span, ul) + 2 per item (li, a)" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Re-renders" }</code></td>
                            <td>{ "On toggle click" }</td>
                            <td>{ "Menu visibility is CSS-driven; no state subscription" }</td>
                        </tr>
                        <tr>
                            <td><code>{ "Event listeners" }</code></td>
                            <td>{ "1 per item" }</td>
                            <td>{ "onclick prevent_default on each NavDropdownItem" }</td>
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
                <h3>{ "When to Use This Pattern" }</h3>
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
                        <h4>{ "Hierarchical Menus" }</h4>
                        <p>{ "Create multi-level navigation by nesting dropdowns within dropdown items" }</p>
                    </div>
                    <div class="feature-card">
                        <h4>{ "Conditional Items" }</h4>
                        <p>{ "Show/hide menu items based on user roles using the disabled prop" }</p>
                    </div>
                </div>
            </div>

            // ── Source ────────────────────────────────────────
            <div class="card">
                <h3>{ "Source Code" }</h3>
                <p>
                    { "Full implementation with documentation:" }
                </p>
                <CopyCode code={SRC.to_string()} />
            </div>
        </div>
    }
}
