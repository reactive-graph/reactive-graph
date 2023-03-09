use async_graphql::dynamic::Interface;
use async_graphql::dynamic::SchemaBuilder;

use crate::graphql::dynamic::component_id_field;
use crate::graphql::dynamic::component_property_field;
use crate::graphql::dynamic::get_entity_interface;
use crate::graphql::dynamic::get_relation_interface;
use crate::graphql::dynamic::DynamicGraphTypeDefinition;
use crate::graphql::dynamic::SchemaBuilderContext;
use crate::model::Component;

pub fn get_interfaces(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    schema = schema.register(get_entity_interface());
    schema = schema.register(get_relation_interface());
    for component in context.component_manager.get_all() {
        schema = schema.register(get_component_interface(component.clone()));
    }
    schema
}

pub fn get_component_interface(component: Component) -> Interface {
    let dy_ty = DynamicGraphTypeDefinition::from(&component.ty);
    let mut interface = Interface::new(&dy_ty.to_string())
        .description(&component.description)
        .field(component_id_field(&component.ty));
    for field in component.properties.iter().map(component_property_field) {
        interface = interface.field(field);
    }
    interface
}
