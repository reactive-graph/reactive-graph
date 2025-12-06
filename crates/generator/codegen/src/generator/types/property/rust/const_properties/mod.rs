use crate::CodeGenerationError;
use crate::type_definition::rust::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::TypePropertiesIdent;
use ident::const_property_variant_ident;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeResolver;

pub mod as_ref;
pub mod const_enum;
pub mod display;
pub mod ident;
pub mod into_iter;
pub mod into_string;
pub mod property_type;
pub mod static_str;

pub fn generate_const_properties<TY: TypeDefinitionGetter + PropertyTypeContainer>(
    type_: &TY,
    resolver: &TypeResolver,
    properties: &Vec<PropertyType>,
) -> Result<TokenStream, CodeGenerationError> {
    let type_properties_ident = TypePropertiesIdent::new(type_);
    let const_properties_enum = const_enum::generate_const_properties_enum(type_, resolver, &type_properties_ident, &properties)?;
    let as_ref_const_properties = as_ref::generate_as_ref_const_properties(&type_properties_ident, &properties);
    let convert_const_properties_into_static_str = static_str::generate_convert_const_properties_into_static_str(&type_properties_ident, &properties);
    let convert_const_properties_into_string = into_string::generate_convert_const_properties_into_string(&type_properties_ident, &properties);
    let convert_const_properties_into_property_type = property_type::generate_convert_const_properties_into_property_type(&type_properties_ident, &properties);
    let into_iter = into_iter::generate_into_iter(type_, &type_properties_ident, &properties);
    let const_properties_display = display::generate_const_properties_display(&type_properties_ident, &properties);

    Ok(quote! {
        #const_properties_enum
        #as_ref_const_properties
        #convert_const_properties_into_static_str
        #convert_const_properties_into_string
        #convert_const_properties_into_property_type
        #into_iter
        #const_properties_display
        // TODO: Generate property extensions
        // impl { pub fn extensions() -> Extensions { ... } }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use reactive_graph_graph::Component;
    use reactive_graph_graph::ComponentTypeIdContainer;
    use reactive_graph_graph::Components;
    use reactive_graph_graph::EntityType;
    use reactive_graph_graph::EntityTypes;
    use reactive_graph_graph::RandomNamespacedType;
    use reactive_graph_graph::RandomNamespacedTypesWithId;
    use reactive_graph_graph::TypeSystem;

    #[test]
    pub fn test_generate_const_properties_for_component() {
        let component = Component::random_type().unwrap();
        let mut properties = component.properties.to_vec();
        properties.sort();
        let components = Components::new();
        components.push(component.clone());
        let resolver = TypeSystem::builder().components(components).build().into();
        let token_stream = generate_const_properties(&component, &resolver, &properties).unwrap();
        println!("{}", token_stream);
        assert!(!token_stream.is_empty())
    }

    #[test]
    pub fn test_generate_const_properties_for_entity_type() {
        let entity_type = EntityType::random_type().unwrap();
        let component_tys = entity_type.get_components_cloned();
        let components = Components::random_types_with_ids(&component_tys).unwrap();
        let entity_types = EntityTypes::new();
        entity_types.push(entity_type.clone());

        let resolver: TypeResolver = TypeSystem::builder().components(components).entity_types(entity_types).build().into();
        let properties = resolver.resolve_properties_sorted(&entity_type).unwrap();
        let token_stream = generate_const_properties(&entity_type, &resolver, &properties).unwrap();
        println!("{}", token_stream);
        assert!(!token_stream.is_empty())
    }
}
