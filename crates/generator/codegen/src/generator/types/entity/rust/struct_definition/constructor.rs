use crate::property::rust::const_properties::ident::property_name_ident;
use crate::property::rust::property_instance::field::generate_property_parameter;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::PropertyType;

pub fn generate_constructor(properties: &Vec<PropertyType>) -> TokenStream {
    let mut parameters = Vec::new();
    let mut property_name_idents = Vec::new();
    for property in properties.into_iter() {
        let property_name_ident = property_name_ident(&property);
        let parameter = generate_property_parameter(&property);
        parameters.push(parameter);
        property_name_idents.push(quote! {
            #property_name_ident,
        });
    }

    quote! {
        pub fn new(
            #(#parameters)*
        ) -> Self {
            Self {
                id: uuid::Uuid::new_v4(),
                #(#property_name_idents)*
                extensions: reactive_graph_graph::Extensions::new()
            }
        }

        pub fn new_with_id(
            id: uuid::Uuid,
            #(#parameters)*
        ) -> Self {
            Self {
                id,
                #(#property_name_idents)*
                extensions: reactive_graph_graph::Extensions::new()
            }
        }

        pub fn new_with_extensions(
            id: uuid::Uuid,
            #(#parameters)*
            extensions: reactive_graph_graph::Extensions,
        ) -> Self {
            Self {
                id,
                #(#property_name_idents)*
                extensions
            }
        }
    }
}
