use crate::extension::divergent::is_divergent;
use crate::extension::field_description::get_dynamic_graph_field_descriptions;
use crate::extension::field_name::get_dynamic_graph_field_names;
use crate::field::instance_component_id_field;
use crate::field::relation_inbound_field;
use crate::field::relation_instance_id_field;
use crate::field::relation_key_field;
use crate::field::relation_outbound_field;
use crate::field::relation_property_field;
use crate::interface::relation::INTERFACE_RELATION;
use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::SchemaBuilder;
use itertools::Itertools;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::RelationTypeId;

pub fn register_relation_type_query_objects(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for relation_type in context.relation_type_manager.get_all().iter() {
        schema = schema.register(create_relation_type_query_object(relation_type.key(), relation_type.value(), context));
    }
    schema
}

pub fn create_relation_type_query_object(relation_ty: &RelationTypeId, relation_type: &RelationType, context: &SchemaBuilderContext) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(relation_ty);
    let mut object = Object::new(dy_ty.to_string())
        .description(&relation_type.description)
        .implement(INTERFACE_RELATION);
    // Components
    for component_ty in relation_type.components.iter() {
        object = object.field(instance_component_id_field(&component_ty));
        let component_dy_ty = DynamicGraphTypeDefinition::from(component_ty.key());
        if !is_divergent(relation_type, component_ty.key()) {
            object = object.implement(component_dy_ty.to_string());
        }
    }
    // Relation Instance ID field
    object = object.field(relation_key_field());
    object = object.field(relation_instance_id_field());
    for property_type in relation_type.properties.iter().sorted_by(|a, b| Ord::cmp(&a.key(), &b.key())) {
        object = object.field(relation_property_field(property_type.value()));
    }
    // Look up field names and descriptions in extensions
    let field_names = get_dynamic_graph_field_names(relation_type);
    let field_descriptions = get_dynamic_graph_field_descriptions(relation_type);
    // Outbound fields
    for field in relation_outbound_field(
        &relation_type.outbound_type,
        field_names.from_relation_to_outbound_entity,
        field_descriptions.from_relation_to_outbound_entity,
        context,
    ) {
        object = object.field(field);
    }
    // Inbound fields
    for field in relation_inbound_field(
        &relation_type.inbound_type,
        field_names.from_relation_to_inbound_entity,
        field_descriptions.from_relation_to_inbound_entity,
        context,
    ) {
        object = object.field(field);
    }
    object
}
