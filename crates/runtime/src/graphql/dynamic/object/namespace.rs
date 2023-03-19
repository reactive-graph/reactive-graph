use async_graphql::dynamic::Object;
use convert_case::Case::Pascal;
use convert_case::Casing;
use log::warn;

use crate::graphql::dynamic::component_query_field;
use crate::graphql::dynamic::entity_creation_field;
use crate::graphql::dynamic::entity_mutation_field;
use crate::graphql::dynamic::entity_query_field;
use crate::graphql::dynamic::namespace_mutation_type_name;
use crate::graphql::dynamic::namespace_type_name;
use crate::graphql::dynamic::relation_creation_field;
use crate::graphql::dynamic::relation_mutation_field;
use crate::graphql::dynamic::relation_query_field;
use crate::graphql::dynamic::SchemaBuilderContext;

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

    for component in components {
        namespace = namespace.field(component_query_field(&component));
    }

    for entity_type in entity_types {
        namespace = namespace.field(entity_query_field(&entity_type));
    }

    for relation_type in relation_types {
        namespace = namespace.field(relation_query_field(&relation_type));
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

    for entity_type in entity_types {
        if let Some(field) = entity_creation_field(&entity_type) {
            namespace = namespace.field(field);
            contains_field = true;
        }
        if let Some(field) = entity_mutation_field(&entity_type) {
            namespace = namespace.field(field);
            contains_field = true;
        }
    }

    for relation_type in relation_types {
        if let Some(field) = relation_creation_field(&relation_type) {
            namespace = namespace.field(field);
            contains_field = true;
        }
        if let Some(field) = relation_mutation_field(&relation_type) {
            namespace = namespace.field(field);
            contains_field = true;
        }
    }

    if contains_field {
        Some(namespace)
    } else {
        None
    }
}
