use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream, Result};
use syn::parse_macro_input::parse;
use syn::punctuated::Punctuated;
use syn::{parenthesized, token, Expr, Field, ItemStruct, Lit, LitStr, Token, Type};

mod kw {
    syn::custom_keyword!(export);
    syn::custom_keyword!(or_greater);
    syn::custom_keyword!(or_lesser);
    syn::custom_keyword!(var);
    syn::custom_keyword!(no_export);
    syn::custom_keyword!(export_range);
    syn::custom_keyword!(export_enum);
    syn::custom_keyword!(export_file);
    syn::custom_keyword!(export_dir);
    syn::custom_keyword!(export_global_file);
    syn::custom_keyword!(export_global_dir);
    syn::custom_keyword!(export_multiline);
    syn::custom_keyword!(export_exp_range);
    syn::custom_keyword!(export_color_no_alpha);
    syn::custom_keyword!(export_node_path);
    syn::custom_keyword!(export_flags);
    syn::custom_keyword!(export_flags_2d_physics);
    syn::custom_keyword!(export_flags_2d_render);
    syn::custom_keyword!(export_flags_3d_physics);
    syn::custom_keyword!(export_flags_3d_render);
}

#[derive(Clone)]
pub enum ExportType {
    NoHint,
    NoExport,
    Export,
    ExportRange(ExportRange),
    ExportExpRange(ExportExpRange),
    ExportEnum(ExportEnum),
    ExportFile(ExportFile),
    ExportDir,
    ExportGlobalFile(ExportGlobalFile),
    ExportGlobalDir,
    ExportMultiline,
    ExportColorNoAlpha,
    ExportNodePath(ExportNodePath),
    ExportFlags(ExportFlags),
    ExportFlags2dPhysics,
    ExportFlags2dRender,
    ExportFlags3dPhysics,
    ExportFlags3dRender,
}

#[derive(Clone)]
pub struct ExportRange {
    pub paren_token: token::Paren,
    pub range: Punctuated<Lit, Token![,]>,
}

impl Parse for ExportRange {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);
        let range = content.parse_terminated(Lit::parse)?;
        Ok(Self { paren_token, range })
    }
}

#[derive(Clone)]
pub struct ExportExpRange {
    pub paren_token: token::Paren,
    pub range: Punctuated<Lit, Token![,]>,
}

impl Parse for ExportExpRange {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);
        let range = content.parse_terminated(Lit::parse)?;
        Ok(Self { paren_token, range })
    }
}

#[derive(Clone)]
pub struct ExportEnum {
    pub paren_token: token::Paren,
    pub values: Punctuated<LitStr, Token![,]>,
}

impl Parse for ExportEnum {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);
        let values = content.parse_terminated(<LitStr as Parse>::parse)?;
        Ok(Self {
            paren_token,
            values,
        })
    }
}

#[derive(Clone)]
pub struct ExportFile {
    pub filter: Option<(token::Paren, LitStr)>,
}

impl Parse for ExportFile {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            filter: if input.peek(token::Paren) {
                let content;
                let paren = parenthesized!(content in input);
                let filter = content.parse()?;
                Some((paren, filter))
            } else {
                None
            },
        })
    }
}

#[derive(Clone)]
pub struct ExportGlobalFile {
    pub filter: Option<(token::Paren, LitStr)>,
}

impl Parse for ExportGlobalFile {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            filter: if input.peek(token::Paren) {
                let content;
                let paren = parenthesized!(content in input);
                let filter = content.parse()?;
                Some((paren, filter))
            } else {
                None
            },
        })
    }
}

#[derive(Clone)]
pub struct ExportMultiline {
    pub export_multiline: kw::export_multiline,
}

#[derive(Clone)]
pub struct ExportColorNoAlpha {
    pub export_color_no_alpha: kw::export_color_no_alpha,
}

#[derive(Clone)]
pub struct ExportNodePath {
    pub types: Option<(token::Paren, Punctuated<Type, Token![,]>)>,
}

impl Parse for ExportNodePath {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            types: if input.peek(token::Paren) {
                let content;
                let paren = parenthesized!(content in input);
                let types = content.parse_terminated(Type::parse)?;
                Some((paren, types))
            } else {
                None
            },
        })
    }
}

#[derive(Clone)]
pub struct ExportFlags {
    pub paren_token: token::Paren,
    pub values: Punctuated<LitStr, Token![,]>,
}

impl Parse for ExportFlags {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);
        let values = content.parse_terminated(<LitStr as Parse>::parse)?;
        Ok(Self {
            paren_token,
            values,
        })
    }
}

pub struct Property {
    pub name: Ident,
    pub ty: Type,
    pub export_type: ExportType,
    pub default: Option<Expr>,
}

impl Property {
    fn new(name: Ident, ty: Type) -> Self {
        Self {
            name,
            ty,
            export_type: ExportType::NoExport,
            default: None,
        }
    }
}

struct DefaultProperty {
    pub expr: Expr,
}

impl Parse for DefaultProperty {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _paren_token = parenthesized!(content in input);
        let expr = content.parse()?;
        Ok(Self { expr })
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn extract_properties(item: &mut ItemStruct) -> Vec<Property> {
    item.fields.iter_mut().map(|x| get_property(x)).collect()
}

pub fn get_property(item: &mut Field) -> Property {
    let mut property = Property::new(
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
                "default" => {
                    let default = parse::<DefaultProperty>(tokens)
                        .expect("Invalid params for default")
                        .expr;
                    property.default = Some(default);
                }
                "export" => property.export_type = ExportType::Export,
                "no_export" => property.export_type = ExportType::NoExport,
                "export_range" => {
                    let range = parse(tokens).expect("Invalid range on export_range");
                    property.export_type = ExportType::ExportRange(range);
                }
                "export_enum" => {
                    let export_enum = parse::<ExportEnum>(tokens).expect("Invalid exportenum");
                    property.export_type = ExportType::ExportEnum(export_enum);
                }
                "export_file" => {
                    let export_file = parse(tokens).expect("Invalid export_file");
                    property.export_type = ExportType::ExportFile(export_file)
                }
                "export_dir" => property.export_type = ExportType::ExportDir,
                "export_global_file" => {
                    let export_global_file = parse(tokens).expect("Invalid export_file");
                    property.export_type = ExportType::ExportGlobalFile(export_global_file)
                }
                "export_global_dir" => property.export_type = ExportType::ExportGlobalDir,
                "export_multiline" => property.export_type = ExportType::ExportMultiline,
                "export_exp_range" => {
                    let range = parse(tokens).expect("Invalid range on export_exp_range");
                    property.export_type = ExportType::ExportExpRange(range);
                }
                "export_color_no_alpha" => property.export_type = ExportType::ExportColorNoAlpha,
                "export_flags" => {
                    let flags = parse(tokens).expect("Invalid export_flags");
                    property.export_type = ExportType::ExportFlags(flags)
                }
                "export_node_path" => {
                    let types = parse(tokens).expect("Invalid args for export_node_path");
                    property.export_type = ExportType::ExportNodePath(types)
                }
                "export_flags_2d_physics" => {
                    property.export_type = ExportType::ExportFlags2dPhysics
                }
                "export_flags_2d_render" => property.export_type = ExportType::ExportFlags2dRender,
                "export_flags_3d_physics" => {
                    property.export_type = ExportType::ExportFlags3dPhysics
                }
                "export_flags_3d_render" => property.export_type = ExportType::ExportFlags3dRender,
                _ => should_filter = true,
            }
            should_filter
        })
        .cloned()
        .collect();
    property
}
