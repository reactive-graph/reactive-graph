use crate::field::component_query_field;
use crate::field::flow::flow_query_field;
use crate::field::namespace_type_name;
use crate::field::query::entity_query_field;
use crate::field::relation_query_field;
use crate::object::namespace::collision::field_name_collision;
use crate::object::namespace::metrics::metrics_field;
use crate::object::namespace::schema::json_schema_field;
use crate::object::namespace::sort::sort_by_key;
use async_graphql::dynamic::Object;
use convert_case::Case::Pascal;
use convert_case::Casing;
use itertools::Itertools;
use log::warn;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::TypeDefinitionJsonSchemaGetter;

pub fn namespace_query(context: SchemaBuilderContext, namespace: &String) -> Option<Object> {
    let components = context.component_manager.get_by_namespace(namespace);
    let entity_types = context.entity_type_manager.get_by_namespace(namespace);
    let relation_types = context.relation_type_manager.get_by_namespace(namespace);
    let flow_types = context.flow_type_manager.get_by_namespace(namespace);
    if components.is_empty() && entity_types.is_empty() && relation_types.is_empty() && flow_types.is_empty() {
        warn!("Skip empty query namespace {}", &namespace);
        return None;
    }
    let type_name = namespace_type_name(namespace);
    let namespace_field_value = namespace.clone();

    let mut namespace =
        Object::new(&type_name).description(format!("Queries for components, entities and relations on the namespace {}", &namespace.to_case(Pascal)));

    for component in components.iter().sorted_by(sort_by_key) {
        namespace = namespace.field(component_query_field(component.value()));
        namespace = namespace.field(json_schema_field(&component.ty, component.json_schema()));
    }

    for entity_type in entity_types.iter().sorted_by(sort_by_key) {
        namespace = namespace.field(entity_query_field(entity_type.value()));
        namespace = namespace.field(json_schema_field(&entity_type.ty, entity_type.json_schema()));
    }
    for relation_type in relation_types.iter().sorted_by(sort_by_key) {
        namespace = namespace.field(relation_query_field(relation_type.value()));
        namespace = namespace.field(json_schema_field(&relation_type.ty, relation_type.json_schema()));
    }

    for flow_type in flow_types.iter().sorted_by(sort_by_key) {
        let has_field_name_collision = field_name_collision(flow_type.value(), entity_types.type_ids());
        let Some(wrapper_entity_type) = context.entity_type_manager.get(&flow_type.wrapper_type()) else {
            continue;
        };
        namespace = namespace.field(flow_query_field(flow_type.value(), &wrapper_entity_type, has_field_name_collision));
        if let Some(entity_type) = entity_types.get(&flow_type.wrapper_type()) {
            if let Ok(json_schema) = flow_type.json_schema(entity_type.value()) {
                namespace = namespace.field(json_schema_field(&flow_type.ty, json_schema));
            }
        }
    }

    namespace = namespace.field(metrics_field(Some(namespace_field_value)));
    Some(namespace)
}
