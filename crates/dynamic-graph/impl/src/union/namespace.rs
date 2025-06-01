use async_graphql::dynamic::*;

use crate::union::entity::get_namespace_entities_union;
use crate::union::relation::get_namespace_relations_union;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;

pub fn get_namespace_unions(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for namespace in context.namespace_manager.get_all() {
        schema = get_namespace_entities_union(schema, context, &namespace);
        schema = get_namespace_relations_union(schema, context, &namespace);
    }
    schema
}
