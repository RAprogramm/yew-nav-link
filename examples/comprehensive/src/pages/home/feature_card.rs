use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct FeatureCardProps {
    pub route: Route,
    pub title: String,
    pub desc: String,
}

#[function_component]
pub fn FeatureCard(props: &FeatureCardProps) -> Html {
    let route = props.route.clone();
    html! {
        <Link<Route> to={route.clone()} classes="feature-card-link">
            <div class="feature-card">
                <h4>{ &props.title }</h4>
                <p>{ &props.desc }</p>
            </div>
        </Link<Route>>
    }
}
