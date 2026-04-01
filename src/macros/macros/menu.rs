//! Creates a nested navigation menu from a hierarchical structure
//!
//! # Example
//!
//! ```ignore
//! use yew::prelude::*;
//! use yew_nav_link_macros::nav_menu;
//! use yew_router::prelude::*;
//!
//! #[derive(Clone, PartialEq, Routable)]
//! enum Route {
//!     #[at("/")]
//!     Home,
//!     #[at("/docs")]
//!     Docs,
//!     #[at("/docs/getting-started")]
//!     DocsGettingStarted,
//!     #[at("/docs/api")]
//!     DocsApi,
//! }
//!
//! #[function_component]
//! fn Menu() -> Html {
//!     html! {
//!         <nav class="menu">
//!             { nav_menu! {
//!                 (Route::Home, "Home", Match::Exact),
//!                 (Route::Docs, "Docs", Match::Partial, {
//!                     (Route::DocsGettingStarted, "Getting Started", Match::Exact),
//!                     (Route::DocsApi, "API Reference", Match::Exact),
//!                 }),
//!             } }
//!         </nav>
//!     }
//! }
//! ```

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, parse::Parse, parse_quote, Ident, Span, Token};

enum MenuEntry {
    Link(NavLinkConfig),
    Group(NavLinkConfig, Vec<NavLinkConfig>),
}

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
        
        Ok(NavLinkConfig { route, label, match_mode })
    }
}

struct NavMenuArgs {
    entries: Vec<MenuEntry>,
}

impl Parse for NavMenuArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut entries = Vec::new();
        
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            
            if lookahead.peek(syn::braces) {
                let group;
                syn::braces!(group in input);
                let inner_entries = group.parse_terminated::<NavLinkConfig, Token![,]>(NavLinkConfig::parse)?;
                let last_entry = entries.pop().ok_or_else(|| {
                    syn::Error::new(input.span(), "Nested menu must follow a menu group")
                })?;
                
                match last_entry {
                    MenuEntry::Link(config) => {
                        entries.push(MenuEntry::Group(config, inner_entries.into_iter().collect()));
                    }
                    MenuEntry::Group(_, _) => {
                        return Err(syn::Error::new(input.span(), "Nested groups not supported"));
                    }
                }
            } else {
                let config = input.parse::<NavLinkConfig>()?;
                entries.push(MenuEntry::Link(config));
                
                if input.peek(Token![,]) {
                    input.parse::<Token![,]()?;
                } else {
                    break;
                }
            }
        }
        
        Ok(NavMenuArgs { entries })
    }
}

fn generate_menu_html(entries: &[MenuEntry]) -> proc_macro2::TokenStream {
    let items = entries.iter().map(|entry| {
        match entry {
            MenuEntry::Link(config) => {
                let route = &config.route;
                let label = &config.label;
                let match_mode = &config.match_mode;
                quote_spanned! { config.route.span() =>
                    yew_html::html! {
                        <li class="nav-item">
                            { yew_nav_link::nav_link(#route, #label, #match_mode) }
                        </li>
                    }
                }
            }
            MenuEntry::Group(config, children) => {
                let route = &config.route;
                let label = &config.label;
                let match_mode = &config.match_mode;
                let child_items = generate_menu_html(children);
                quote_spanned! { config.route.span() =>
                    yew_html::html! {
                        <li class="nav-item dropdown">
                            <a class="nav-link dropdown-toggle" href="#" data-bs-toggle="dropdown">
                                #label
                            </a>
                            <ul class="dropdown-menu">
                                #child_items
                            </ul>
                        </li>
                    }
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
        }.into();
    }
    
    let expanded = generate_menu_html(&args.entries);
    
    TokenStream::from(quote! {
        {
            #expanded
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_nav_menu_parse() {
        let input = "(Route::Home, \"Home\", Match::Exact)";
        let result = parse_macro_input!(input as NavMenuArgs);
        assert_eq!(result.entries.len(), 1);
    }
}
