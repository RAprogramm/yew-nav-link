use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{
    components::{NavIcon, NavIconSize, NavLinkWithIcon},
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
pub fn IconDoc() -> Html {
    html! {
        <div>
            <div class="card">
                <h2>{ "NavIcon" }</h2>
                <p>{ "Icon wrapper with size variants for navigation items." }</p>
            </div>

            <div class="card">
                <h3>{ "Icon Sizes" }</h3>
                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
                    <NavList>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>
                                <NavIcon name="home" size={NavIconSize::Small} />
                                { " Small icon" }
                            </NavLink<Route>>
                        </NavItem>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>
                                <NavIcon name="gear" size={NavIconSize::Medium} />
                                { " Medium icon (default)" }
                            </NavLink<Route>>
                        </NavItem>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>
                                <NavIcon name="user" size={NavIconSize::Large} />
                                { " Large icon" }
                            </NavLink<Route>>
                        </NavItem>
                    </NavList>
                </div>

                { code_block(&[
                    "use yew_nav_link::{",
                    "    NavList, NavItem, NavLink,",
                    "    components::{NavIcon, NavIconSize}",
                    "};",
                    "",
                    "<NavIcon name=\"home\" size={NavIconSize::Small} />",
                    "<NavIcon name=\"gear\" size={NavIconSize::Medium} />",
                    "<NavIcon name=\"user\" size={NavIconSize::Large} />",
                ]) }
            </div>

            <div class="card">
                <h3>{ "NavLinkWithIcon" }</h3>
                <p>{ "Wrapper combining icon and content:" }</p>
                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
                    <NavLinkWithIcon icon={NavIconSize::Medium}>
                        <NavIcon name="star" />
                        { " Featured" }
                    </NavLinkWithIcon>
                </div>

                { code_block(&[
                    "use yew_nav_link::components::{NavLinkWithIcon, NavIcon, NavIconSize};",
                    "",
                    "<NavLinkWithIcon icon={NavIconSize::Medium}>",
                    "    <NavIcon name=\"star\" />",
                    "    { \"Featured\" }",
                    "</NavLinkWithIcon>",
                ]) }
            </div>

            <div class="card">
                <h3>{ "Props" }</h3>
                <p><strong>{ "NavIcon" }</strong></p>
                <table>
                    <tr><td><code>{ "name" }</code></td><td>{ "Icon name identifier" }</td></tr>
                    <tr><td><code>{ "size" }</code></td><td>{ "NavIconSize::Small / Medium / Large" }</td></tr>
                </table>
            </div>
        </div>
    }
}
