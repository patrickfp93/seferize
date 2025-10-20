use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_str, ItemFn, Pat};


pub fn build_const(name: &str, content: &str) -> TokenStream {
    let name: TokenStream = parse_str(name).unwrap();
    quote! {
        pub const #name: &'static str = #content;
    }
}

pub fn generate_call(fn_item: &ItemFn) -> TokenStream {
    // Extrai os nomes dos argumentos
    let arg_names: Vec<TokenStream> = fn_item.sig.inputs.iter().map(|arg| {
        match arg {
            syn::FnArg::Receiver(_) => quote! { self },   // &self ou self
            syn::FnArg::Typed(pat_type) => {
                match *pat_type.pat.clone() {
                    Pat::Ident(ref ident) => {
                        let name = &ident.ident;
                        quote! { #name }
                    },
                    _ => quote! { _ } // caso não seja simples
                }
            }
        }
    }).collect();

    let fn_name = &fn_item.sig.ident;

    // Cria a chamada
    quote! {
        #fn_name(#(#arg_names),*)
    }
}