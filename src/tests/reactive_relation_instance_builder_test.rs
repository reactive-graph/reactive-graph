use crate::tests::utils::r_string;
use crate::{
    EntityTypeBuilder, ReactiveEntityInstanceBuilder, ReactiveRelationInstanceBuilder,
    RelationTypeBuilder,
};
use inexor_rgf_core_model::{DataType, PropertyInstanceGetter};
use serde_json::json;
use uuid::Uuid;

#[test]
fn reactive_relation_instance_builder_test() {
    let entity_type_name = r_string();
    let entity_type = EntityTypeBuilder::new(entity_type_name.clone()).build();
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let outbound = ReactiveEntityInstanceBuilder::from(entity_type.clone())
        .id(outbound_id)
        .get();
    let inbound = ReactiveEntityInstanceBuilder::from(entity_type.clone())
        .id(inbound_id)
        .get();

    let type_name = r_string();
    let property_1_name = r_string();
    let property_1_value = r_string();
    let relation_instance =
        ReactiveRelationInstanceBuilder::new(outbound, type_name.clone(), inbound)
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
