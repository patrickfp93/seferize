mod extractor;
mod filter;
pub mod parameters;

mod samples;
mod tests;
mod util;
static OCCURRENCES: [&'static str; 2] = ["seferize::stringify", "stringify"];
static IGNORE_OCCURRENCES: [&'static str; 2] = ["seferize::ignore", "ignore"];
const CONST_DEFAULT_PREFIX: &'static str = "CODE_";
const EXPOSE_TEST_PREFIX : &'static str = "testable_";

pub use crate::parameters::Parameters;

use crate::extractor::Extractor;
#[allow(unused)]
pub(crate) use crate::samples::{
    extract_into::{enumerate::*, implementation::*, structs::*},
    samples_with_ignore::{implemetation::*, module::*},
    simple_samples::{enumerate::*, implemetation::*, structs::*, tuple::*},
};
use crate::{
    filter::Filter,
    util::build_const,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{parse_quote, Item, ItemFn};

pub fn stringify(parameters: Parameters, item: TokenStream) -> TokenStream {
    // Lê o item e o atributo (se existir)
    let mut original: Item = parse_quote!(#item);
    let mut modified_item = original.clone();
    let consts = Extractor::get_stringified_internal_constants(&mut modified_item);
    let _ = Filter::remove_self_invocations(&mut modified_item);
    if consts.len() > 0 {
        original = modified_item.clone();
    }
    // Converte o item em token stream e string
    let code_str = modified_item.to_token_stream().to_string();

    // Verifica se o atributo tem um nome fornecido
    let const_ident = parameters.const_ident(&modified_item.to_token_stream());

    let main_const = build_const(&const_ident.to_string(), &code_str);
    // Gera novo código: item original + constante de string
    quote! {
        #main_const
        #(#consts)*

        #original
    }
}

pub fn expose_for_tests(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse da função original
    let input_fn : ItemFn = parse_quote!(#item);
    let fn_name = format!("{}{}",EXPOSE_TEST_PREFIX,input_fn.sig.ident);

    // Gera o nome da função de teste (pode ser igual, com pub e cfg)
    quote! {
        #input_fn

        #[cfg(test)]
        pub fn #fn_name(field: &Field, index: usize) -> ItemFn {
            #fn_name(field, index)
        }
    }
}
