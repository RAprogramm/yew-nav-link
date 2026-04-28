use yew::prelude::*;
use yew_nav_link::NavLink;

use crate::{
    doc_page::{DemoCard, DocPage, Tip},
    routes::Route
};

const NAV_LINK_SRC: &str = include_str!("../../../src/active_link/nav_link.rs");

const CODE_CUSTOM_CLASS: &str = r#"
use yew_nav_link::NavLink;
use yew_router::prelude::*;

#[component]
fn Nav() -> Html {
    html! {
        <nav>
            // Custom base class
            <NavLink<Route> to={Route::Home} class="menu-item">{ "Home" }</NavLink<Route>>
            
            // Custom active class
            <NavLink<Route> to={Route::About} active_class="is-selected">{ "About" }</NavLink<Route>>
            
            // Both custom
            <NavLink<Route> to={Route::Contact} class="sidebar-link" active_class="highlighted">{ "Contact" }</NavLink<Route>>
        </nav>
    }
}
"#;

const CODE_TAILWIND: &str = r#"
use yew_nav_link::NavLink;

#[component]
fn TailwindNav() -> Html {
    html! {
        <nav class="flex space-x-4">
            <NavLink<Route> 
                to={Route::Home} 
                class="px-4 py-2 rounded-lg hover:bg-gray-100" 
                active_class="bg-blue-600 text-white hover:bg-blue-700"
            >
                { "Home" }
            </NavLink<Route>>
            <NavLink<Route> 
                to={Route::About} 
                class="px-4 py-2 rounded-lg hover:bg-gray-100" 
                active_class="bg-blue-600 text-white hover:bg-blue-700"
            >
                { "About" }
            </NavLink<Route>>
        </nav>
    }
}
"#;

#[function_component]
pub fn CustomCssFeatures() -> Html {
    html! {
        <DocPage source={NAV_LINK_SRC.to_string()}>

            <div class="alert alert-info" style="margin-bottom: 2rem;">
                <strong>{ "New in v0.8.0" }</strong>
                { " — Customize the default CSS classes applied to navigation links." }
            </div>

            <DemoCard
                title="Custom CSS Classes"
                description={html!{
                    <>
                        { "Replace the default " }<code>{ "nav-link" }</code>
                        { " and " }<code>{ "active" }</code>
                        { " classes with your own:" }
                    </>
                }}
                code={CODE_CUSTOM_CLASS.to_string()}
                tip={html!{
                    <Tip>
                        { "Use " }<code>{ "class" }</code>
                        { " for the base class and " }<code>{ "active_class" }</code>
                        { " for the active state class." }
                    </Tip>
                }}
            >
                <nav class="demo-nav-custom">
                    <ul class="custom-nav-list">
                        <li>
                            <NavLink<Route> to={Route::Home} class="custom-link" active_class="custom-active">
                                { "Home (custom classes)" }
                            </NavLink<Route>>
                        </li>
                        <li>
                            <NavLink<Route> to={Route::CustomCssFeatures} class="custom-link" active_class="custom-active">
                                { "Custom CSS (custom classes)" }
                            </NavLink<Route>>
                        </li>
                    </ul>
                </nav>
            </DemoCard>

            <DemoCard
                title="Tailwind CSS Integration"
                description={html!{
                    <>
                        { "Perfect for Tailwind — pass utility classes directly:" }
                    </>
                }}
                code={CODE_TAILWIND.to_string()}
                tip={html!{
                    <Tip>
                        { "No need for " }<code>{ "@apply" }</code>
                        { " directives — use Tailwind classes inline." }
                    </Tip>
                }}
            >
                <nav class="demo-tailwind-nav">
                    <NavLink<Route>
                        to={Route::Home}
                        class="px-4 py-2 rounded-lg text-gray-700 hover:bg-gray-100 transition-colors"
                        active_class="bg-blue-600 text-white hover:bg-blue-700"
                    >
                        { "Home" }
                    </NavLink<Route>>
                    <NavLink<Route>
                        to={Route::CustomCssFeatures}
                        class="px-4 py-2 rounded-lg text-gray-700 hover:bg-gray-100 transition-colors"
                        active_class="bg-blue-600 text-white hover:bg-blue-700"
                    >
                        { "Custom CSS" }
                    </NavLink<Route>>
                    <NavLink<Route>
                        to={Route::NavigationHooks}
                        class="px-4 py-2 rounded-lg text-gray-700 hover:bg-gray-100 transition-colors"
                        active_class="bg-blue-600 text-white hover:bg-blue-700"
                    >
                        { "Navigation Hooks" }
                    </NavLink<Route>>
                </nav>
            </DemoCard>

            <DemoCard
                title="Comparison"
                description={html!{
                    <>
                        { "Default vs. custom classes side by side:" }
                    </>
                }}
                code={"// Default\n<NavLink<Route> to={Route::Home}>{ \"Home\" }</NavLink<Route>>\n\n// Custom\n<NavLink<Route> to={Route::Home} class=\"my-link\" active_class=\"my-active\">{ \"Home\" }</NavLink<Route>>".to_string()}
                tip={html! { <></> }}
            >
                <div class="comparison-container">
                    <div class="comparison-item">
                        <h5>{ "Default Classes" }</h5>
                        <nav>
                            <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                            <NavLink<Route> to={Route::CustomCssFeatures}>{ "Custom CSS" }</NavLink<Route>>
                        </nav>
                        <p class="comparison-code">{ "<a class=\"nav-link\">" }</p>
                        <p class="comparison-code">{ "<a class=\"nav-link active\">" }</p>
                    </div>
                    <div class="comparison-item">
                        <h5>{ "Custom Classes" }</h5>
                        <nav>
                            <NavLink<Route> to={Route::Home} class="pill-link" active_class="pill-active">
                                { "Home" }
                            </NavLink<Route>>
                            <NavLink<Route> to={Route::CustomCssFeatures} class="pill-link" active_class="pill-active">
                                { "Custom CSS" }
                            </NavLink<Route>>
                        </nav>
                        <p class="comparison-code">{ "<a class=\"pill-link\">" }</p>
                        <p class="comparison-code">{ "<a class=\"pill-link pill-active\">" }</p>
                    </div>
                </div>
            </DemoCard>

        </DocPage>
    }
}
