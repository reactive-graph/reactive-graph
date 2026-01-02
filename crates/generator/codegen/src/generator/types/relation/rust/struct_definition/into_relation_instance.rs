use crate::type_definition::rust::ConstComponentsIdent;
use crate::type_definition::rust::TypeValueIdent;
use crate::type_definition::rust::ident::AssocTypeDefinitionIdentType;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::TypeDefinitionGetter;

pub fn generate_into_relation_instance<TY: TypeDefinitionGetter + PropertyTypeContainer>(type_: &TY, type_name_ident: &Ident) -> TokenStream {
    let type_value_ident = TypeValueIdent::new(type_);
    let const_components_ident = ConstComponentsIdent::new(type_);
    quote! {
        #[doc(newline)]
        impl From<#type_name_ident> for reactive_graph_graph::RelationInstance {
            fn from(#type_value_ident: #type_name_ident) -> Self {
                reactive_graph_graph::RelationInstance::builder()
                    .outbound_id(#type_value_ident.outbound_id())
                    .ty(#type_value_ident.id())
                    .inbound_id(#type_value_ident.inbound_id())
                    .components(#const_components_ident.clone())
                    .properties(#type_value_ident.properties())
                    .build()
            }
        }
    }
}
