use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, Ident, Item, Meta, parse_quote};
use syn::{Lit, LitStr};

use crate::util::build_const;

pub struct Filter;

static OCCURRENCES: [&'static str; 2] = ["seferize::stringify", "stringify"];

static IGNORE_OCCURRENCES: [&'static str; 2] = ["seferize::ignore", "ignore"];

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
            }
            Item::Enum(e) => {
                e.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                /*e.variants.retain(|variant| {
                        variant.attrs.retain(|attr| !Self::is_target_macro_attribure(attr));
                        true // variantes individuais não removemos ainda
                    });*/
            }
            Item::Trait(t) => {
                t.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                for item in &mut t.items {
                    match item {
                        syn::TraitItem::Fn(m) => {
                            m.attrs
                                .retain(|attr| !Self::is_target_macro_attribure(attr));
                        }
                        _ => {}
                    }
                }
            }
            Item::Impl(i) => {
                i.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                i.items.retain(|impl_item| {
                    match impl_item {
                        syn::ImplItem::Fn(m) => m
                            .attrs
                            .clone()
                            .retain(|a| !Self::is_target_macro_attribure(a)),
                        syn::ImplItem::Const(c) => c
                            .attrs
                            .clone()
                            .retain(|a| !Self::is_target_macro_attribure(a)),
                        syn::ImplItem::Type(t) => t
                            .attrs
                            .clone()
                            .retain(|a| !Self::is_target_macro_attribure(a)),
                        _ => {}
                    }
                    true
                });
            }
            Item::Mod(m) => {
                m.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                if let Some((_, items)) = &mut m.content {
                    items.retain(|sub_item| {
                        !Self::remove_self_invocations(&mut sub_item.clone())
                    });
                    // chama recursivamente os sub-itens
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
            _ => {}
        }

        false
    }

    /// 🔹 Nova função: coleta e gera constantes `#[stringify("NOME")]`
    pub fn extract_stringified_internal_constants(item: &Item) -> Vec<Item> {
        let mut consts = Vec::new();

        match item {
            Item::Struct(s) => {
                // Extrai a tag da struct, se houver
                /*for attr in &s.attrs {
                        if let Some(name) = Self::extract_stringify_name(attr,&s.ident.to_string()) {
                            let content = s.to_token_stream().to_string();
                            consts.push(build_const(&name, &content));
                        }
                    }*/

                // Extrai as tags dos fields
                for (index, field) in s.fields.iter().enumerate() {
                    let name = if let Some(ident) = &field.ident {
                        ident.to_string()
                    } else {
                        format!("field_{}", index.to_string())
                    };
                    for attr in &field.attrs {
                        if let Some(name) = Self::extract_stringify_name(attr, &name) {
                            let content = field.to_token_stream().to_string();
                            consts.push(build_const(&name, &content));
                        }
                    }
                }
            }

            Item::Impl(i) => {
                /*for attr in &i.attrs {
                        if let Some(name) = Self::extract_stringify_name(attr) {
                            let content = i.to_token_stream().to_string();
                            consts.push(build_const(&name, &content));
                        }
                    }*/

                // Para cada método dentro do impl
                for impl_item in &i.items {
                    if let syn::ImplItem::Fn(f) = impl_item {
                        let name = f.sig.ident.to_string();
                        for attr in &f.attrs {
                            if let Some(name) = Self::extract_stringify_name(attr, &name) {
                                let content = f.to_token_stream().to_string();
                                consts.push(build_const(&name, &content));
                            }
                        }
                    }
                }
            }

            Item::Enum(e) => {
                // Extrai a tag do enum, se houver
                /*for attr in &e.attrs {
                        if let Some(name) = Self::extract_stringify_name(attr) {
                            let content = e.to_token_stream().to_string();
                            consts.push(build_const(&name, &content));
                        }
                    }*/

                // Extrai as tags das variantes
                for variant in &e.variants {
                    for attr in &variant.attrs {
                        let name = variant.ident.to_string();
                        if let Some(name) = Self::extract_stringify_name(attr, &name) {
                            let content = variant.to_token_stream().to_string();
                            consts.push(build_const(&name, &content));
                        }
                    }
                }
            }

            _ => {}
        }

        consts
    }

    fn extract_stringify_name(attr: &Attribute, defalt_name: &str) -> Option<String> {
        if !attr.path().is_ident("attr") {
            return None;
        }

        // 1) Caso comum: #[attr("value")]
        if let Ok(litstr) = attr.parse_args::<LitStr>() {
            return Some(litstr.value());
        } else {
            return Some(format!("{}{}", super::CONST_DEFAULT_PREFIX, defalt_name));
        }
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
