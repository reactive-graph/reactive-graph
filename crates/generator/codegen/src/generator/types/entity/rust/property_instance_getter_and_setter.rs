// use crate::property::rust::property_instance::field::generate_property_field;
// use proc_macro2::Ident;
// use proc_macro2::Span;
// use proc_macro2::TokenStream;
// use quote::quote;
// use reactive_graph_graph::NamespacedTypeGetter;
// use reactive_graph_graph::PropertyTypeContainer;
// use reactive_graph_graph::TypeDefinitionGetter;
//
// pub fn generate_impl_trait_property_instance_getter<TY: TypeDefinitionGetter + PropertyTypeContainer>(type_: &TY) -> TokenStream {
//     let type_definition = type_.type_definition();
//     let type_name_ident = Ident::new(&type_definition.type_name().to_string(), Span::call_site());
//     let mut token_stream = Vec::new();
//     let mut properties = type_.get_own_properties_cloned().to_vec();
//     properties.sort();
//     for property in properties.into_iter() {
//         token_stream.push(generate_property_field(&property));
//     }
//     quote! {
//         #[doc(newline)]
//         impl #type_name_ident {
//             #(#token_stream)*
//         }
//     }
// }
