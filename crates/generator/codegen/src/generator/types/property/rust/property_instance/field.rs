use crate::CodeGenerationConfig;
use crate::property::rust::const_properties::const_enum::const_property_comment;
use crate::property::rust::const_properties::ident::property_name_ident;
use crate::rust::Rust;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::DataType;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::TypeResolver;

pub fn property_instance_fields(properties: &Vec<PropertyType>, config: &CodeGenerationConfig, resolver: &TypeResolver) -> Vec<TokenStream> {
    let mut property_instance_fields = Vec::new();
    for property in properties.into_iter() {
        property_instance_fields.push(generate_property_field(&property, config, resolver));
    }
    property_instance_fields
}

pub fn generate_property_field(property: &PropertyType, config: &CodeGenerationConfig, resolver: &TypeResolver) -> TokenStream {
    let ident = property_name_ident(property);
    let doc_comment = Rust::multiline_doc_comment(const_property_comment(&property, resolver));
    let builder_setter_into = if config.generate_builders {
        quote! { #[builder(setter(into))] }
    } else {
        quote! {}
    };
    match property.data_type {
        DataType::Bool => {
            quote! {
                #[doc(newline)]
                #doc_comment
                pub #ident: bool,
            }
        }
        DataType::Number => {
            quote! {
                #[doc(newline)]
                #doc_comment
                pub #ident: u64,
            }
        }
        DataType::String => {
            quote! {
                #[doc(newline)]
                #doc_comment
                #builder_setter_into
                pub #ident: String,
            }
        }
        DataType::Array => {
            quote! {
                #[doc(newline)]
                #doc_comment
                #builder_setter_into
                pub #ident: Vec<serde_json::Value>,
            }
        }
        DataType::Object => {
            quote! {
                #[doc(newline)]
                #doc_comment
                #builder_setter_into
                pub #ident: serde_json::Map<String, serde_json::Value>,
            }
        }
        _ => {
            quote! {
                #[doc(newline)]
                #doc_comment
                pub #ident: serde_json::Value,
            }
        }
    }
}

pub fn generate_property_parameter(property: &PropertyType) -> TokenStream {
    let ident = property_name_ident(property);
    match property.data_type {
        DataType::Bool => {
            quote! {
                #ident: bool,
            }
        }
        DataType::Number => {
            quote! {
                #ident: u64,
            }
        }
        DataType::String => {
            quote! {
                #ident: String,
            }
        }
        DataType::Array => {
            quote! {
                #ident: Vec<serde_json::Value>,
            }
        }
        DataType::Object => {
            quote! {
                #ident: serde_json::Map<String, serde_json::Value>,
            }
        }
        _ => {
            quote! {
                #ident: serde_json::Value,
            }
        }
    }
}
