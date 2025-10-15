mod tests;
mod samples;
mod util;
mod filter;
const CONST_DEFAULT_PREFIX: &'static str = "CODE_";

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, parse_str, Ident, Item};
use crate::{filter::Filter, util::{build_const, generate_default_name}};
pub use crate::samples::*;


pub fn stringify(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Lê o item e o atributo (se existir)
    let mut original : Item = parse_quote!(#item);
    let mut modified_item = original.clone();
    let consts = Filter::extract_stringified_internal_constants(&original);
    let _ = Filter::remove_self_invocations(&mut modified_item);
    if consts.len() > 0 {
        original = modified_item.clone();
    } 
    // Converte o item em token stream e string
    let code_str = modified_item.to_token_stream().to_string();

    // Verifica se o atributo tem um nome fornecido
    let const_ident = if !attr.is_empty() {
        // Se tiver um literal string, usa ele
        attr.to_string()
    } else {
        // Gera nome padrão com base no identificador do item
        let default_name = generate_default_name(&modified_item, CONST_DEFAULT_PREFIX);
        default_name
    };

    let main_const = build_const(&const_ident, &code_str);
    // Gera novo código: item original + constante de string
    quote! {
        #main_const
        #(#consts)*

        #original
    }
}