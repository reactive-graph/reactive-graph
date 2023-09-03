// TODO: move unit test to plugin


use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::entity::const_value::{ConstValue, ConstValueProperties};
use crate::entity::const_value::ConstValueReactiveEntityFactory as Factory;
use crate::reactive::ReactiveEntity;
use crate::tests::utils::r_string;
use indradb::{VertexProperties, Type, NamedProperty, Vertex};
use uuid::Uuid;
use serde_json::json;
use std::str::FromStr;
use std::sync::Arc;

#[test]
fn const_value_test () {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let t = Type::from_str(type_name.as_str()).unwrap();
    let initial_property_value = 0;
    let property_value_json = json!(initial_property_value);
    let property = NamedProperty {
        name: ConstValueProperties::VALUE.to_string(),
        value: property_value_json
    };
    let properties = vec![
        property
    ];
    let vertex_properties = VertexProperties {
        vertex: Vertex { id: uuid, t: t.clone() },
        props: properties.clone()
    };
    // Create const value
    let const_value= ConstValue::from(vertex_properties);
    let entity_instance = const_value.entity.clone();
    // Retrieve number of cpus
    let cpus = num_cpus::get();
    // Set value of the constant value
    // const_value.internal_value.read().unwrap().send(&json!(cpus));
    const_value.set(&json!(cpus));
    // Read value from entity instance property
    // let value = entity_instance.properties.get("value").unwrap().get().as_u64().unwrap() as usize;
    let value = entity_instance.as_u64(ConstValueProperties::VALUE.to_string()).unwrap() as usize;
    // Check if entity instance properties has been set correctly
    assert_eq!(cpus, value);
    assert_ne!(initial_property_value, value);
    assert_eq!(cpus, const_value.get().unwrap());
    assert_ne!(initial_property_value, const_value.get().unwrap());
}

#[test]
fn create_const_value_test () {

    let uuid = Uuid::new_v4();

    let type_name = r_string();

    let t = Type::from_str(type_name.as_str()).unwrap();

    let initial_property_value = 0;

    let property_value_json = json!(initial_property_value);

    let property = NamedProperty {
        name: ConstValueProperties::VALUE.to_string(),
        value: property_value_json
    };

    let properties = vec![
        property
    ];

    let vertex_properties = VertexProperties {
        vertex: Vertex { id: uuid, t: t.clone() },
        props: properties.clone()

    };

    // Create const value
    let entity_instance = Arc::new(ReactiveEntity::from(vertex_properties));

    let const_value = ConstValue::from(Arc::clone(&entity_instance));
    // let const_value= ConstValue::from(vertex_properties);
    // let entity_instance = const_value.entity_instance.clone();

    // Retrieve number of cpus
    let cpus = num_cpus::get();

    // Set value of the constant value
    // const_value.internal_value.read().unwrap().send(&json!(cpus));
    const_value.set(&json!(cpus));

    // Read value from entity instance property
    // let value = entity_instance.properties.get("value").unwrap().get().as_u64().unwrap() as usize;
    let value = entity_instance.as_u64(ConstValueProperties::VALUE.to_string()).unwrap() as usize;

    // Check if entity instance properties has been set correctly
    assert_eq!(cpus, value);
    assert_ne!(initial_property_value, value);

    assert_eq!(cpus, const_value.get().unwrap());
    assert_ne!(initial_property_value, const_value.get().unwrap());

}

#[test]
fn const_value_connect_streams_test () {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let t = Type::from_str(type_name.as_str()).unwrap();
    let initial_property_value = 0;
    let property_value_json = json!(initial_property_value);
    let property = NamedProperty {
        name: ConstValueProperties::VALUE.to_string(),
        value: property_value_json
    };
    let properties = vec![
        property
    ];
    let vertex_properties = VertexProperties {
        vertex: Vertex { id: uuid, t: t.clone() },
        props: properties.clone()
    };
    let entity_instance_1 = Arc::new(ReactiveEntity::from(vertex_properties.clone()));
    let const_value_1 = ConstValue::from(entity_instance_1.clone());
    let entity_instance_2 = Arc::new(ReactiveEntity::from(vertex_properties.clone()));
    let const_value_2 = ConstValue::from(entity_instance_2.clone());
    let cpus = num_cpus::get();
    // Not connected
    const_value_1.set(&json!(cpus));
    assert_eq!(cpus, const_value_1.get().unwrap());
    assert_ne!(cpus, const_value_2.get().unwrap());
    assert_ne!(const_value_1.get().unwrap(), const_value_2.get().unwrap());
    // Connect
    let e = const_value_2.entity.clone();
    const_value_1
        .internal_value
        .read()
        .unwrap()
        .observe(move |v| {
            e.set(ConstValueProperties::VALUE.to_string(), v.clone());
        });
    // Connected, not it should propagate the changes from const_value_1 to const_value_2
    const_value_1.set(&json!(cpus));
    assert_eq!(cpus, const_value_1.get().unwrap());
    assert_eq!(cpus, const_value_2.get().unwrap());
    assert_eq!(const_value_1.get().unwrap(), const_value_2.get().unwrap());
}

#[test]
fn const_value_factory_test () {
    let entity_instance = Factory::new_default();
    assert_eq!(ConstValue::DEFAULT_TYPE_NAME, entity_instance.type_name.clone().as_str());
    let const_value = ConstValue::from(entity_instance.clone());
    let cpus = num_cpus::get();
    const_value.set(&json!(cpus))
}
