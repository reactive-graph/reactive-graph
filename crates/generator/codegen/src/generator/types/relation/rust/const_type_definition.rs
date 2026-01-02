use crate::CodeGenerationError;
use crate::namespace::rust::fully_qualified_ident::FullyQualifiedNamespacedTypeIdent;
use crate::type_definition::rust::ConstComponentsIdent;
use crate::type_definition::rust::ConstExtensionsIdent;
use crate::type_definition::rust::ConstTypeIdent;
use crate::type_definition::rust::TypePropertiesIdent;
use crate::type_definition::rust::TypeTypeIdent;
use crate::type_definition::rust::ident::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::ident::ConstTypeIdIdent;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::InboundOutboundType;
use reactive_graph_graph::MatchingInboundOutboundType;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::TypeResolver;

pub fn generate_const_type_definition(type_: &RelationType, resolver: &TypeResolver) -> Result<TokenStream, CodeGenerationError> {
    let const_type_id_ident = ConstTypeIdIdent::new(type_);
    let type_type_ident = TypeTypeIdent::new(type_);
    let const_type_ident = ConstTypeIdent::new(type_);
    let const_components_ident = ConstComponentsIdent::new(type_);
    let type_properties_ident = TypePropertiesIdent::new(type_);
    let const_extensions_ident = ConstExtensionsIdent::new(type_);
    let description = type_.description.clone();
    let outbound_type = to_tokenstream(&type_.outbound_type, resolver)?;
    let inbound_type = to_tokenstream(&type_.inbound_type, resolver)?;
    Ok(quote! {
        #[doc(newline)]
        pub static #const_type_ident: std::sync::LazyLock<reactive_graph_graph::#type_type_ident> = std::sync::LazyLock::new(|| {
            reactive_graph_graph::#type_type_ident::builder()
                .outbound_type(#outbound_type)
                .ty(core::ops::Deref::deref(&#const_type_id_ident))
                .inbound_type(#inbound_type)
                .description(#description)
                .components(#const_components_ident.clone())
                .properties(#type_properties_ident::property_types())
                .extensions(#const_extensions_ident.clone())
                .build()
        });
    })
}

fn to_tokenstream(inbound_outbound_type: &InboundOutboundType, resolver: &TypeResolver) -> Result<TokenStream, CodeGenerationError> {
    match inbound_outbound_type {
        InboundOutboundType::Component(ty) => match ty {
            MatchingInboundOutboundType::NamespacedType(ty) => {
                let ty_ident = ty.fully_qualified_ident_of_type::<ConstTypeIdIdent>(resolver)?;
                // let namespace = ty.namespace().to_string();
                // reactive_graph_graph::ComponentTypeId::parse_str(#namespace).unwrap()
                Ok(quote! {
                    reactive_graph_graph::InboundOutboundType::Component(
                        reactive_graph_graph::MatchingInboundOutboundType::NamespacedType(
                            std::ops::Deref::deref(&#ty_ident).clone()
                        )
                    )
                })
            }
            MatchingInboundOutboundType::Any => Ok(quote! {
                reactive_graph_graph::InboundOutboundType::Component(reactive_graph_graph::MatchingInboundOutboundType::Any)
            }),
        },
        InboundOutboundType::EntityType(ty) => match ty {
            MatchingInboundOutboundType::NamespacedType(ty) => {
                // let namespace = ty.namespace().to_string();
                // reactive_graph_graph::EntityTypeId::parse_str(#namespace).unwrap()
                let ty_ident = ty.fully_qualified_ident_of_type::<ConstTypeIdIdent>(resolver)?;
                Ok(quote! {
                    reactive_graph_graph::InboundOutboundType::EntityType(
                        reactive_graph_graph::MatchingInboundOutboundType::NamespacedType(
                            std::ops::Deref::deref(&#ty_ident).clone()
                        )
                    )
                })
            }
            MatchingInboundOutboundType::Any => Ok(quote! {
                reactive_graph_graph::InboundOutboundType::EntityType(reactive_graph_graph::MatchingInboundOutboundType::Any)
            }),
        },
    }
}
