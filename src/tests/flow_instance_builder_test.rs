use crate::tests::utils::r_string;
use crate::{EntityInstanceBuilder, FlowInstanceBuilder, RelationInstanceBuilder};
use serde_json::json;
use uuid::Uuid;

#[test]
fn flow_builder_test() {
    let name = r_string();
    let description = r_string();
    let id = Uuid::new_v4();
    let type_name_1 = r_string();
    let property_1_name = r_string();
    let property_1_value = r_string();
    let type_name_2 = r_string();
    let property_2_name = r_string();
    let property_2_value = r_string();
    let type_name_3 = r_string();
    let property_3_name = r_string();
    let property_3_value = r_string();
    let entity_instance_1 = EntityInstanceBuilder::new(type_name_1.clone())
        .id(id)
        .property(property_1_name.clone(), json!(property_1_value.clone()))
        .get();
    let entity_instance_2 = EntityInstanceBuilder::new(type_name_2.clone())
        .id(id)
        .property(property_2_name.clone(), json!(property_2_value.clone()))
        .get();
    let entity_instance_3 = EntityInstanceBuilder::new(type_name_3.clone())
        .id(id)
        .property(property_3_name.clone(), json!(property_3_value.clone()))
        .get();

    let rel_type_name = r_string();
    let relation_instance_1 = RelationInstanceBuilder::new(entity_instance_1.id, rel_type_name.clone(), entity_instance_2.id).get();
    let relation_instance_2 = RelationInstanceBuilder::new(entity_instance_1.id, rel_type_name.clone(), entity_instance_3.id).get();
    let relation_instance_3 = RelationInstanceBuilder::new(entity_instance_2.id, rel_type_name.clone(), entity_instance_3.id).get();

    let flow_instance = FlowInstanceBuilder::new(entity_instance_1.clone())
        .name(name.clone())
        .description(description.clone())
        .entity(entity_instance_2.clone())
        .entity(entity_instance_3.clone())
        .relation(relation_instance_1.clone())
        .relation(relation_instance_2.clone())
        .relation(relation_instance_3.clone())
        .get();
    assert_eq!(id, flow_instance.id);
    assert_eq!(name, flow_instance.name);
    assert_eq!(description, flow_instance.description);
    assert_eq!(type_name_1, flow_instance.type_name);
    assert_eq!(3, flow_instance.entity_instances.len());
    assert_eq!(3, flow_instance.relation_instances.len());
}
