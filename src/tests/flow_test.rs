use uuid::Uuid;

use crate::tests::utils::create_entity_instance_with_type;
use crate::tests::utils::{create_random_entity_instance_with_type, r_string};
use crate::Flow;

#[test]
fn flow_test() {
    let flow_id = Uuid::new_v4();
    let flow_type_name = r_string();
    let flow_name = r_string();
    let flow_description = r_string();

    let flow = Flow {
        id: flow_id,
        type_name: flow_type_name.clone(),
        name: flow_name.clone(),
        description: flow_description.to_string(),
        entity_instances: Vec::new(),
        relation_instances: Vec::new(),
    };

    assert_eq!(flow_type_name.clone(), flow.type_name.clone());
    assert_eq!(flow_id.clone(), flow.id.clone());
    assert_eq!(flow_name.clone(), flow.name.clone());
    assert_eq!(flow_description.clone(), flow.description.clone());
    assert_eq!(0, flow.entity_instances.len());
    assert_eq!(0, flow.relation_instances.len());
}

#[test]
fn flow_from_test() {
    let wrapper_entity_instance = create_entity_instance_with_type("generic_flow", "test");
    let flow = Flow::from(wrapper_entity_instance.clone());
    assert_eq!(wrapper_entity_instance.id, flow.id);
    assert_eq!("generic_flow", flow.type_name);
    assert_eq!(String::new(), flow.name);
}

#[test]
fn flow_from_instance_with_name_test() {
    let wrapper_entity_instance = create_entity_instance_with_type("generic_flow", "test");
    let flow_name = r_string();
    let flow = Flow::from_instance_with_name(wrapper_entity_instance.clone(), flow_name.clone());
    assert_eq!(wrapper_entity_instance.id, flow.id);
    assert_eq!("generic_flow", flow.type_name);
    assert_eq!(flow_name, flow.name);
}
