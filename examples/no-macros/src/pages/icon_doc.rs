use crate::demo_popup::DemoBox;
use crate::doc_parser::{parse_doc_block, DocRenderer};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{
    components::{NavIcon, NavIconSize, NavLinkWithIcon},
    NavItem, NavLink, NavList,
};

const ICON_SRC: &str = include_str!("../../../../src/components/icon.rs");

#[function_component]
pub fn IconDoc() -> Html {
    let doc = parse_doc_block(ICON_SRC);

    html! {
        <div>
            <DocRenderer {doc} />

            <div class="card">
                <h3>{ "NavIcon Sizes — Live Demo" }</h3>
                <p>{ "Three sizes: " }<code>{ "Small" }</code>{ ", " }<code>{ "Medium" }</code>{ ", " }<code>{ "Large" }</code>{ ":" }</p>
                <DemoBox>
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
                </DemoBox>
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Renders an " }<code>{ "<i aria-hidden=\"true\">" }</code>
                    { " with size class " }<code>{ "nav-icon-sm" }</code>{ "/" }<code>{ "nav-icon-md" }</code>{ "/" }<code>{ "nav-icon-lg" }</code>{ "." }
                </p>
            </div>

            <div class="card">
                <h3>{ "NavLinkWithIcon — Live Demo" }</h3>
                <p>{ "Wrap icon + text together:" }</p>
                <DemoBox>
                    <NavLinkWithIcon icon={NavIconSize::Medium}>
                        <NavIcon name="star" />
                        { " Featured" }
                    </NavLinkWithIcon>
                </DemoBox>
            </div>
        </div>
    }
}
