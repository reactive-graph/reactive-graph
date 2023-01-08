use async_graphql::dynamic::Object;
use log::warn;

use crate::graphql::dynamic::component_field;
use crate::graphql::dynamic::entity_field;
use crate::graphql::dynamic::relation_field;
use crate::graphql::dynamic::SchemaBuilderContext;

pub fn get_namespace(context: SchemaBuilderContext, namespace: &String) -> Option<Object> {
    let components = context.component_manager.get_by_namespace(&namespace);
    let entity_types = context.entity_type_manager.get_by_namespace(&namespace);
    let relation_types = context.relation_type_manager.get_by_namespace(&namespace);
    if components.is_empty() && entity_types.is_empty() && relation_types.is_empty() {
        warn!("Skip empty namespace {}", &namespace);
        return None;
    }
    let mut namespace = Object::new(namespace);

    for component in components {
        namespace = namespace.field(component_field(&component));
    }

    for entity_type in entity_types {
        namespace = namespace.field(entity_field(&entity_type));
    }

    for relation_type in relation_types {
        namespace = namespace.field(relation_field(&relation_type));
    }

    Some(namespace)
}
