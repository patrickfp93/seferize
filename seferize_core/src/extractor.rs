use crate::*;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, Item, LitStr};

use crate::util::build_const;

pub struct Extractor;

impl Extractor {
    /// 🔹 Nova função: coleta e gera constantes `#[stringify("NOME")]`
    pub fn get_stringified_internal_constants(item: &mut Item) -> Vec<TokenStream> {
        let mut consts = Vec::new();

        match item {
            Item::Struct(s) => {
                // Itera sobre todos os campos da struct
                for field in s.fields.iter_mut() {
                    // Coleta apenas atributos que devem ser mantidos
                    let mut retained_attrs = Vec::new();

                    for (index, attr) in field.attrs.iter().enumerate() {
                        if let Some(name) = Self::extract_stringify_name(attr) {
                            let mut clone_field = field.clone();
                            clone_field.attrs.remove(index);
                            let content = clone_field.to_token_stream().to_string();
                            consts.push(build_const(&name, &content));
                            // Se for `stringify`, não mantém o atributo
                        } else {
                            retained_attrs.push(attr.clone());
                        }
                    }
                    // Substitui os atributos pelos que não foram processados
                    field.attrs = retained_attrs;
                }
            }
            Item::Impl(i) => {
                // Para cada método dentro do impl
                for impl_item in &mut i.items {
                    if let syn::ImplItem::Fn(method) = impl_item {
                        let mut retained_attrs = Vec::new();
                        for (index, attr) in method.attrs.iter().enumerate() {
                            if let Some(name) = Self::extract_stringify_name(attr) {
                                let mut cloned_method = method.clone();
                                cloned_method.attrs.remove(index);
                                let content = cloned_method.to_token_stream().to_string();
                                consts.push(build_const(&name, &content));
                            } else {
                                retained_attrs.push(attr.clone());
                            }
                        }
                        method.attrs = retained_attrs;
                    }
                }
            }

            Item::Enum(e) => {
                // Extrai as tags das variantes
                for variant in &mut e.variants {
                    let mut retained_attrs = Vec::new();
                    for (index, attr) in variant.attrs.iter().enumerate() {
                        if let Some(name) = Self::extract_stringify_name(attr) {
                            let mut cloned_variant = variant.clone();
                            cloned_variant.attrs.remove(index);
                            let content = cloned_variant.to_token_stream().to_string();
                            consts.push(build_const(&name, &content));
                        } else {
                            retained_attrs.push(attr.clone());
                        }
                    }
                    variant.attrs = retained_attrs;
                }
            }
            _ => {}
        }

        consts
    }

    fn extract_stringify_name(attr: &Attribute) -> Option<String> {
        if OCCURRENCES
            .iter()
            .find(|&&o| attr.path().to_token_stream().to_string().contains(o))
            .is_none()
        {
            return None;
        }
        // 1) Caso comum: #[attr("value")]
        return Parameters::new_from_atribute(attr, &attr.to_token_stream()).map_or(None, |p|
        {
            Some(p.const_ident(&attr.to_token_stream()).to_string())
        })
    }
}
