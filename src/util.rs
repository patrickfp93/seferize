use quote::ToTokens;
use syn::Item;

pub fn generate_default_name(item_ast: &Item, prefix: &str) -> String {
match item_ast {
    Item::Const(i) => format!("{}{}", prefix, i.ident),
    Item::Enum(i) => format!("{}{}", prefix, i.ident),
    Item::ExternCrate(i) => format!("{}{}", prefix, i.ident),
    Item::Fn(i) => format!("{}{}", prefix, i.sig.ident),
    Item::ForeignMod(_) => format!("{}FOREIGN_MOD", prefix),
    Item::Impl(i) => format!("{}{}", prefix, i.self_ty.to_token_stream().to_string()),
    Item::Macro(i) => format!("{}{}", prefix, i.mac.path.to_token_stream().to_string()),
    Item::Mod(i) => format!("{}{}", prefix, i.ident),
    Item::Static(i) => format!("{}{}", prefix, i.ident),
    Item::Struct(i) => format!("{}{}", prefix, i.ident),
    Item::Trait(i) => format!("{}{}", prefix, i.ident),
    Item::TraitAlias(i) => format!("{}{}", prefix, i.ident),
    Item::Type(i) => format!("{}{}", prefix, i.ident),
    Item::Union(i) => format!("{}{}", prefix, i.ident),
    Item::Use(i) => format!("{}USE_{}", prefix, i.tree.to_token_stream().to_string()),
    Item::Verbatim(v) => format!("{}VERBATIM_{}", prefix, v.to_token_stream().to_string()),
    _ => format!("{}CODE_ITEM", prefix),
}
}
