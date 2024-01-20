use async_graphql::dynamic::Interface;
use async_graphql::dynamic::SchemaBuilder;

use crate::component_id_field;
use crate::component_property_field;
use crate::get_entity_interface;
use crate::get_relation_interface;
use crate::DynamicGraphTypeDefinition;
use inexor_rgf_dynamic_graph_api::SchemaBuilderContext;
use inexor_rgf_graph::Component;

pub fn get_interfaces(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    schema = schema.register(get_entity_interface());
    schema = schema.register(get_relation_interface());
    for (_, component) in context.component_manager.get_all() {
        schema = schema.register(get_component_interface(component.clone()));
    }
    schema
}

pub fn get_component_interface(component: Component) -> Interface {
    let dy_ty = DynamicGraphTypeDefinition::from(&component.ty);
    let mut interface = Interface::new(&dy_ty.to_string())
        .description(&component.description)
        .field(component_id_field(&component.ty));
    for field in component.properties.iter() {
        interface = interface.field(component_property_field(field.value()));
    }
    interface
}
