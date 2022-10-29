use indradb::EdgeKey;
use inexor_rgf_core_model::RelationInstanceTypeId;
use serde_json::json;
use uuid::Uuid;

use crate::model::NamespacedTypeGetter;
use crate::model::PropertyInstanceGetter;
use crate::model::RelationTypeId;
use crate::model::TypeDefinitionGetter;
use crate::tests::utils::r_string;
use crate::RelationInstanceBuilder;

#[test]
fn relation_instance_builder_test() {
    let namespace = r_string();
    let type_name = r_string();
    let rt_ty = RelationTypeId::new_from_type(&namespace, &type_name);
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let property_1_name = r_string();
    let property_1_value = r_string();
    let ri_ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
    let relation_instance = RelationInstanceBuilder::new(outbound_id, ri_ty.clone(), inbound_id)
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .build();
    assert_eq!(rt_ty, relation_instance.relation_type_id());
    assert_eq!(namespace, relation_instance.namespace());
    assert_eq!(type_name, relation_instance.type_name());
    let edge_key = EdgeKey::new(outbound_id, ri_ty.type_id(), inbound_id);
    assert_eq!(edge_key, relation_instance.get_key());
    assert_eq!(property_1_value.clone().as_str(), relation_instance.get(property_1_name.clone()).unwrap().as_str().unwrap());
    assert!(relation_instance.get(r_string()).is_none());
    let relation_instance_2 = RelationInstanceBuilder::new(outbound_id, ri_ty.clone(), inbound_id)
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .build();
    assert_eq!(namespace, relation_instance_2.namespace());
    assert_eq!(type_name, relation_instance_2.type_name());
    assert_eq!(ri_ty, relation_instance_2.ty);
}

#[test]
fn relation_instance_from_edge_key_test() {
    let namespace = r_string();
    let type_name = r_string();
    let ty = RelationTypeId::new_from_type(&namespace, &type_name);
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let property_1_name = r_string();
    let property_1_value = r_string();
    // let t = fully_qualified_identifier(&namespace, &type_name, &NAMESPACE_RELATION_TYPE);

    let edge_key = EdgeKey::new(outbound_id, ty.type_id(), inbound_id);
    let relation_instance = RelationInstanceBuilder::try_from(&edge_key)
        .unwrap()
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .build();
    assert_eq!(namespace, relation_instance.namespace());
    assert_eq!(type_name, relation_instance.type_name());
    assert_eq!(ty, relation_instance.relation_type_id());
    assert_eq!(edge_key, relation_instance.get_key());
    assert_eq!(property_1_value.clone().as_str(), relation_instance.get(property_1_name.clone()).unwrap().as_str().unwrap());
}
