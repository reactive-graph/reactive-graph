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

pub fn generate_property_instance_setter_default_impl(property: &PropertyType, resolver: &TypeResolver) -> TokenStream {
    let property_name = property.name.clone();
    let ident = property_name_ident(property);
    let setter_ident = Ident::new(&format!("set_{property_name}"), Span::call_site());
    let doc_comment = Rust::multiline_doc_comment(const_property_comment(&property, resolver));
    match property.data_type {
        DataType::Bool => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #setter_ident(&self, #ident: bool) {
                    reactive_graph_graph::PropertyInstanceSetter::set(self, #property_name, serde_json::json!(#ident))
                }
            }
        }
        DataType::Number => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #setter_ident(&self, #ident: u64) {
                    reactive_graph_graph::PropertyInstanceSetter::set(self, #property_name, serde_json::json!(#ident))
                }
            }
        }
        DataType::String => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #setter_ident(&self, #ident: String) {
                    reactive_graph_graph::PropertyInstanceSetter::set(self, #property_name, serde_json::json!(#ident))
                }
            }
        }
        DataType::Array => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #setter_ident(&self, #ident: Vec<serde_json::Value>) {
                    reactive_graph_graph::PropertyInstanceSetter::set(self, #property_name, serde_json::json!(#ident))
                }
            }
        }
        DataType::Object => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #setter_ident(&self, #ident: serde_json::Map<String, serde_json::Value>) {
                    reactive_graph_graph::PropertyInstanceSetter::set(self, #property_name, serde_json::json!(#ident))
                }
            }
        }
        _ => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #setter_ident(&self, #ident: serde_json::Value) {
                    reactive_graph_graph::PropertyInstanceSetter::set(self, #property_name, #ident)
                }
            }
        }
    }
}
