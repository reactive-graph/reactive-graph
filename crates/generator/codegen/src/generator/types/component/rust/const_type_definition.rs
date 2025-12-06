use crate::CodeGenerationError;
use crate::type_definition::rust::ConstTypeIdent;
use crate::type_definition::rust::TypePropertiesIdent;
use crate::type_definition::rust::TypeTypeIdent;
use crate::type_definition::rust::ident::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::ident::ConstTypeIdIdent;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::Component;

pub fn generate_const_type_definition(type_: &Component) -> Result<TokenStream, CodeGenerationError> {
    let const_type_id_ident = ConstTypeIdIdent::new(type_);
    let type_type_ident = TypeTypeIdent::new(type_);
    let const_type_ident = ConstTypeIdent::new(type_);
    let type_properties_ident = TypePropertiesIdent::new(type_);
    let description = type_.description.clone();
    Ok(quote! {
        #[doc(newline)]
        pub static #const_type_ident: std::sync::LazyLock<reactive_graph_graph::#type_type_ident> = std::sync::LazyLock::new(|| {
            reactive_graph_graph::#type_type_ident::builder()
                .ty(core::ops::Deref::deref(&#const_type_id_ident))
                .description(#description)
                .properties(#type_properties_ident::property_types())
                .build()
        });
    })
}
