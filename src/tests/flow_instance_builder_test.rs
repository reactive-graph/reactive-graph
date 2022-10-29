use inexor_rgf_core_model::RelationInstanceTypeId;
use serde_json::json;
use uuid::Uuid;

use crate::model::EntityTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::RelationTypeId;
use crate::tests::utils::r_string;
use crate::EntityInstanceBuilder;
use crate::FlowInstanceBuilder;
use crate::RelationInstanceBuilder;

#[test]
fn flow_builder_test() {
    let flow_instance_name = r_string();
    let description = r_string();
    let id = Uuid::new_v4();

    let entity_type_1_namespace = r_string();
    let entity_type_1_name = r_string();
    let entity_type_1_ty = EntityTypeId::new_from_type(&entity_type_1_namespace, &entity_type_1_name);

    let property_1_name = r_string();
    let property_1_value = r_string();

    let entity_type_2_namespace = r_string();
    let entity_type_2_name = r_string();
    let entity_type_2_ty = EntityTypeId::new_from_type(&entity_type_2_namespace, &entity_type_2_name);

    let property_2_name = r_string();
    let property_2_value = r_string();

    let entity_type_3_namespace = r_string();
    let entity_type_3_name = r_string();
    let entity_type_3_ty = EntityTypeId::new_from_type(&entity_type_3_namespace, &entity_type_3_name);

    let property_3_name = r_string();
    let property_3_value = r_string();

    let entity_instance_1 = EntityInstanceBuilder::new(entity_type_1_ty.clone())
        .id(id)
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .build();
    let entity_instance_2 = EntityInstanceBuilder::new(entity_type_2_ty)
        .id(id)
        .property(property_2_name.clone(), json!(property_2_value.clone()))
        .build();
    let entity_instance_3 = EntityInstanceBuilder::new(entity_type_3_ty)
        .id(id)
        .property(property_3_name.clone(), json!(property_3_value.clone()))
        .build();

    let relation_type_1_namespace = r_string();
    let relation_type_1_name = r_string();
    let relation_type_1_ty = RelationTypeId::new_from_type(&relation_type_1_namespace, &relation_type_1_name);
    let relation_instance_1_ty = RelationInstanceTypeId::new_unique_id(relation_type_1_ty.clone());

    let relation_type_2_namespace = r_string();
    let relation_type_2_name = r_string();
    let relation_type_2_ty = RelationTypeId::new_from_type(&relation_type_2_namespace, &relation_type_2_name);
    let relation_instance_2_ty = RelationInstanceTypeId::new_unique_id(relation_type_2_ty.clone());

    let relation_type_3_namespace = r_string();
    let relation_type_3_name = r_string();
    let relation_type_3_ty = RelationTypeId::new_from_type(&relation_type_3_namespace, &relation_type_3_name);
    let relation_instance_3_ty = RelationInstanceTypeId::new_unique_id(relation_type_3_ty.clone());

    let relation_instance_1 = RelationInstanceBuilder::new(entity_instance_1.id, relation_instance_1_ty, entity_instance_2.id).build();
    let relation_instance_2 = RelationInstanceBuilder::new(entity_instance_1.id, relation_instance_2_ty, entity_instance_3.id).build();
    let relation_instance_3 = RelationInstanceBuilder::new(entity_instance_2.id, relation_instance_3_ty, entity_instance_3.id).build();

    let flow_instance = FlowInstanceBuilder::new(entity_instance_1.clone())
        .name(flow_instance_name.clone())
        .description(description.clone())
        .entity(entity_instance_2.clone())
        .entity(entity_instance_3.clone())
        .relation(relation_instance_1.clone())
        .relation(relation_instance_2.clone())
        .relation(relation_instance_3.clone())
        .build();
    assert_eq!(id, flow_instance.id);
    assert_eq!(flow_instance_name, flow_instance.name);
    assert_eq!(description, flow_instance.description);
    assert_eq!(entity_type_1_namespace, flow_instance.namespace());
    assert_eq!(entity_type_1_name, flow_instance.type_name());
    assert_eq!(entity_type_1_ty, flow_instance.ty);
    assert_eq!(3, flow_instance.entity_instances.len());
    assert_eq!(3, flow_instance.relation_instances.len());
}
