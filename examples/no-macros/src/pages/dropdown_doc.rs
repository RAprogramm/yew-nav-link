use crate::doc_page::{DemoCard, DocPage, Tip};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{
    components::{NavDropdown, NavDropdownDivider, NavDropdownItem},
    NavItem, NavLink, NavList,
};

const SRC: &str = include_str!("../../../../src/components/dropdown.rs");

const CODE_DROPDOWN: &str = "\
use yew_nav_link::components::{NavDropdown, NavDropdownItem, NavDropdownDivider};
use yew_nav_link::{NavItem, NavLink, NavList};

# #[derive(Clone, PartialEq, Routable)]
# enum Route { #[at(\"/\")] Home }
#[component]
fn Nav() -> Html {
    html! {
        <NavList>
            <NavItem>
                <NavLink<Route> to={Route::Home}>{ \"Home\" }</NavLink<Route>>
            </NavItem>
            <NavDropdown toggle_text=\"Settings\">
                <NavDropdownItem>
                    <NavLink<Route> to={Route::Home}>{ \"Profile\" }</NavLink<Route>>
                </NavDropdownItem>
                <NavDropdownDivider />
                <NavDropdownItem disabled=true>
                    { \"Admin (disabled)\" }
                </NavDropdownItem>
            </NavDropdown>
        </NavList>
    }
}";

#[function_component]
pub fn DropdownDoc() -> Html {
    html! {
        <DocPage source={SRC.to_string()}>

            <DemoCard
                title="NavDropdown"
                description={html!{ "Click the toggle to open the menu:" }}
                code={CODE_DROPDOWN.to_string()}
                tip={html!{
                    <Tip>
                        { "Use " }<code>{ "<NavDropdownDivider />" }</code>
                        { " between groups. Set " }<code>{ "disabled=true" }</code>
                        { " on items to prevent selection." }
                    </Tip>
                }}
            >
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
            </DemoCard>

        </DocPage>
    }
}
