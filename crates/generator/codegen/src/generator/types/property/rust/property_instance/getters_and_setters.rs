use crate::CodeGenerationConfig;
use crate::property::rust::property_instance::visibility::Visibility;
use crate::property::rust::property_instance_getter_impl::generate_property_instance_getter_impl;
use crate::property::rust::property_instance_setter_impl::generate_property_instance_setter_impl;
use proc_macro2::TokenStream;
use reactive_graph_graph::Mutability;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::TypeResolver;

pub fn property_instance_getters_and_setters(
    properties: &Vec<PropertyType>,
    _config: &CodeGenerationConfig,
    resolver: &TypeResolver,
    visibility: Visibility,
) -> Vec<TokenStream> {
    let mut property_instance_getters = Vec::new();
    for property in properties.into_iter() {
        property_instance_getters.push(generate_property_instance_getter_impl(&property, resolver, visibility));
        if property.mutability == Mutability::Mutable {
            property_instance_getters.push(generate_property_instance_setter_impl(&property, resolver, visibility));
        }
    }
    property_instance_getters
}
