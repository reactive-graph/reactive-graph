use crate::property::rust::const_properties::ident::property_name_ident;
use crate::property::rust::property_instance::field::generate_property_parameter;
use crate::type_definition::rust::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::ConstTypeIdIdent;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::TypeDefinitionGetter;

pub fn generate_constructor<TY: TypeDefinitionGetter + PropertyTypeContainer>(type_: &TY, properties: &Vec<PropertyType>) -> TokenStream {
    let const_type_id_ident = ConstTypeIdIdent::new(type_);
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
            outbound_id: uuid::Uuid,
            id: reactive_graph_graph::InstanceId,
            inbound_id: uuid::Uuid,
            #(#parameters)*
        ) -> Self {
            let id = reactive_graph_graph::RelationInstanceTypeId::new(std::ops::Deref::deref(&#const_type_id_ident), id);
            Self {
                outbound_id,
                id,
                inbound_id,
                #(#property_name_idents)*
                extensions: reactive_graph_graph::Extensions::new()
            }
        }

        #[doc(newline)]
        pub fn new_with_extensions(
            outbound_id: uuid::Uuid,
            id: reactive_graph_graph::InstanceId,
            inbound_id: uuid::Uuid,
            #(#parameters)*
            extensions: reactive_graph_graph::Extensions,
        ) -> Self {
            let id = reactive_graph_graph::RelationInstanceTypeId::new(std::ops::Deref::deref(&#const_type_id_ident), id);
            Self {
                outbound_id,
                id,
                inbound_id,
                #(#property_name_idents)*
                extensions
            }
        }
    }
}
