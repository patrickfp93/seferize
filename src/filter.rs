use syn::{Attribute, Item, parse_quote};
pub struct Filter;

static OCCURRENCES: [&'static str; 4] = [
    "seferize::stringify",
    "stringify",
    "seferize::ignore",
    "ignore",
];

impl Filter {
    pub fn remove_self_invocations(item: &mut Item) {
        match item {
            Item::Struct(s) => {
                // Remove macros do struct
                s.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));

                // Não há sub-itens dentro de struct tuple ou unit
            }
            Item::Enum(e) => {
                e.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                // Percorre variantes
                for variant in &mut e.variants {
                    variant
                        .attrs
                        .retain(|attr| !Self::is_target_macro_attribure(attr));
                }
            }
            Item::Trait(t) => {
                t.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                // Métodos do trait
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
                for item in &mut i.items {
                    match item {
                        syn::ImplItem::Fn(m) => {
                            m.attrs
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
            }
            Item::Mod(m) => {
                m.attrs
                    .retain(|attr| !Self::is_target_macro_attribure(attr));
                // Recursão: processa conteúdo do módulo se houver
                if let Some((_, items)) = &mut m.content {
                    for sub_item in items {
                        Self::remove_self_invocations(sub_item);
                    }
                }
            }
            Item::Macro(m) => {
                // Se for a macro alvo, substitui por nada
                let segments: Vec<String> = m
                    .mac
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect();
                if Self::is_target_macro_from_slice(&segments) {
                    *m = parse_quote!(); // remove
                }
            }
            _ => {}
        }
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
}
