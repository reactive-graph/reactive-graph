use crate::CodeGenerationError;
use crate::namespace::rust::fully_qualified_ident::FullyQualifiedNamespacedTypeIdent;
use crate::property::rust::property_instance::visibility::Visibility;
use crate::property::rust::property_instance_getter_impl::generate_property_instance_getter_impl;
use crate::property::rust::property_instance_setter_impl::generate_property_instance_setter_impl;
use crate::type_definition::rust::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::TypeIdent;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_generator_documentation::GenerateDocumentation;
use reactive_graph_graph::ComponentTypeIdContainer;
use reactive_graph_graph::Mutability;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeDescriptionGetter;
use reactive_graph_graph::TypeResolver;

pub fn generate_impl_component_traits<TY: TypeDefinitionGetter + ComponentTypeIdContainer + TypeDescriptionGetter + GenerateDocumentation<TY>>(
    type_: &TY,
    resolver: &TypeResolver,
) -> Result<TokenStream, CodeGenerationError> {
    let type_name_ident = TypeIdent::new(type_);
    let component_tys = type_.get_components_cloned();
    let mut components = resolver.components(&component_tys)?.to_vec();
    components.sort();
    let mut impl_component_traits = Vec::new();
    for component in components {
        let fully_qualified_component_ident = component.ty.fully_qualified_ident_of_type::<TypeIdent>(resolver)?;
        let mut properties = component.properties.to_vec();
        properties.sort();
        let mut properties_token_stream = Vec::new();
        for property in properties {
            properties_token_stream.push(generate_property_instance_getter_impl(&property, resolver, Visibility::Private));
            if property.mutability == Mutability::Mutable {
                properties_token_stream.push(generate_property_instance_setter_impl(&property, resolver, Visibility::Private));
            }
        }
        let impl_component_trait = quote! {
            impl #fully_qualified_component_ident for #type_name_ident {
                #(#properties_token_stream)*
            }
        };
        impl_component_traits.push(impl_component_trait);
    }
    Ok(quote! {
        #(#impl_component_traits)*
    })
}
