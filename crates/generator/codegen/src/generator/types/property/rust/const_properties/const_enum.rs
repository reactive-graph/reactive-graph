use crate::CodeGenerationError;
use crate::property::rust::const_properties::ident::const_property_variant_ident;
use crate::property::rust::const_properties::property_type;
use crate::rust::Rust;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_generator_documentation::GenerateDocumentation;
use reactive_graph_generator_documentation::types::config::DocumentationConfig;
use reactive_graph_generator_documentation::types::config::DocumentationConfigPreset;
use reactive_graph_generator_documentation::types::config::FromDocumentationConfigPreset;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::TypeDefinition;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeResolver;

pub fn generate_const_properties_enum<TY: TypeDefinitionGetter + PropertyTypeContainer>(
    type_: &TY,
    resolver: &TypeResolver,
    const_properties_ident: &Ident,
    properties: &Vec<PropertyType>,
) -> Result<TokenStream, CodeGenerationError> {
    let type_definition = type_.type_definition();
    let const_properties_doc_comment = Rust::multiline_doc_comment(const_properties_comment(&type_definition));
    let mut token_stream = Vec::new();
    for property in properties.into_iter() {
        let variant_ident = const_property_variant_ident(&property);
        let doc_comment = Rust::multiline_doc_comment(const_property_comment(&property, resolver));
        token_stream.push(quote! {
            #[doc(newline)]
            #doc_comment
            #variant_ident,
        });
    }
    let property_types_len = property_type::generate_property_types_len(properties);
    let const_property_types = property_type::generate_const_property_types(&const_properties_ident, properties);
    Ok(quote! {
        #[doc(newline)]
        #const_properties_doc_comment
        #[allow(non_camel_case_types)]
        #[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
        pub enum #const_properties_ident {
            #(#token_stream)*
        }

        impl #const_properties_ident {
            #property_types_len
            #[doc(newline)]
            #const_property_types
        }
    })
}

#[inline]
pub fn const_properties_comment(type_definition: &TypeDefinition) -> String {
    format!(
        " The properties of {} `{}`.",
        type_definition.type_id_type.full_name(),
        type_definition.type_name().to_string()
    )
}

#[inline]
pub fn const_property_comment(property: &PropertyType, resolver: &TypeResolver) -> String {
    property
        .generate_documentation(&DocumentationConfig::new_from_preset(DocumentationConfigPreset::Short), resolver)
        .map(|documentation| format!(" {}", documentation.to_string()))
        .unwrap_or_else(|_| format!(" {}", property.description))
}
