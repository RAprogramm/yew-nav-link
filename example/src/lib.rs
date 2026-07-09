// SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_nav_link::{
    BreadcrumbItem, BreadcrumbLabelProvider, BreadcrumbLabelProviderContext, Match, NavBadge,
    NavDivider, NavDropdown, NavDropdownDivider, NavDropdownItem, NavError, NavHeader, NavIcon,
    NavIconSize, NavItem, NavLink, NavLinkWithIcon, NavList, NavResult, NavTab, NavTabPanel,
    NavTabs, NavText, Navigation, Pagination, is_absolute, join_paths, nav_link, normalize_path,
    use_breadcrumbs, use_is_active, use_is_exact_active, use_is_partial_active, use_navigation,
    use_query_params, use_route_info,
    utils::{urlencoding_decode, urlencoding_encode}
};
use yew_router::prelude::*;

// ============================================================================
// Routes
// ============================================================================

#[derive(Clone, PartialEq, Debug, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/navlink")]
    NavLink,
    #[at("/navlink/lab/:slug")]
    NavLinkLab { slug: String },
    #[at("/components")]
    Components,
    #[at("/hooks")]
    Hooks,
    #[at("/hooks/team/:team")]
    HooksTeam { team: String },
    #[at("/utilities")]
    Utilities,
    #[not_found]
    #[at("/404")]
    NotFound
}

// ============================================================================
// DemoCard — the building block: title + description + live preview + code
// ============================================================================

#[wasm_bindgen]
extern "C" {
    /// Re-runs highlight.js across the document. Loaded from CDN in
    /// `index.html`; we trigger it after Yew has mounted any fresh code
    /// blocks. Safe to call repeatedly — hljs marks already-highlighted
    /// nodes with a sentinel class.
    #[wasm_bindgen(js_namespace = hljs, js_name = highlightAll, catch)]
    fn highlight_all() -> Result<(), JsValue>;
}

/// Writes `text` to the system clipboard. Fire-and-forget — we don't await
/// the returned `Promise`. On browsers without `navigator.clipboard` the
/// call is a no-op.
fn write_to_clipboard(text: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.navigator().clipboard().write_text(text);
    }
}

#[derive(Properties, PartialEq)]
struct CopyButtonProps {
    text: AttrValue
}

#[function_component]
fn CopyButton(props: &CopyButtonProps) -> Html {
    let copied = use_state(|| false);

    let onclick = {
        let text = props.text.clone();
        let copied = copied.clone();
        Callback::from(move |_: MouseEvent| {
            write_to_clipboard(&text);
            copied.set(true);
            let copied_revert = copied.clone();
            gloo_timers::callback::Timeout::new(1500, move || {
                copied_revert.set(false);
            })
            .forget();
        })
    };

    let (label, class) = if *copied {
        ("Copied", "demo-card__copy demo-card__copy--success")
    } else {
        ("Copy", "demo-card__copy")
    };

    html! {
        <button
            type="button"
            class={class}
            onclick={onclick}
            aria-label="Copy code to clipboard"
        >
            { label }
        </button>
    }
}

#[derive(Properties, PartialEq)]
struct DemoCardProps {
    title:       AttrValue,
    #[prop_or_default]
    description: Option<AttrValue>,
    /// Optional language hint passed to highlight.js (defaults to `rust`).
    #[prop_or(AttrValue::Static("rust"))]
    language:    AttrValue,
    /// When `true` the card stacks preview-then-code so wide components
    /// (tabs, pagination, dropdown menus) get full card width.
    #[prop_or(false)]
    wide:        bool,
    code:        AttrValue,
    children:    Children
}

#[function_component]
fn DemoCard(props: &DemoCardProps) -> Html {
    // Re-highlight on every mount. Cheap; hljs deduplicates internally.
    use_effect_with(props.code.clone(), |_| {
        let _ = highlight_all();
        || ()
    });

    let code_class = format!("language-{}", props.language);
    let card_class = if props.wide {
        "demo-card demo-card--wide"
    } else {
        "demo-card"
    };

    html! {
        <article class={card_class}>
            <header class="demo-card__head">
                <h3 class="demo-card__title">{ props.title.clone() }</h3>
                if let Some(desc) = &props.description {
                    <p class="demo-card__desc">{ desc }</p>
                }
            </header>
            <div class="demo-card__body">
                <div class="demo-card__preview" aria-label="Live preview">
                    { for props.children.iter() }
                </div>
                <div class="demo-card__code-wrap">
                    <CopyButton text={props.code.clone()} />
                    <pre class="demo-card__code" aria-label="Source code">
                        <code class={code_class}>{ props.code.clone() }</code>
                    </pre>
                </div>
            </div>
        </article>
    }
}

// ============================================================================
// Page chrome
// ============================================================================

#[derive(Properties, PartialEq)]
struct PageHeaderProps {
    title:    AttrValue,
    subtitle: AttrValue
}

#[function_component]
fn PageHeader(props: &PageHeaderProps) -> Html {
    html! {
        <header class="page-header">
            <h1>{ props.title.clone() }</h1>
            <p>{ props.subtitle.clone() }</p>
        </header>
    }
}

#[derive(Properties, PartialEq)]
struct PageSectionProps {
    title:    AttrValue,
    #[prop_or_default]
    intro:    Option<AttrValue>,
    children: Children
}

#[function_component]
fn PageSection(props: &PageSectionProps) -> Html {
    html! {
        <section class="page-section">
            <h2 class="page-section__title">{ props.title.clone() }</h2>
            if let Some(intro) = &props.intro {
                <p class="page-section__intro">{ intro }</p>
            }
            { for props.children.iter() }
        </section>
    }
}

// ============================================================================
// Top navigation
// ============================================================================

#[function_component]
fn TopNav() -> Html {
    html! {
        <nav class="top-nav" aria-label="Main navigation">
            <div class="top-nav__inner">
                <NavLink<Route> to={Route::Home} class="top-nav__brand">
                    { "yew-nav-link" }
                </NavLink<Route>>
                <ul class="top-nav__links">
                    <li>
                        <NavLink<Route> to={Route::NavLink} partial=true>
                            { "NavLink" }
                        </NavLink<Route>>
                    </li>
                    <li>
                        <NavLink<Route> to={Route::Components}>{ "Components" }</NavLink<Route>>
                    </li>
                    <li>
                        <NavLink<Route> to={Route::Hooks} partial=true>{ "Hooks" }</NavLink<Route>>
                    </li>
                    <li>
                        <NavLink<Route> to={Route::Utilities}>{ "Utilities" }</NavLink<Route>>
                    </li>
                </ul>
                <a
                    class="top-nav__source"
                    href="https://github.com/RAprogramm/yew-nav-link"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    { "Source" }
                </a>
            </div>
        </nav>
    }
}

// ============================================================================
// Home
// ============================================================================

#[function_component]
fn HomePage() -> Html {
    html! {
        <div class="container">
            <PageHeader
                title="yew-nav-link"
                subtitle="Navigation primitives for Yew with automatic active-state detection."
            />

            <PageSection
                title="Cargo dependencies"
                intro="Add this to your Cargo.toml. Every snippet on the next pages compiles against it."
            >
                <DemoCard
                    title="Cargo.toml"
                    language="ini"
                    code={r#"[dependencies]
yew         = { version = "0.23", features = ["csr"] }
yew-router  = "0.20"
yew-nav-link = "0.10""#}
                >
                    <p>{ "These are the only dependencies you need." }</p>
                </DemoCard>
            </PageSection>

            <PageSection
                title="Hello, NavLink"
                intro="Drop in <NavLink> wherever you'd reach for yew-router's <Link>. The 'active' class arrives for free."
            >
                <DemoCard
                    title="The smallest possible nav"
                    description="The link below targets this page. Try clicking the others up top — the previously-active link will lose 'active', and the new one gains it. No wiring."
                    code={r#"html! {
    <NavLink<Route> to={Route::Home}>
        { "Home" }
    </NavLink<Route>>
}"#}
                >
                    <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                </DemoCard>
            </PageSection>

            <PageSection
                title="Where to next"
                intro="Each page below is a long scroll of self-contained code-and-preview cards. Pick a topic."
            >
                <ul class="topic-grid">
                    <li>
                        <NavLink<Route> to={Route::NavLink}>
                            <strong>{ "NavLink" }</strong>
                            <span>{ "Active state, partial matching, custom classes" }</span>
                        </NavLink<Route>>
                    </li>
                    <li>
                        <NavLink<Route> to={Route::Components}>
                            <strong>{ "Components" }</strong>
                            <span>{ "Badges, dropdowns, tabs, pagination, headers" }</span>
                        </NavLink<Route>>
                    </li>
                    <li>
                        <NavLink<Route> to={Route::Hooks}>
                            <strong>{ "Hooks" }</strong>
                            <span>{ "Active state, breadcrumbs, navigation, query params" }</span>
                        </NavLink<Route>>
                    </li>
                    <li>
                        <NavLink<Route> to={Route::Utilities}>
                            <strong>{ "Utilities" }</strong>
                            <span>{ "Path normalisation, URL codec, errors" }</span>
                        </NavLink<Route>>
                    </li>
                </ul>
            </PageSection>
        </div>
    }
}

// ============================================================================
// NavLink page — with a self-contained "lab" that uses sub-routes
// ============================================================================

#[function_component]
fn NavLinkPage() -> Html {
    html! {
        <div class="container">
            <PageHeader
                title="NavLink"
                subtitle="Wraps yew-router's Link. Adds an 'active' class when the route matches."
            />

            <PageSection
                title="Component syntax"
                intro="Use <NavLink<R>> with arbitrary children. The library compares the target route to the current one on every render and toggles the 'active' class."
            >
                <DemoCard
                    title="Default classes"
                    description="Always emits 'nav-link'. Adds 'active' when the current path matches /."
                    code={r#"html! {
    <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
}"#}
                >
                    <NavLink<Route> to={Route::Home}>{ "Home" }</NavLink<Route>>
                </DemoCard>

                <DemoCard
                    title="Children can be any Html"
                    description="A NavLink is just a wrapper — put icons, badges, anything inside."
                    code={r#"html! {
    <NavLink<Route> to={Route::Home}>
        <NavIcon size={NavIconSize::Small}>{ "🏠" }</NavIcon>
        { " Home " }
        <NavBadge variant="primary">{ "new" }</NavBadge>
    </NavLink<Route>>
}"#}
                >
                    <NavLink<Route> to={Route::Home}>
                        <NavIcon size={NavIconSize::Small}>{ "🏠" }</NavIcon>
                        { " Home " }
                        <NavBadge variant="primary">{ "new" }</NavBadge>
                    </NavLink<Route>>
                </DemoCard>
            </PageSection>

            <PageSection
                title="Function syntax"
                intro="For text-only links, nav_link() is a one-liner. The Match enum picks exact vs prefix matching."
            >
                <DemoCard
                    title="nav_link with explicit Match::Exact"
                    code={r#"{ nav_link(Route::Home, "Home", Match::Exact) }"#}
                >
                    { nav_link(Route::Home, "Home", Match::Exact) }
                </DemoCard>

                <DemoCard
                    title="nav_link with Match::Partial"
                    description="Same call, different match mode. Active when the path is a prefix of the current URL."
                    code={r#"{ nav_link(Route::NavLink, "NavLink section", Match::Partial) }"#}
                >
                    { nav_link(Route::NavLink, "NavLink section", Match::Partial) }
                </DemoCard>
            </PageSection>

            <NavLinkLab />

            <PageSection
                title="Custom CSS classes"
                intro="Both the base class and the active-state class are overridable via &'static str props."
            >
                <DemoCard
                    title="Custom base class"
                    description="Replaces 'nav-link' with whatever you pass. Active class stays the default 'active'."
                    code={r#"html! {
    <NavLink<Route> to={Route::NavLink} class="my-link">
        { "Anchor with custom base class" }
    </NavLink<Route>>
}"#}
                >
                    <NavLink<Route> to={Route::NavLink} class="my-link">
                        { "Anchor with custom base class" }
                    </NavLink<Route>>
                </DemoCard>

                <DemoCard
                    title="Custom active class (Bulma-style)"
                    description="Bulma uses 'is-active'. yew-nav-link doesn't care — pass the framework's class through active_class."
                    code={r#"html! {
    <NavLink<Route> to={Route::NavLink} active_class="is-active">
        { "Bulma-style active marker" }
    </NavLink<Route>>
}"#}
                >
                    <NavLink<Route> to={Route::NavLink} active_class="is-active">
                        { "Bulma-style active marker" }
                    </NavLink<Route>>
                </DemoCard>

                <DemoCard
                    title="Both overridden"
                    code={r#"html! {
    <NavLink<Route>
        to={Route::NavLink}
        class="pill"
        active_class="pill--current"
    >
        { "Pill" }
    </NavLink<Route>>
}"#}
                >
                    <NavLink<Route>
                        to={Route::NavLink}
                        class="pill"
                        active_class="pill--current"
                    >
                        { "Pill" }
                    </NavLink<Route>>
                </DemoCard>
            </PageSection>
        </div>
    }
}

// The active-state lab. Lives at /navlink/lab/:slug. Clicking buttons inside
// rewrites the URL but stays on this page.
#[function_component]
fn NavLinkLab() -> Html {
    let current = use_route::<Route>();
    let slug = match current.clone() {
        Some(Route::NavLinkLab { slug }) => slug,
        _ => "stay".into()
    };

    html! {
        <PageSection
            title="Active-state lab"
            intro="Clicking these NavLinks rewrites the URL to /navlink/lab/<slug>. You stay on this page; only the URL and the active class change. Use the browser back button to return to /navlink."
        >
            <DemoCard
                title="Three NavLinks pointing into the same page"
                description={AttrValue::from(format!("Current sub-route: {slug}"))}
                code={r#"<NavLink<Route> to={Route::NavLinkLab { slug: "alpha".into()  }}>{ "Alpha"  }</NavLink<Route>>
<NavLink<Route> to={Route::NavLinkLab { slug: "bravo".into()  }}>{ "Bravo"  }</NavLink<Route>>
<NavLink<Route> to={Route::NavLinkLab { slug: "charlie".into() }}>{ "Charlie" }</NavLink<Route>>"#}
            >
                <div class="lab-row">
                    <NavLink<Route> to={Route::NavLinkLab { slug: "alpha".into() }}>
                        { "Alpha" }
                    </NavLink<Route>>
                    <NavLink<Route> to={Route::NavLinkLab { slug: "bravo".into() }}>
                        { "Bravo" }
                    </NavLink<Route>>
                    <NavLink<Route> to={Route::NavLinkLab { slug: "charlie".into() }}>
                        { "Charlie" }
                    </NavLink<Route>>
                </div>
            </DemoCard>

            <DemoCard
                title="Partial match keeps a parent link active"
                description="The 'NavLink section' link uses partial=true and stays active across every /navlink/* path."
                code={r#"<NavLink<Route> to={Route::NavLink} partial=true>
    { "NavLink section" }
</NavLink<Route>>"#}
            >
                <NavLink<Route> to={Route::NavLink} partial=true>
                    { "NavLink section" }
                </NavLink<Route>>
            </DemoCard>
        </PageSection>
    }
}

// ============================================================================
// Components page
// ============================================================================

#[function_component]
fn ComponentsPage() -> Html {
    let active_tab = use_state(|| 0u32);
    let current_page = use_state(|| 1u32);

    let on_tab = {
        let active_tab = active_tab.clone();
        move |idx: u32| {
            let active_tab = active_tab.clone();
            Callback::from(move |_: MouseEvent| active_tab.set(idx))
        }
    };

    let on_page_change = {
        let current_page = current_page.clone();
        Callback::from(move |page: u32| current_page.set(page))
    };

    html! {
        <div class="container">
            <PageHeader
                title="Components"
                subtitle="Drop-in widgets that follow the same conventions as NavLink."
            />

            <PageSection title="Lists">
                <DemoCard
                    title="NavList + NavItem + NavDivider"
                    description="The structural primitives. NavList is a <ul> with sensible ARIA defaults; NavDivider renders a <hr>."
                    code={r#"html! {
    <NavList>
        <NavHeader>{ "Account" }</NavHeader>
        <NavItem>
            <NavLink<Route> to={Route::Home}>{ "Profile" }</NavLink<Route>>
        </NavItem>
        <NavItem>
            <NavLink<Route> to={Route::Components}>{ "Settings" }</NavLink<Route>>
        </NavItem>
        <NavDivider />
        <NavText text="v0.9.x" />
    </NavList>
}"#}
                >
                    <NavList>
                        <NavHeader>{ "Account" }</NavHeader>
                        <NavItem>
                            <NavLink<Route> to={Route::Home}>{ "Profile" }</NavLink<Route>>
                        </NavItem>
                        <NavItem>
                            <NavLink<Route> to={Route::Components}>{ "Settings" }</NavLink<Route>>
                        </NavItem>
                        <NavDivider />
                        <NavText text="v0.9.x" />
                    </NavList>
                </DemoCard>
            </PageSection>

            <PageSection title="Badges">
                <DemoCard
                    title="Variants"
                    code={r#"<NavBadge variant="primary">{ "5"  }</NavBadge>
<NavBadge variant="success">{ "OK" }</NavBadge>
<NavBadge variant="warning">{ "!"  }</NavBadge>
<NavBadge variant="danger" pill=true>{ "99+" }</NavBadge>"#}
                >
                    <span class="inline-row">
                        <NavBadge variant="primary">{ "5" }</NavBadge>
                        <NavBadge variant="success">{ "OK" }</NavBadge>
                        <NavBadge variant="warning">{ "!" }</NavBadge>
                        <NavBadge variant="danger" pill=true>{ "99+" }</NavBadge>
                    </span>
                </DemoCard>

                <DemoCard
                    title="Inside a NavLink"
                    description="Badges combine cleanly with NavLink children to surface counters."
                    code={r#"<NavLink<Route> to={Route::Components}>
    { "Inbox " }
    <NavBadge variant="danger">{ "12" }</NavBadge>
</NavLink<Route>>"#}
                >
                    <NavLink<Route> to={Route::Components}>
                        { "Inbox " }
                        <NavBadge variant="danger">{ "12" }</NavBadge>
                    </NavLink<Route>>
                </DemoCard>
            </PageSection>

            <PageSection title="Icons">
                <DemoCard
                    title="Three sizes"
                    code={r#"<NavIcon size={NavIconSize::Small}>{ "★" }</NavIcon>
<NavIcon size={NavIconSize::Medium}>{ "★" }</NavIcon>
<NavIcon size={NavIconSize::Large}>{ "★" }</NavIcon>"#}
                >
                    <span class="inline-row">
                        <NavIcon size={NavIconSize::Small}>{ "★" }</NavIcon>
                        <NavIcon size={NavIconSize::Medium}>{ "★" }</NavIcon>
                        <NavIcon size={NavIconSize::Large}>{ "★" }</NavIcon>
                    </span>
                </DemoCard>

                <DemoCard
                    title="NavLinkWithIcon"
                    description="Pairs an icon with text, stacked or inline."
                    code={r#"<NavLinkWithIcon icon={NavIconSize::Small}>
    { "Sized link content" }
</NavLinkWithIcon>"#}
                >
                    <NavLinkWithIcon icon={NavIconSize::Small}>
                        { "Sized link content" }
                    </NavLinkWithIcon>
                </DemoCard>
            </PageSection>

            <PageSection title="Dropdown">
                <DemoCard
                    title="Self-managed open/close"
                    description="NavDropdown owns its own boolean state — no parent wiring required. Wrap it in NavList so the rendered <li> sits inside a proper <ul>."
                    wide=true
                    code={r#"<NavList>
    <NavDropdown toggle_text="Account">
        <NavDropdownItem>
            <NavLink<Route> to={Route::Home}>{ "Profile"  }</NavLink<Route>>
        </NavDropdownItem>
        <NavDropdownItem>
            <NavLink<Route> to={Route::Hooks}>{ "Hooks demo" }</NavLink<Route>>
        </NavDropdownItem>
        <NavDropdownDivider />
        <NavDropdownItem disabled=true>
            { "Sign out (disabled)" }
        </NavDropdownItem>
    </NavDropdown>
</NavList>"#}
                >
                    <NavList>
                        <NavDropdown toggle_text="Account">
                            <NavDropdownItem>
                                <NavLink<Route> to={Route::Home}>
                                    { "Profile" }
                                </NavLink<Route>>
                            </NavDropdownItem>
                            <NavDropdownItem>
                                <NavLink<Route> to={Route::Hooks}>
                                    { "Hooks demo" }
                                </NavLink<Route>>
                            </NavDropdownItem>
                            <NavDropdownDivider />
                            <NavDropdownItem disabled=true>
                                { "Sign out (disabled)" }
                            </NavDropdownItem>
                        </NavDropdown>
                    </NavList>
                </DemoCard>
            </PageSection>

            <PageSection title="Tabs">
                <DemoCard
                    title="Controlled tabs"
                    description="Active tab and panel visibility are driven by use_state — yew-nav-link only renders, you decide what's selected."
                    wide=true
                    code={r#"let active = use_state(|| 0u32);

html! {
    <NavTabs id="demo-tabs">
        <NavTab active={*active == 0} onclick={set(0)} panel_id={Some("p-0")}>
            { "Overview" }
        </NavTab>
        <NavTab active={*active == 1} onclick={set(1)} panel_id={Some("p-1")}>
            { "Details" }
        </NavTab>
        <NavTab active={*active == 2} disabled=true>{ "Disabled" }</NavTab>
    </NavTabs>

    <NavTabPanel id={Some("p-0")} hidden={*active != 0}>
        <p>{ "Overview content." }</p>
    </NavTabPanel>
    <NavTabPanel id={Some("p-1")} hidden={*active != 1}>
        <p>{ "Details content." }</p>
    </NavTabPanel>
}"#}
                >
                    <NavTabs id="demo-tabs">
                        <NavTab
                            active={*active_tab == 0}
                            onclick={Some(on_tab(0))}
                            panel_id={Some("p-0")}
                        >
                            { "Overview" }
                        </NavTab>
                        <NavTab
                            active={*active_tab == 1}
                            onclick={Some(on_tab(1))}
                            panel_id={Some("p-1")}
                        >
                            { "Details" }
                        </NavTab>
                        <NavTab
                            active={*active_tab == 2}
                            disabled=true
                            onclick={None}
                        >
                            { "Disabled" }
                        </NavTab>
                    </NavTabs>
                    <NavTabPanel id={Some("p-0")} hidden={*active_tab != 0}>
                        <p>{ "Overview content." }</p>
                    </NavTabPanel>
                    <NavTabPanel id={Some("p-1")} hidden={*active_tab != 1}>
                        <p>{ "Details content." }</p>
                    </NavTabPanel>
                </DemoCard>
            </PageSection>

            <PageSection title="Pagination">
                <DemoCard
                    title="Pagination with siblings + first/last"
                    description={AttrValue::from(format!(
                        "Current page: {}. The component renders prev/next, first/last, sibling pages, and ellipses.",
                        *current_page
                    ))}
                    wide=true
                    code={r#"let current_page = use_state(|| 1u32);
let on_page_change = {
    let current_page = current_page.clone();
    Callback::from(move |p: u32| current_page.set(p))
};

html! {
    <Pagination
        current_page={*current_page}
        total_pages={20}
        siblings={2}
        show_first_last=true
        on_page_change={Some(on_page_change)}
    />
}"#}
                >
                    <Pagination
                        current_page={*current_page}
                        total_pages={20}
                        siblings={2}
                        show_first_last=true
                        on_page_change={Some(on_page_change.clone())}
                    />
                </DemoCard>
            </PageSection>
        </div>
    }
}

// ============================================================================
// Hooks page (with breadcrumbs lab)
// ============================================================================

#[function_component]
fn HooksPage() -> Html {
    let route_info: Option<Route> = use_route_info::<Route>();
    let is_active_home = use_is_active(Route::Home);
    let is_exact_active_hooks = use_is_exact_active(Route::Hooks);
    let is_partial_active_hooks = use_is_partial_active(Route::Hooks);
    let query = use_query_params();
    let nav: Navigation<Route> = use_navigation::<Route>();
    let trail: Vec<BreadcrumbItem<Route>> = use_breadcrumbs();

    let push_home = nav.push_callback(Route::Home).reform(|_: MouseEvent| ());
    let replace_components = nav.replace_callback(Route::Components).reform(|_: MouseEvent| ());
    let go_back = nav.go_back.clone().reform(|_: MouseEvent| ());
    let go_forward = nav.go_forward.clone().reform(|_: MouseEvent| ());

    html! {
        <div class="container">
            <PageHeader
                title="Hooks"
                subtitle="Reactive hooks that read the current route, walk it, and steer it."
            />

            <PageSection title="Active state hooks">
                <DemoCard
                    title="use_is_active / use_is_exact_active / use_is_partial_active"
                    description={AttrValue::from(format!(
                        "Right now Home is active = {}, Hooks (exact) = {}, Hooks (partial) = {}.",
                        is_active_home, is_exact_active_hooks, is_partial_active_hooks
                    ))}
                    code={r#"let is_home          = use_is_active(Route::Home);
let is_exact_hooks   = use_is_exact_active(Route::Hooks);
let is_partial_hooks = use_is_partial_active(Route::Hooks);"#}
                >
                    <ul class="status-list">
                        <li>
                            { "Home: " }<HookStatus active={is_active_home} />
                        </li>
                        <li>
                            { "Hooks (exact): " }<HookStatus active={is_exact_active_hooks} />
                        </li>
                        <li>
                            { "Hooks (partial): " }<HookStatus active={is_partial_active_hooks} />
                        </li>
                    </ul>
                </DemoCard>
            </PageSection>

            <PageSection title="use_route_info">
                <DemoCard
                    title="The current route as Option<R>"
                    description={AttrValue::from(format!("Current route: {:?}", route_info))}
                    code={r#"let current: Option<Route> = use_route_info::<Route>();"#}
                >
                    <code class="inline-code">{ format!("{:?}", route_info) }</code>
                </DemoCard>
            </PageSection>

            <PageSection title="use_navigation">
                <DemoCard
                    title="Programmatic navigation"
                    description="use_navigation gives you ready-made Callback<()> for every browser-history action. Wrap them with .reform(...) to plug into onclick directly."
                    code={r#"let nav: Navigation<Route> = use_navigation::<Route>();

let push     = nav.push_callback(Route::Home).reform(|_: MouseEvent| ());
let replace  = nav.replace_callback(Route::Components).reform(|_: MouseEvent| ());
let back     = nav.go_back.clone().reform(|_: MouseEvent| ());
let forward  = nav.go_forward.clone().reform(|_: MouseEvent| ());"#}
                >
                    <div class="button-row">
                        <button class="btn" onclick={push_home}>{ "push Home" }</button>
                        <button class="btn" onclick={replace_components}>
                            { "replace Components" }
                        </button>
                        <button class="btn" onclick={go_back}>{ "back" }</button>
                        <button class="btn" onclick={go_forward}>{ "forward" }</button>
                    </div>
                </DemoCard>
            </PageSection>

            <PageSection title="use_query_params">
                <DemoCard
                    title="Reactive query string"
                    description="Append ?foo=bar&page=2 to the URL bar and watch this map update without a reload."
                    code={r#"let query: HashMap<String, String> = use_query_params();
let page = query.get("page");"#}
                >
                    <pre class="inline-pre">{ format!("{:#?}", query) }</pre>
                </DemoCard>
            </PageSection>

            <PageSection
                title="use_breadcrumbs"
                intro="The trail is generated from the current path. A BreadcrumbLabelProvider injected via context turns the raw segments into human labels."
            >
                <DemoCard
                    title="Live trail"
                    description={AttrValue::from(format!("{} item(s) in the trail.", trail.len()))}
                    code={r#"let trail: Vec<BreadcrumbItem<Route>> = use_breadcrumbs();

html! {
    <nav aria-label="Breadcrumb" class="trail">
        { for trail.iter().enumerate().map(|(i, item)| html! {
            <>
                if i > 0 { <span class="trail__sep">{ "/" }</span> }
                <span aria-current={if item.is_active { "page" } else { "" }}>
                    { &item.label }
                </span>
            </>
        }) }
    </nav>
}"#}
                >
                    <nav aria-label="Breadcrumb" class="trail">
                        { for trail.iter().enumerate().map(|(i, item)| {
                            let aria = if item.is_active { "page" } else { "" };
                            html! {
                                <>
                                    if i > 0 {
                                        <span class="trail__sep">{ "/" }</span>
                                    }
                                    <span aria-current={aria} class={if item.is_active {"trail__current"} else {""}}>
                                        { &item.label }
                                    </span>
                                </>
                            }
                        }) }
                    </nav>
                </DemoCard>

                <DemoCard
                    title="Drill into a team"
                    description="Each link rewrites the URL to /hooks/team/<name>. The provider in App turns the segment into 'Team <name>'."
                    code={r#"<NavLink<Route> to={Route::HooksTeam { team: "alpha".into() }}>
    { "Alpha" }
</NavLink<Route>>"#}
                >
                    <div class="lab-row">
                        <NavLink<Route> to={Route::HooksTeam { team: "alpha".into() }}>
                            { "Alpha" }
                        </NavLink<Route>>
                        <NavLink<Route> to={Route::HooksTeam { team: "bravo".into() }}>
                            { "Bravo" }
                        </NavLink<Route>>
                        <NavLink<Route> to={Route::HooksTeam { team: "charlie".into() }}>
                            { "Charlie" }
                        </NavLink<Route>>
                        <NavLink<Route> to={Route::Hooks}>{ "Reset" }</NavLink<Route>>
                    </div>
                </DemoCard>

                <DemoCard
                    title="Custom provider"
                    description="Implement BreadcrumbLabelProvider, wrap App in ContextProvider<BreadcrumbLabelProviderContext>."
                    code={r#"use std::rc::Rc;
use yew_nav_link::{BreadcrumbLabelProvider, BreadcrumbLabelProviderContext};

struct DemoLabels;

impl BreadcrumbLabelProvider for DemoLabels {
    fn label_for_path(&self, path: &str) -> String {
        match path {
            "/"      => "Home".into(),
            "/hooks" => "Hooks".into(),
            p if p.starts_with("/hooks/team/") => format!("Team {}", &p[12..]),
            other    => other.into(),
        }
    }
}

#[function_component]
fn App() -> Html {
    let ctx = use_memo((), |()| {
        BreadcrumbLabelProviderContext::new(Rc::new(DemoLabels))
    });
    html! {
        <ContextProvider<BreadcrumbLabelProviderContext> context={(*ctx).clone()}>
            <BrowserRouter> /* ... */ </BrowserRouter>
        </ContextProvider<BreadcrumbLabelProviderContext>>
    }
}"#}
                >
                    <p class="muted">
                        { "This page is wrapped in exactly that. Click the team links above and watch the trail rename." }
                    </p>
                </DemoCard>
            </PageSection>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct HookStatusProps {
    active: bool
}

#[function_component]
fn HookStatus(props: &HookStatusProps) -> Html {
    if props.active {
        html! { <span class="status status--active">{ "active" }</span> }
    } else {
        html! { <span class="status status--idle">{ "idle" }</span> }
    }
}

// ============================================================================
// Utilities page
// ============================================================================

#[function_component]
fn UtilitiesPage() -> Html {
    let parsed_ok: NavResult<&'static str> = parse_route("/components");
    let parsed_err: NavResult<&'static str> = parse_route("not a path");

    html! {
        <div class="container">
            <PageHeader
                title="Utilities"
                subtitle="Path manipulation, URL encoding, and the typed error story."
            />

            <PageSection title="Path helpers">
                <DemoCard
                    title="join_paths"
                    code={r#"join_paths("/foo/bar/", "/baz") // -> "/foo/bar/baz"
join_paths("foo",       "bar")  // -> "foo/bar""#}
                >
                    <div class="util-table-wrap">
                    <table class="util-table">
                        <thead>
                            <tr><th>{ "Inputs" }</th><th>{ "Output" }</th></tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><code>{ r#"("/foo/bar/", "/baz")"# }</code></td>
                                <td><code>{ join_paths("/foo/bar/", "/baz") }</code></td>
                            </tr>
                            <tr>
                                <td><code>{ r#"("foo", "bar")"# }</code></td>
                                <td><code>{ join_paths("foo", "bar") }</code></td>
                            </tr>
                        </tbody>
                    </table>
                    </div>
                </DemoCard>

                <DemoCard
                    title="normalize_path"
                    code={r#"normalize_path("/foo/bar/../baz/") // -> "/foo/baz/"
normalize_path("/a/./b/c/../d")    // -> "/a/b/d""#}
                >
                    <div class="util-table-wrap">
                    <table class="util-table">
                        <thead>
                            <tr><th>{ "Input" }</th><th>{ "Output" }</th></tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><code>{ "/foo/bar/../baz/" }</code></td>
                                <td><code>{ normalize_path("/foo/bar/../baz/") }</code></td>
                            </tr>
                            <tr>
                                <td><code>{ "/a/./b/c/../d" }</code></td>
                                <td><code>{ normalize_path("/a/./b/c/../d") }</code></td>
                            </tr>
                        </tbody>
                    </table>
                    </div>
                </DemoCard>

                <DemoCard
                    title="is_absolute"
                    code={r#"is_absolute("https://example.com") // true
is_absolute("/relative/path")     // false"#}
                >
                    <div class="util-table-wrap">
                    <table class="util-table">
                        <thead>
                            <tr><th>{ "URL" }</th><th>{ "Absolute?" }</th></tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><code>{ "https://example.com" }</code></td>
                                <td><HookStatus active={is_absolute("https://example.com")} /></td>
                            </tr>
                            <tr>
                                <td><code>{ "/relative/path" }</code></td>
                                <td><HookStatus active={is_absolute("/relative/path")} /></td>
                            </tr>
                        </tbody>
                    </table>
                    </div>
                </DemoCard>
            </PageSection>

            <PageSection title="URL codec">
                <DemoCard
                    title="urlencoding_encode + urlencoding_decode round-trip"
                    code={r#"let encoded = urlencoding_encode("rust 2024");
// "rust%202024"

let decoded: Option<String> = urlencoding_decode(&encoded);
// Some("rust 2024")"#}
                >
                    <div class="util-table-wrap">
                    <table class="util-table">
                        <thead>
                            <tr>
                                <th>{ "Input" }</th>
                                <th>{ "Encoded" }</th>
                                <th>{ "Decoded" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for ["hello world", "rust 2024 / wasm", "a&b=c"]
                                .iter()
                                .map(|s| {
                                    let enc = urlencoding_encode(s);
                                    let dec = urlencoding_decode(&enc).unwrap_or_default();
                                    html! {
                                        <tr>
                                            <td><code>{ s }</code></td>
                                            <td><code>{ enc }</code></td>
                                            <td><code>{ dec }</code></td>
                                        </tr>
                                    }
                                })
                            }
                        </tbody>
                    </table>
                    </div>
                </DemoCard>
            </PageSection>

            <PageSection
                title="NavError + NavResult"
                intro="Navigation operations return a typed Result. Three variants cover the realistic cases."
            >
                <DemoCard
                    title="The variants"
                    code={r#"NavError::route_not_found()       // "route not found"
NavError::invalid_route("...")    // "invalid route: ..."
NavError::navigation_cancelled()  // "navigation cancelled""#}
                >
                    <div class="util-table-wrap">
                    <table class="util-table">
                        <thead>
                            <tr><th>{ "Constructor" }</th><th>{ "Display" }</th></tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><code>{ "route_not_found()" }</code></td>
                                <td><code>{ format!("{}", NavError::route_not_found()) }</code></td>
                            </tr>
                            <tr>
                                <td><code>{ r#"invalid_route("oops")"# }</code></td>
                                <td><code>{ format!("{}", NavError::invalid_route("oops")) }</code></td>
                            </tr>
                            <tr>
                                <td><code>{ "navigation_cancelled()" }</code></td>
                                <td><code>{ format!("{}", NavError::navigation_cancelled()) }</code></td>
                            </tr>
                        </tbody>
                    </table>
                    </div>
                </DemoCard>

                <DemoCard
                    title="A NavResult-returning function"
                    description="The output below comes from the function in the code block, called twice on this page."
                    code={r#"fn parse_route(input: &str) -> NavResult<&'static str> {
    if !input.starts_with('/') {
        return Err(NavError::invalid_route(format!("got {input:?}")));
    }
    match input {
        "/components" => Ok("/components"),
        "/utilities"  => Ok("/utilities"),
        _             => Err(NavError::route_not_found()),
    }
}"#}
                >
                    <ul class="status-list">
                        <li>
                            <code>{ r#"parse_route("/components")"# }</code>{ " → " }
                            { match &parsed_ok {
                                Ok(s)  => html! { <span class="status status--active">{ format!("Ok({s:?})") }</span> },
                                Err(e) => html! { <span class="status status--idle">{ format!("Err({e})") }</span> }
                            } }
                        </li>
                        <li>
                            <code>{ r#"parse_route("not a path")"# }</code>{ " → " }
                            { match &parsed_err {
                                Ok(s)  => html! { <span class="status status--active">{ format!("Ok({s:?})") }</span> },
                                Err(e) => html! { <span class="status status--idle">{ format!("Err({e})") }</span> }
                            } }
                        </li>
                    </ul>
                </DemoCard>
            </PageSection>
        </div>
    }
}

fn parse_route(input: &str) -> NavResult<&'static str> {
    if !input.starts_with('/') {
        return Err(NavError::invalid_route(format!("got {input:?}")));
    }
    match input {
        "/components" => Ok("/components"),
        "/utilities" => Ok("/utilities"),
        _ => Err(NavError::route_not_found())
    }
}

// ============================================================================
// 404
// ============================================================================

#[function_component]
fn NotFoundPage() -> Html {
    html! {
        <div class="container">
            <PageHeader
                title="Not found"
                subtitle="The path you followed isn't part of the demo."
            />
            <p class="muted">
                <NavLink<Route> to={Route::Home}>{ "← Back to Home" }</NavLink<Route>>
            </p>
        </div>
    }
}

// ============================================================================
// Breadcrumb provider
// ============================================================================

struct DemoLabels;

impl BreadcrumbLabelProvider for DemoLabels {
    fn label_for_path(&self, path: &str) -> String {
        match path {
            "/" => "Home".into(),
            "/navlink" => "NavLink".into(),
            "/navlink/lab" => "Lab".into(),
            "/components" => "Components".into(),
            "/hooks" => "Hooks".into(),
            "/hooks/team" => "Team".into(),
            "/utilities" => "Utilities".into(),
            other if other.starts_with("/navlink/lab/") => format!("Lab {}", &other[13..]),
            other if other.starts_with("/hooks/team/") => format!("Team {}", &other[12..]),
            other => other.into()
        }
    }
}

// ============================================================================
// App
// ============================================================================

/// Pages-aware basename detection.
///
/// On GitHub Pages the demo lives at `/yew-nav-link/`; under
/// `trunk serve` it lives at `/`. We read the current pathname once at
/// app startup and ask yew-router to honour the project subpath when it
/// is present, so every NavLink / Navigator generates URLs that stay
/// inside the deployment.
const PAGES_BASENAME: &str = "/yew-nav-link";

fn detect_basename() -> Option<AttrValue> {
    let pathname = web_sys::window()?.location().pathname().ok()?;
    if pathname == PAGES_BASENAME || pathname.starts_with(&format!("{PAGES_BASENAME}/")) {
        Some(AttrValue::Static(PAGES_BASENAME))
    } else {
        None
    }
}

#[function_component]
fn App() -> Html {
    let label_ctx = use_memo((), |()| {
        BreadcrumbLabelProviderContext::new(Rc::new(DemoLabels))
    });
    let basename = detect_basename();

    html! {
        <ContextProvider<BreadcrumbLabelProviderContext> context={(*label_ctx).clone()}>
            <BrowserRouter basename={basename}>
                <div class="app-shell">
                    <a class="skip-link" href="#main-content">{ "Skip to content" }</a>
                    <TopNav />
                    <main id="main-content" tabindex="-1">
                        <Switch<Route> render={|route: Route| match route {
                            Route::Home => html! { <HomePage/> },
                            Route::NavLink | Route::NavLinkLab { .. } => html! { <NavLinkPage/> },
                            Route::Components => html! { <ComponentsPage/> },
                            Route::Hooks | Route::HooksTeam { .. } => html! { <HooksPage/> },
                            Route::Utilities => html! { <UtilitiesPage/> },
                            Route::NotFound => html! { <NotFoundPage/> }
                        }} />
                    </main>
                </div>
            </BrowserRouter>
        </ContextProvider<BreadcrumbLabelProviderContext>>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
