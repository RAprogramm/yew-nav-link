use crate::demo_popup::DemoBox;
use crate::doc_parser::{parse_doc_block, DocRenderer};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{NavDivider, NavItem, NavLink, NavList};

const NAV_DIVIDER_SRC: &str = include_str!("../../../../src/nav/divider.rs");

#[function_component]
pub fn NavDividerDoc() -> Html {
    let doc = parse_doc_block(NAV_DIVIDER_SRC);

    html! {
        <div>
            <DocRenderer {doc} />

            <div class="card">
                <h3>{ "Basic Divider — Live Demo" }</h3>
                <p>{ "Place " }<code>{ "<NavDivider />" }</code>{ " between items to separate sections:" }</p>
                <DemoBox>
                    <NavList>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                        </NavItem>
                        <NavDivider />
                        <NavItem>
                            <NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>>
                        </NavItem>
                    </NavList>
                </DemoBox>
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Renders " }<code>{ "<li role=\"separator\">" }</code>
                    { " with a top border." }
                </p>
            </div>

            <div class="card">
                <h3>{ "Divider with Text — Live Demo" }</h3>
                <p>{ "Add a label with " }<code>{ "text=\"...\"" }</code>{ ":" }</p>
                <DemoBox>
                    <NavList>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                        </NavItem>
                        <NavDivider text="Section" />
                        <NavItem>
                            <NavLink<Route> to={Route::BadgeDoc}>{ "Badge" }</NavLink<Route>>
                        </NavItem>
                    </NavList>
                </DemoBox>
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Text is centered on the divider line. Useful for grouping nav items." }
                </p>
            </div>
        </div>
    }
}
