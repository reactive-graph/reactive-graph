use async_graphql::dynamic::*;
use convert_case::Case::Pascal;
use convert_case::Casing;

use crate::graphql::dynamic::DynamicGraphTypeDefinition;
use crate::graphql::dynamic::SchemaBuilderContext;

pub const UNION_ALL_ENTITIES: &str = "AllEntities";
pub const UNION_NAMESPACE_ENTITIES_SUFFIX: &str = "Entities";

pub fn namespace_entities_union_type_name(namespace: &str) -> String {
    format!("{}{}", namespace.to_case(Pascal), UNION_NAMESPACE_ENTITIES_SUFFIX)
}

pub fn get_namespace_entities_union(schema: SchemaBuilder, context: &SchemaBuilderContext, namespace: &String) -> SchemaBuilder {
    let type_name = namespace_entities_union_type_name(namespace);
    let mut union = Union::new(type_name);
    for entity_type in context.entity_type_manager.get_by_namespace(&namespace) {
        let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
        union = union.possible_type(&dy_ty.to_string());
    }
    schema.register(union)
}

pub fn get_all_entities_union(schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    let mut union = Union::new(UNION_ALL_ENTITIES);
    for entity_type in context.entity_type_manager.get_all() {
        let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
        union = union.possible_type(&dy_ty.to_string());
    }
    schema.register(union)
}
