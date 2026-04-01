//! Creates a navigation list with multiple nav items
//!
//! # Example
//!
//! ```ignore
//! use yew::prelude::*;
//! use yew_nav_link_macros::nav_list;
//! use yew_router::prelude::*;
//!
//! #[derive(Clone, PartialEq, Routable)]
//! enum Route {
//!     #[at("/")]
//!     Home,
//!     #[at("/about")]
//!     About,
//!     #[at("/contact")]
//!     Contact,
//! }
//!
//! #[function_component]
//! fn NavListExample() -> Html {
//!     html! {
//!         { nav_list! {
//!             (Route::Home, "Home", Match::Exact),
//!             (Route::About, "About", Match::Exact),
//!             (Route::Contact, "Contact", Match::Exact),
//!             disabled = vec![false, false, false],
//!         } }
//!     }
//! }
//! ```

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, parse::Parse, parse_quote, Span, Token};

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
        
        Ok(NavItemConfig { route, label, match_mode })
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
            
            if lookahead.peek(syn::ident) && input.peek2(Token![=]) {
                let ident: Ident = input.parse()?;
                if ident == "disabled" {
                    input.parse::<Token![=]>()?;
                    disabled = Some(input.parse()?);
                    
                    if input.peek(Token![,]) {
                        input.parse::<Token![,]()?;
                    }
                } else {
                    return Err(syn::Error::new(ident.span(), "Expected 'disabled' property"));
                }
            } else {
                items.push(input.parse::<NavItemConfig>()?);
                
                if input.peek(Token![,]) {
                    input.parse::<Token![,]()?;
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
        }.into();
    }
    
    let spans: Vec<_> = args.items.iter().map(|i| i.route.span()).collect();
    
    let (disabled_vec, disabled_access) = if let Some(ref d) = args.disabled {
        (quote! { #d }, quote! { #d.get(i).copied().unwrap_or(false) })
    } else {
        let disableds: Vec<syn::Expr> = args.items.iter().map(|_| parse_quote!(false)).collect();
        (quote! { vec![#(#disableds),*] }, quote! { disableds.get(i).copied().unwrap_or(false) })
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
            yew_html::html! {
                <ul class="nav-list">
                    #(#items_html)*
                </ul>
            }
        }
    };
    
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_nav_list_parse() {
        let input = "(Route::Home, \"Home\", Match::Exact)";
        let result = parse_macro_input!(input as NavListArgs);
        assert_eq!(result.items.len(), 1);
    }
    
    #[test]
    fn test_nav_list_disabled() {
        let input = "(Route::Home, \"Home\", Match::Exact), disabled = vec![false]";
        let result = parse_macro_input!(input as NavListArgs);
        assert!(result.disabled.is_some());
    }
}
