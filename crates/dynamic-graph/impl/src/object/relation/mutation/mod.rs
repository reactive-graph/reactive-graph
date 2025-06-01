use crate::object::relation::mutation::delete::relation_delete_field;
use crate::object::relation::mutation::export::relation_export_field;
use crate::object::relation::mutation::trigger::relation_trigger_field;
use crate::object::relation::mutation::update::relation_update_field;
use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::SchemaBuilder;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;
use reactive_graph_graph::RelationType;

pub mod delete;
pub mod export;
pub mod trigger;
pub mod update;

pub fn register_relation_type_mutation_objects(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for (_, relation_type) in context.relation_type_manager.get_all() {
        schema = schema.register(create_relation_type_mutation_object(&relation_type));
    }
    schema
}

pub fn create_relation_type_mutation_object(relation_type: &RelationType) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(&relation_type.ty);
    let mut object = Object::new(dy_ty.mutation_type_name());
    if let Some(update_field) = relation_update_field(relation_type) {
        object = object.field(update_field);
    }
    if let Some(trigger_field) = relation_trigger_field(relation_type) {
        object = object.field(trigger_field);
    }
    object = object.field(relation_export_field());
    object = object.field(relation_delete_field());
    object
}
