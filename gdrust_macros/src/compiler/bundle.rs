use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream, Result};
use syn::parse_macro_input::parse;
use syn::{parenthesized, Expr, Field, ItemStruct, Type};

mod kw {
    syn::custom_keyword!(value);
    syn::custom_keyword!(node);
    syn::custom_keyword!(component);
}

pub struct Bundle {
    pub name: Ident,
    pub ty: Type,
    pub value: Option<Expr>,
    pub component: Option<Expr>,
}

impl Bundle {
    fn new(name: Ident, ty: Type) -> Self {
        Self {
            name,
            ty,
            value: None,
            component: None,
        }
    }
}

struct BundleProperty {
    pub expr: Expr,
}

impl Parse for BundleProperty {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _paren_token = parenthesized!(content in input);
        let expr = content.parse()?;
        Ok(Self { expr })
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn extract_bundles(item: &mut ItemStruct) -> Vec<Bundle> {
    item.fields.iter_mut().map(|x| get_value(x)).collect()
}

pub fn get_value(item: &mut Field) -> Bundle {
    let mut property = Bundle::new(
        item.ident
            .clone()
            .expect("Properties must be on named field"),
        item.ty.clone(),
    );
    item.attrs = item
        .attrs
        .iter()
        .filter(|x| {
            let ident = x
                .path
                .get_ident()
                .expect("Expected valid attr on property")
                .to_string();
            let mut should_filter = false;
            let tokens = x.tokens.clone().into();
            match ident.as_str() {
                "value" => {
                    let value = parse::<BundleProperty>(tokens)
                        .expect("Invalid params for value")
                        .expr;
                    property.value = Some(value);
                }
                "component" => {
                    let component = parse::<BundleProperty>(tokens)
                        .expect("Invalid params for component")
                        .expr;
                    property.component = Some(component);
                }
                _ => should_filter = true,
            }
            should_filter
        })
        .cloned()
        .collect();
    property
}
