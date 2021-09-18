use crate::tests::utils::r_string;
use crate::{RelationInstanceBuilder, RelationTypeBuilder};
use indradb::{EdgeKey, Type};
use inexor_rgf_core_model::PropertyInstanceGetter;
use serde_json::json;
use uuid::Uuid;

#[test]
fn relation_instance_builder_test() {
    let type_name = r_string();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let property_1_name = r_string();
    let property_1_value = r_string();
    let relation_instance =
        RelationInstanceBuilder::new(outbound_id, type_name.clone(), inbound_id)
            .property(property_1_name.clone(), json!(property_1_value.clone()))
            .get();
    assert_eq!(type_name, relation_instance.type_name);
    assert_eq!(
        property_1_value.clone().as_str(),
        relation_instance
            .get(property_1_name.clone())
            .unwrap()
            .as_str()
            .unwrap()
    );
    assert!(relation_instance.get(r_string()).is_none());
}

#[test]
fn relation_instance_from_edge_key_test() {
    let type_name = r_string();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let property_1_name = r_string();
    let property_1_value = r_string();
    let t = Type(type_name.clone());
    let edge_key = EdgeKey::new(outbound_id, t, inbound_id);
    let relation_instance = RelationInstanceBuilder::from(edge_key.clone())
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .get();
    assert_eq!(type_name, relation_instance.type_name);
    assert_eq!(
        property_1_value.clone().as_str(),
        relation_instance
            .get(property_1_name.clone())
            .unwrap()
            .as_str()
            .unwrap()
    );
}
