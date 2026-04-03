use super::metadata::CrateMeta;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeroProps {
    pub meta: CrateMeta,
}

#[function_component]
pub fn Hero(props: &HeroProps) -> Html {
    let m = props.meta.clone();
    html! {
        <div class="hero">
            <h1>{ "yew-nav-link" }</h1>
            <p>{ m.desc }</p>
            <div class="hero-badges">
                <div class="tag tag-version">
                    <span class="tag-label">{ "version" }</span>
                    <span class="tag-value">{ m.version }</span>
                </div>
                <div class="tag tag-yew">
                    <span class="tag-label">{ "yew" }</span>
                    <span class="tag-value">{ m.yew_ver }</span>
                </div>
                <div class="tag tag-router">
                    <span class="tag-label">{ "yew-router" }</span>
                    <span class="tag-value">{ m.router_ver }</span>
                </div>
                <div class="tag tag-edition">
                    <span class="tag-label">{ "edition" }</span>
                    <span class="tag-value">{ m.edition }</span>
                </div>
                <div class="tag tag-msrv">
                    <span class="tag-label">{ "MSRV" }</span>
                    <span class="tag-value">{ m.msrv }</span>
                </div>
                <div class="tag tag-license">
                    <span class="tag-label">{ "license" }</span>
                    <span class="tag-value">{ m.license }</span>
                </div>
                <div class="tag tag-wasm">
                    <span class="tag-label">{ "target" }</span>
                    <span class="tag-value">{ "wasm32" }</span>
                </div>
                <div class="tag tag-no-std">
                    <span class="tag-label">{ "crate-type" }</span>
                    <span class="tag-value">{ "cdylib + rlib" }</span>
                </div>
            </div>
            <div class="hero-links">
                <div class="tag tag-github">
                    <span class="tag-label">{ "GitHub" }</span>
                    <a href={m.repository} target="_blank" rel="noopener noreferrer" class="tag-value-link">
                        { "repo" }
                    </a>
                </div>
                <div class="tag tag-docsrs">
                    <span class="tag-label">{ "docs.rs" }</span>
                    <a href={m.documentation} target="_blank" rel="noopener noreferrer" class="tag-value-link">
                        { "docs" }
                    </a>
                </div>
                <div class="tag tag-crates">
                    <span class="tag-label">{ "crates.io" }</span>
                    <a href="https://crates.io/crates/yew-nav-link" target="_blank" rel="noopener noreferrer" class="tag-value-link">
                        { "crate" }
                    </a>
                </div>
                <div class="tag tag-codecov">
                    <span class="tag-label">{ "Codecov" }</span>
                    <a href="https://codecov.io/gh/RAprogramm/yew-nav-link" target="_blank" rel="noopener noreferrer" class="tag-value-link">
                        { "coverage" }
                    </a>
                </div>
            </div>
        </div>
    }
}
