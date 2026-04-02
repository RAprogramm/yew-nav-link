use crate::demo_popup::DemoBox;
use crate::doc_parser::{parse_doc_block, DocRenderer};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{
    components::{NavDropdown, NavDropdownDivider, NavDropdownItem},
    NavItem, NavLink, NavList,
};

const DROPDOWN_SRC: &str = include_str!("../../../../src/components/dropdown.rs");

#[function_component]
pub fn DropdownDoc() -> Html {
    let doc = parse_doc_block(DROPDOWN_SRC);

    html! {
        <div>
            <DocRenderer {doc} />

            <div class="card">
                <h3>{ "NavDropdown — Live Demo" }</h3>
                <p>{ "Click the toggle to open the menu:" }</p>
                <DemoBox>
                    <NavList>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                        </NavItem>
                        <NavDropdown toggle_text="Settings">
                            <NavDropdownItem>
                                <NavLink<Route> to={Route::Home}>{ "Profile" }</NavLink<Route>>
                            </NavDropdownItem>
                            <NavDropdownItem>
                                <NavLink<Route> to={Route::Home}>{ "Security" }</NavLink<Route>>
                            </NavDropdownItem>
                            <NavDropdownDivider />
                            <NavDropdownItem disabled=true>
                                { "Admin (disabled)" }
                            </NavDropdownItem>
                        </NavDropdown>
                    </NavList>
                </DemoBox>
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Use " }<code>{ "<NavDropdownDivider />" }</code>
                    { " between groups. Set " }<code>{ "disabled=true" }</code>
                    { " on items to prevent selection." }
                </p>
            </div>
        </div>
    }
}
