use inexor_rgf_core_model::RelationInstanceTypeId;
use serde_json::json;
use uuid::Uuid;

use crate::model::DataType;
use crate::model::EntityTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyInstanceGetter;
use crate::model::RelationTypeId;
use crate::tests::utils::r_string;
use crate::EntityTypeBuilder;
use crate::ReactiveEntityInstanceBuilder;
use crate::ReactiveRelationInstanceBuilder;
use crate::RelationTypeBuilder;

#[test]
fn reactive_relation_instance_builder_test() {
    let entity_type_1_namespace = r_string();
    let entity_type_1_name = r_string();
    let entity_type_1_ty = EntityTypeId::new_from_type(&entity_type_1_namespace, &entity_type_1_name);
    let entity_type_1 = EntityTypeBuilder::new(entity_type_1_ty.clone()).build();
    let outbound_id = Uuid::new_v4();
    let outbound = ReactiveEntityInstanceBuilder::from(entity_type_1.clone()).id(outbound_id).build();

    let entity_type_2_namespace = r_string();
    let entity_type_2_name = r_string();
    let entity_type_2_ty = EntityTypeId::new_from_type(&entity_type_2_namespace, &entity_type_2_name);
    let entity_type_2 = EntityTypeBuilder::new_from_type(&entity_type_2_namespace, &entity_type_2_name).build();
    let inbound_id = Uuid::new_v4();
    let inbound = ReactiveEntityInstanceBuilder::from(entity_type_2.clone()).id(inbound_id).build();

    let relation_type_namespace = r_string();
    let relation_type_name = r_string();
    let rt_ty = RelationTypeId::new_from_type(&relation_type_namespace, &relation_type_name);
    let property_1_name = r_string();
    let property_1_value = r_string();
    let ri_ty = RelationInstanceTypeId::new_from_type_unique_id(&relation_type_namespace, &relation_type_name);
    let relation_instance = ReactiveRelationInstanceBuilder::new(outbound, ri_ty.clone(), inbound)
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .build();
    assert_eq!(relation_type_namespace, relation_instance.namespace());
    assert_eq!(relation_type_name, relation_instance.type_name());
    assert_eq!(ri_ty, relation_instance.ty);
    assert_eq!(rt_ty, relation_instance.relation_type_id());
    assert_eq!(entity_type_1_namespace, relation_instance.outbound.namespace());
    assert_eq!(entity_type_1_name, relation_instance.outbound.type_name());
    assert_eq!(entity_type_1_ty, relation_instance.outbound.ty);
    assert_eq!(entity_type_2_namespace, relation_instance.inbound.namespace());
    assert_eq!(entity_type_2_name, relation_instance.inbound.type_name());
    assert_eq!(entity_type_2_ty, relation_instance.inbound.ty);
    assert_eq!(property_1_value.clone().as_str(), relation_instance.get(property_1_name.clone()).unwrap().as_str().unwrap());
    assert!(relation_instance.get(r_string()).is_none());
}

#[test]
fn reactive_relation_instance_builder_set_property_defaults_test() {
    let entity_type_1_namespace = r_string();
    let entity_type_1_name = r_string();
    let entity_type_1_ty = EntityTypeId::new_from_type(&entity_type_1_namespace, &entity_type_1_name);
    let entity_type_1 = EntityTypeBuilder::new(entity_type_1_ty.clone()).build();
    let outbound_id = Uuid::new_v4();
    let outbound = ReactiveEntityInstanceBuilder::from(entity_type_1.clone()).id(outbound_id).build();

    let entity_type_2_namespace = r_string();
    let entity_type_2_name = r_string();
    let entity_type_2_ty = EntityTypeId::new_from_type(&entity_type_2_namespace, &entity_type_2_name);
    let entity_type_2 = EntityTypeBuilder::new_from_type(&entity_type_2_namespace, &entity_type_2_name).build();
    let inbound_id = Uuid::new_v4();
    let inbound = ReactiveEntityInstanceBuilder::from(entity_type_2.clone()).id(inbound_id).build();

    let relation_type_namespace = r_string();
    let relation_type_name = r_string();
    let rt_ty = RelationTypeId::new_from_type(&relation_type_namespace, &relation_type_name);
    let property_1_name = r_string();
    let property_2_name = r_string();
    let property_3_name = r_string();
    let property_3_value = r_string();
    let relation_type = RelationTypeBuilder::new(entity_type_1_ty.clone(), rt_ty.clone(), entity_type_2_ty.clone())
        .property(property_1_name.clone(), DataType::String)
        .property(property_2_name.clone(), DataType::Number)
        .property(property_3_name.clone(), DataType::String)
        .build();
    let ri_ty = RelationInstanceTypeId::new_unique_id(rt_ty.clone());
    let relation_instance = ReactiveRelationInstanceBuilder::new(outbound, ri_ty.clone(), inbound)
        .set_properties_defaults(relation_type.clone())
        .property(property_3_name.clone(), json!(property_3_value.clone()))
        .build();
    assert_eq!(relation_type_namespace, relation_instance.namespace());
    assert_eq!(relation_type_name, relation_instance.type_name());
    assert_eq!(ri_ty, relation_instance.ty);
    assert_eq!(rt_ty, relation_instance.relation_type_id());
    assert_eq!(entity_type_1_namespace, relation_instance.outbound.namespace());
    assert_eq!(entity_type_1_name, relation_instance.outbound.type_name());
    assert_eq!(entity_type_1_ty, relation_instance.outbound.ty);
    assert_eq!(entity_type_2_namespace, relation_instance.inbound.namespace());
    assert_eq!(entity_type_2_name, relation_instance.inbound.type_name());
    assert_eq!(entity_type_2_ty, relation_instance.inbound.ty);
    assert_eq!(DataType::String.default_value(), relation_instance.get(property_1_name.clone()).unwrap().as_str().unwrap());
    assert_eq!(DataType::Number.default_value(), relation_instance.get(property_2_name.clone()).unwrap().as_i64().unwrap());
    assert_eq!(property_3_value.clone().as_str(), relation_instance.get(property_3_name.clone()).unwrap().as_str().unwrap());
    assert!(relation_instance.get(r_string()).is_none());
}
