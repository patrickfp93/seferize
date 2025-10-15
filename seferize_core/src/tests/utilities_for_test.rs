use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Item;

pub fn extract_content_from_module(item : &Item) -> Option<TokenStream>{
    if let Item::Mod(item_mod) = item{
        if let Some(content) = item_mod.clone().content.map(|c| c.1){
            return Some(quote! (#(#content)*));
        }else{
            return None;
        }
    }
    None
}
