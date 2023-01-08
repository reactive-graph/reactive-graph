use async_graphql::dynamic::*;

use crate::graphql::dynamic::entity_id_field;
use crate::graphql::dynamic::entity_inbound_field;
use crate::graphql::dynamic::entity_outbound_field;
use crate::graphql::dynamic::entity_property_field;
use crate::graphql::dynamic::instance_component_id_field;
use crate::graphql::dynamic::is_divergent;
use crate::graphql::dynamic::DynamicGraphTypeDefinition;
use crate::graphql::dynamic::SchemaBuilderContext;
use crate::graphql::dynamic::INTERFACE_ENTITY;
use crate::model::*;

pub fn get_entity_types(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for entity_type in context.entity_type_manager.get_all() {
        let ty = ComponentOrEntityTypeId::EntityType(entity_type.ty.clone());
        let outbound_types = context.relation_type_manager.get_outbound_relation_types(&ty, false);
        let inbound_types = context.relation_type_manager.get_inbound_relation_types(&ty, false);
        schema = schema.register(get_entity_type(entity_type.clone(), outbound_types, inbound_types));
    }
    schema
}

pub fn get_entity_type(entity_type: EntityType, outbound_types: Vec<RelationType>, inbound_types: Vec<RelationType>) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    let mut object = Object::new(&dy_ty.to_string())
        .description(&entity_type.description)
        .implement(INTERFACE_ENTITY);
    // Components
    for component_ty in entity_type.components.iter() {
        object = object.field(instance_component_id_field(&component_ty));
        let component_dy_ty = DynamicGraphTypeDefinition::from(component_ty);
        if !is_divergent(&entity_type, &component_ty) {
            object = object.implement(component_dy_ty.to_string());
        }
    }
    // ID field
    object = object.field(entity_id_field());
    // Property Fields
    for field in entity_type.properties.iter().map(entity_property_field) {
        object = object.field(field);
    }
    // Outbound Relations
    for outbound_relation_type in outbound_types {
        object = object.field(entity_outbound_field(&outbound_relation_type));
    }
    // Inbound Relations
    for inbound_relation_type in inbound_types {
        object = object.field(entity_inbound_field(&inbound_relation_type));
    }
    object
}
