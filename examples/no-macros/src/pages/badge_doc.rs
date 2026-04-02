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
                <h3>{ "NavBadge — Live Demo" }</h3>
                <p>{ "Place " }<code>{ "<NavBadge>" }</code>{ " inside a link to show counts or status." }</p>
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
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Variants: " }<code>{ "primary" }</code>{ ", " }<code>{ "danger" }</code>
                    { ", " }<code>{ "success" }</code>{ ", " }<code>{ "warning" }</code>
                    { ". Use " }<code>{ "pill=true" }</code>{ " for rounded shape." }
                </p>
            </div>

            <div class="card">
                <h3>{ "NavHeader — Live Demo" }</h3>
                <p>{ "Label sections inside a navigation list:" }</p>
                <DemoBox>
                    <NavList>
                        <NavHeader text="Main" />
                        <NavItem><NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>></NavItem>
                        <NavHeader text="Documentation" />
                        <NavItem><NavLink<Route> to={Route::NavLinkDoc}>{ "NavLink" }</NavLink<Route>></NavItem>
                    </NavList>
                </DemoBox>
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Renders a " }<code>{ "<li role=\"presentation\">" }</code>
                    { " with " }<code>{ "class=\"nav-header\"" }</code>{ "." }
                </p>
            </div>

            <div class="card">
                <h3>{ "NavText — Live Demo" }</h3>
                <p>{ "Static text label, useful for version info or notes:" }</p>
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
