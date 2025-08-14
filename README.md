<h1>NavLink Component for Yew Router</h1>


<div>
<a href="https://crates.io/crates/yew-nav-link"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/yew-nav-link"></a>
<a href="https://docs.rs/yew-nav-link/0.2.3/yew_nav_link"><img alt="docs.rs" src="https://img.shields.io/docsrs/yew-nav-link"></a>
<a href="https://crates.io/crates/yew-nav-link"><img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/yew-nav-link"></a>
<a href="https://releases.rs/docs/1.89.0/"><img alt="Crates.io MSRV" src="https://img.shields.io/crates/msrv/yew-nav-link"></a>
<a href="https://github.com/RAprogramm/yew-nav-link"><img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/RAprogramm/yew-nav-link"></a>
</div>
<hr>

<p>NavLink is a component for Yew applications using Yew Router. It creates a navigational link that is aware of its active state based on the current route in the application.</p>
<hr>


<h2>Usage</h2>

```rs
use yew::{html, prelude::*};
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <nav>
            <NavLink<AppRoute> to={AppRoute::Home}>{ "Home" }</NavLink<AppRoute>>
            <NavLink<AppRoute> to={AppRoute::About}>{ "About" }</NavLink<AppRoute>>
            <!-- ... other NavLinks -->
        </nav>
    }
}
```

<br>
<hr></hr>


<h2>Examples</h2>

### Using *nav_link* function

> <details>
> <summary>in header or navbar</summary>
>
> ```html
>     ...
>     <li class="nav-item">
>         { nav_link(HomeRoute::IntroPage, "Home") }
>     </li>
>     <!-- same  
>     <li class="nav-item">
>      <NavLink<HomeRoute> to={HomeRoute::IntroPage}>
>          {"Home"}
>      </NavLink<HomeRoute>>
>     </li>
>     -->
>     ...
> ```
> </details>

### Advanced example with [bootstrap](https://getbootstrap.com/)

> [![yew](https://shields.io/badge/yew-0.21.0-darkgreen)](https://docs.rs/yew/0.21.0/yew/index.html)
> [![yew-router](https://shields.io/badge/yew_router-0.18.0-darkgreen)](https://docs.rs/yew-router/0.18.0/yew_router/index.html)
> 
> <details>
> <summary>index.html</summary>
> 
> ```html
> <!doctype html>
> <html lang="en">
> 
> <head>
>   <meta charset="UTF-8" />
>   <meta http-equiv="X-UA-Compatible" content="IE=edge" />
>   <meta name="viewport" content="width=device-width, initial-scale=1.0" />
> 
>   <title>Your title</title>
> 
>   <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN" crossorigin="anonymous" />
> </head>
> 
> <body>
>   <script src="https://cdn.jsdelivr.net/npm/@popperjs/core@2.11.8/dist/umd/popper.min.js" integrity="sha384-I7E8VVD/ismYTF4hNIPjVp/Zjvgyol6VFvRkX/vR+Vc4jQkC+hVqc2pM8ODewa9r" crossorigin="anonymous"></script>
>   <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.min.js" integrity="sha384-BBtl+eGJRgqQAUMxJ7pMwbEyER4l1g+O15P+16Ep7Q9Q+zqX6gSbd85u4mG4QzX+" crossorigin="anonymous"></script>
> </body>
> 
> </html>
> ```
> </details>
> 
> <details>
> <summary>main.rs</summary>
> 
> ```rs
> fn main() {
>     yew::Renderer::<app::App>::new().render();
> }
> ```
> </details>
> 
> <details>
> <summary>app.rs</summary>
> 
> ```rs
> #[function_component(App)]
> pub fn app() -> Html {
>     html! {
>         <BrowserRouter>
>             <Switch<MainRoute> render={switch_main} />
>         </BrowserRouter>
>     }
> }
> ```
> </details>
> 
> <details>
> <summary>main_routes.rs</summary>
> 
> ```rs
> #[derive(Clone, Routable, PartialEq)]
> pub enum MainRoute {
>     #[at("/home")]
>     HomeRoot,
>     #[at("/home/*")]
>     Home,
>     #[at("/register")]
>     RegisterPage,
>     #[at("/login")]
>     LoginPage,
>     #[not_found]
>     #[at("/404")]
>     NotFoundPage,
> }
> 
> pub fn switch_main(routes: MainRoute) -> Html {
>     match routes {
>         MainRoute::HomeRoot | MainRoute::Home => {
>             html! { <Switch<HomeRoute> render={switch_home} /> }
>         }
>         MainRoute::RegisterPage => html! { <HomeLayout> {html! { <RegisterPage/> }} </HomeLayout> },
>         MainRoute::LoginPage => html! { <HomeLayout> {html! { <LoginPage/> }} </HomeLayout> },
>         MainRoute::NotFoundPage => html! { <NotFoundPage/> },
>     }
> }
> ```
> 
> </details>
> 
> <details>
> <summary>home_routes.rs</summary>
> 
> ```rs
> #[derive(Clone, Routable, PartialEq)]
> pub enum HomeRoute {
>     #[at("/home")]
>     HomePage,
>     #[at("/home/intro")]
>     IntroPage,
>     #[at("/home/features")]
>     FeaturesPage,
>     #[at("/home/billings")]
>     BillingsPage,
>     #[at("/home/faq")]
>     FaqPage,
>     #[not_found]
>     #[at("/home/404")]
>     NotFoundPage,
> }
> 
> pub fn switch_home(route: HomeRoute) -> Html {
>     match route {
>         HomeRoute::HomePage => html! {<Intro/>},
>         HomeRoute::IntroPage => html! { <HomeLayout> { html! { <Intro/> } } </HomeLayout> },
>         HomeRoute::FeaturesPage => html! { <HomeLayout> { html! { <Features/> } } </HomeLayout> },
>         HomeRoute::BillingsPage => html! { <HomeLayout> { html! { <Billings/> } } </HomeLayout> },
>         HomeRoute::FaqPage => html! { <HomeLayout> { html! { <FAQ/> } } </HomeLayout> },
>         HomeRoute::NotFoundPage => html! {<Redirect<MainRoute> to={MainRoute::NotFoundPage}/>},
>     }
> }
> ```
> </details>
> 
> <details>
> <summary>in navbar or header</summary>
> 
> ```html
>     ...
>     <ul class="nav nav-pills d-inline-flex mt-2 mt-md-0 ms-md-auto" style="justify-content:center;">
>         <li class="nav-item">
>             <NavLink<HomeRoute> to={HomeRoute::IntroPage}>
>                 {"Home"}
>             </NavLink<HomeRoute>>
>         </li>
>         <li class="nav-item">
>             <NavLink<HomeRoute> to={HomeRoute::FeaturesPage}>
>                 {"Features"}
>             </NavLink<HomeRoute>>
>         </li>
>         <li class="nav-item">
>             <NavLink<HomeRoute> to={HomeRoute::BillingsPage}>
>                 {"Billing"}
>             </NavLink<HomeRoute>>
>         </li>
>         <li class="nav-item">
>             <NavLink<HomeRoute> to={HomeRoute::FaqPage}>
>                 {"FAQ"}
>             </NavLink<HomeRoute>>
>         </li>
>         <li class="nav-item">
>             <NavLink<MainRoute> to={MainRoute::RegisterPage}>
>                 {"Register"}
>             </NavLink<MainRoute>>
>         </li>
>         <li class="nav-item">
>             <NavLink<MainRoute> to={MainRoute::LoginPage}>
>                 {"Login"}
>             </NavLink<MainRoute>>
>         </li>
>     </ul>
>     ...
> ```
> </details>


<h2>License</h2>

<p>This project is licensed under the <a href="LICENSE">Apache License 2.0</a>.</p>
