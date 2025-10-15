use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse, parse_macro_input, Ident, Item, LitStr};
use crate::{filter::Filter, util::{build_const, generate_default_name}};

mod tests;
mod util;
mod filter;
const CONST_DEFAULT_PREFIX: &'static str = "CODE_";

pub fn stringify(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Lê o item e o atributo (se existir)
    let mut original : Item = parse(item.into()).unwrap();
    let mut modified_item = original.clone();
    let consts = Filter::extract_stringified_internal_constants(&original);
    let _ = Filter::remove_self_invocations(&mut modified_item);
    if consts.len() > 0 {
        original = modified_item.clone();
    } 
    // Converte o item em token stream e string
    let tokens = quote! { #modified_item };
    let code_str = tokens.to_string();

    // Verifica se o atributo tem um nome fornecido
    let const_ident = if !attr.is_empty() {
        // Se tiver um literal string, usa ele
        let lit: LitStr = parse(attr.into()).unwrap();
        lit.value()
    } else {
        // Gera nome padrão com base no identificador do item
        let default_name = generate_default_name(&modified_item, CONST_DEFAULT_PREFIX);
        default_name
    };

    let main_const = build_const(&const_ident.to_string(), &code_str);
    // Gera novo código: item original + constante de string
    quote! {
        #main_const
        #(#consts)*

        #original
    }
}