use async_graphql::dynamic::*;
use convert_case::Case::Pascal;
use convert_case::Casing;

use crate::DynamicGraphTypeDefinition;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;

pub const UNION_ALL_RELATIONS: &str = "AllRelations";
pub const UNION_NAMESPACE_RELATIONS_SUFFIX: &str = "Relations";

pub fn get_namespace_relations_union(schema: SchemaBuilder, context: &SchemaBuilderContext, namespace: &String) -> SchemaBuilder {
    let type_name = format!("{}{}", namespace.to_case(Pascal), UNION_NAMESPACE_RELATIONS_SUFFIX);
    let mut union = Union::new(type_name).description(format!("Any relation of the namespace {}", namespace));
    for relation_ty in context.relation_type_manager.get_types_by_namespace(namespace) {
        let dy_ty = DynamicGraphTypeDefinition::from(&relation_ty);
        union = union.possible_type(dy_ty.to_string());
    }
    schema.register(union)
}

pub fn get_all_relations_union(schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    let mut union = Union::new(UNION_ALL_RELATIONS).description("Any relation.");
    for relation_ty in context.relation_type_manager.get_type_ids() {
        let dy_ty = DynamicGraphTypeDefinition::from(&relation_ty);
        union = union.possible_type(dy_ty.to_string());
    }
    schema.register(union)
}
