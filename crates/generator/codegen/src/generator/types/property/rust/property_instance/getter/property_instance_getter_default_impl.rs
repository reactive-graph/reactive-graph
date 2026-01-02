use crate::property::rust::const_properties::const_enum::const_property_comment;
use crate::property::rust::const_properties::ident::property_name_ident;
use crate::rust::Rust;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::DataType;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::TypeResolver;

pub fn generate_property_instance_getter_default_impl(property: &PropertyType, resolver: &TypeResolver) -> TokenStream {
    let property_name = property.name.clone();
    let ident = property_name_ident(property);
    let doc_comment = Rust::multiline_doc_comment(const_property_comment(&property, resolver));

    match property.data_type {
        DataType::Bool => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #ident(&self) -> Option<bool> {
                    reactive_graph_graph::PropertyInstanceGetter::as_bool(self, #property_name)
                }
            }
        }
        DataType::Number => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #ident(&self) -> Option<u64> {
                    reactive_graph_graph::PropertyInstanceGetter::as_u64(self, #property_name)
                }
            }
        }
        DataType::String => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #ident(&self) -> Option<String> {
                    reactive_graph_graph::PropertyInstanceGetter::as_string(self, #property_name).map(String::from)
                }
            }
        }
        DataType::Array => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #ident(&self) -> Option<Vec<serde_json::Value>> {
                    reactive_graph_graph::PropertyInstanceGetter::as_array(self, #property_name)
                }
            }
        }
        DataType::Object => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #ident(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
                    reactive_graph_graph::PropertyInstanceGetter::as_object(self, #property_name)
                }
            }
        }
        _ => {
            quote! {
                #[doc(newline)]
                #doc_comment
                fn #ident(&self) -> Option<serde_json::Value> {
                    reactive_graph_graph::PropertyInstanceGetter::get(self, #property_name)
                }
            }
        }
    }
}
