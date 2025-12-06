use crate::type_definition::rust::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::ConstComponentsIdent;
use crate::type_definition::rust::ConstTypeIdIdent;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::TypeDefinitionGetter;

// TODO: Implement converter from rust type to EntityInstance.
pub fn generate_try_from_entity_instance<TY: TypeDefinitionGetter + PropertyTypeContainer>(type_: &TY, type_name_ident: &Ident) -> TokenStream {
    // let type_definition = type_.type_definition();
    // let type_name_value_ident = type_name_value_ident(&type_definition);
    // let type_id_ident = const_type_id_ident(&type_definition);
    let const_type_id_ident = ConstTypeIdIdent::new(type_);
    let const_components_ident = ConstComponentsIdent::new(type_);
    quote! {
        #[doc(newline)]
        impl TryFrom<reactive_graph_graph::EntityInstance> for #type_name_ident {
            type Error = ();
            fn try_from(entity_instance: reactive_graph_graph::EntityInstance) -> Result<Self, Self::Error> {
                Err(())
            }
        }
        // impl From<#type_name_ident> for reactive_graph_graph::EntityInstance {
        //     fn from(#type_name_value_ident: #type_name_ident) -> Self {
        //         reactive_graph_graph::EntityInstance::builder()
        //             .ty(std::ops::Deref::deref(&#type_id_ident))
        //             .components(#const_components_ident.clone())
        //             .properties(#type_name_value_ident.properties())
        //             .build()
        //     }
        // }
    }
}
//
// pub fn type_name_value_ident(type_definition: &TypeDefinition) -> Ident {
//     Ident::new(&type_definition.type_name().to_case(Case::Snake), Span::call_site())
// }
