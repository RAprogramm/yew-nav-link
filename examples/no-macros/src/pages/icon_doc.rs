use crate::demo_popup::DemoBox;
use crate::doc_parser::{parse_doc_block, DocRenderer};
use crate::routes::Route;
use yew::prelude::*;
use yew_nav_link::{
    components::{NavIcon, NavIconSize, NavLinkWithIcon},
    NavItem, NavLink, NavList,
};

const ICON_SRC: &str = include_str!("../../../../src/components/icon.rs");

#[function_component]
pub fn IconDoc() -> Html {
    let doc = parse_doc_block(ICON_SRC);

    html! {
        <div>
            <DocRenderer {doc} />

            <div class="card">
                <h3>{ "Live Demo — Icon Sizes" }</h3>
                <DemoBox>
                    <NavList>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>
                                <NavIcon name="home" size={NavIconSize::Small} />
                                { " Small icon" }
                            </NavLink<Route>>
                        </NavItem>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>
                                <NavIcon name="gear" size={NavIconSize::Medium} />
                                { " Medium icon (default)" }
                            </NavLink<Route>>
                        </NavItem>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>
                                <NavIcon name="user" size={NavIconSize::Large} />
                                { " Large icon" }
                            </NavLink<Route>>
                        </NavItem>
                    </NavList>
                </DemoBox>
            </div>

            <div class="card">
                <h3>{ "Live Demo — NavLinkWithIcon" }</h3>
                <DemoBox>
                    <NavLinkWithIcon icon={NavIconSize::Medium}>
                        <NavIcon name="star" />
                        { " Featured" }
                    </NavLinkWithIcon>
                </DemoBox>
            </div>
        </div>
    }
}
