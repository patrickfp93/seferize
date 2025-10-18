use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, Field, Ident, ImplItem, Item, LitStr, Variant, parse, parse_str, parse2};

pub struct Parameters {
    const_ident_str: String,
}

impl Parameters {
    pub fn new(
        params_attr: TokenStream,
        reference_token: &TokenStream,
    ) -> Result<Self, syn::Error> {
        if params_attr.is_empty() {
            return Ok(Self {
                const_ident_str: "".into(),
            });
        }
        match parse_str::<Ident>(&params_attr.clone().to_string()) {
            Ok(ident) => Ok(Self {
                const_ident_str: ident.to_string(),
            }),
            Err(_) => match parse_str::<LitStr>(&params_attr.to_string()) {
                Ok(lit) => Ok(Self {
                    const_ident_str: lit.value(),
                }),
                Err(_) => {
                    return Err(syn::Error::new_spanned(
                        proc_macro2::TokenStream::from(reference_token.clone()),
                        "Expected an identifier or a string literal, e.g. #[stringify(Name)] or #[stringify(\"Name\")]",
                    ));
                }
            },
        }
    }

    pub fn new_from_atribute(
        macro_attr: &Attribute,
        reference_token: &TokenStream,
    ) -> Result<Self, syn::Error> { 
        let params_attr = match &macro_attr.meta {
            syn::Meta::List(meta_list) => meta_list.parse_args::<TokenStream>()?,
            _ => TokenStream::new(), /*syn::Meta::NameValue(meta_name_value) => todo!(),*/
        };
        Self::new(params_attr, reference_token)
    }

    pub fn const_ident(&self, tokens: &TokenStream) -> Ident {
        let ident_str = if !self.const_ident_str.is_empty() {
            // Se tiver um literal string, usa ele
            self.const_ident_str.clone()
        } else {
            // Gera nome padrão com base no identificador do item
            let default_name = if let Ok(item) = parse2::<Item>(tokens.clone()) {
                Self::generate_default_name_from_item(&item, super::CONST_DEFAULT_PREFIX)
            } else {
                "".to_string()
            };
            default_name
        };
        parse_str(&ident_str).unwrap()
    }

    fn parse_field(tokens: TokenStream) -> Result<Field,syn::Error> {
        // Envolvemos o campo em um contexto de struct temporária
        let no_field = syn::Error::new_spanned(tokens.clone(), "Expected a field!");
        let wrapped: syn::ItemStruct = syn::parse2(quote::quote! {
            struct __Temp { #tokens }
        })?;
        match wrapped.fields {
            syn::Fields::Named(fields_named) => if fields_named.named.len() == 1{
                return Ok(fields_named.named.get(0).unwrap().clone())
            }else{
                return Err(no_field.clone());
            },
            syn::Fields::Unnamed(fields_unnamed) => Ok(fields_unnamed.unnamed.get(0).unwrap().clone()),
            syn::Fields::Unit => Err(no_field),
        }
    }

    pub fn generate_default_name_from_item(item_ast: &Item, prefix: &str) -> String {
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
            Item::Verbatim(v) => {
                format!("{}VERBATIM_{}", prefix, v.to_token_stream().to_string())
            }
            _ => format!("{}CODE_ITEM", prefix),
        }
    }

    pub fn generate_default_name_from_tokens(tokens: TokenStream, prefix: &str) -> String {
        // 1️⃣ Se for um campo de struct
        if let Ok(field) = Self::parse_field(tokens.clone()) {
            let name = field
                .ident
                .as_ref()
                .map(|n| n.to_string())
                .unwrap_or("_".into());
            let ty = field.ty.to_token_stream().to_string();
            return format!(
                "{}FIELD_{}_{}",
                prefix,
                name.to_uppercase(),
                ty.replace(' ', "")
            );
        }

        // 2️⃣ Se for uma variante de enum
        if let Ok(variant) = parse2::<Variant>(tokens.clone()) {
            return format!("{}VARIANT_{}", prefix, variant.ident);
        }

        // 3️⃣ Se for um item dentro de um `impl`
        if let Ok(impl_item) = parse2::<ImplItem>(tokens.clone()) {
            let name = match impl_item {
                ImplItem::Fn(f) => format!("FN_{}", f.sig.ident),
                ImplItem::Const(c) => format!("CONST_{}", c.ident),
                ImplItem::Type(t) => format!("TYPE_{}", t.ident),
                _ => "IMPL_ITEM_OTHER".into(),
            };
            return format!("{}{}", prefix, name);
        }

        // 4️⃣ Caso seja um item de topo, mas genérico (ex: struct, enum, impl)
        if let Ok(item) = parse2::<Item>(tokens.clone()) {
            return match item {
                Item::Struct(i) => format!("{}STRUCT_{}", prefix, i.ident),
                Item::Enum(e) => format!("{}ENUM_{}", prefix, e.ident),
                Item::Impl(i) => {
                    format!("{}IMPL_{}", prefix, i.self_ty.to_token_stream().to_string())
                }
                _ => format!("{}ITEM_GENERIC", prefix),
            };
        }

        // 5️⃣ Caso nada se aplique, retorna genérico
        format!("{}UNKNOWN_ITEM", prefix)
    }
}

/*impl From<String> for Parameters {
    fn from(value: String) -> Parameters {
        Parameters{const_ident_str : value}
    }
}*/

impl<'a> From<&'a str> for Parameters {
    fn from(value: &'a str) -> Parameters {
        Parameters {
            const_ident_str: value.to_string(),
        }
    }
}
