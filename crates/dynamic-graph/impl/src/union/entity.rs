use async_graphql::dynamic::*;
use convert_case::Case::Pascal;
use convert_case::Casing;

use crate::object::types::DynamicGraphTypeDefinition;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;

pub const UNION_ALL_ENTITIES: &str = "AllEntities";
pub const UNION_NAMESPACE_ENTITIES_SUFFIX: &str = "Entities";

pub fn namespace_entities_union_type_name(namespace: &str) -> String {
    format!("{}{}", namespace.to_case(Pascal), UNION_NAMESPACE_ENTITIES_SUFFIX)
}

pub fn get_namespace_entities_union(schema: SchemaBuilder, context: &SchemaBuilderContext, namespace: &String) -> SchemaBuilder {
    let type_name = namespace_entities_union_type_name(namespace);
    let mut union = Union::new(type_name).description(format!("Any entity of the namespace {namespace}"));
    for entity_tys in context.entity_type_manager.get_types_by_namespace(namespace) {
        let dy_ty = DynamicGraphTypeDefinition::from(&entity_tys);
        union = union.possible_type(dy_ty.to_string());
    }
    schema.register(union)
}

pub fn get_all_entities_union(schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    if context.entity_type_manager.get_type_ids().is_empty() {
        return schema;
    }
    let mut union = Union::new(UNION_ALL_ENTITIES).description("Any entity.");
    for entity_ty in context.entity_type_manager.get_type_ids() {
        let dy_ty = DynamicGraphTypeDefinition::from(&entity_ty);
        union = union.possible_type(dy_ty.to_string());
    }
    schema.register(union)
}
