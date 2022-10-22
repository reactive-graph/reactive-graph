use std::collections::HashMap;

use indradb::NamedProperty;
use indradb::Vertex;
use indradb::VertexProperties;
use serde_json::json;
use uuid::Uuid;

use crate::property_identifier;
use crate::tests::utils::r_string;
use crate::EntityInstance;
use crate::EntityTypeType;
use crate::ReactiveEntityInstance;
use crate::TypeDefinitionGetter;

pub fn create_random_entity_instance<S: Into<String>>(property_name: S) -> ReactiveEntityInstance {
    create_random_entity_instance_with_type(r_string(), r_string(), property_name.into())
}

pub fn create_random_entity_instance_with_type<S: Into<String>>(namespace: S, type_name: S, property_name: S) -> ReactiveEntityInstance {
    let ty = EntityTypeType::new_from_type(namespace, type_name);
    let uuid = Uuid::new_v4();
    let property_value = r_string();
    let property_value_json = json!(property_value);
    let property = NamedProperty {
        name: property_identifier(property_name.into()),
        value: property_value_json,
    };
    let properties = vec![property];
    let vertex_properties = VertexProperties {
        vertex: Vertex { id: uuid, t: ty.type_id() },
        props: properties.clone(),
    };
    ReactiveEntityInstance::try_from(vertex_properties).unwrap()
}

pub fn create_entity_instance<S: Into<String>>(property_name: S) -> EntityInstance {
    create_entity_instance_from_type_with_property(r_string(), r_string(), property_name.into())
}

pub fn create_entity_instance_from_type_with_property<S: Into<String>>(namespace: S, type_name: S, property_name: S) -> EntityInstance {
    create_entity_instance_with_property(EntityTypeType::new_from_type(namespace, type_name), property_name)
}

pub fn create_entity_instance_with_property<T: Into<EntityTypeType>, S: Into<String>>(ty: T, property_name: S) -> EntityInstance {
    let id = Uuid::new_v4();
    let property_value = json!(r_string());
    let mut properties = HashMap::new();
    properties.insert(property_name.into(), property_value);
    EntityInstance::new(ty, id, properties)
}

pub fn create_entity_instance_from_type<S: Into<String>>(namespace: S, type_name: S) -> EntityInstance {
    EntityInstance::new_from_type(namespace, type_name, Uuid::new_v4(), HashMap::new())
}
