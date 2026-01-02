use super::ident::const_property_variant_ident;
use crate::property::rust::data_type_token_stream;
use crate::property::rust::mutability_token_stream;
use crate::property::rust::socket_type_token_stream;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::PropertyType;

pub fn generate_convert_const_properties_into_property_type(const_properties_ident: &Ident, properties: &Vec<PropertyType>) -> TokenStream {
    let mut token_stream = Vec::new();
    for property in properties.into_iter() {
        let property_name = property.name.clone();
        let property_description = property.description.clone();
        let property_data_type = data_type_token_stream(&property);
        let property_socket_type = socket_type_token_stream(&property);
        let property_mutability = mutability_token_stream(&property);

        let variant_ident = const_property_variant_ident(&property);
        token_stream.push(quote! {
            #const_properties_ident::#variant_ident => {
                reactive_graph_graph::PropertyType::new_with_all(
                    #property_name,
                    #property_description,
                    #property_data_type,
                    #property_socket_type,
                    #property_mutability,
                    reactive_graph_graph::Extensions::new(),
                )
            },
        })
    }
    quote! {
        #[doc(newline)]
        impl From<#const_properties_ident> for reactive_graph_graph::PropertyType {
            #[inline]
            fn from(properties: #const_properties_ident) -> reactive_graph_graph::PropertyType {
                match properties {
                    #(#token_stream)*
                }
            }
        }
    }
}

pub fn generate_const_property_types(const_properties_ident: &Ident, properties: &Vec<PropertyType>) -> TokenStream {
    let mut token_stream = Vec::new();
    for property in properties.into_iter() {
        let variant_ident = const_property_variant_ident(&property);
        token_stream.push(quote! {
            property_types.push(#const_properties_ident::#variant_ident);
        });
    }
    quote! {
        pub fn property_types() -> reactive_graph_graph::PropertyTypes {
            let property_types = reactive_graph_graph::PropertyTypes::new();
            #(#token_stream)*
            property_types
        }
    }
}

pub fn generate_property_types_len(properties: &Vec<PropertyType>) -> TokenStream {
    let len = properties.len();
    quote! {
        pub fn len() -> usize {
            #len
        }
    }
}
