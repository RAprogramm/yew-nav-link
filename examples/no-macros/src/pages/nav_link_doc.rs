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

            // ── Exact Match ────────────────────────────────
            <div class="card">
                <h3>{ "Exact Match — Live Demo" }</h3>
                <p>{ "Click a link to see its active state in the popup." }</p>
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
                <p style="margin-top:0.5rem; color: var(--text-muted); font-size:0.8125rem;">
                    { "Only the link matching the current URL has " }
                    <code>{ "class=\"nav-link active\"" }</code>
                    { ". All others have " }
                    <code>{ "class=\"nav-link\"" }</code>
                    { "." }
                </p>
            </div>

            // ── Partial Match ──────────────────────────────
            <div class="card">
                <h3>{ "Partial Match — Live Demo" }</h3>
                <p>
                    { "With " }<code>{ "partial=true" }</code>
                    { ", the link stays active on any nested route." }
                </p>
                <DemoBox>
                    <nav>
                        <ul class="nav-list">
                            <li class="nav-item">
                                <NavLink<Route> to={Route::Home} partial=true>
                                    { "Home (partial)" }
                                </NavLink<Route>>
                            </li>
                        </ul>
                    </nav>
                </DemoBox>
                <p style="margin-top:0.5rem; color: var(--text-muted); font-size:0.8125rem;">
                    { "Partial matching compares segments: " }
                    <code>{ "/docs" }</code>
                    { " matches " }<code>{ "/docs/api" }</code>
                    { " but NOT " }<code>{ "/documentation" }</code>
                    { "." }
                </p>
            </div>

            // ── Function Syntax ────────────────────────────
            <div class="card">
                <h3>{ "Function Syntax — Live Demo" }</h3>
                <p>
                    { "Use " }<code>{ "nav_link()" }</code>
                    { " for text-only links without JSX:" }
                </p>
                <DemoBox>
                    <nav>
                        <ul class="nav-list">
                            <li class="nav-item">
                                { nav_link(Route::Home, "Home (Exact)", Match::Exact) }
                            </li>
                            <li class="nav-item">
                                { nav_link(Route::NavLinkDoc, "NavLink (Exact)", Match::Exact) }
                            </li>
                        </ul>
                    </nav>
                </DemoBox>
                <p style="margin-top:0.5rem; color: var(--text-muted); font-size:0.8125rem;">
                    { "The third argument controls matching: " }
                    <code>{ "Match::Exact" }</code>
                    { " or " }<code>{ "Match::Partial" }</code>
                    { "." }
                </p>
            </div>
        </div>
    }
}
