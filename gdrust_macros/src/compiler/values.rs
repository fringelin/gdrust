use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream, Result};
use syn::parse_macro_input::parse;
use syn::punctuated::Punctuated;
use syn::{parenthesized, token, Expr, Field, ItemStruct, Lit, LitStr, Token, Type};

mod kw {
    syn::custom_keyword!(value);
    syn::custom_keyword!(node);
    syn::custom_keyword!(component);
}

pub struct Value {
    pub name: Ident,
    pub ty: Type,
    pub value: Option<Expr>,
    pub is_node: bool,
    pub component: Option<Expr>,
    pub script: Option<Expr>,
}

impl Value {
    fn new(name: Ident, ty: Type) -> Self {
        Self {
            name,
            ty,
            value: None,
            is_node: false,
            component: None,
            script: None,
        }
    }
}

struct ValueProperty {
    pub expr: Expr,
}

impl Parse for ValueProperty {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _paren_token = parenthesized!(content in input);
        let expr = content.parse()?;
        Ok(Self { expr })
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn extract_values(item: &mut ItemStruct) -> Vec<Value> {
    item.fields.iter_mut().map(|x| get_value(x)).collect()
}

pub fn get_value(item: &mut Field) -> Value {
    let mut property = Value::new(
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
                    let value = parse::<ValueProperty>(tokens)
                        .expect("Invalid params for value")
                        .expr;
                    property.value = Some(value);
                }
                "node" => property.is_node = true,
                "component" => {
                    let component = parse::<ValueProperty>(tokens)
                        .expect("Invalid params for component")
                        .expr;
                    property.component = Some(component);
                }
                "script" => {
                    let script = parse::<ValueProperty>(tokens)
                        .expect("Invalid params for script")
                        .expr;
                    property.script = Some(script);
                }
                _ => should_filter = true,
            }
            should_filter
        })
        .cloned()
        .collect();
    property
}
