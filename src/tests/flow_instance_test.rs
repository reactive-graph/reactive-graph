use uuid::Uuid;

use crate::tests::utils::create_entity_instance_from_type;
use crate::tests::utils::r_string;
use crate::EntityTypeType;
use crate::FlowInstance;
use crate::NamespacedTypeGetter;

#[test]
fn flow_instance_test() {
    let namespace = r_string();
    let flow_id = Uuid::new_v4();
    let flow_type_name = r_string();
    let flow_name = r_string();
    let flow_description = r_string();

    let ty = EntityTypeType::new_from_type(&namespace, &flow_type_name);
    let flow_instance = FlowInstance {
        id: flow_id,
        ty: ty.clone(),
        name: flow_name.clone(),
        description: flow_description.to_string(),
        entity_instances: Vec::new(),
        relation_instances: Vec::new(),
    };

    assert_eq!(namespace, flow_instance.namespace());
    assert_eq!(flow_type_name.clone(), flow_instance.type_name());
    assert_eq!(flow_id.clone(), flow_instance.id.clone());
    assert_eq!(flow_name.clone(), flow_instance.name.clone());
    assert_eq!(flow_description.clone(), flow_instance.description.clone());
    assert_eq!(0, flow_instance.entity_instances.len());
    assert_eq!(0, flow_instance.relation_instances.len());
}

#[test]
fn flow_instance_from_entity_instance_test() {
    let namespace = r_string();
    let type_name = r_string();
    let wrapper_entity_instance = create_entity_instance_from_type(&namespace, &type_name);
    let flow_instance = FlowInstance::from(wrapper_entity_instance.clone());
    assert_eq!(namespace, flow_instance.namespace());
    assert_eq!(type_name.clone(), flow_instance.type_name());
    assert_eq!(wrapper_entity_instance.id, flow_instance.id);
    assert_eq!(String::new(), flow_instance.name);
}

#[test]
fn flow_instance_from_entity_instance_with_name_test() {
    let namespace = r_string();
    let type_name = r_string();
    let wrapper_entity_instance = create_entity_instance_from_type(&namespace, &type_name);
    let flow_name = r_string();
    let flow_instance = FlowInstance::from_instance_with_name(wrapper_entity_instance.clone(), flow_name.clone());
    assert_eq!(namespace, flow_instance.namespace());
    assert_eq!(type_name.clone(), flow_instance.type_name());
    assert_eq!(wrapper_entity_instance.id, flow_instance.id);
    assert_eq!(flow_name, flow_instance.name);
}
