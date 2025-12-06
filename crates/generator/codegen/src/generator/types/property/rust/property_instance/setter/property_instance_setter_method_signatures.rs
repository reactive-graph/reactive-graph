use crate::property::rust::const_properties::const_enum::const_property_comment;
use crate::property::rust::const_properties::ident::property_name_ident;
use crate::rust::Rust;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::DataType;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::TypeResolver;

pub fn generate_property_instance_setter_method_signatures(property: &PropertyType, resolver: &TypeResolver) -> TokenStream {
    let property_name = property.name.clone();
    let ident = property_name_ident(property);
    let setter_ident = Ident::new(&format!("set_{property_name}"), Span::call_site());
    let doc_comment = Rust::multiline_doc_comment(const_property_comment(&property, resolver));
    match property.data_type {
        DataType::Bool => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #setter_ident(&mut self, #ident: bool);
            }
        }
        DataType::Number => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #setter_ident(&mut self, #ident: u64);
            }
        }
        DataType::String => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #setter_ident(&mut self, #ident: String);
            }
        }
        DataType::Array => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #setter_ident(&mut self, #ident: Vec<serde_json::Value>);
            }
        }
        DataType::Object => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #setter_ident(&mut self, #ident: serde_json::Map<String, serde_json::Value>);
            }
        }
        _ => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #setter_ident(&mut self, #ident: serde_json::Value);
            }
        }
    }
}
