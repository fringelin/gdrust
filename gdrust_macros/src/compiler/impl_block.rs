use crate::compiler::bundle::Bundle;
use crate::compiler::hints::property_hint;
use crate::compiler::properties::{ExportType, Property};
use crate::compiler::signal_args::create_signal_arg;
use crate::compiler::signals::SignalDecl;
use crate::compiler::values::Value;
use crate::kw::extends;
use crate::Extends;
use heck::ShoutySnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use syn::{Expr, ItemStruct};

pub(crate) fn impl_block(
    properties: &[Property],
    signals: &[SignalDecl],
    extends: &Extends,
    item: &ItemStruct,
) -> TokenStream {
    let struct_name = &item.ident;
    let extends = &extends.ty;

    let property_inits = property_inits(properties);
    let register_properties = register_properties(properties, signals);
    let signal_consts = signal_consts(signals);

    quote::quote! {
        impl #struct_name {

            #(#signal_consts)*

            #[allow(clippy::default_trait_access)]
            fn new(_owner: gdnative::prelude::TRef<#extends>) -> Self {
                Self {
                    #(#property_inits,)*
                }
            }

            #register_properties
        }
    }
}

fn property_inits(properties: &[Property]) -> Vec<TokenStream> {
    properties
        .iter()
        .map(|x| {
            let ident = &x.name;
            let default = get_default(x.default.as_ref());
            quote::quote! { #ident: #default}
        })
        .collect()
}

fn register_properties(properties: &[Property], signals: &[SignalDecl]) -> TokenStream {
    let properties: Vec<TokenStream> = properties.iter().map(|x| builder_for_property(x)).collect();
    let signals: Vec<TokenStream> = signals.iter().map(|x| builder_for_signal(x)).collect();
    quote::quote! {
        #[allow(clippy::default_trait_access)]
        fn __register_properties_and_signals(builder: &gdnative::prelude::ClassBuilder<Self>) {
            #(#properties)*

            #(#signals)*
        }
    }
}

fn builder_for_property(property: &Property) -> TokenStream {
    if let ExportType::NoExport = property.export_type {
        return quote::quote! {};
    }
    let ty = &property.ty;
    let ident = &property.name;
    let default = &property.default;
    let ident_str = ident.to_string();
    let hint = property_hint(&property.export_type, &property.ty);
    let setter = quote::quote! { .with_setter(|this, _owner, val| {
        this.#ident = val
    })};
    let getter = quote::quote! { .with_ref_getter(|this, _owner| {
        &this.#ident
    })};
    let default = get_default(default.as_ref());
    quote::quote! {
        builder.property(#ident_str)
            #hint
            #getter
            #setter
            .with_default(#default)
            .done();
    }
}

fn builder_for_signal(signal: &SignalDecl) -> TokenStream {
    let name_str = signal.name.to_string();
    let args: Vec<TokenStream> = signal.args.iter().map(|x| create_signal_arg(x)).collect();
    quote::quote! {
        builder.signal(#name_str)#(#args)*.done();
    }
}

fn signal_consts(signals: &[SignalDecl]) -> Vec<TokenStream> {
    signals
        .iter()
        .map(|x| {
            let name_str = x.name.to_string();
            let name = Ident::new(
                x.name.to_string().to_shouty_snake_case().as_str(),
                Span::call_site(),
            );
            quote::quote! {
                pub const #name: &'static str = #name_str;
            }
        })
        .collect()
}

fn get_default(default: Option<&Expr>) -> TokenStream {
    if let Some(default) = default {
        quote::quote! { #default }
    } else {
        quote::quote! {
            Default::default()
        }
    }
}

pub(crate) fn value_blocks(values: &[Value], extends: &Extends) -> Vec<TokenStream> {
    let extends = &extends.ty;

    values
        .iter()
        .map(|x| {
            let name = x.name.clone();
            let value = x.value.clone();
            let ty = x.ty.clone();

            if x.is_node {
                quote::quote! {
                    #name: node.cast::<#extends>().unwrap().claim(),
                }
            } else if let Some(component) = x.component.clone() {
                quote::quote! {
                    #name: #ty::new(node.expect_node::<gdnative::prelude::Node,&str>(#component).claim()),
                }
            } else if let Some(value) = value {
                quote::quote! {
                    #name: #value,
                }
            } else if let Some(property) = x.property.clone() {
                quote::quote! {
                    #name: node.get(#property).try_to::<#ty>().unwrap(),
                }
            } else {
                quote::quote! {
                    #name: Default::default(),
                }
            }
        })
        .collect()
}

pub(crate) fn component_blocks(values: &[Bundle]) -> Vec<TokenStream> {
    values
        .iter()
        .map(|x| {
            let name = x.name.clone();
            let value = x.value.clone();
            let ty = x.ty.clone();

            if let Some(component) = x.component.clone() {
                quote::quote! {
                    #name: #ty::new(node.expect_node::<gdnative::prelude::Node,&str>(#component).claim()),
                }
            } else if let Some(value) = value {
                quote::quote! {
                    #name: #value,
                }
            }  else {
                quote::quote! {
                    #name: Default::default(),
                }
            }
        })
        .collect()
}
