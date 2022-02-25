use proc_macro::TokenStream;
mod compiler;

use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, ItemStruct, Result, Token, Type};

mod kw {
    syn::custom_keyword!(extends);
}

pub(crate) struct Extends {
    ty: Type,
}

impl Parse for Extends {
    fn parse(input: ParseStream) -> Result<Self> {
        let _extends = input.parse::<kw::extends>()?;
        let _eq = input.parse::<Token![=]>()?;
        let ty = input.parse()?;
        Ok(Self { ty })
    }
}

#[proc_macro_attribute]
pub fn gdrust(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut parsed = syn::parse_macro_input!(item as ItemStruct);
    let extends = syn::parse_macro_input::parse::<Extends>(attr).unwrap_or(Extends {
        ty: parse_quote! { gdnative::api::Object },
    });
    let compiled = compiler::compile(&mut parsed, &extends);
    // println!("{}", compiled.to_string());
    compiled.into()
}

#[proc_macro_attribute]
pub fn single_value(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as ItemStruct);
    let struct_name = &item.ident;
    let extends = syn::parse_macro_input::parse::<Extends>(attr).unwrap_or(Extends {
        ty: parse_quote! { f32 },
    });
    let extends_type = &extends.ty;

    let compiled = quote::quote! {
        use std::ops::Deref;
        #item

        impl #struct_name {
            pub fn new(value: #extends_type) -> Self {
                Self { value }
            }
        }

        impl Deref for #struct_name {
            type Target = #extends_type;

            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

    };
    compiled.into()
}
