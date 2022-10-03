use indradb::EdgeKey;
use serde_json::json;
use uuid::Uuid;

use crate::model::fully_qualified_identifier;
use crate::model::PropertyInstanceGetter;
use crate::model::NAMESPACE_RELATION_TYPE;
use crate::tests::utils::r_string;
use crate::RelationInstanceBuilder;

#[test]
fn relation_instance_builder_test() {
    let namespace = r_string();
    let type_name = r_string();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let property_1_name = r_string();
    let property_1_value = r_string();
    let relation_instance = RelationInstanceBuilder::new(namespace.clone(), outbound_id, type_name.clone(), inbound_id)
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .build();
    assert_eq!(namespace, relation_instance.namespace);
    assert_eq!(type_name, relation_instance.type_name);
    let t = fully_qualified_identifier(&namespace, &type_name, &NAMESPACE_RELATION_TYPE);
    let edge_key = EdgeKey::new(outbound_id, t, inbound_id);
    assert_eq!(edge_key, relation_instance.get_key());
    assert_eq!(property_1_value.clone().as_str(), relation_instance.get(property_1_name.clone()).unwrap().as_str().unwrap());
    assert!(relation_instance.get(r_string()).is_none());
}

#[test]
fn relation_instance_from_edge_key_test() {
    let namespace = r_string();
    let type_name = r_string();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let property_1_name = r_string();
    let property_1_value = r_string();
    let t = fully_qualified_identifier(&namespace, &type_name, &NAMESPACE_RELATION_TYPE);
    let edge_key = EdgeKey::new(outbound_id, t, inbound_id);
    let relation_instance = RelationInstanceBuilder::from(edge_key.clone())
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .build();
    assert_eq!(namespace, relation_instance.namespace);
    assert_eq!(type_name, relation_instance.type_name);
    assert_eq!(edge_key, relation_instance.get_key());
    assert_eq!(property_1_value.clone().as_str(), relation_instance.get(property_1_name.clone()).unwrap().as_str().unwrap());
}
