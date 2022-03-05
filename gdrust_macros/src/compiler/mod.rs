mod bundle;
mod hints;
mod impl_block;
mod properties;
mod signal_args;
mod signals;
mod values;

use crate::compiler::bundle::extract_bundles;
use crate::compiler::properties::extract_properties;
use crate::compiler::signals::extract_signals;
use crate::compiler::values::extract_values;
use crate::Extends;
use proc_macro2::TokenStream;
use syn::{parse_quote, ItemStruct};

pub(crate) fn compile(item: &mut ItemStruct, extends: &Extends) -> TokenStream {
    let signals = extract_signals(item);
    let properties = extract_properties(item);
    let extends_type = &extends.ty;
    item.attrs.push(parse_quote! { #[derive(NativeClass)] });
    item.attrs.push(parse_quote! { #[inherit(#extends_type)]});
    item.attrs
        .push(parse_quote! { #[register_with(Self::__register_properties_and_signals)] });

    let impl_block = impl_block::impl_block(&properties, &signals, extends, item);
    quote::quote! {
        #item

        #impl_block
    }
}

pub(crate) fn compile_gd_bundle(item: &mut ItemStruct) -> TokenStream {
    let components = extract_bundles(item);
    let struct_name = &item.ident;
    item.attrs.push(parse_quote! { #[derive(Bundle)] });
    let component_blocks = impl_block::component_blocks(&components);

    quote::quote! {
        #item

        impl #struct_name {
            pub fn new(node: Ref<gdnative::prelude::Node>) -> Self {
                let node = node.expect_safe();

                Self {
                    #(#component_blocks)*
                }
            }
        }
    }
}

pub(crate) fn compile_gd_component(item: &mut ItemStruct, extends: &Extends) -> TokenStream {
    let (values, node) = extract_values(item);
    let node = node.unwrap();

    let struct_name = &item.ident;
    item.attrs.push(parse_quote! { #[derive(Component)] });
    let value_blocks = impl_block::value_blocks(&values, extends);
    let script_variables = impl_block::script_variables(&values, &node);

    let node_name = node.name;
    let extends = &extends.ty;

    quote::quote! {
        #item

        impl #struct_name {
            pub fn new(node: Ref<gdnative::prelude::Node>) -> Self {
                let node = node.expect_safe();

                Self {
                    #(#value_blocks)*
                }
            }

            #(#script_variables)*
        }
    }
}

pub(crate) fn compile_single_value(item: &mut ItemStruct, extends: &Extends) -> TokenStream {
    let struct_name = &item.ident;
    let extends_type = &extends.ty;

    quote::quote! {
        #item

        impl #struct_name {
            pub fn new(value: #extends_type) -> Self {
                Self { value }
            }
        }

        impl std::ops::Deref for #struct_name {
            type Target = #extends_type;

            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        impl std::ops::DerefMut for #struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.value
            }
        }

    }
}
