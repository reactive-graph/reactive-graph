use async_graphql::dynamic::Interface;
use async_graphql::dynamic::SchemaBuilder;

use reactive_graph_dynamic_graph_api::SchemaBuilderContext;
use reactive_graph_graph::Component;

use crate::field::component_id_field;
use crate::field::component_property_field;
use crate::interface::entity::get_entity_interface;
use crate::interface::flow::get_flow_interface;
use crate::interface::relation::get_relation_interface;
use crate::object::types::DynamicGraphTypeDefinition;

pub fn get_interfaces(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    schema = schema.register(get_entity_interface());
    schema = schema.register(get_relation_interface());
    schema = schema.register(get_flow_interface());
    for (_, component) in context.component_manager.get_all() {
        schema = schema.register(get_component_interface(component.clone()));
    }
    schema
}

pub fn get_component_interface(component: Component) -> Interface {
    let dy_ty = DynamicGraphTypeDefinition::from(&component.ty);
    let mut interface = Interface::new(dy_ty.to_string())
        .description(&component.description)
        .field(component_id_field(&component.ty));
    for field in component.properties.iter() {
        interface = interface.field(component_property_field(field.value()));
    }
    interface
}
