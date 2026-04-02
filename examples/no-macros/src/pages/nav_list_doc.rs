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
                <h3>{ "NavList — Live Demo" }</h3>
                <p>{ "Wrap " }<code>{ "<NavLink>" }</code>{ " items inside " }<code>{ "<NavList>" }</code>{ ":" }</p>
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
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Renders " }<code>{ "<ul role=\"list\" aria-label=\"...\">" }</code>
                    { ". Each child gets " }<code>{ "<li role=\"listitem\">" }</code>{ "." }
                </p>
            </div>

            <div class="card">
                <h3>{ "Disabled Items — Live Demo" }</h3>
                <p>{ "Use " }<code>{ "disabled=true" }</code>{ " on " }<code>{ "<NavItem>" }</code>{ ":" }</p>
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
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Disabled items get " }<code>{ "class=\"nav-item disabled\"" }</code>{ "." }
                </p>
            </div>
        </div>
    }
}
