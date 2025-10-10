use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, LitStr, Ident};

/// Macro que converte um item (struct, impl, trait, etc.)
/// em uma `&'static str` contendo o código-fonte do item.
///
/// # Uso:
/// ```rust
/// use stringify_item_macro::stringify_item;
///
/// #[stringify_item] // usa nome padrão: _CODE_<ident>
/// struct Example {
///     a: i32,
/// }
///
/// #[stringify_item("MY_CONST")]
/// trait MyTrait {}
/// ```
///
/// Isso gera uma constante `&str` com o código do item.
#[proc_macro_attribute]
pub fn stringify(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Lê o item e o atributo (se existir)
    let item_ast = parse_macro_input!(item as Item);

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
        let default_name = match &item_ast {
            Item::Struct(s) => format!("_CODE_{}", s.ident),
            Item::Enum(e) => format!("_CODE_{}", e.ident),
            Item::Trait(t) => format!("_CODE_{}", t.ident),
            Item::Impl(_) => "_CODE_IMPL".to_string(),
            _ => "_CODE_ITEM".to_string(),
        };
        Ident::new(&default_name, proc_macro2::Span::call_site())
    };

    // Gera novo código: item original + constante de string
    let expanded = quote! {
        #item_ast

        pub const #const_ident: &str = #code_str;
    };

    expanded.into()
}
