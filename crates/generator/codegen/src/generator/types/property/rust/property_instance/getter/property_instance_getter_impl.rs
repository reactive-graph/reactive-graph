use crate::property::rust::const_properties::const_enum::const_property_comment;
use crate::property::rust::const_properties::ident::property_name_ident;
use crate::property::rust::property_instance::visibility::Visibility;
use crate::rust::Rust;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::DataType;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::TypeResolver;

pub fn generate_property_instance_getter_impl(property: &PropertyType, resolver: &TypeResolver, visibility: Visibility) -> TokenStream {
    let ident = property_name_ident(property);
    let doc_comment = Rust::multiline_doc_comment(const_property_comment(&property, resolver));

    match property.data_type {
        DataType::Bool => {
            quote! {
                #[doc(newline)]
                #doc_comment
                #visibility fn #ident(&self) -> bool {
                    self.#ident
                }
            }
        }
        DataType::Number => {
            quote! {
                #[doc(newline)]
                #doc_comment
                #visibility fn #ident(&self) -> u64 {
                    self.#ident
                }
            }
        }
        DataType::String => {
            quote! {
                #[doc(newline)]
                #doc_comment
                #visibility fn #ident(&self) -> String {
                    self.#ident.clone()
                }
            }
        }
        DataType::Array => {
            quote! {
                #[doc(newline)]
                #doc_comment
                #visibility fn #ident(&self) -> Vec<serde_json::Value> {
                    self.#ident.clone()
                }
            }
        }
        DataType::Object => {
            quote! {
                #[doc(newline)]
                #doc_comment
                #visibility fn #ident(&self) -> serde_json::Map<String, serde_json::Value> {
                    self.#ident.clone()
                }
            }
        }
        _ => {
            quote! {
                #[doc(newline)]
                #doc_comment
                #visibility fn #ident(&self) -> serde_json::Value {
                    self.#ident.clone()
                }
            }
        }
    }
}
