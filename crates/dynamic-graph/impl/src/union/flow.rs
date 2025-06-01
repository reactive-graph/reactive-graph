use async_graphql::dynamic::*;
use convert_case::Case::Pascal;
use convert_case::Casing;

use crate::object::types::DynamicGraphTypeDefinition;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;

pub const UNION_ALL_FLOWS: &str = "AllFlows";
pub const UNION_NAMESPACE_FLOWS_SUFFIX: &str = "Flows";

pub fn namespace_flows_union_type_name(namespace: &str) -> String {
    format!("{}{}", namespace.to_case(Pascal), UNION_NAMESPACE_FLOWS_SUFFIX)
}

pub fn get_namespace_flows_union(schema: SchemaBuilder, context: &SchemaBuilderContext, namespace: &String) -> SchemaBuilder {
    let type_name = namespace_flows_union_type_name(namespace);
    let mut union = Union::new(type_name).description(format!("Any flow of the namespace {namespace}"));
    for flow_tys in context.flow_type_manager.get_types_by_namespace(namespace) {
        let dy_ty = DynamicGraphTypeDefinition::from(&flow_tys);
        union = union.possible_type(dy_ty.to_string());
    }
    schema.register(union)
}

pub fn get_all_flows_union(schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    if context.flow_type_manager.get_type_ids().is_empty() {
        return schema;
    }
    let mut union = Union::new(UNION_ALL_FLOWS).description("Any flow.");
    for flow_ty in context.flow_type_manager.get_type_ids() {
        let dy_ty = DynamicGraphTypeDefinition::from(&flow_ty);
        union = union.possible_type(dy_ty.to_string());
    }
    schema.register(union)
}
