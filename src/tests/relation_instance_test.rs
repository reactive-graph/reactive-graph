use std::collections::HashMap;
use std::str::FromStr;

use indradb::Edge;
use indradb::EdgeKey;
use indradb::EdgeProperties;
use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use uuid::Uuid;

use crate::property_identifier;
use crate::tests::utils::r_string;
use crate::tests::utils::r_string_1000;
use crate::tests::utils::r_string_255;
use crate::MutablePropertyInstanceSetter;
use crate::PropertyInstanceGetter;
use crate::RelationInstance;

#[test]
fn relation_instance_test() {
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let property_value = json!(r_string());
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), property_value.clone());
    let relation_instance = RelationInstance {
        outbound_id,
        type_name: type_name.clone(),
        inbound_id,
        description: description.to_string(),
        properties: properties.clone(),
    };
    assert_eq!(outbound_id.clone(), relation_instance.outbound_id.clone());
    assert_eq!(type_name.clone(), relation_instance.type_name.clone());
    assert_eq!(inbound_id.clone(), relation_instance.inbound_id.clone());
    assert_eq!(description.clone(), relation_instance.description.clone());
    assert_eq!(properties.clone(), relation_instance.properties.clone());
    assert!(relation_instance.get(property_name.clone()).is_some());
    assert!(relation_instance.get(r_string()).is_none());
    assert_eq!(property_value.clone(), relation_instance.get(property_name.clone()).unwrap());
}

#[test]
fn invalid_type_name_test() {
    let relation_instance = RelationInstance {
        outbound_id: Uuid::new_v4(),
        type_name: r_string_1000(),
        inbound_id: Uuid::new_v4(),
        description: r_string(),
        properties: HashMap::new(),
    };
    assert!(relation_instance.get_key().is_none());
}

#[test]
fn valid_type_name_test() {
    let relation_instance = RelationInstance {
        outbound_id: Uuid::new_v4(),
        type_name: r_string_255(),
        inbound_id: Uuid::new_v4(),
        description: r_string(),
        properties: HashMap::new(),
    };
    assert!(relation_instance.get_key().is_some());
}

#[test]
fn create_relation_instance_test() {
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let property_name = r_string();
    let property_value = json!(r_string());
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), property_value.clone());
    let relation_instance = RelationInstance::new(outbound_id, type_name.clone(), inbound_id, properties.clone());
    assert_eq!(outbound_id.clone(), relation_instance.outbound_id.clone());
    assert_eq!(type_name.clone(), relation_instance.type_name.clone());
    assert_eq!(inbound_id.clone(), relation_instance.inbound_id.clone());
    assert_eq!(properties.clone(), relation_instance.properties.clone());
    assert!(relation_instance.get(property_name.clone()).is_some());
    assert!(relation_instance.get(r_string()).is_none());
    assert_eq!(property_value.clone(), relation_instance.get(property_name.clone()).unwrap());
}

#[test]
fn create_relation_instance_without_properties_test() {
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let relation_instance = RelationInstance::new_without_properties(outbound_id, type_name.clone(), inbound_id);
    assert_eq!(outbound_id.clone(), relation_instance.outbound_id.clone());
    assert_eq!(type_name.clone(), relation_instance.type_name.clone());
    assert_eq!(inbound_id.clone(), relation_instance.inbound_id.clone());
    assert_eq!(0, relation_instance.properties.len());
}

#[test]
fn create_relation_instance_from_edge_properties() {
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let t = Identifier::from_str(type_name.as_str()).unwrap();
    let property_name = r_string();
    let property_value = r_string();
    let property_value_json = json!(property_value);
    let property = NamedProperty {
        name: property_identifier(&property_name),
        value: property_value_json,
    };
    let properties = vec![property];
    let edge_key = EdgeKey::new(outbound_id, t, inbound_id);
    let edge_properties = EdgeProperties::new(Edge::new_with_current_datetime(edge_key), properties.clone());
    let relation_instance = RelationInstance::from(edge_properties);
    assert_eq!(outbound_id.clone(), relation_instance.outbound_id.clone());
    assert_eq!(type_name.clone(), relation_instance.type_name.clone());
    assert_eq!(inbound_id.clone(), relation_instance.inbound_id.clone());
    assert_eq!(property_value.as_str(), relation_instance.properties.get(property_name.as_str()).unwrap().as_str().unwrap());
}

#[test]
fn relation_instance_typed_getter_test() {
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let property_name = r_string();
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), json!(false));
    let mut i = RelationInstance::new(outbound_id, type_name.clone(), inbound_id, properties.clone());
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
    i.set(property_name.clone(), json!([]));
    assert_eq!(0, i.as_array(property_name.clone()).unwrap().len());
    i.set(property_name.clone(), json!({}));
    assert_eq!(0, i.as_object(property_name.clone()).unwrap().len());
}

#[test]
fn relation_instance_get_key_test() {
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let description = r_string();
    let properties = HashMap::new();
    let relation_instance = RelationInstance {
        outbound_id,
        type_name: type_name.clone(),
        inbound_id,
        description: description.to_string(),
        properties: properties.clone(),
    };
    let edge_key = relation_instance.get_key();
    assert!(edge_key.is_some());
    assert_eq!(EdgeKey::new(outbound_id, Identifier::new(type_name.clone()).unwrap(), inbound_id), edge_key.unwrap());
}
