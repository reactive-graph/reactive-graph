use crate::object::entity::mutation::delete::entity_delete_field;
use crate::object::entity::mutation::export::entity_export_field;
use crate::object::entity::mutation::trigger::entity_trigger_field;
use crate::object::entity::mutation::update::entity_update_field;
use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::SchemaBuilder;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;
use reactive_graph_graph::EntityType;

pub mod delete;
pub mod export;
pub mod trigger;
pub mod update;

pub fn register_entity_type_mutation_objects(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for (_, entity_type) in context.entity_type_manager.get_all() {
        schema = schema.register(create_entity_mutation_object(&entity_type));
    }
    schema
}

pub fn create_entity_mutation_object(entity_type: &EntityType) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    let mut object = Object::new(dy_ty.mutation_type_name());
    if let Some(update_field) = entity_update_field(entity_type) {
        object = object.field(update_field);
    }
    if let Some(trigger_field) = entity_trigger_field(entity_type) {
        object = object.field(trigger_field);
    }
    object = object.field(entity_export_field());
    object = object.field(entity_delete_field());
    object
}
