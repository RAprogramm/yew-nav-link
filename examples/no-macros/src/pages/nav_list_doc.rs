use crate::demo_popup::DemoBox;
use crate::doc_parser::{parse_doc_block, DocRenderer};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{NavItem, NavLink, NavList};

const NAV_LIST_SRC: &str = include_str!("../../../../src/nav/list.rs");

#[function_component]
pub fn NavListDoc() -> Html {
    let doc = parse_doc_block(NAV_LIST_SRC);

    html! {
        <div>
            <DocRenderer {doc} />

            <div class="card">
                <h3>{ "Live Demo — Basic Usage" }</h3>
                <DemoBox>
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
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "Live Demo — Disabled Items" }</h3>
                <DemoBox>
                    <NavList>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>{ "Enabled" }</NavLink<Route>>
                        </NavItem>
                        <NavItem disabled=true>
                            <NavLink<Route> to={Route::NotFound}>{ "Disabled Item" }</NavLink<Route>>
                        </NavItem>
                    </NavList>
                </DemoBox>
            </div>
        </div>
    }
}
