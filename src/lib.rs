use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Ident, Item, LitStr, parse_macro_input};

mod filter;
use crate::{filter::Filter, util::generate_default_name};

const CONST_DEFAULT_PREFIX : &'static str = "CODE_";

mod util;

/// Macro that converts an item (struct, impl, trait, etc.)
/// into a `&'static str` containing the item's source code.
///
/// # Usage:
/// ```rust
/// use seferize::stringify;
///
/// #[stringify] // uses default name: CODE_<ident>
/// struct Example {
/// a: i32,
/// }
///
/// #[stringify("MY_CONST")]
/// trait MyTrait {}
/// ```
///
/// This generates a constant `&str` with the item's code.
#[proc_macro_attribute]
pub fn stringify(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Lê o item e o atributo (se existir)
    let item_copy = item.clone();
    let original = parse_macro_input!(item as Item).clone();
    let mut item_ast = parse_macro_input!(item_copy as Item);
    let _ = Filter::remove_self_invocations(&mut item_ast);
    // Converte o item em token stream e string
    let tokens = quote! { #item_ast };
    let code_str = tokens.to_string();

    // Verifica se o atributo tem um nome fornecido
    let const_ident = if !attr.is_empty() {
        // Se tiver um literal string, usa ele
        let lit = parse_macro_input!(attr as LitStr);
        Ident::new(&lit.value(), lit.span())
    } else {
        // Gera nome padrão com base no identificador do item
        let default_name = generate_default_name(&item_ast, CONST_DEFAULT_PREFIX);
        Ident::new(&default_name, proc_macro2::Span::call_site())
    };

    // Gera novo código: item original + constante de string
    let expanded = quote! {        
        pub const #const_ident: &str = #code_str;
        #original
    };

    expanded.into()
}

/// Macro causes the #[stringify] macro to ignore some item like:
/// module, trait, struct, etc.
///
/// # Usage:
/// ```rust
/// use seferize::*;
///
/// #[stringify]
/// mod module{
///     #[ignore]
///     const a : u32= 1;
///     //...
/// }
/// 
/// ```
/// 
/// This filters out items that shouldn't be in the `&str` constant..
#[proc_macro_attribute]
pub fn ignore(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}
