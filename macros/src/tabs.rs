//! Creates a tabbed navigation interface
//!
//! # Example
//!
//! ```ignore
//! use yew::prelude::*;
//! use yew_nav_link_macros::nav_tabs;
//! use yew_router::prelude::*;
//!
//! #[derive(Clone, PartialEq, Routable)]
//! enum Route {
//!     #[at("/tab1")]
//!     Tab1,
//!     #[at("/tab2")]
//!     Tab2,
//!     #[at("/tab3")]
//!     Tab3,
//! }
//!
//! #[function_component]
//! fn Tabs() -> Html {
//!     html! {
//!         { nav_tabs! {
//!             (Route::Tab1, "Tab 1"),
//!             (Route::Tab2, "Tab 2"),
//!             (Route::Tab3, "Tab 3"),
//!             active = Route::Tab1,
//!         } }
//!     }
//! }
//! ```

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, parse::Parse, parse_quote,Span, Token};

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
            
            if lookahead.peek(syn::ident) && input.peek2(Token![=]) {
                let ident: Ident = input.parse()?;
                if ident == "active" {
                    input.parse::<Token![=]>()?;
                    active = Some(input.parse::<syn::ExprPath>()?);
                    
                    if input.peek(Token![,]) {
                        input.parse::<Token![,]()?;
                    }
                } else {
                    return Err(syn::Error::new(ident.span(), "Expected 'active' property"));
                }
            } else {
                tabs.push(input.parse::<TabConfig>()?);
                
                if input.peek(Token![,]) {
                    input.parse::<Token![,]()?;
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
        }.into();
    }
    
    let spans: Vec<_> = args.tabs.iter().map(|t| t.route.span()).collect();
    
    let (active_route, active_expr) = if let Some(ref active) = args.active {
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
        yew_html::html! {
            <ul class="nav-tabs" role="tablist">
                #(#tabs_html)*
            </ul>
        }
    };
    
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_nav_tabs_parse() {
        let input = "(Route::Tab1, \"Tab 1\")";
        let result = parse_macro_input!(input as NavTabsArgs);
        assert_eq!(result.tabs.len(), 1);
    }
    
    #[test]
    fn test_nav_tabs_active() {
        let input = "(Route::Tab1, \"Tab 1\"), active = Route::Tab1";
        let result = parse_macro_input!(input as NavTabsArgs);
        assert!(result.active.is_some());
    }
}
