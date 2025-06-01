use async_graphql::dynamic::SchemaBuilder;

use crate::union::entity::get_all_entities_union;
use crate::union::flow::get_all_flows_union;
use crate::union::namespace::get_namespace_unions;
use crate::union::relation::get_all_relations_union;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;

pub mod entity;
pub mod flow;
pub mod namespace;
pub mod relation;

pub fn get_unions(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    schema = get_all_entities_union(schema, context);
    schema = get_all_relations_union(schema, context);
    schema = get_all_flows_union(schema, context);
    schema = get_namespace_unions(schema, context);
    schema
}
