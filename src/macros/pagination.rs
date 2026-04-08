//! Creates a pagination component with configurable options
//!
//! # Example
//!
//! ```ignore
//! use yew::prelude::*;
//! use yew_nav_link_macros::nav_pagination;
//!
//! #[function_component]
//! fn Paginator() -> Html {
//!     html! {
//!         { nav_pagination! {
//!             current_page = 1,
//!             total_pages = 10,
//!             siblings = 2,
//!             show_first_last = true,
//!             on_page_change =|page| { /* callback */ },
//!         } }
//!     }
//! }
//! ```

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, parse::Parse, parse_quote, Span, Token};

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
            let ident: Ident = input.parse()?;
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
                return Err(syn::Error::new(ident.span(), format!("Unknown property: {}", ident)));
            }
            
            if input.peek(Token![,]) {
                input.parse::<Token![,]()?;
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
    let show_first_last = &config.show_first_last;
    let show_prev_next = &config.show_prev_next;
    
    let (callback_param, callback_emit) = if let Some(ref cb) = config.on_page_change {
        (quote! { callback: Callback<u32> }, quote! { callback.emit(page) })
    } else {
        (quote! {}, quote! {})
    };
    
    let span = Span::call_site();
    
    let expanded = quote_spanned! { span =>
        yew_html::html! {
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
                        yew_html::html! {
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pagination_parse() {
        let input = "current_page = 1, total_pages = 10";
        let result = parse_macro_input!(input as PaginationConfig);
        assert_eq!(result.current_page.to_token_stream().to_string(), "1");
        assert_eq!(result.total_pages.to_token_stream().to_string(), "10");
    }
}
