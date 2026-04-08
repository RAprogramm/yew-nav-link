//! Creates multiple navigation links at once from an array of route
//! configurations
//!
//! # Example
//!
//! ```ignore
//! use yew::prelude::*;
//! use yew_nav_link_macros::nav_links;
//! use yew_router::prelude::*;
//!
//! #[derive(Clone, PartialEq, Routable)]
//! enum Route {
//!     #[at("/")]
//!     Home,
//!     #[at("/about")]
//!     About,
//!     #[at("/docs")]
//!     Docs,
//! }
//!
//! #[function_component]
//! fn Navigation() -> Html {
//!     html! {
//!         <nav class="nav">
//!             { nav_links! {
//!                 (Route::Home, "Home", Match::Exact),
//!                 (Route::About, "About", Match::Exact),
//!                 (Route::Docs, "Docs", Match::Partial),
//!             } }
//!         </nav>
//!     }
//! }
//! ```

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse::Parse, parse_macro_input, Token};

/// Represents a single navigation link configuration
struct NavLinkConfig {
    route:      syn::ExprPath,
    label:      syn::LitStr,
    match_mode: syn::ExprPath
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
            match_mode
        })
    }
}

/// Parses thenav_links! macro arguments
struct NavLinksArgs {
    entries: Vec<NavLinkConfig>
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

        Ok(NavLinksArgs {
            entries
        })
    }
}

/// Creates multiple navigation links efficiently
#[proc_macro]
pub fn nav_links(input: TokenStream) -> TokenStream {
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

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nav_links_empty() {
        let input = "";
        let result = parse_macro_input!(input as NavLinksArgs);
        assert!(result.entries.is_empty());
    }
}
