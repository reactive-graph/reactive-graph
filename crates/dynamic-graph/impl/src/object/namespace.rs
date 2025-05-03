use async_graphql::dynamic::Object;
use convert_case::Case::Pascal;
use convert_case::Casing;
use itertools::Itertools;
use log::warn;

use crate::component_query_field;
use crate::entity_creation_field;
use crate::entity_mutation_field;
use crate::entity_query_field;
use crate::namespace_mutation_type_name;
use crate::namespace_type_name;
use crate::relation_creation_field;
use crate::relation_mutation_field;
use crate::relation_query_field;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;

pub fn namespace_query(context: SchemaBuilderContext, namespace: &String) -> Option<Object> {
    let components = context.component_manager.get_by_namespace(namespace);
    let entity_types = context.entity_type_manager.get_by_namespace(namespace);
    let relation_types = context.relation_type_manager.get_by_namespace(namespace);
    if components.is_empty() && entity_types.is_empty() && relation_types.is_empty() {
        warn!("Skip empty query namespace {}", &namespace);
        return None;
    }
    let type_name = namespace_type_name(namespace);
    let mut namespace =
        Object::new(&type_name).description(format!("Queries for components, entities and relations on the namespace {}", &namespace.to_case(Pascal)));

    for component in components.iter().sorted_by(|a, b| Ord::cmp(&a.key(), &b.key())) {
        namespace = namespace.field(component_query_field(component.key(), component.value()));
    }

    for entity_type in entity_types.iter().sorted_by(|a, b| Ord::cmp(&a.key(), &b.key())) {
        namespace = namespace.field(entity_query_field(entity_type.key(), entity_type.value()));
    }

    for relation_type in relation_types.iter().sorted_by(|a, b| Ord::cmp(&a.key(), &b.key())) {
        namespace = namespace.field(relation_query_field(relation_type.key(), relation_type.value()));
    }

    Some(namespace)
}

pub fn namespace_mutation(context: SchemaBuilderContext, namespace: &String) -> Option<Object> {
    let entity_types = context.entity_type_manager.get_by_namespace(namespace);
    let relation_types = context.relation_type_manager.get_by_namespace(namespace);
    if entity_types.is_empty() && relation_types.is_empty() {
        warn!("Skip empty mutation namespace {}", &namespace);
        return None;
    }
    let type_name = namespace_mutation_type_name(namespace);
    let mut namespace = Object::new(&type_name).description(format!("Mutations for entities and relations on the namespace {}", &namespace.to_case(Pascal)));

    let mut contains_field = false;

    for entity_type in entity_types.iter().sorted_by(|a, b| Ord::cmp(&a.key(), &b.key())) {
        if let Some(field) = entity_creation_field(entity_type.key(), entity_type.value()) {
            namespace = namespace.field(field);
            contains_field = true;
        }
        if let Some(field) = entity_mutation_field(entity_type.value()) {
            namespace = namespace.field(field);
            contains_field = true;
        }
    }

    for relation_type in relation_types.iter().sorted_by(|a, b| Ord::cmp(&a.key(), &b.key())) {
        if let Some(field) = relation_creation_field(relation_type.key(), relation_type.value()) {
            namespace = namespace.field(field);
            contains_field = true;
        }
        if let Some(field) = relation_mutation_field(relation_type.key(), relation_type.value()) {
            namespace = namespace.field(field);
            contains_field = true;
        }
    }

    if contains_field { Some(namespace) } else { None }
}
