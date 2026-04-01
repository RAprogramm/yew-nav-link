use yew::prelude::*;
use yew_nav_link::components::{NavTab, NavTabPanel, NavTabs};

fn code_block(lines: &[&str]) -> Html {
    html! {
        <pre><code>
            { for lines.iter().map(|l| html! { <>{ l }{"\n"}</> }) }
        </code></pre>
    }
}

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
        <div>
            <div class="card">
                <h2>{ "NavTabs / NavTab / NavTabPanel" }</h2>
                <p>{ "Accessible tab navigation with ARIA roles and keyboard support." }</p>
            </div>

            <div class="card">
                <h3>{ "Interactive Tabs" }</h3>
                <div class="demo">
                    <div class="demo-label">{ "Live Demo — click tabs" }</div>
                    <NavTabs id="demo-tabs">
                        <NavTab
                            active={*tab1}
                            id="dtab-1"
                            panel_id="dpanel-1"
                            onclick={Some(on_1)}
                        >
                            { "Overview" }
                        </NavTab>
                        <NavTab
                            active={*tab2}
                            id="dtab-2"
                            panel_id="dpanel-2"
                            onclick={Some(on_2)}
                        >
                            { "Details" }
                        </NavTab>
                        <NavTab active=false disabled=true onclick={None}>
                            { "Disabled" }
                        </NavTab>
                    </NavTabs>
                    <NavTabPanel id="dpanel-1" labelled_by="dtab-1" hidden={!*tab1}>
                        <p>{ "This is the overview panel. NavTabs renders a semantic <ul> with role=tablist." }</p>
                    </NavTabPanel>
                    <NavTabPanel id="dpanel-2" labelled_by="dtab-2" hidden={!*tab2}>
                        <p>{ "This is the details panel. Each NavTab has aria-selected and aria-controls." }</p>
                    </NavTabPanel>
                </div>

                { code_block(&[
                    "use yew_nav_link::components::{NavTabs, NavTab, NavTabPanel};",
                    "",
                    "let active_tab = use_state(|| 0u8);",
                    "",
                    "html! {",
                    "    <NavTabs id=\"my-tabs\">",
                    "        <NavTab",
                    "            active={*active_tab == 0}",
                    "            id=\"tab-1\"",
                    "            panel_id=\"panel-1\"",
                    "            onclick={Some(on_tab_1)}",
                    "        >",
                    "            { \"Tab 1\" }",
                    "        </NavTab>",
                    "        <NavTab active=false onclick={None}>",
                    "            { \"Tab 2\" }",
                    "        </NavTab>",
                    "    </NavTabs>",
                    "    <NavTabPanel",
                    "        id=\"panel-1\"",
                    "        labelled_by=\"tab-1\"",
                    "        hidden={*active_tab != 0}",
                    "    >",
                    "        <p>{ \"Content\" }</p>",
                    "    </NavTabPanel>",
                    "}",
                ]) }
            </div>

            <div class="card">
                <h3>{ "Full Width Tabs" }</h3>
                <div class="demo">
                    <div class="demo-label">{ "Live Demo" }</div>
                    <NavTabs full_width=true>
                        <NavTab active=true onclick={None}>{ "Tab A" }</NavTab>
                        <NavTab active=false onclick={None}>{ "Tab B" }</NavTab>
                        <NavTab active=false onclick={None}>{ "Tab C" }</NavTab>
                    </NavTabs>
                </div>
                { code_block(&[
                    "<NavTabs full_width=true>",
                    "    <NavTab active=true onclick={None}>",
                    "        { \"Tab A\" }",
                    "    </NavTab>",
                    "</NavTabs>",
                ]) }
            </div>

            <div class="card">
                <h3>{ "Props" }</h3>
                <p><strong>{ "NavTabs" }</strong></p>
                <table>
                    <tr><td><code>{ "full_width" }</code></td><td>{ "bool — stretch tabs to fill width" }</td></tr>
                    <tr><td><code>{ "id" }</code></td><td>{ "Element id" }</td></tr>
                </table>
                <p style="margin-top:16px;"><strong>{ "NavTab" }</strong></p>
                <table>
                    <tr><td><code>{ "active" }</code></td><td>{ "bool (required)" }</td></tr>
                    <tr><td><code>{ "disabled" }</code></td><td>{ "bool" }</td></tr>
                    <tr><td><code>{ "id" }</code></td><td>{ "Tab button id" }</td></tr>
                    <tr><td><code>{ "panel_id" }</code></td><td>{ "aria-controls target" }</td></tr>
                    <tr><td><code>{ "onclick" }</code></td><td>{ "Callback<MouseEvent>" }</td></tr>
                </table>
                <p style="margin-top:16px;"><strong>{ "NavTabPanel" }</strong></p>
                <table>
                    <tr><td><code>{ "hidden" }</code></td><td>{ "bool (required)" }</td></tr>
                    <tr><td><code>{ "id" }</code></td><td>{ "Panel id" }</td></tr>
                    <tr><td><code>{ "labelled_by" }</code></td><td>{ "aria-labelledby target" }</td></tr>
                </table>
            </div>
        </div>
    }
}
