use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use indradb::{NamedProperty, Type, Vertex, VertexProperties};
use serde_json::json;
use uuid::Uuid;

use crate::tests::utils::{create_random_entity_instance, r_json_string, r_string};
use crate::{EntityInstance, ReactiveEntityInstance, ReactivePropertyInstance};
use crate::{PropertyInstanceGetter, PropertyInstanceSetter};

#[test]
fn reactive_entity_instance_test() {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let property_value = r_json_string();

    let mut properties = HashMap::new();
    properties.insert(
        property_name.clone(),
        ReactivePropertyInstance::new(
            Uuid::new_v4(),
            property_name.clone(),
            property_value.clone(),
        ),
    );

    let reactive_entity_instance = Arc::new(ReactiveEntityInstance {
        type_name: type_name.clone(),
        id: uuid.clone(),
        description: description.clone(),
        properties,
    });
    assert_eq!(
        type_name.clone(),
        reactive_entity_instance.type_name.clone()
    );
    assert_eq!(uuid.clone(), reactive_entity_instance.id.clone());
    assert_eq!(
        description.clone(),
        reactive_entity_instance.description.clone()
    );

    let entity_instance: EntityInstance = reactive_entity_instance.into();
    assert_eq!(type_name.clone(), entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), entity_instance.id.clone());
    assert_eq!(description.clone(), entity_instance.description.clone());
    assert!(entity_instance
        .properties
        .contains_key(property_name.as_str()));
    assert_eq!(
        property_value,
        *entity_instance
            .properties
            .get(property_name.as_str())
            .unwrap()
    );
}

#[test]
fn reactive_entity_instance_from_vertex_properties_test() {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let t = Type::from_str(type_name.as_str()).unwrap();
    let property_name = r_string();
    let property_value = r_string();
    let property_value_json = json!(property_value);
    let property = NamedProperty {
        name: property_name.clone(),
        value: property_value_json.clone(),
    };
    let properties = vec![property];
    let vertex_properties = VertexProperties {
        vertex: Vertex {
            id: uuid,
            t: t.clone(),
        },
        props: properties.clone(),
    };
    let reactive_entity_instance = Arc::new(ReactiveEntityInstance::from(vertex_properties));
    assert_eq!(
        type_name.clone(),
        reactive_entity_instance.type_name.clone()
    );
    assert_eq!(uuid.clone(), reactive_entity_instance.id.clone());
    assert_eq!(
        property_name.clone(),
        reactive_entity_instance.properties[&property_name.clone()].name
    );
    assert_eq!(
        property_value.clone(),
        reactive_entity_instance.properties[&property_name.clone()].get()
    );

    let entity_instance: EntityInstance = reactive_entity_instance.into();
    assert_eq!(type_name.clone(), entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), entity_instance.id.clone());
    assert!(entity_instance
        .properties
        .contains_key(property_name.as_str()));
    assert_eq!(
        property_value_json,
        *entity_instance
            .properties
            .get(property_name.as_str())
            .unwrap()
    );
}

#[test]
fn reactive_entity_instance_typed_getter_test() {
    let property_name = r_string();
    let i = create_random_entity_instance(property_name.clone());
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
}

#[test]
fn reactive_entity_instance_typed_eq_bool_test() {
    let property_name = r_string();
    let instance1 = create_random_entity_instance(property_name.clone());
    let instance2 = create_random_entity_instance(property_name.clone());

    instance1.set(property_name.clone(), json!(false));
    instance2.set(property_name.clone(), json!(false));
    assert!(instance1.get(property_name.clone()) == instance2.get(property_name.clone()));

    instance1.set(property_name.clone(), json!(true));
    instance2.set(property_name.clone(), json!(false));
    assert!(instance1.get(property_name.clone()) != instance2.get(property_name.clone()));

    instance1.set(property_name.clone(), json!(false));
    instance2.set(property_name.clone(), json!(true));
    assert!(instance1.get(property_name.clone()) != instance2.get(property_name.clone()));

    instance1.set(property_name.clone(), json!(true));
    instance2.set(property_name.clone(), json!(true));
    assert!(instance1.get(property_name.clone()) == instance2.get(property_name.clone()));
}
