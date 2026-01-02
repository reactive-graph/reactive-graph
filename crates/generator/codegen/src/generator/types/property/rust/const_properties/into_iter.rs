use crate::property::rust::const_properties::ident::const_property_variant_ident;
use crate::type_definition::rust::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::TypePropertiesIteratorIdent;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::TypeDefinitionGetter;

pub(crate) fn generate_into_iter<TY: TypeDefinitionGetter + PropertyTypeContainer>(
    type_: &TY,
    const_properties_ident: &Ident,
    properties: &Vec<PropertyType>,
) -> TokenStream {
    let iterator_ident = TypePropertiesIteratorIdent::new(type_);
    let iterator_struct = quote! {
        #[doc(newline)]
        pub struct #iterator_ident(Option<#const_properties_ident>);

        #[doc(newline)]
        impl #const_properties_ident {
            pub fn into_iter() -> #iterator_ident {
                #iterator_ident(None)
            }
        }
    };
    match properties.first() {
        None => {
            quote! {
                #iterator_struct

                #[doc(newline)]
                impl Iterator for #iterator_ident {
                    type Item = #const_properties_ident;

                    fn next(&mut self) -> Option<Self::Item> {
                        None
                    }
                }

                #[doc(newline)]
                impl IntoIterator for #const_properties_ident {
                    type Item = #const_properties_ident;
                    type IntoIter = #iterator_ident;

                    fn into_iter(self) -> Self::IntoIter {
                        #iterator_ident(None)
                    }
                }
            }
        }
        Some(first_property) => {
            let first_variant_ident = const_property_variant_ident(&first_property);
            let mut next_variants_token_stream = Vec::new();
            for (index, property) in properties.iter().enumerate() {
                let variant_ident = const_property_variant_ident(&property);
                match properties.get(index + 1).map(|next_property| const_property_variant_ident(&next_property)) {
                    None => {
                        next_variants_token_stream.push(quote! {
                            Some(#const_properties_ident::#variant_ident) => None
                        });
                    }
                    Some(next_variant_ident) => {
                        next_variants_token_stream.push(quote! {
                            Some(#const_properties_ident::#variant_ident) => Some(#const_properties_ident::#next_variant_ident),
                        });
                    }
                }
            }
            quote! {
                #iterator_struct

                #[doc(newline)]
                impl Iterator for #iterator_ident {
                    type Item = #const_properties_ident;

                    fn next(&mut self) -> Option<Self::Item> {
                        self.0 = match self.0 {
                            None => Some(#const_properties_ident::#first_variant_ident),
                            #(#next_variants_token_stream)*
                        };
                        self.0.clone()
                    }
                }

                #[doc(newline)]
                impl IntoIterator for #const_properties_ident {
                    type Item = #const_properties_ident;
                    type IntoIter = #iterator_ident;

                    fn into_iter(self) -> Self::IntoIter {
                        #iterator_ident(None)
                    }
                }
            }
        }
    }
}
