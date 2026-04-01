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
                <h3>{ "Live Demo — Basic Divider" }</h3>
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
            </div>

            <div class="card">
                <h3>{ "Live Demo — Divider with Text" }</h3>
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
            </div>
        </div>
    }
}
