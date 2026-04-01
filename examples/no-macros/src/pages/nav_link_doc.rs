use crate::demo_popup::DemoBox;
use crate::doc_parser::{parse_doc_block, DocRenderer};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{nav_link, Match, NavLink};

const NAV_LINK_SRC: &str = include_str!("../../../../src/nav_link.rs");

#[function_component]
pub fn NavLinkDoc() -> Html {
    let doc = parse_doc_block(NAV_LINK_SRC);

    html! {
        <div>
            <DocRenderer {doc} />

            <div class="card">
                <h3>{ "Live Demo — Exact Match" }</h3>
                <DemoBox>
                    <nav>
                        <ul class="nav-list">
                            <li class="nav-item">
                                <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                            </li>
                            <li class="nav-item">
                                <NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>>
                            </li>
                            <li class="nav-item">
                                <NavLink<Route> to={Route::NavListDoc}>{ "NavList" }</NavLink<Route>>
                            </li>
                        </ul>
                    </nav>
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "Live Demo — Partial Match" }</h3>
                <DemoBox>
                    <nav>
                        <ul class="nav-list">
                            <li class="nav-item">
                                <NavLink<Route> to={Route::Home} partial=true>
                                    { "Home (partial) — stays active on nested" }
                                </NavLink<Route>>
                            </li>
                        </ul>
                    </nav>
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "Live Demo — Function Syntax" }</h3>
                <DemoBox>
                    <nav>
                        <ul class="nav-list">
                            <li>{ nav_link(Route::Home, "Home (Exact)", Match::Exact) }</li>
                            <li>{ nav_link(Route::NavLinkDoc, "NavLink Doc (Exact)", Match::Exact) }</li>
                        </ul>
                    </nav>
                </DemoBox>
            </div>
        </div>
    }
}
