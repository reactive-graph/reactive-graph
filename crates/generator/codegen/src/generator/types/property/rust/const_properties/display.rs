use super::ident::const_property_variant_ident;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::PropertyType;

pub fn generate_const_properties_display(const_properties_ident: &Ident, properties: &Vec<PropertyType>) -> TokenStream {
    let mut token_stream = Vec::new();
    for property in properties.into_iter() {
        let property_name = property.name.clone();
        let variant_ident = const_property_variant_ident(&property);
        token_stream.push(quote! {
            #const_properties_ident::#variant_ident => core::fmt::Display::fmt(#property_name, f),
        })
    }
    quote! {
        #[doc(newline)]
        impl core::fmt::Display for #const_properties_ident {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
                match *self {
                    #(#token_stream)*
            }
          }
        }
    }
}
