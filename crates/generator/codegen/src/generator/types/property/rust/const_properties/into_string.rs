use super::ident::const_property_variant_ident;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::PropertyType;

pub fn generate_convert_const_properties_into_string(const_properties_ident: &Ident, properties: &Vec<PropertyType>) -> TokenStream {
    let mut token_stream = Vec::new();
    for property in properties.into_iter() {
        let property_name = property.name.clone();
        let variant_ident = const_property_variant_ident(&property);
        token_stream.push(quote! {
            #const_properties_ident::#variant_ident => #property_name.to_owned(),
        })
    }
    quote! {
        #[doc(newline)]
        impl From<#const_properties_ident> for String {
            #[inline]
            fn from(properties: #const_properties_ident) -> String {
                match properties {
                    #(#token_stream)*
                }
            }
        }
    }
}
