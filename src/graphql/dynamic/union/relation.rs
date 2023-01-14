use async_graphql::dynamic::*;
use convert_case::Case::Pascal;
use convert_case::Casing;

use crate::graphql::dynamic::DynamicGraphTypeDefinition;
use crate::graphql::dynamic::SchemaBuilderContext;

pub const UNION_ALL_RELATIONS: &str = "AllRelations";
pub const UNION_NAMESPACE_RELATIONS_SUFFIX: &str = "Relations";

pub fn get_namespace_relations_union(schema: SchemaBuilder, context: &SchemaBuilderContext, namespace: &String) -> SchemaBuilder {
    let type_name = format!("{}{}", namespace.to_case(Pascal), UNION_NAMESPACE_RELATIONS_SUFFIX);
    let mut union = Union::new(type_name).description(format!("Any relation of the namespace {}", namespace));
    for relation_type in context.relation_type_manager.get_by_namespace(&namespace) {
        let dy_ty = DynamicGraphTypeDefinition::from(&relation_type.ty);
        union = union.possible_type(&dy_ty.to_string());
    }
    schema.register(union)
}

pub fn get_all_relations_union(schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    let mut union = Union::new(UNION_ALL_RELATIONS).description("Any relation.");
    for relation_type in context.relation_type_manager.get_all() {
        let dy_ty = DynamicGraphTypeDefinition::from(&relation_type.ty);
        union = union.possible_type(&dy_ty.to_string());
    }
    schema.register(union)
}
