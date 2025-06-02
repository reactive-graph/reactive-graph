use crate::field::component_query_field;
use crate::field::entity_creation_field;
use crate::field::entity_mutation_field;
use crate::field::entity_query_field;
use crate::field::flow::flow_creation_field;
use crate::field::flow::flow_mutation_field;
use crate::field::flow::flow_query_field;
use crate::field::namespace_mutation_type_name;
use crate::field::namespace_type_name;
use crate::field::relation_creation_field;
use crate::field::relation_mutation_field;
use crate::field::relation_query_field;
use crate::object::namespace::schema::json_schema_field;
use async_graphql::dynamic::Object;
use convert_case::Case::Pascal;
use convert_case::Casing;
use dashmap::mapref::multiple::RefMulti;
use itertools::Itertools;
use log::warn;
use metrics::metrics_field;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;
use reactive_graph_graph::TypeDefinitionJsonSchemaGetter;
use std::cmp::Ordering;
use std::hash::Hash;

pub mod metrics;
pub mod schema;

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
        let entity_type = entity_type.value();
        namespace = namespace.field(entity_query_field(entity_type));
        namespace = namespace.field(json_schema_field(&entity_type.ty, entity_type.json_schema()));
    }
    for relation_type in relation_types.iter().sorted_by(sort_by_key) {
        namespace = namespace.field(relation_query_field(relation_type.value()));
        namespace = namespace.field(json_schema_field(&relation_type.ty, relation_type.json_schema()));
    }

    for flow_type in flow_types.iter().sorted_by(sort_by_key) {
        namespace = namespace.field(flow_query_field(flow_type.value()));
        if let Some(entity_type) = entity_types.get(&flow_type.wrapper_type()) {
            if let Ok(json_schema) = flow_type.json_schema(entity_type.value()) {
                namespace = namespace.field(json_schema_field(&flow_type.ty, json_schema));
            }
        }
    }

    namespace = namespace.field(metrics_field(Some(namespace_field_value)));
    Some(namespace)
}

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

    for entity_type in entity_types.iter().sorted_by(|a, b| Ord::cmp(&a.key(), &b.key())) {
        if let Some(field) = entity_creation_field(entity_type.value()) {
            namespace = namespace.field(field);
            contains_field = true;
        }
        if let Some(field) = entity_mutation_field(entity_type.value()) {
            namespace = namespace.field(field);
            contains_field = true;
        }
        // if let Some(field) = entity_export_field(entity_type.key()) {
        //     namespace = namespace.field(field);
        //     contains_field = true;
        // }
    }

    for relation_type in relation_types.iter().sorted_by(|a, b| Ord::cmp(&a.key(), &b.key())) {
        if let Some(field) = relation_creation_field(relation_type.value()) {
            namespace = namespace.field(field);
            contains_field = true;
        }
        if let Some(field) = relation_mutation_field(relation_type.value()) {
            namespace = namespace.field(field);
            contains_field = true;
        }
    }

    for flow_type in flow_types.iter().sorted_by(|a, b| Ord::cmp(&a.key(), &b.key())) {
        if let Some(field) = flow_creation_field(flow_type.value()) {
            namespace = namespace.field(field);
            contains_field = true;
        }
        if let Some(field) = flow_mutation_field(flow_type.value()) {
            namespace = namespace.field(field);
            contains_field = true;
        }
    }

    namespace = namespace.field(metrics_field(Some(namespace_field_value)));

    if contains_field { Some(namespace) } else { None }
}

fn sort_by_key<ID, TY>(a: &RefMulti<ID, TY>, b: &RefMulti<ID, TY>) -> Ordering
where
    ID: Hash + Ord,
    ID: Eq,
{
    Ord::cmp(&a.key(), &b.key())
}
