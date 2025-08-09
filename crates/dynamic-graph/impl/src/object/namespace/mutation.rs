use crate::field::creation::entity_creation_field;
use crate::field::flow::flow_creation_field;
use crate::field::flow::flow_mutation_field;
use crate::field::mutation::entity_mutation_field;
use crate::field::namespace_mutation_type_name;
use crate::field::relation_creation_field;
use crate::field::relation_mutation_field;
use crate::object::namespace::collision::field_name_collision;
use crate::object::namespace::metrics::metrics_field;
use crate::object::namespace::sort::sort_by_key;
use async_graphql::dynamic::Object;
use convert_case::Case::Pascal;
use convert_case::Casing;
use itertools::Itertools;
use log::warn;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;
use reactive_graph_graph::NamespacedTypeContainer;

pub fn namespace_mutation(context: SchemaBuilderContext, namespace: &String) -> Option<Object> {
    let entity_types = context.entity_type_manager.get_by_namespace(namespace);
    let relation_types = context.relation_type_manager.get_by_namespace(namespace);
    let flow_types = context.flow_type_manager.get_by_namespace(namespace);
    if entity_types.is_empty() && relation_types.is_empty() && flow_types.is_empty() {
        warn!("Skip empty mutation namespace {}", &namespace);
        return None;
    }
    let type_name = namespace_mutation_type_name(namespace);
    let namespace_field_value = namespace.clone();

    let mut namespace = Object::new(&type_name).description(format!("Mutations for entities and relations on the namespace {}", &namespace.to_case(Pascal)));

    let mut contains_field = false;

    for entity_type in entity_types.iter().sorted_by(sort_by_key) {
        if let Some(field) = entity_creation_field(entity_type.value()) {
            namespace = namespace.field(field);
            contains_field = true;
        }
        if let Some(field) = entity_mutation_field(entity_type.value()) {
            namespace = namespace.field(field);
            contains_field = true;
        }
    }

    for relation_type in relation_types.iter().sorted_by(sort_by_key) {
        if let Some(field) = relation_creation_field(relation_type.value()) {
            namespace = namespace.field(field);
            contains_field = true;
        }
        if let Some(field) = relation_mutation_field(relation_type.value()) {
            namespace = namespace.field(field);
            contains_field = true;
        }
    }

    for flow_type in flow_types.iter().sorted_by(sort_by_key) {
        let has_field_name_collision = field_name_collision(flow_type.value(), entity_types.type_ids());
        let Some(wrapper_entity_type) = context.entity_type_manager.get(&flow_type.wrapper_type()) else {
            continue;
        };
        if let Some(field) = flow_creation_field(flow_type.value(), &wrapper_entity_type, has_field_name_collision) {
            namespace = namespace.field(field);
            contains_field = true;
        }
        if let Some(field) = flow_mutation_field(flow_type.value(), &wrapper_entity_type, has_field_name_collision) {
            namespace = namespace.field(field);
            contains_field = true;
        }
    }

    namespace = namespace.field(metrics_field(Some(namespace_field_value)));

    if contains_field { Some(namespace) } else { None }
}
