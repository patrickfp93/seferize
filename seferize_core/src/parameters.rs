use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_str, Ident, Item, LitStr};

pub struct Parameters {
    const_ident_str: String,
}

impl Parameters {
    pub fn new(attr: TokenStream, item: &TokenStream) -> Result<Self, syn::Error> {
        if attr.is_empty(){
            return Ok(Self { const_ident_str: "".into() });
        }
        match parse_str::<Ident>(&attr.clone().to_string()) {
            Ok(ident) => Ok(Self {
                const_ident_str: ident.to_string(),
            }),
            Err(_) => match parse_str::<LitStr>(&attr.to_string()) {
                Ok(lit) => Ok(Self {
                    const_ident_str: lit.value(),
                }),
                Err(_) => {
                    return Err(syn::Error::new_spanned(
                        proc_macro2::TokenStream::from(item.clone()),
                        "Expected an identifier or a string literal, e.g. #[stringify(Name)] or #[stringify(\"Name\")]",
                    ));
                }
            },
        }
    }

    /* 
    pub fn new_from_atribute(macro_attr : &Attribute, item:&TokenStream) -> Result<Self,syn::Error>{
        let attr = match &macro_attr.meta {
            syn::Meta::List(meta_list) => meta_list.to_token_stream(),
            _ =>{
                TokenStream::new()
            }
            /*syn::Meta::Path(path) => TokenStream::new(),
            syn::Meta::NameValue(meta_name_value) => todo!(),*/
        };
        Self::new(attr, item)
    }*/

    pub fn const_ident(&self, item: &Item) -> Ident {
        let ident_str = if !self.const_ident_str.is_empty() {
            // Se tiver um literal string, usa ele
            self.const_ident_str.clone()
        } else {
            // Gera nome padrão com base no identificador do item
            let default_name = Self::generate_default_name(item, super::CONST_DEFAULT_PREFIX);
            default_name
        };
        parse_str(&ident_str).unwrap()
    }

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
            Item::Verbatim(v) => {
                format!("{}VERBATIM_{}", prefix, v.to_token_stream().to_string())
            }
            _ => format!("{}CODE_ITEM", prefix),
        }
    }
}

/*impl From<String> for Parameters {
    fn from(value: String) -> Parameters {
        Parameters{const_ident_str : value}
    }
}*/

impl<'a> From<&'a str> for Parameters {
    fn from(value: &'a str) -> Parameters {
        Parameters{const_ident_str : value.to_string()}
    }
}


