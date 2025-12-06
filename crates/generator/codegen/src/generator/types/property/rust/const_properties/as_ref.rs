use super::const_property_variant_ident;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::PropertyType;

pub fn generate_as_ref_const_properties(const_properties_ident: &Ident, properties: &Vec<PropertyType>) -> TokenStream {
    let mut token_stream = Vec::new();
    for property in properties.into_iter() {
        let property_name = property.name.clone();
        let variant_ident = const_property_variant_ident(&property);
        token_stream.push(quote! {
            #const_properties_ident::#variant_ident => #property_name,
        });
    }

    quote! {
        #[doc(newline)]
        impl AsRef<str> for #const_properties_ident {
            #[inline]
            fn as_ref(&self) -> &str {
                match *self {
                    #(#token_stream)*
                }
            }
        }
    }
}
