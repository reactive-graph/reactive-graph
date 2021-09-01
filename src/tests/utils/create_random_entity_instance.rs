use crate::model::ReactiveEntityInstance;
use uuid::Uuid;
use crate::tests::utils::r_string;
use indradb::{Type, NamedProperty, VertexProperties, Vertex};
use std::str::FromStr;
use serde_json::json;

pub fn create_random_entity_instance(property_name: String) -> ReactiveEntityInstance {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let t = Type::from_str(type_name.as_str()).unwrap();
    let property_value = r_string();
    let property_value_json = json!(property_value);
    let property = NamedProperty {
        name: property_name.clone(),
        value: property_value_json
    };
    let properties = vec![
        property
    ];
    let vertex_properties = VertexProperties {
        vertex: Vertex { id: uuid, t: t.clone() },
        props: properties.clone()
    };
    ReactiveEntityInstance::from(vertex_properties)
}
