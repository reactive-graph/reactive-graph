use crate::type_definition::rust::ConstComponentsIdent;
use crate::type_definition::rust::TypeValueIdent;
use crate::type_definition::rust::ident::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::ident::ConstTypeIdIdent;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::TypeDefinitionGetter;

pub fn generate_into_entity_instance<TY: TypeDefinitionGetter + PropertyTypeContainer>(type_: &TY, type_name_ident: &Ident) -> TokenStream {
    let type_value_ident = TypeValueIdent::new(type_);
    let const_type_id_ident = ConstTypeIdIdent::new(type_);
    let const_components_ident = ConstComponentsIdent::new(type_);
    quote! {
        #[doc(newline)]
        impl From<#type_name_ident> for reactive_graph_graph::EntityInstance {
            fn from(#type_value_ident: #type_name_ident) -> Self {
                reactive_graph_graph::EntityInstance::builder()
                    .ty(std::ops::Deref::deref(&#const_type_id_ident))
                    .id(#type_value_ident.id())
                    .components(#const_components_ident.clone())
                    .properties(#type_value_ident.properties())
                    .build()
            }
        }
    }
}
