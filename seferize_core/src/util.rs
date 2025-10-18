use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;


pub fn build_const(name: &str, content: &str) -> TokenStream {
    let name: TokenStream = parse_str(name).unwrap();
    quote! {
        pub const #name: &'static str = #content;
    }
}
