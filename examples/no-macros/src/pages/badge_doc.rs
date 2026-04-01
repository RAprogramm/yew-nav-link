use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{NavBadge, NavHeader, NavItem, NavLink, NavList, NavText};

fn code_block(lines: &[&str]) -> Html {
    html! {
        <pre><code>
            { for lines.iter().map(|l| html! { <>{ l }{"\n"}</> }) }
        </code></pre>
    }
}

#[function_component]
pub fn BadgeDoc() -> Html {
    html! {
        <div>
            <div class="card">
                <h2>{ "NavBadge, NavHeader, NavText" }</h2>
                <p>{ "Inline decorations for navigation lists." }</p>
            </div>

            <div class="card">
                <h3>{ "NavBadge" }</h3>
                <p>{ "Notification counts and status indicators inline:" }</p>
                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
                    <NavList>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>
                                { "Messages " }
                                <NavBadge variant="danger">{ "3" }</NavBadge>
                            </NavLink<Route>>
                        </NavItem>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>
                                { "Tasks " }
                                <NavBadge variant="success" pill=true>{ "12" }</NavBadge>
                            </NavLink<Route>>
                        </NavItem>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>
                                { "Alerts " }
                                <NavBadge variant="warning">{ "!" }</NavBadge>
                            </NavLink<Route>>
                        </NavItem>
                    </NavList>
                </div>
                { code_block(&[
                    "use yew_nav_link::NavBadge;",
                    "",
                    "<NavBadge variant=\"danger\">{ \"3\" }</NavBadge>",
                    "<NavBadge variant=\"success\" pill=true>{ \"12\" }</NavBadge>",
                ]) }
            </div>

            <div class="card">
                <h3>{ "NavHeader" }</h3>
                <p>{ "Section headers inside navigation lists:" }</p>
                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
                    <NavList>
                        <NavHeader text="Main" />
                        <NavItem><NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>></NavItem>
                        <NavHeader text="Documentation" />
                        <NavItem><NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>></NavItem>
                        <NavItem><NavLink<Route> to={Route::NavListDoc}>{ "NavList" }</NavLink<Route>></NavItem>
                    </NavList>
                </div>
                { code_block(&[
                    "use yew_nav_link::NavHeader;",
                    "",
                    "<NavList>",
                    "    <NavHeader text=\"Section Title\" />",
                    "    <NavItem> ... </NavItem>",
                    "</NavList>",
                ]) }
            </div>

            <div class="card">
                <h3>{ "NavText" }</h3>
                <p>{ "Static text elements in navigation:" }</p>
                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
                    <NavList>
                        <NavItem><NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>></NavItem>
                        <NavText text="v0.6.0" />
                    </NavList>
                </div>
                { code_block(&[
                    "use yew_nav_link::NavText;",
                    "",
                    "<NavText text=\"Version info\" />",
                ]) }
            </div>

            <div class="card">
                <h3>{ "Props" }</h3>
                <p><strong>{ "NavBadge" }</strong></p>
                <table>
                    <tr><td><code>{ "variant" }</code></td><td>{ "\"primary\", \"success\", \"danger\", \"warning\"" }</td></tr>
                    <tr><td><code>{ "pill" }</code></td><td>{ "bool — rounded pill style" }</td></tr>
                </table>
                <p style="margin-top:16px;"><strong>{ "NavHeader" }</strong></p>
                <table>
                    <tr><td><code>{ "text" }</code></td><td>{ "Header text" }</td></tr>
                </table>
                <p style="margin-top:16px;"><strong>{ "NavText" }</strong></p>
                <table>
                    <tr><td><code>{ "text" }</code></td><td>{ "Text content (required)" }</td></tr>
                </table>
            </div>
        </div>
    }
}
