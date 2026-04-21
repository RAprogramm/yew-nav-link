use yew::prelude::*;
use yew_nav_link::components::{NavTab, NavTabPanel, NavTabs};

use crate::doc_page::{DemoCard, DocPage, Tip};

const SRC: &str = include_str!("../../../../src/components/tabs.rs");

const CODE_TABS: &str = r#"
use yew_nav_link::components::{NavTabs, NavTab, NavTabPanel};
use yew::prelude::*;

#[component]
fn App() -> Html {
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
        <>
            <NavTabs id="my-tabs">
                <NavTab active={*tab1} id="t1" panel_id="p1" onclick={Some(on_1)}>
                    { "Overview" }
                </NavTab>
                <NavTab active={*tab2} id="t2" panel_id="p2" onclick={Some(on_2)}>
                    { "Details" }
                </NavTab>
                <NavTab active=false disabled=true onclick={None}>
                    { "Disabled" }
                </NavTab>
            </NavTabs>
            <NavTabPanel id="p1" labelled_by="t1" hidden={!*tab1}>
                <p>{ "Panel 1 content" }</p>
            </NavTabPanel>
            <NavTabPanel id="p2" labelled_by="t2" hidden={!*tab2}>
                <p>{ "Panel 2 content" }</p>
            </NavTabPanel>
        </>
    }
}"#;

#[function_component]
pub fn TabsDoc() -> Html {
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
        <DocPage source={SRC.to_string()}>

            <DemoCard
                title="Interactive Tabs"
                description={html!{
                    <>
                        { "Click tabs to switch panels. Link with " }
                        <code>{ "id" }</code>{ "/" }<code>{ "panel_id" }</code>{ ":" }
                    </>
                }}
                code={CODE_TABS.to_string()}
                tip={html!{
                    <Tip>
                        { "Use " }<code>{ "full_width=true" }</code>
                        { " on " }<code>{ "<NavTabs>" }</code>
                        { " to stretch tabs across the container." }
                    </Tip>
                }}
            >
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
            </DemoCard>

        </DocPage>
    }
}
