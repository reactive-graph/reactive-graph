use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use indradb::{Edge, EdgeKey, EdgeProperties, Identifier, NamedProperty};
use serde_json::json;
use uuid::Uuid;

use crate::tests::utils::create_random_entity_instance::create_random_entity_instance;
use crate::tests::utils::{r_json_string, r_string};
use crate::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::{ReactiveEntityInstance, ReactivePropertyInstance, ReactiveRelationInstance, RelationInstance};

#[test]
fn reactive_relation_instance_test() {
    let relation_type_name = r_string();
    let relation_description = r_string();
    let property_name = r_string();
    let property_value = r_json_string();
    let outbound_entity = Arc::new(create_random_entity_instance(property_name.clone()));
    let inbound_entity = Arc::new(create_random_entity_instance(property_name.clone()));

    let mut properties = HashMap::new();
    properties.insert(
        property_name.clone(),
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), property_value.clone()),
    );

    let reactive_relation_instance = Arc::new(ReactiveRelationInstance {
        outbound: outbound_entity.clone(),
        type_name: relation_type_name.clone(),
        inbound: inbound_entity.clone(),
        description: relation_description.clone(),
        properties,
    });
    assert_eq!(relation_type_name.clone(), reactive_relation_instance.type_name.clone());
    assert_eq!(outbound_entity.id, reactive_relation_instance.outbound.id);
    assert_eq!(inbound_entity.id, reactive_relation_instance.inbound.id);
    assert_eq!(relation_description.clone(), reactive_relation_instance.description.clone());

    let relation_instance: RelationInstance = reactive_relation_instance.into();
    assert_eq!(outbound_entity.id, relation_instance.outbound_id);
    assert_eq!(relation_type_name.clone(), relation_instance.type_name.clone());
    assert_eq!(inbound_entity.id, relation_instance.inbound_id);
    assert_eq!(relation_description.clone(), relation_instance.description.clone());
    assert!(relation_instance.properties.contains_key(property_name.as_str()));
    assert_eq!(property_value, *relation_instance.properties.get(property_name.as_str()).unwrap());
}

#[test]
fn reactive_relation_instance_from_edge_properties_test() {
    let outbound_id = Uuid::new_v4();
    let outbound_type_name = r_string();
    let outbound_description = r_string();
    let outbound_entity = Arc::new(ReactiveEntityInstance {
        type_name: outbound_type_name.clone(),
        id: outbound_id.clone(),
        description: outbound_description.clone(),
        properties: HashMap::new(),
    });

    let inbound_id = Uuid::new_v4();
    let inbound_type_name = r_string();
    let inbound_description = r_string();
    let inbound_entity = Arc::new(ReactiveEntityInstance {
        type_name: inbound_type_name.clone(),
        id: inbound_id.clone(),
        description: inbound_description.clone(),
        properties: HashMap::new(),
    });

    let relation_type_name = r_string();
    let relation_description = r_string();
    let reactive_relation_instance = Arc::new(ReactiveRelationInstance {
        outbound: outbound_entity.clone(), // Arc::clone -> Reference Counted
        type_name: relation_type_name.clone(),
        inbound: inbound_entity.clone(), // Arc::clone -> Reference Counted
        description: relation_description.clone(),
        properties: HashMap::new(),
    });

    assert_eq!(relation_type_name.clone(), reactive_relation_instance.type_name.clone());
    assert_eq!(outbound_id, reactive_relation_instance.outbound.id);
    assert_eq!(inbound_id, reactive_relation_instance.inbound.id);
    assert_eq!(relation_description.clone(), reactive_relation_instance.description.clone());

    let relation_instance: RelationInstance = reactive_relation_instance.into();
    assert_eq!(outbound_entity.id, relation_instance.outbound_id);
    assert_eq!(relation_type_name.clone(), relation_instance.type_name.clone());
    assert_eq!(inbound_entity.id, relation_instance.inbound_id);
    assert_eq!(relation_description.clone(), relation_instance.description.clone());
}

#[test]
fn reactive_relation_instance_typed_getter_test() {
    let property_name = r_string();
    let outbound_entity = Arc::new(create_random_entity_instance(property_name.clone()));
    let inbound_entity = Arc::new(create_random_entity_instance(property_name.clone()));
    let i = create_random_relation_instance(outbound_entity.clone(), inbound_entity.clone(), property_name.clone());
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
fn reactive_relation_instance_with_properties_typed_getter_test() {
    let property_name = r_string();
    let outbound_entity = Arc::new(create_random_entity_instance(property_name.clone()));
    let inbound_entity = Arc::new(create_random_entity_instance(property_name.clone()));
    let i = create_random_relation_instance_with_properties(outbound_entity.clone(), inbound_entity.clone(), property_name.clone());
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

fn create_random_relation_instance(
    outbound_entity: Arc<ReactiveEntityInstance>,
    inbound_entity: Arc<ReactiveEntityInstance>,
    property_name: String,
) -> ReactiveRelationInstance {
    let type_name = r_string();
    let t = Identifier::from_str(type_name.as_str()).unwrap();
    let property_value = r_string();
    let property_value_json = json!(property_value);
    let property = NamedProperty {
        name: Identifier::new(property_name).unwrap(),
        value: property_value_json,
    };
    let properties = vec![property];
    let outbound_id = outbound_entity.id;
    let inbound_id = inbound_entity.id;
    let edge_key = EdgeKey::new(outbound_id, t, inbound_id);
    let edge_properties = EdgeProperties::new(Edge::new_with_current_datetime(edge_key), properties.clone());
    let outbound_entity = outbound_entity.clone();
    let inbound_entity = outbound_entity.clone();
    ReactiveRelationInstance::from(outbound_entity, inbound_entity, edge_properties)
}

pub fn create_random_relation_instance_with_properties(
    outbound_entity: Arc<ReactiveEntityInstance>,
    inbound_entity: Arc<ReactiveEntityInstance>,
    property_name: String,
) -> ReactiveRelationInstance {
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), json!(r_string()));
    ReactiveRelationInstance::create_with_properties(outbound_entity.clone(), r_string(), inbound_entity.clone(), properties)
}
