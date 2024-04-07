use async_graphql::dynamic::SchemaBuilder;

pub use entity::*;
pub use namespace::*;
pub use relation::*;

use reactive_graph_dynamic_graph_api::SchemaBuilderContext;

pub mod entity;
pub mod namespace;
pub mod relation;

pub fn get_unions(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    schema = get_all_entities_union(schema, context);
    schema = get_all_relations_union(schema, context);
    schema = get_namespace_unions(schema, context);
    schema
}
