//! Procedural macros for yew-nav-link

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{parse::Parse, parse_macro_input, parse_quote, spanned::Spanned, Token};

// ============================================================================
// Breadcrumbs Macro
// ============================================================================

struct BreadcrumbEntry {
    route: syn::ExprPath,
    label: syn::Expr,
}

impl Parse for BreadcrumbEntry {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let route = input.parse::<syn::ExprPath>()?;
        input.parse::<Token![,]>()?;
        let label = input.parse::<syn::Expr>()?;

        Ok(BreadcrumbEntry { route, label })
    }
}

struct BreadcrumbsArgs {
    entries: Vec<BreadcrumbEntry>,
}

impl Parse for BreadcrumbsArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut entries = Vec::new();

        while !input.is_empty() {
            entries.push(input.parse::<BreadcrumbEntry>()?);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        Ok(BreadcrumbsArgs { entries })
    }
}

/// Creates breadcrumbs navigation from route-label pairs.
///
/// The last entry automatically receives `aria-current="page"`.
#[proc_macro]
pub fn breadcrumbs(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as BreadcrumbsArgs);

    if args.entries.is_empty() {
        let span = Span::call_site();
        return quote_spanned! { span =>
            yew::html! {}
        }
        .into();
    }

    let spans: Vec<_> = args.entries.iter().map(|e| e.route.span()).collect();

    let expanded = args.entries.iter().enumerate().map(|(i, entry)| {
        let route = &entry.route;
        let label = &entry.label;
        let span = spans[i];

        let is_last = i == args.entries.len() - 1;
        let aria_current = if is_last {
            quote! { aria-current="page" }
        } else {
            quote! {}
        };

        quote_spanned! { span =>
            <li class={classes!("breadcrumb-item", if #is_last { "active" } else { "" })} #aria_current>
                <Link<#route> to={#route.clone()}>
                    #label
                </Link<#route>>
            </li>
        }
    });

    let expanded = quote! {
        yew::html! {
            <nav aria-label="breadcrumb">
                <ol class="breadcrumb">
                    #(#expanded)*
                </ol>
            </nav>
        }
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Nav Links Macro
// ============================================================================

struct NavLinkConfig {
    route: syn::ExprPath,
    label: syn::LitStr,
    match_mode: syn::ExprPath,
}

impl Parse for NavLinkConfig {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let route = input.parse::<syn::ExprPath>()?;
        input.parse::<Token![,]>()?;
        let label = input.parse::<syn::LitStr>()?;
        input.parse::<Token![,]>()?;
        let match_mode = input.parse::<syn::ExprPath>()?;

        Ok(NavLinkConfig {
            route,
            label,
            match_mode,
        })
    }
}

struct NavLinksArgs {
    entries: Vec<NavLinkConfig>,
}

impl Parse for NavLinksArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut entries = Vec::new();

        while !input.is_empty() {
            entries.push(input.parse::<NavLinkConfig>()?);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        Ok(NavLinksArgs { entries })
    }
}

/// Creates multiple navigation links efficiently
#[proc_macro]
pub fn nav_links(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(input as NavLinksArgs);

    if args.entries.is_empty() {
        let span = Span::call_site();
        return quote_spanned! { span =>
            yew::html! {}
        }
        .into();
    }

    let spans: Vec<_> = args.entries.iter().map(|e| e.route.span()).collect();

    let expanded = args.entries.iter().enumerate().map(|(i, entry)| {
        let route = &entry.route;
        let label = &entry.label;
        let match_mode = &entry.match_mode;
        let span = spans[i];

        quote_spanned! { span =>
            yew_nav_link::nav_link(#route, #label, #match_mode)
        }
    });

    let expanded = quote! {
        vec![
            #(#expanded),*
        ]
        .into_iter()
        .collect::<yew::Html>()
    };

    TokenStream::from(expanded).into()
}

// ============================================================================
// Nav List Macro
// ============================================================================

struct NavItemConfig {
    route: syn::ExprPath,
    label: syn::LitStr,
    match_mode: syn::ExprPath,
}

impl Parse for NavItemConfig {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let route = input.parse::<syn::ExprPath>()?;
        input.parse::<Token![,]>()?;
        let label = input.parse::<syn::LitStr>()?;
        input.parse::<Token![,]>()?;
        let match_mode = input.parse::<syn::ExprPath>()?;

        Ok(NavItemConfig {
            route,
            label,
            match_mode,
        })
    }
}

struct NavListArgs {
    items: Vec<NavItemConfig>,
    disabled: Option<syn::Expr>,
}

impl Parse for NavListArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        let mut disabled = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if lookahead.peek(syn::Ident) && input.peek2(Token![=]) {
                let ident: syn::Ident = input.parse()?;
                if ident == "disabled" {
                    input.parse::<Token![=]>()?;
                    disabled = Some(input.parse()?);

                    if input.peek(Token![,]) {
                        input.parse::<Token![,]>()?;
                    }
                } else {
                    return Err(syn::Error::new(
                        ident.span(),
                        "Expected 'disabled' property",
                    ));
                }
            } else {
                items.push(input.parse::<NavItemConfig>()?);

                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                } else {
                    break;
                }
            }
        }

        Ok(NavListArgs { items, disabled })
    }
}

/// Creates a navigation list with multiple nav items from route tuples.
///
/// Each entry is `(Route, "Label", MatchMode)`. An optional `disabled = vec![...]`
/// argument controls which items are disabled.
#[proc_macro]
pub fn nav_list(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as NavListArgs);

    if args.items.is_empty() {
        let span = Span::call_site();
        return quote_spanned! { span =>
            yew::html! {}
        }
        .into();
    }

    let spans: Vec<_> = args.items.iter().map(|i| i.route.span()).collect();

    let (disabled_vec, disabled_access) = if let Some(ref d) = args.disabled {
        (
            quote! { #d },
            quote! { #d.get(i).copied().unwrap_or(false) },
        )
    } else {
        let disableds: Vec<syn::Expr> = args.items.iter().map(|_| parse_quote!(false)).collect();
        (
            quote! { vec![#(#disableds),*] },
            quote! { disableds.get(i).copied().unwrap_or(false) },
        )
    };

    let items_html = args.items.iter().enumerate().map(|(i, item)| {
        let route = &item.route;
        let label = &item.label;
        let match_mode = &item.match_mode;
        let span = spans[i];

        quote_spanned! { span =>
            <li class={classes!("nav-item", if #disabled_access { "disabled" } else { "" })}>
                { yew_nav_link::nav_link(#route, #label, #match_mode) }
            </li>
        }
    });

    let expanded = quote! {
        {
            let disableds = #disabled_vec;
            yew::html! {
                <ul class="nav-list">
                    #(#items_html)*
                </ul>
            }
        }
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Nav Menu Macro
// ============================================================================

enum MenuEntry {
    Link(NavLinkConfig),
    Group(NavLinkConfig, Vec<NavLinkConfig>),
}

struct NavMenuArgs {
    entries: Vec<MenuEntry>,
}

impl Parse for NavMenuArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut entries = Vec::new();

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if lookahead.peek(syn::token::Brace) {
                let group;
                syn::braced!(group in input);
                let inner_entries = group.parse_terminated(NavLinkConfig::parse, Token![,])?;
                let last_entry = entries.pop().ok_or_else(|| {
                    syn::Error::new(input.span(), "Nested menu must follow a menu group")
                })?;

                match last_entry {
                    MenuEntry::Link(config) => {
                        entries.push(MenuEntry::Group(
                            config,
                            inner_entries.into_iter().collect(),
                        ));
                    }
                    MenuEntry::Group(_, _) => {
                        return Err(syn::Error::new(input.span(), "Nested groups not supported"));
                    }
                }
            } else {
                let config = input.parse::<NavLinkConfig>()?;
                entries.push(MenuEntry::Link(config));

                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                } else {
                    break;
                }
            }
        }

        Ok(NavMenuArgs { entries })
    }
}

fn generate_menu_html(entries: &[MenuEntry]) -> proc_macro2::TokenStream {
    let items = entries.iter().map(|entry| match entry {
        MenuEntry::Link(config) => {
            let route = &config.route;
            let label = &config.label;
            let match_mode = &config.match_mode;
            quote_spanned! { config.route.span() =>
                yew::html! {
                    <li class="nav-item">
                        { yew_nav_link::nav_link(#route, #label, #match_mode) }
                    </li>
                }
            }
        }
        MenuEntry::Group(config, children) => {
            let _route = &config.route;
            let label = &config.label;
            let _match_mode = &config.match_mode;
            let child_items: Vec<_> = children
                .iter()
                .map(|c| {
                    let child_route = &c.route;
                    let child_label = &c.label;
                    let child_match_mode = &c.match_mode;
                    quote_spanned! { c.route.span() =>
                        yew::html! {
                            <li class="nav-item">
                                { yew_nav_link::nav_link(#child_route, #child_label, #child_match_mode) }
                            </li>
                        }
                    }
                })
                .collect();
            let child_html = quote! { #(#child_items)* };
            quote_spanned! { config.route.span() =>
                yew::html! {
                    <li class="nav-item dropdown">
                        <a class="nav-link dropdown-toggle" href="#" data-bs-toggle="dropdown">
                            #label
                        </a>
                        <ul class="dropdown-menu">
                            #child_html
                        </ul>
                    </li>
                }
            }
        }
    });

    quote! { #(#items)* }
}

/// Creates a nested navigation menu from a hierarchical structure.
///
/// Supports one level of nesting by appending `{ ... }` after a menu group entry.
#[proc_macro]
pub fn nav_menu(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as NavMenuArgs);

    if args.entries.is_empty() {
        let span = Span::call_site();
        return quote_spanned! { span =>
            yew::html! {}
        }
        .into();
    }

    let expanded = generate_menu_html(&args.entries);

    TokenStream::from(quote! {
        {
            #expanded
        }
    })
}

// ============================================================================
// Nav Pagination Macro
// ============================================================================

struct PaginationConfig {
    current_page: syn::Expr,
    total_pages: syn::Expr,
    siblings: syn::Expr,
    show_first_last: syn::Expr,
    show_prev_next: syn::Expr,
    on_page_change: Option<syn::Expr>,
}

impl Parse for PaginationConfig {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut current_page = parse_quote!(1);
        let mut total_pages = parse_quote!(10);
        let mut siblings = parse_quote!(1);
        let mut show_first_last = parse_quote!(false);
        let mut show_prev_next = parse_quote!(true);
        let mut on_page_change = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            if ident == "current_page" {
                current_page = input.parse()?;
            } else if ident == "total_pages" {
                total_pages = input.parse()?;
            } else if ident == "siblings" {
                siblings = input.parse()?;
            } else if ident == "show_first_last" {
                show_first_last = input.parse()?;
            } else if ident == "show_prev_next" {
                show_prev_next = input.parse()?;
            } else if ident == "on_page_change" {
                on_page_change = Some(input.parse()?);
            } else {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("Unknown property: {}", ident),
                ));
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        Ok(PaginationConfig {
            current_page,
            total_pages,
            siblings,
            show_first_last,
            show_prev_next,
            on_page_change,
        })
    }
}

/// Creates a pagination component with configurable options.
///
/// Supports `current_page`, `total_pages`, `siblings`, `show_first_last`,
/// `show_prev_next`, and `on_page_change` properties.
#[proc_macro]
pub fn nav_pagination(input: TokenStream) -> TokenStream {
    let config = parse_macro_input!(input as PaginationConfig);

    let current_page = &config.current_page;
    let total_pages = &config.total_pages;
    let siblings = &config.siblings;
    let _show_first_last = &config.show_first_last;
    let show_prev_next = &config.show_prev_next;

    let (_callback_param, callback_emit) = if let Some(ref _cb) = config.on_page_change {
        (
            quote! { callback: Callback<u32> },
            quote! { callback.emit(page) },
        )
    } else {
        (quote! {}, quote! {})
    };

    let span = Span::call_site();

    let expanded = quote_spanned! { span =>
        yew::html! {
            <nav aria-label="pagination">
                <ul class="pagination">
                    if #show_prev_next {
                        <li class="page-item">
                            <button
                                class="page-link"
                                disabled={#current_page <= 1}
                                onclick={move |_| {
                                    #callback_emit
                                }}
                            >
                                {"Previous"}
                            </button>
                        </li>
                    }

                    <li class={classes!("page-item", if #current_page == 1 { "active" } else { "" })}>
                        <button class="page-link" onclick={move |_| { #callback_emit }}>
                            {"1"}
                        </button>
                    </li>

                    { for (1..*#total_pages as usize).filter(|&i| {
                        let lower = #current_page.saturating_sub(*#siblings as u32) as usize;
                        let upper = (#current_page + *#siblings as u32) as usize;
                        i >= lower && i <= upper && i < #total_pages as usize
                    }).map(|i| {
                        yew::html! {
                            <li class={classes!("page-item", if i == #current_page { "active" } else { "" })}>
                                <button class="page-link" onclick={move |_| { #callback_emit }}>
                                    { i }
                                </button>
                            </li>
                        }
                    }) }

                    if #total_pages > 1 {
                        <li class={classes!("page-item", if #current_page == #total_pages { "active" } else { "" })}>
                            <button class="page-link" onclick={move |_| { #callback_emit }}>
                                { #total_pages }
                            </button>
                        </li>
                    }

                    if #show_prev_next {
                        <li class="page-item">
                            <button
                                class="page-link"
                                disabled={#current_page >= #total_pages}
                                onclick={move |_| {
                                    #callback_emit
                                }}
                            >
                                {"Next"}
                            </button>
                        </li>
                    }
                </ul>
            </nav>
        }
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Nav Tabs Macro
// ============================================================================

struct TabConfig {
    route: syn::ExprPath,
    label: syn::LitStr,
}

struct NavTabsArgs {
    tabs: Vec<TabConfig>,
    active: Option<syn::ExprPath>,
}

impl Parse for TabConfig {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let route = input.parse::<syn::ExprPath>()?;
        input.parse::<Token![,]>()?;
        let label = input.parse::<syn::LitStr>()?;

        Ok(TabConfig { route, label })
    }
}

impl Parse for NavTabsArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut tabs = Vec::new();
        let mut active = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if lookahead.peek(syn::Ident) && input.peek2(Token![=]) {
                let ident: syn::Ident = input.parse()?;
                if ident == "active" {
                    input.parse::<Token![=]>()?;
                    active = Some(input.parse::<syn::ExprPath>()?);

                    if input.peek(Token![,]) {
                        input.parse::<Token![,]>()?;
                    }
                } else {
                    return Err(syn::Error::new(ident.span(), "Expected 'active' property"));
                }
            } else {
                tabs.push(input.parse::<TabConfig>()?);

                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                } else {
                    break;
                }
            }
        }

        Ok(NavTabsArgs { tabs, active })
    }
}

/// Creates a tabbed navigation interface from route-label pairs.
///
/// An optional `active = Route::Variant` selects the initially active tab.
#[proc_macro]
pub fn nav_tabs(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as NavTabsArgs);

    if args.tabs.is_empty() {
        let span = Span::call_site();
        return quote_spanned! { span =>
            yew::html! {}
        }
        .into();
    }

    let spans: Vec<_> = args.tabs.iter().map(|t| t.route.span()).collect();

    let (active_route, _active_expr) = if let Some(ref active) = args.active {
        (active, quote! { true })
    } else {
        let default = &args.tabs[0].route;
        (default, quote! { false })
    };

    let tabs_html = args.tabs.iter().enumerate().map(|(i, tab)| {
        let route = &tab.route;
        let label = &tab.label;
        let span = spans[i];

        let is_active = if route == active_route {
            quote! { true }
        } else {
            quote! { false }
        };

        quote_spanned! { span =>
            <li class={classes!("nav-tab", if #is_active { "active" } else { "" })}>
                <a href={route.to_path()}>
                    #label
                </a>
            </li>
        }
    });

    let expanded = quote! {
        yew::html! {
            <ul class="nav-tabs" role="tablist">
                #(#tabs_html)*
            </ul>
        }
    };

    TokenStream::from(expanded)
}
