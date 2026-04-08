//! Creates breadcrumbs navigation from a route path
//!
//! # Example
//!
//! ```ignore
//! use yew::prelude::*;
//! use yew_nav_link_macros::breadcrumbs;
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
//! fn Breadcrumb() -> Html {
//!     let current_route = use_route::<Route>();
//!     
//!     html! {
//!         { breadcrumbs! {
//!             (Route::Home, "/"),
//!             (Route::Docs, "Docs"),
//!             current_route.map(|r| match r {
//!                 Route::DocsGettingStarted => (Route::DocsGettingStarted, "Getting Started"),
//!                 Route::DocsApi => (Route::DocsApi, "API"),
//!                 _ => unreachable!(),
//!             }),
//!         } }
//!     }
//! }
//! ```

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, parse::Parse, parse_quote, Token};

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
                input.parse::<Token![,]()?;
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
        }.into();
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
            <li class="breadcrumb-item#(if #is_last { " active" } else { "" })" #aria_current>
                <Link<#route> to={#route.clone()}>
                    #label
                </Link<#route>>
            </li>
        }
    });
    
    let expanded = quote! {
        yew_html::html! {
            <nav aria-label="breadcrumb">
                <ol class="breadcrumb">
                    #(#expanded)*
                </ol>
            </nav>
        }
    };
    
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_breadcrumbs_parse() {
        let input = "(Route::Home, \"/\")";
        let result = parse_macro_input!(input as BreadcrumbsArgs);
        assert_eq!(result.entries.len(), 1);
    }
}
