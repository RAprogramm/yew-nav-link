//! # Macros
//!
//! yew-nav-link provides both declarative macros (`macro_rules!`) and
//! procedural macros for reducing boilerplate and building navigation
//! structures from route enums.
//!
//! # Declarative Macros
//!
//! Three macros are available via `yew_nav_link::*`:
//!
//! - `nav_link!` — Shorthand for `nav_link(route, label, mode)`
//! - `nav_item!` — Generates a `nav_path()` method on a route enum
//! - `routable_ext!` — Generates route validation methods
//!
//! # Proc Macros
//!
//! Six procedural macros are available via `yew_nav_link::*`:
//!
//! - `nav_list!` — Build a nav list from route-label pairs
//! - `nav_menu!` — Build a nested dropdown menu
//! - `nav_tabs!` — Build a tabbed interface
//! - `nav_pagination!` — Build a pagination component
//! - `breadcrumbs!` — Build breadcrumbs from route pairs
//!
//! # How Macros Work
//!
//! Each proc macro expands at compile time to the equivalent component
//! tree. No runtime overhead — the macro generates the same Rust code
//! you'd write by hand.
//!
//! # Memory & Performance
//!
//! Macros have zero runtime cost — they expand at compile time into
//! the same component tree you'd write manually.

use crate::code_utils::CopyCode;
use crate::doc_parser::{parse_doc_block, DocRenderer};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{nav_link, Match};

const SRC: &str = include_str!("macros_doc.rs");

const CODE_NAV_LINK: &str = "\
use yew_nav_link::{nav_link, Match};

# #[derive(Clone, PartialEq, Debug, Routable)]
# enum Route {
#     #[at(\"/\")]
#     Home,
# }
html! { nav_link(Route::Home, \"Home\", Match::Exact) }";

const CODE_NAV_LIST: &str = "\
use yew_nav_link::nav_list;

# #[derive(Clone, PartialEq, Debug, Routable)]
# enum Route {
#     #[at(\"/\")]
#     Home,
#     #[at(\"/about\")]
#     About,
# }
let nav = nav_list!(Route::Home \"Home\", Route::About \"About\");";

const CODE_NAV_MENU: &str = "\
use yew_nav_link::nav_menu;

# #[derive(Clone, PartialEq, Debug, Routable)]
# enum Route {
#     #[at(\"/\")]
#     Home,
#     #[at(\"/settings\")]
#     Settings,
# }
let menu = nav_menu!(
    Route::Home \"Home\",
    Route::Settings \"Settings\" {
        Route::Home \"Profile\",
    }
);";

#[function_component]
pub fn MacrosDoc() -> Html {
    let doc = parse_doc_block(SRC);

    html! {
        <div>
            <DocRenderer {doc} />

            <div class="card">
                <h3>{ "nav_link! — Live Demo" }</h3>
                <p>{ "Shorthand for " }<code>{ "nav_link(route, label, mode)" }</code>{ ":" }</p>
                <CopyCode code={CODE_NAV_LINK.to_string()} />
                <h3>{ "Live Result" }</h3>
                <nav>
                    <ul class="nav-list">
                        <li class="nav-item">{ nav_link(Route::Home, "Home (macro)", Match::Exact) }</li>
                        <li class="nav-item">{ nav_link(Route::MacrosDoc, "Macros (macro)", Match::Exact) }</li>
                    </ul>
                </nav>
            </div>

            <div class="card">
                <h3>{ "nav_list! — Proc Macro" }</h3>
                <p>{ "Build a nav list from route-label pairs:" }</p>
                <CopyCode code={CODE_NAV_LIST.to_string()} />
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Expands at compile time to " }<code>{ "<NavList>" }</code>
                    { " with " }<code>{ "<NavLink>" }</code>{ " children." }
                </p>
            </div>

            <div class="card">
                <h3>{ "nav_menu! — Proc Macro" }</h3>
                <p>{ "Build a nested dropdown menu:" }</p>
                <CopyCode code={CODE_NAV_MENU.to_string()} />
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Use " }<code>{ "{ ... }" }</code>
                    { " after a label to nest items inside a dropdown." }
                </p>
            </div>

            <div class="card">
                <h3>{ "All Proc Macros" }</h3>
                <table class="doc-table">
                    <thead>
                        <tr><th>{ "Macro" }</th><th>{ "Expands To" }</th></tr>
                    </thead>
                    <tbody>
                        <tr><td><code>{ "nav_list!" }</code></td><td>{ "<NavList> with <NavLink> items" }</td></tr>
                        <tr><td><code>{ "nav_menu!" }</code></td><td>{ "<NavDropdown> with nested items" }</td></tr>
                        <tr><td><code>{ "nav_tabs!" }</code></td><td>{ "<NavTabs> with <NavTab> items" }</td></tr>
                        <tr><td><code>{ "nav_pagination!" }</code></td><td>{ "<Pagination> with config" }</td></tr>
                        <tr><td><code>{ "breadcrumbs!" }</code></td><td>{ "Breadcrumb trail from route pairs" }</td></tr>
                    </tbody>
                </table>
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "All macros expand at compile time — zero runtime overhead." }
                </p>
            </div>
        </div>
    }
}
