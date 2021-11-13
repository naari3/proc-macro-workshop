use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(_input: TokenStream) -> TokenStream {
    let expanded = quote! {};
    TokenStream::from(expanded)
}
