use async_graphql::dynamic::*;

use crate::graphql::dynamic::field_description::get_dynamic_graph_field_descriptions;
use crate::graphql::dynamic::field_name::get_dynamic_graph_field_names;
use crate::graphql::dynamic::instance_component_id_field;
use crate::graphql::dynamic::is_divergent;
use crate::graphql::dynamic::relation_inbound_field;
use crate::graphql::dynamic::relation_instance_id_field;
use crate::graphql::dynamic::relation_key_field;
use crate::graphql::dynamic::relation_outbound_field;
use crate::graphql::dynamic::relation_property_field;
use crate::graphql::dynamic::DynamicGraphTypeDefinition;
use crate::graphql::dynamic::SchemaBuilderContext;
use crate::graphql::dynamic::INTERFACE_RELATION;
use crate::model::*;

pub fn get_relation_types(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for relation_type in context.relation_type_manager.get_all() {
        schema = schema.register(get_relation_type(relation_type.clone(), &context));
    }
    schema
}

pub fn get_relation_type(relation_type: RelationType, context: &SchemaBuilderContext) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(&relation_type.ty);
    let mut object = Object::new(&dy_ty.to_string())
        .description(&relation_type.description)
        .implement(INTERFACE_RELATION);
    // Components
    for component_ty in relation_type.components.iter() {
        object = object.field(instance_component_id_field(&component_ty));
        let component_dy_ty = DynamicGraphTypeDefinition::from(component_ty);
        if !is_divergent(&relation_type, &component_ty) {
            object = object.implement(component_dy_ty.to_string());
        }
    }
    // Edge key field
    object = object.field(relation_key_field());
    object = object.field(relation_instance_id_field());
    for field in relation_type.properties.iter().map(relation_property_field) {
        object = object.field(field);
    }
    // Look up field names and descriptions in extensions
    let field_names = get_dynamic_graph_field_names(&relation_type);
    let field_descriptions = get_dynamic_graph_field_descriptions(&relation_type);
    // Outbound fields
    for field in relation_outbound_field(
        &relation_type.outbound_type,
        field_names.from_relation_to_outbound_entity,
        field_descriptions.from_relation_to_outbound_entity,
        &context,
    ) {
        object = object.field(field);
    }
    // Inbound fields
    for field in relation_inbound_field(
        &relation_type.inbound_type,
        field_names.from_relation_to_inbound_entity,
        field_descriptions.from_relation_to_inbound_entity,
        &context,
    ) {
        object = object.field(field);
    }
    object
}
