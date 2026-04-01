use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{NavDivider, NavItem, NavLink, NavList};

fn code_block(lines: &[&str]) -> Html {
    html! {
        <pre><code>
            { for lines.iter().map(|l| html! { <>{ l }{"\n"}</> }) }
        </code></pre>
    }
}

#[function_component]
pub fn NavDividerDoc() -> Html {
    html! {
        <div>
            <div class="card">
                <h2>{ "NavDivider" }</h2>
                <p>{ "Visual separator between navigation items. Renders " }
                    <code>{ "<hr>" }</code>{ " with semantic classes." }</p>
            </div>

            <div class="card">
                <h3>{ "Basic Divider" }</h3>
                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
                    <NavList>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                        </NavItem>
                        <NavDivider />
                        <NavItem>
                            <NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>>
                        </NavItem>
                    </NavList>
                </div>

                { code_block(&[
                    "use yew_nav_link::{NavLink, NavList, NavItem, NavDivider};",
                    "",
                    "html! {",
                    "    <NavList>",
                    "        <NavItem>",
                    "            <NavLink<Route> to={Route::Home}>",
                    "                { \"Home\" }",
                    "            </NavLink<Route>>",
                    "        </NavItem>",
                    "        <NavDivider />",
                    "        <NavItem>",
                    "            <NavLink<Route> to={Route::Settings}>",
                    "                { \"Settings\" }",
                    "            </NavLink<Route>>",
                    "        </NavItem>",
                    "    </NavList>",
                    "}",
                ]) }
            </div>

            <div class="card">
                <h3>{ "Divider with Text" }</h3>
                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
                    <NavList>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                        </NavItem>
                        <NavDivider text="Section" />
                        <NavItem>
                            <NavLink<Route> to={Route::BadgeDoc}>{ "Badge" }</NavLink<Route>>
                        </NavItem>
                    </NavList>
                </div>

                { code_block(&[
                    "<NavDivider text=\"Section Title\" />",
                ]) }
            </div>

            <div class="card">
                <h3>{ "Props" }</h3>
                <table>
                    <tr><td><code>{ "text" }</code></td><td>{ "Optional text label" }</td></tr>
                    <tr><td><code>{ "vertical" }</code></td><td>{ "bool — vertical divider style" }</td></tr>
                    <tr><td><code>{ "classes" }</code></td><td>{ "Additional CSS classes" }</td></tr>
                </table>
            </div>
        </div>
    }
}
