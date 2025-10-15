use proc_macro::TokenStream;

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
    seferize_core::stringify(attr.into(), item.into()).into()
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
