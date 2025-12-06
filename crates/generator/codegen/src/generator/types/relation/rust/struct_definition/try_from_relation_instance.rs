use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::TypeDefinitionGetter;

// TODO: Implement
pub fn generate_try_from_relation_instance<TY: TypeDefinitionGetter + PropertyTypeContainer>(type_: &TY, type_name_ident: &Ident) -> TokenStream {
    quote! {
        #[doc(newline)]
        impl TryFrom<reactive_graph_graph::RelationInstance> for #type_name_ident {
            type Error = ();
            fn try_from(relation_instance: reactive_graph_graph::RelationInstance) -> Result<Self, Self::Error> {
                Err(())
            }
        }
    }
}
