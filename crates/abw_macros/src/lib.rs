extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn leptos_app(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_block = &input.block;

    let expanded = quote! {
        pub fn #fn_name() -> ::std::sync::Arc<dyn Fn() -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ()> + Send>> + Send + Sync> {
            ::std::sync::Arc::new(|| {
                Box::pin(async move #fn_block) as ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ()> + Send>>
            })
        }
    };

    TokenStream::from(expanded)
}
