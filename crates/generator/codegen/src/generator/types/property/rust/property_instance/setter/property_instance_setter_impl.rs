use crate::property::rust::const_properties::const_enum::const_property_comment;
use crate::property::rust::const_properties::ident::property_name_ident;
use crate::property::rust::property_instance::visibility::Visibility;
use crate::rust::Rust;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::DataType;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::TypeResolver;

pub fn generate_property_instance_setter_impl(property: &PropertyType, resolver: &TypeResolver, visibility: Visibility) -> TokenStream {
    let property_name = property.name.clone();
    let ident = property_name_ident(property);
    let setter_ident = Ident::new(&format!("set_{property_name}"), Span::call_site());
    let doc_comment = Rust::multiline_doc_comment(const_property_comment(&property, resolver));
    match property.data_type {
        DataType::Bool => {
            quote! {
                #[doc(newline)]
                #doc_comment
                #visibility fn #setter_ident(&mut self, #ident: bool) {
                    self.#ident = #ident;
                }
            }
        }
        DataType::Number => {
            quote! {
                #[doc(newline)]
                #doc_comment
                #visibility fn #setter_ident(&mut self, #ident: u64) {
                    self.#ident = #ident;
                }
            }
        }
        DataType::String => {
            quote! {
                #[doc(newline)]
                #doc_comment
                #visibility fn #setter_ident(&mut self, #ident: String) {
                    self.#ident = #ident;
                }
            }
        }
        DataType::Array => {
            quote! {
                #[doc(newline)]
                #doc_comment
                #visibility fn #setter_ident(&mut self, #ident: Vec<serde_json::Value>) {
                    self.#ident = #ident;
                }
            }
        }
        DataType::Object => {
            quote! {
                #[doc(newline)]
                #doc_comment
                #visibility fn #setter_ident(&mut self, #ident: serde_json::Map<String, serde_json::Value>) {
                    self.#ident = #ident;
                }
            }
        }
        _ => {
            quote! {
                #[doc(newline)]
                #doc_comment
                #visibility fn #setter_ident(&mut self, #ident: serde_json::Value) {
                    self.#ident = #ident;
                }
            }
        }
    }
}
