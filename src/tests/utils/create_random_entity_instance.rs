use crate::tests::utils::r_string;
use indradb::{Identifier, NamedProperty, Vertex, VertexProperties};
use inexor_rgf_core_model::ReactiveEntityInstance;
use serde_json::json;
use std::str::FromStr;
use uuid::Uuid;

pub fn create_random_entity_instance(property_name: String) -> ReactiveEntityInstance {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let t = Identifier::from_str(type_name.as_str()).unwrap();
    let property_value = r_string();
    let property_value_json = json!(property_value);
    let property = NamedProperty {
        name: Identifier::new(&property_name).unwrap(),
        value: property_value_json,
    };
    let properties = vec![property];
    let vertex_properties = VertexProperties {
        vertex: Vertex {
            id: uuid,
            t: t.clone(),
        },
        props: properties.clone(),
    };
    ReactiveEntityInstance::from(vertex_properties)
}
