use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{
    components::{NavDropdown, NavDropdownDivider, NavDropdownItem},
    NavItem, NavLink, NavList,
};

fn code_block(lines: &[&str]) -> Html {
    html! {
        <pre><code>
            { for lines.iter().map(|l| html! { <>{ l }{"\n"}</> }) }
        </code></pre>
    }
}

#[function_component]
pub fn DropdownDoc() -> Html {
    html! {
        <div>
            <div class="card">
                <h2>{ "NavDropdown" }</h2>
                <p>{ "Collapsible dropdown menu for grouped navigation items." }</p>
            </div>

            <div class="card">
                <h3>{ "Basic Dropdown" }</h3>
                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
                    <NavList>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                        </NavItem>
                        <NavDropdown toggle_text="Settings">
                            <NavDropdownItem>
                                <NavLink<Route> to={Route::Home}>{ "Profile" }</NavLink<Route>>
                            </NavDropdownItem>
                            <NavDropdownItem>
                                <NavLink<Route> to={Route::Home}>{ "Security" }</NavLink<Route>>
                            </NavDropdownItem>
                            <NavDropdownDivider />
                            <NavDropdownItem disabled=true>
                                { "Admin (disabled)" }
                            </NavDropdownItem>
                        </NavDropdown>
                    </NavList>
                </div>

                { code_block(&[
                    "use yew_nav_link::{",
                    "    NavLink, NavList, NavItem,",
                    "    components::{",
                    "        NavDropdown, NavDropdownItem,",
                    "        NavDropdownDivider",
                    "    }",
                    "};",
                    "",
                    "html! {",
                    "    <NavList>",
                    "        <NavItem>",
                    "            <NavLink<Route> to={Route::Home}>",
                    "                { \"Home\" }",
                    "            </NavLink<Route>>",
                    "        </NavItem>",
                    "        <NavDropdown toggle_text=\"Settings\">",
                    "            <NavDropdownItem>",
                    "                <NavLink<Route> to={Route::Profile}>",
                    "                    { \"Profile\" }",
                    "                </NavLink<Route>>",
                    "            </NavDropdownItem>",
                    "            <NavDropdownDivider />",
                    "            <NavDropdownItem disabled=true>",
                    "                { \"Admin\" }",
                    "            </NavDropdownItem>",
                    "        </NavDropdown>",
                    "    </NavList>",
                    "}",
                ]) }
            </div>

            <div class="card">
                <h3>{ "Props" }</h3>
                <p><strong>{ "NavDropdown" }</strong></p>
                <table>
                    <tr><td><code>{ "toggle_text" }</code></td><td>{ "Button label (default: \"dropdown\")" }</td></tr>
                    <tr><td><code>{ "id" }</code></td><td>{ "Optional element id" }</td></tr>
                </table>
                <p style="margin-top:16px;"><strong>{ "NavDropdownItem" }</strong></p>
                <table>
                    <tr><td><code>{ "disabled" }</code></td><td>{ "bool — disable the item" }</td></tr>
                </table>
            </div>
        </div>
    }
}
