use crate::demo_popup::DemoBox;
use crate::doc_parser::{parse_doc_block, DocRenderer};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{NavBadge, NavHeader, NavItem, NavLink, NavList, NavText};

const BADGE_SRC: &str = include_str!("../../../../src/components/badge.rs");

#[function_component]
pub fn BadgeDoc() -> Html {
    let doc = parse_doc_block(BADGE_SRC);

    html! {
        <div>
            <DocRenderer {doc} />

            <div class="card">
                <h3>{ "Live Demo — NavBadge" }</h3>
                <DemoBox>
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
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "Live Demo — NavHeader" }</h3>
                <DemoBox>
                    <NavList>
                        <NavHeader text="Main" />
                        <NavItem><NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>></NavItem>
                        <NavHeader text="Documentation" />
                        <NavItem><NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>></NavItem>
                    </NavList>
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "Live Demo — NavText" }</h3>
                <DemoBox>
                    <NavList>
                        <NavItem><NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>></NavItem>
                        <NavText text="v0.6.0" />
                    </NavList>
                </DemoBox>
            </div>
        </div>
    }
}
