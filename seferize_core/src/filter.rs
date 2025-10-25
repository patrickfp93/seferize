use crate::*;
use quote::ToTokens;
use syn::{
    Attribute, ImplItem, ImplItemConst, ImplItemFn, ImplItemType, Item, ItemImpl, parse_quote,
};
pub struct Filter;

//static ALL_OCCURRENCES: [&'static str; 4] = ["seferize::stringify", "stringify","seferize::ignore", "ignore"];

impl Filter {
    pub fn remove_self_invocations(item_to_str: &mut Item) -> bool {
        // Se o próprio item tem ignore, sinaliza remoção
        if Self::should_remove_item(item_to_str) {
            return true; // remove este item do bloco externo
        }

        match item_to_str {
            Item::Struct(s) => {
                s.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                s.fields.iter_mut().for_each(|f| {
                    f.attrs
                        .retain(|attr| !Self::is_target_macro_attribure(attr))
                });
            }
            Item::Enum(e) => {
                e.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                // Opcional: iterar sobre variantes se desejar filtrar atributos
                for variant in &mut e.variants {
                    variant
                        .attrs
                        .retain(|attr| !Self::is_target_macro_attribure(attr));
                }
            }
            Item::Trait(t) => {
                t.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                for item in &mut t.items {
                    match item {
                        syn::TraitItem::Fn(m) => m
                            .attrs
                            .retain(|attr| !Self::is_target_macro_attribure(attr)),
                        syn::TraitItem::Const(c) => c
                            .attrs
                            .retain(|attr| !Self::is_target_macro_attribure(attr)),
                        syn::TraitItem::Type(ty) => ty
                            .attrs
                            .retain(|attr| !Self::is_target_macro_attribure(attr)),
                        _ => {}
                    }
                }
            }
            Item::Impl(item_impl) => {
                item_impl.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                Self::remove_self_invocations_from_impl(item_impl);
            }
            Item::Mod(m) => {
                m.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                if let Some((_brace, items)) = &mut m.content {
                    items.retain(|sub_item| !Self::remove_self_invocations(&mut sub_item.clone()));
                    for sub_item in items {
                        Self::remove_self_invocations(sub_item);
                    }
                }
            }
            Item::Macro(mac) => {
                let segments: Vec<String> = mac
                    .mac
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect();
                if Self::is_target_macro_from_slice(&segments) {
                    *mac = parse_quote!(); // remove a macro
                }
            }
            // Itens adicionais que possuem atributos
            Item::Const(c) => c
                .attrs
                .retain(|attr| !Self::is_target_macro_attribure(attr)),
            Item::Static(s) => s
                .attrs
                .retain(|attr| !Self::is_target_macro_attribure(attr)),
            Item::Type(t) => t
                .attrs
                .retain(|attr| !Self::is_target_macro_attribure(attr)),
            Item::Union(u) => u
                .attrs
                .retain(|attr| !Self::is_target_macro_attribure(attr)),
            Item::TraitAlias(ta) => ta
                .attrs
                .retain(|attr| !Self::is_target_macro_attribure(attr)),
            _ => {}
        }

        false
    }

    pub fn remove_self_invocations_from_impl(item_impl: &mut ItemImpl) -> bool {

        // Se o próprio impl tem #[ignore], sinaliza remoção
        if Self::should_remove_item(&Item::Impl(item_impl.clone())) {
            return true;
        }
        
        Self::remove_ignored_from_impl(item_impl);

        // Remove atributos do próprio impl
        item_impl.attrs
            .retain(|attr| !Self::is_target_macro_attribure(attr));

        // Itera sobre cada item do impl (métodos, const, type)
        for impl_item in &mut item_impl.items {
            match impl_item {
                syn::ImplItem::Fn(method) => {
                    method
                        .attrs
                        .retain(|attr| !Self::is_target_macro_attribure(attr));
                }
                syn::ImplItem::Const(c) => {
                    c.attrs
                        .retain(|attr| !Self::is_target_macro_attribure(attr));
                }
                syn::ImplItem::Type(t) => {
                    t.attrs
                        .retain(|attr| !Self::is_target_macro_attribure(attr));
                }
                _ => {}
            }
        }

        false
    }

    fn remove_ignored_from_impl(item_impl: &mut ItemImpl) {
        item_impl.items.retain(|impl_item| !match impl_item {
            ImplItem::Fn(ImplItemFn { attrs, .. })
            | ImplItem::Const(ImplItemConst { attrs, .. })
            | ImplItem::Type(ImplItemType { attrs, .. }) => Self::has_ignore_attr(attrs),
            _ => false,
        });
    }

    fn has_ignore_attr(attrs: &[Attribute]) -> bool {
        attrs.iter().any(|attr| {
            IGNORE_OCCURRENCES
                .iter()
                .find(|&&i_o| &attr.path().to_token_stream().to_string() == i_o)
                .is_some()
        })
    }

    /// Verifica se o item possui "ignore" ou "seferize::ignore" e deve ser removido
    fn should_remove_item(item: &Item) -> bool {
        let attrs = Self::get_item_attributes(item);

        for attr in attrs {
            let path_segments = attr
                .path()
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>();
            let path = path_segments.join("::");
            return IGNORE_OCCURRENCES
                .iter()
                .find(|&i_o| *i_o == &path)
                .is_some();
        }

        false
    }
    /// Verifica se o path da macro é um dos alvos
    fn is_target_macro_attribure(attribute: &Attribute) -> bool {
        let path_segments = attribute
            .path()
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<String>>();
        Self::is_target_macro_from_slice(&path_segments)
    }

    fn is_target_macro_from_slice(path_segments: &[String]) -> bool {
        let path = path_segments.join("::");
        OCCURRENCES.iter().any(|&occ| occ == path)
    }

    pub fn get_item_attributes(item: &Item) -> &[Attribute] {
        match item {
            Item::Const(i) => &i.attrs,
            Item::Enum(i) => &i.attrs,
            Item::ExternCrate(i) => &i.attrs,
            Item::Fn(i) => &i.attrs,
            Item::ForeignMod(i) => &i.attrs,
            Item::Impl(i) => &i.attrs,
            Item::Macro(i) => &i.attrs,
            Item::Mod(i) => &i.attrs,
            Item::Static(i) => &i.attrs,
            Item::Struct(i) => &i.attrs,
            Item::Trait(i) => &i.attrs,
            Item::TraitAlias(i) => &i.attrs,
            Item::Type(i) => &i.attrs,
            Item::Union(i) => &i.attrs,
            Item::Use(i) => &i.attrs,
            Item::Verbatim(_) => &[],
            _ => &[],
        }
    }
}
