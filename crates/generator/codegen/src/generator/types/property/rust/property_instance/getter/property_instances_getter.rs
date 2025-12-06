use crate::property::rust::const_properties::ident::const_property_variant_ident;
use crate::property::rust::const_properties::ident::property_name_ident;
use crate::type_definition::rust::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::TypePropertiesIdent;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::TypeDefinitionGetter;

pub fn generate_property_instances_getter<TY: TypeDefinitionGetter + PropertyTypeContainer>(type_: &TY, properties: &Vec<PropertyType>) -> TokenStream {
    let type_properties_ident = TypePropertiesIdent::new(type_);
    let mut property_name_idents = Vec::new();
    for property in properties.into_iter() {
        let property_variant_ident = const_property_variant_ident(&property);
        let property_name_ident = property_name_ident(&property);
        property_name_idents.push(quote! {
            .property(#type_properties_ident::#property_variant_ident, self.#property_name_ident.clone())
        });
    }
    quote! {
        pub fn properties(&self) -> reactive_graph_graph::PropertyInstances {
            reactive_graph_graph::PropertyInstances::new()
                #(#property_name_idents)*
        }
    }
}
