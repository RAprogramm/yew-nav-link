use crate::demo_popup::DemoBox;
use crate::doc_parser::{parse_doc_block, DocRenderer};
use yew::prelude::*;
use yew_nav_link::components::{NavTab, NavTabPanel, NavTabs};

const TABS_SRC: &str = include_str!("../../../../src/components/tabs.rs");

#[function_component]
pub fn TabsDoc() -> Html {
    let doc = parse_doc_block(TABS_SRC);

    let tab1 = use_state(|| true);
    let tab2 = use_state(|| false);

    let on_1 = {
        let (t1, t2) = (tab1.clone(), tab2.clone());
        Callback::from(move |_: MouseEvent| {
            t1.set(true);
            t2.set(false);
        })
    };
    let on_2 = {
        let (t1, t2) = (tab1.clone(), tab2.clone());
        Callback::from(move |_: MouseEvent| {
            t1.set(false);
            t2.set(true);
        })
    };

    html! {
        <div>
            <DocRenderer {doc} />

            <div class="card">
                <h3>{ "Interactive Tabs — Live Demo" }</h3>
                <p>{ "Click tabs to switch panels. The " }<code>{ "onclick" }</code>{ " callback controls state:" }</p>
                <DemoBox>
                    <NavTabs id="demo-tabs">
                        <NavTab active={*tab1} id="dtab-1" panel_id="dpanel-1" onclick={Some(on_1)}>
                            { "Overview" }
                        </NavTab>
                        <NavTab active={*tab2} id="dtab-2" panel_id="dpanel-2" onclick={Some(on_2)}>
                            { "Details" }
                        </NavTab>
                        <NavTab active=false disabled=true onclick={None}>
                            { "Disabled" }
                        </NavTab>
                    </NavTabs>
                    <NavTabPanel id="dpanel-1" labelled_by="dtab-1" hidden={!*tab1}>
                        <p>{ "This is the overview panel." }</p>
                    </NavTabPanel>
                    <NavTabPanel id="dpanel-2" labelled_by="dtab-2" hidden={!*tab2}>
                        <p>{ "This is the details panel." }</p>
                    </NavTabPanel>
                </DemoBox>
                <p style="margin-top:0.5rem; color:var(--text-muted); font-size:0.8125rem;">
                    { "Link tabs to panels with " }<code>{ "id" }</code>{ "/" }<code>{ "panel_id" }</code>
                    { " and " }<code>{ "labelled_by" }</code>{ " for ARIA." }
                </p>
            </div>

            <div class="card">
                <h3>{ "Full Width Tabs — Live Demo" }</h3>
                <p>{ "Use " }<code>{ "full_width=true" }</code>{ " to stretch tabs across the container:" }</p>
                <DemoBox>
                    <NavTabs full_width=true>
                        <NavTab active=true onclick={None}>{ "Tab A" }</NavTab>
                        <NavTab active=false onclick={None}>{ "Tab B" }</NavTab>
                        <NavTab active=false onclick={None}>{ "Tab C" }</NavTab>
                    </NavTabs>
                </DemoBox>
            </div>
        </div>
    }
}
