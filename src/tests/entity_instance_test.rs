use std::collections::HashMap;
use std::ops::Index;
use std::str::FromStr;

use indradb::{NamedProperty, Type, Vertex, VertexProperties};
use serde_json::json;
use uuid::Uuid;

use crate::tests::utils::r_string;
use crate::EntityInstance;
use crate::{MutablePropertyInstanceSetter, PropertyInstanceGetter};

#[test]
fn entity_instance_test() {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let property_value = json!(r_string());
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), property_value.clone());
    let entity_instance = EntityInstance {
        type_name: type_name.clone(),
        id: uuid.clone(),
        description: description.to_string(),
        properties: properties.clone(),
    };
    assert_eq!(type_name.clone(), entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), entity_instance.id.clone());
    assert_eq!(description.clone(), entity_instance.description.clone());
    assert_eq!(properties.clone(), entity_instance.properties.clone());
    assert!(entity_instance.get(property_name.clone()).is_some());
    assert!(entity_instance.get(r_string()).is_none());
    assert_eq!(
        property_value.clone(),
        entity_instance.get(property_name.clone()).unwrap()
    );
}

#[test]
fn create_entity_instance_test() {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let property_name = r_string();
    let property_value = json!(r_string());
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), property_value.clone());
    let entity_instance = EntityInstance::new(type_name.clone(), uuid.clone(), properties.clone());
    assert_eq!(type_name.clone(), entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), entity_instance.id.clone());
    assert_eq!(properties.clone(), properties.clone());
    assert!(entity_instance.get(property_name.clone()).is_some());
    assert!(entity_instance.get(r_string()).is_none());
    assert_eq!(
        property_value.clone(),
        entity_instance.get(property_name.clone()).unwrap()
    );
}

#[test]
fn create_entity_instance_without_properties_test() {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let entity_instance = EntityInstance::new_without_properties(type_name.clone(), uuid.clone());
    assert_eq!(type_name.clone(), entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), entity_instance.id.clone());
    assert!(entity_instance.get(r_string()).is_none());
}

#[test]
fn create_entity_instance_from_vertex_properties() {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let t = Type::from_str(type_name.as_str()).unwrap();
    let property_name = r_string();
    let property_value = r_string();
    let property_value_json = json!(property_value);
    let property = NamedProperty {
        name: property_name.clone(),
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
    let entity_instance = EntityInstance::from(vertex_properties);
    assert_eq!(type_name.clone(), entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), entity_instance.id.clone());
    assert_eq!(
        property_value.as_str(),
        entity_instance
            .properties
            .get(property_name.as_str())
            .unwrap()
            .as_str()
            .unwrap()
    );
}

#[test]
fn entity_instance_typed_getter_test() {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let property_name = r_string();
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), json!(false));
    let mut i = EntityInstance::new(type_name.clone(), uuid.clone(), properties.clone());
    i.set(property_name.clone(), json!(true));
    assert!(i.as_bool(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(false));
    assert!(!i.as_bool(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(123));
    assert_eq!(123, i.as_u64(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(-123));
    assert_eq!(-123, i.as_i64(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(1.23));
    assert_eq!(1.23, i.as_f64(property_name.clone()).unwrap());
    let s = r_string();
    i.set(property_name.clone(), json!(s.clone()));
    assert_eq!(s, i.as_string(property_name.clone()).unwrap());
    let a = json!([1, 2, 3]);
    i.set(property_name.clone(), a.clone());
    assert_eq!(
        json!(1),
        i.as_array(property_name.clone()).unwrap().index(0).clone()
    );
    assert_eq!(
        json!(2),
        i.as_array(property_name.clone()).unwrap().index(1).clone()
    );
    assert_eq!(
        json!(3),
        i.as_array(property_name.clone()).unwrap().index(2).clone()
    );
    let o = json!({
        "k": "v"
    });
    i.set(property_name.clone(), o.clone());
    assert_eq!(
        json!("v"),
        i.as_object(property_name.clone())
            .unwrap()
            .index("k")
            .clone()
    );
}
