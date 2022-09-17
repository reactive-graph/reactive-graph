use crate::tests::utils::r_string;
use crate::{EntityTypeBuilder, ReactiveEntityInstanceBuilder, ReactiveRelationInstanceBuilder, RelationTypeBuilder};
use inexor_rgf_core_model::{DataType, PropertyInstanceGetter};
use serde_json::json;
use uuid::Uuid;

#[test]
fn reactive_relation_instance_builder_test() {
    let entity_type_name = r_string();
    let entity_type = EntityTypeBuilder::new(entity_type_name.clone()).build();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let outbound = ReactiveEntityInstanceBuilder::from(entity_type.clone()).id(outbound_id).get();
    let inbound = ReactiveEntityInstanceBuilder::from(entity_type.clone()).id(inbound_id).get();

    let type_name = r_string();
    let property_1_name = r_string();
    let property_1_value = r_string();
    let relation_instance = ReactiveRelationInstanceBuilder::new(outbound, type_name.clone(), inbound)
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .get();
    assert_eq!(type_name, relation_instance.type_name);
    assert_eq!(property_1_value.clone().as_str(), relation_instance.get(property_1_name.clone()).unwrap().as_str().unwrap());
    assert!(relation_instance.get(r_string()).is_none());
}

#[test]
fn reactive_relation_instance_builder_set_property_defaults_test() {
    let entity_type_name = r_string();
    let entity_type = EntityTypeBuilder::new(entity_type_name.clone()).build();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let outbound = ReactiveEntityInstanceBuilder::from(entity_type.clone()).id(outbound_id).get();
    let inbound = ReactiveEntityInstanceBuilder::from(entity_type.clone()).id(inbound_id).get();

    let type_name = r_string();
    let property_1_name = r_string();
    let property_2_name = r_string();
    let property_3_name = r_string();
    let property_3_value = r_string();
    let relation_type = RelationTypeBuilder::new(entity_type_name.clone(), type_name.clone(), entity_type_name.clone())
        .property(property_1_name.clone(), DataType::String)
        .property(property_2_name.clone(), DataType::Number)
        .property(property_3_name.clone(), DataType::String)
        .build();
    let relation_instance = ReactiveRelationInstanceBuilder::new(outbound, type_name.clone(), inbound)
        .set_properties_defaults(relation_type.clone())
        .property(property_3_name.clone(), json!(property_3_value.clone()))
        .get();
    assert_eq!(type_name, relation_instance.type_name);
    assert_eq!(DataType::String.default_value(), relation_instance.get(property_1_name.clone()).unwrap().as_str().unwrap());
    assert_eq!(DataType::Number.default_value(), relation_instance.get(property_2_name.clone()).unwrap().as_i64().unwrap());
    assert_eq!(property_3_value.clone().as_str(), relation_instance.get(property_3_name.clone()).unwrap().as_str().unwrap());
    assert!(relation_instance.get(r_string()).is_none());
}
