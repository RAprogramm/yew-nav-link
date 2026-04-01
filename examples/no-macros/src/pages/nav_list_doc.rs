use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{NavItem, NavLink, NavList};

fn code_block(lines: &[&str]) -> Html {
    html! {
        <pre><code>
            { for lines.iter().map(|l| html! { <>{ l }{"\n"}</> }) }
        </code></pre>
    }
}

#[function_component]
pub fn NavListDoc() -> Html {
    html! {
        <div>
            <div class="card">
                <h2>{ "NavList & NavItem" }</h2>
                <p>{ "Semantic wrappers that render " }<code>{ "<ul>" }</code>{ " and " }
                    <code>{ "<li>" }</code>{ " with proper ARIA roles." }</p>
            </div>

            <div class="card">
                <h3>{ "Basic Usage" }</h3>
                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
                    <NavList aria_label="Main Navigation">
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                        </NavItem>
                        <NavItem>
                            <NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>>
                        </NavItem>
                        <NavItem>
                            <NavLink<Route> to={Route::NavListDoc}>{ "NavList" }</NavLink<Route>>
                        </NavItem>
                    </NavList>
                </div>

                { code_block(&[
                    "use yew_nav_link::{NavLink, NavList, NavItem};",
                    "",
                    "html! {",
                    "    <NavList aria_label=\"Primary Navigation\">",
                    "        <NavItem>",
                    "            <NavLink<Route> to={Route::Home}>",
                    "                { \"Home\" }",
                    "            </NavLink<Route>>",
                    "        </NavItem>",
                    "    </NavList>",
                    "}",
                ]) }
            </div>

            <div class="card">
                <h3>{ "Disabled Items" }</h3>
                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
                    <NavList>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>{ "Enabled" }</NavLink<Route>>
                        </NavItem>
                        <NavItem disabled=true>
                            <NavLink<Route> to={Route::NotFound}>{ "Disabled Item" }</NavLink<Route>>
                        </NavItem>
                    </NavList>
                </div>

                { code_block(&[
                    "<NavItem disabled=true>",
                    "    <NavLink<Route> to={Route::Settings}>",
                    "        { \"Settings\" }",
                    "    </NavLink<Route>>",
                    "</NavItem>",
                ]) }
            </div>

            <div class="card">
                <h3>{ "Props" }</h3>
                <p><strong>{ "NavList" }</strong></p>
                <table>
                    <tr><td><code>{ "aria_label" }</code></td><td>{ "ARIA label for navigation" }</td></tr>
                    <tr><td><code>{ "id" }</code></td><td>{ "HTML id attribute" }</td></tr>
                    <tr><td><code>{ "classes" }</code></td><td>{ "Additional CSS classes" }</td></tr>
                </table>
                <p style="margin-top:16px;"><strong>{ "NavItem" }</strong></p>
                <table>
                    <tr><td><code>{ "disabled" }</code></td><td>{ "bool — disabled state" }</td></tr>
                    <tr><td><code>{ "classes" }</code></td><td>{ "Additional CSS classes" }</td></tr>
                </table>
            </div>
        </div>
    }
}
