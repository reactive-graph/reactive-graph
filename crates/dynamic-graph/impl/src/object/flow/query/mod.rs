use crate::extension::divergent::is_divergent;
use crate::field::flow::flow_id_field;
use crate::field::flow::flow_property_field;
use crate::field::instance_component_id_field;
use crate::interface::flow::INTERFACE_FLOW;
use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::SchemaBuilder;
use itertools::Itertools;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;
use reactive_graph_graph::FlowType;

pub fn register_flow_type_query_objects(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for flow_type in context.flow_type_manager.get_all().iter() {
        schema = schema.register(create_flow_type_query_object(flow_type.value(), context));
    }
    schema
}

pub fn create_flow_type_query_object(flow_type: &FlowType, context: &SchemaBuilderContext) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(&flow_type.ty);
    let mut object = Object::new(dy_ty.to_string()).description(&flow_type.description).implement(INTERFACE_FLOW);
    // ID field
    object = object.field(flow_id_field());
    // wrapper entity instance
    // entities
    // relations
    // variables

    // Only applicable if the entity type of the flow type actually exists
    let entity_ty = flow_type.wrapper_type();
    if let Some(entity_type) = context.entity_type_manager.get(&entity_ty) {
        // Components
        for component_ty in entity_type.components.iter() {
            object = object.field(instance_component_id_field(&component_ty));
            let component_dy_ty = DynamicGraphTypeDefinition::from(component_ty.key());
            if !is_divergent(&entity_type, component_ty.key()) {
                object = object.implement(component_dy_ty.to_string());
            }
        }
        // Property Fields
        for property_type in entity_type.properties.iter().sorted_by(|a, b| Ord::cmp(&a.key(), &b.key())) {
            object = object.field(flow_property_field(&property_type));
        }
    }

    object
}
