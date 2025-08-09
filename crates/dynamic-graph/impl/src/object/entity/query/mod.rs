use crate::extension::divergent::is_divergent;
use crate::extension::field_description::get_dynamic_graph_field_descriptions;
use crate::extension::field_name::get_dynamic_graph_field_names;
use crate::field::id::entity_id_field;
use crate::field::inbound::outbound_field::inbound_entity_to_outbound_field;
use crate::field::inbound::relation_field::entity_inbound_relation_field;
use crate::field::instance_component_id_field;
use crate::field::outbound::inbound_field::outbound_entity_to_inbound_field;
use crate::field::outbound::relation_field::entity_outbound_relation_field;
use crate::field::property::property_field::entity_property_field;
use crate::interface::entity::INTERFACE_ENTITY;
use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::SchemaBuilder;
use itertools::Itertools;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::InboundOutboundType;
use reactive_graph_graph::RelationTypes;

pub fn register_entity_type_query_objects(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for entity_type in context.entity_type_manager.get_all().iter() {
        let ty = InboundOutboundType::EntityType(entity_type.key().clone());
        let outbound_types = context.relation_type_manager.get_outbound_relation_types(&ty, false);
        let inbound_types = context.relation_type_manager.get_inbound_relation_types(&ty, false);
        let entity_type = create_entity_type_query_object(entity_type.value(), outbound_types, inbound_types, context);
        schema = schema.register(entity_type);
    }
    schema
}

pub fn create_entity_type_query_object(
    entity_type: &EntityType,
    outbound_types: RelationTypes,
    inbound_types: RelationTypes,
    context: &SchemaBuilderContext,
) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    let mut object = Object::new(dy_ty.to_string()).description(&entity_type.description).implement(INTERFACE_ENTITY);
    // ID field
    object = object.field(entity_id_field());
    // Components
    for component_ty in entity_type.components.iter() {
        object = object.field(instance_component_id_field(component_ty.key()));
        let component_dy_ty = DynamicGraphTypeDefinition::from(component_ty.key());
        if !is_divergent(entity_type, component_ty.key()) {
            object = object.implement(component_dy_ty.to_string());
        }
    }
    // Property Fields
    for property_type in entity_type.properties.iter().sorted_by(|a, b| Ord::cmp(&a.key(), &b.key())) {
        object = object.field(entity_property_field(&property_type));
    }
    // Outbound Relations
    for outbound_relation_type in outbound_types.iter() {
        // Look up field names and descriptions in extensions
        let field_names = get_dynamic_graph_field_names(outbound_relation_type.value());
        let field_descriptions = get_dynamic_graph_field_descriptions(outbound_relation_type.value());

        if let Some(entity_outbound_relation_field) = entity_outbound_relation_field(outbound_relation_type.value(), &field_names, &field_descriptions) {
            object = object.field(entity_outbound_relation_field);
        }
        for field in outbound_entity_to_inbound_field(&outbound_relation_type, &field_names, &field_descriptions, context) {
            object = object.field(field);
        }
    }
    // Inbound Relations
    for inbound_relation_type in inbound_types.iter() {
        // Look up field names and descriptions in extensions
        let field_names = get_dynamic_graph_field_names(inbound_relation_type.value());
        let field_descriptions = get_dynamic_graph_field_descriptions(inbound_relation_type.value());

        if let Some(entity_inbound_relation_field) = entity_inbound_relation_field(&inbound_relation_type, &field_names, &field_descriptions) {
            object = object.field(entity_inbound_relation_field);
        }
        for field in inbound_entity_to_outbound_field(&inbound_relation_type, &field_names, &field_descriptions, context) {
            object = object.field(field);
        }
    }
    object
}
