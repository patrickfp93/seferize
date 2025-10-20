use proc_macro::TokenStream;
use syn::{Ident, LitStr};

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
    let params = seferize_core::Parameters::new(attr.into(), &item.clone().into()).unwrap();
    seferize_core::stringify(params, item.into()).into()
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

/// `expose_for_tests` is a proc-macro attribute that creates a **public version of a private function**
/// **only for tests**. The original function stays private.
///
/// # Example
///
/// ```rust
/// use expose_for_tests_macro::expose_for_tests;
///
///
/// #[expose_for_tests]
/// fn hidden(&self) -> i32 {
///      42
/// }
///
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///
///     #[test]
///     fn can_call_hidden() {
///         // Call the generated public test version
///         assert_eq!(test_hidden(), 42);
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn expose_for_tests(_attr: TokenStream, item: TokenStream) -> TokenStream {
    seferize_core::expose_for_tests(_attr.into(), item.into()).into()
}
