use crate::property::rust::property_instance::getter::property_instance_getter_default_impl::generate_property_instance_getter_default_impl;
use crate::property::rust::property_instance::setter::property_instance_setter_default_impl::generate_property_instance_setter_default_impl;
use crate::property::rust::property_instance_getter_method_signatures::generate_property_instance_getter_method_signatures;
use crate::property::rust::property_instance_setter_method_signatures::generate_property_instance_setter_method_signatures;
use crate::rust::Rust;
use crate::type_definition::rust::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::TypeIdent;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_generator_documentation::GenerateDocumentation;
use reactive_graph_generator_documentation::types::config::DocumentationConfig;
use reactive_graph_generator_documentation::types::config::DocumentationConfigPreset;
use reactive_graph_generator_documentation::types::config::FromDocumentationConfigPreset;
use reactive_graph_graph::Mutability;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeDescriptionGetter;
use reactive_graph_graph::TypeResolver;

pub fn generate_impl_trait_property_instance_getter_and_setter_method_signatures<
    TY: TypeDefinitionGetter + PropertyTypeContainer + TypeDescriptionGetter + GenerateDocumentation<TY>,
>(
    type_: &TY,
    resolver: &TypeResolver,
) -> TokenStream {
    let type_name_ident = TypeIdent::new(type_);
    let mut token_stream = Vec::new();
    let mut properties = type_.get_own_properties_cloned().to_vec();
    properties.sort();
    for property in properties.into_iter() {
        token_stream.push(generate_property_instance_getter_method_signatures(&property, resolver));
        if property.mutability == Mutability::Mutable {
            token_stream.push(generate_property_instance_setter_method_signatures(&property, resolver));
        }
    }
    let doc_comment = Rust::multiline_doc_comment(trait_doc_comment(type_, resolver));
    quote! {
        #[doc(newline)]
        #doc_comment
        pub trait #type_name_ident {
            #(#token_stream)*
        }
    }
}

pub fn generate_impl_trait_property_instance_getter_and_setter_default_impl<
    TY: TypeDefinitionGetter + PropertyTypeContainer + TypeDescriptionGetter + GenerateDocumentation<TY>,
>(
    type_: &TY,
    resolver: &TypeResolver,
) -> TokenStream {
    let type_name_ident = TypeIdent::new(type_);
    let mut token_stream = Vec::new();
    let mut properties = type_.get_own_properties_cloned().to_vec();
    properties.sort();
    for property in properties.into_iter() {
        token_stream.push(generate_property_instance_getter_default_impl(&property, resolver));
        if property.mutability == Mutability::Mutable {
            token_stream.push(generate_property_instance_setter_default_impl(&property, resolver));
        }
    }
    let doc_comment = Rust::multiline_doc_comment(trait_doc_comment(type_, resolver));
    quote! {
        #[doc(newline)]
        #doc_comment
        pub trait #type_name_ident: reactive_graph_graph::PropertyInstanceGetter + reactive_graph_graph::PropertyInstanceSetter {
            #(#token_stream)*
        }
    }
}

#[inline]
pub fn trait_doc_comment<TY: TypeDescriptionGetter + GenerateDocumentation<TY>>(type_: &TY, resolver: &TypeResolver) -> String {
    type_
        .generate_documentation(&DocumentationConfig::new_from_preset(DocumentationConfigPreset::Short), resolver)
        .map(|documentation| format!(" {}", documentation.to_string()))
        .unwrap_or_else(|_| type_.description())
}
