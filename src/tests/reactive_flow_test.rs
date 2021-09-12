use crate::tests::utils::{
    create_random_entity_instance, create_random_entity_instance_with_type, r_string,
};
use crate::{Flow, ReactiveFlow};
use std::convert::{TryFrom, TryInto};
use std::sync::Arc;
use uuid::Uuid;

#[test]
fn reactive_flow_test() {
    let wrapper_entity_instance = Arc::new(create_random_entity_instance_with_type(
        "generic_flow",
        "test",
    ));
    let reactive_flow = ReactiveFlow::new(wrapper_entity_instance.clone());
    assert_eq!(wrapper_entity_instance.id, reactive_flow.id);
    assert_eq!(wrapper_entity_instance.type_name, reactive_flow.type_name);
    assert!(reactive_flow.has_entity(wrapper_entity_instance.clone()));
    assert!(reactive_flow.has_entity_by_id(wrapper_entity_instance.id));
    assert!(!reactive_flow.has_entity_by_id(Uuid::new_v4()));
    assert_eq!(
        wrapper_entity_instance.id,
        reactive_flow
            .get_entity(wrapper_entity_instance.id)
            .unwrap()
            .id
    );
    assert!(reactive_flow.get_entity(Uuid::new_v4()).is_none());
    assert_eq!(
        wrapper_entity_instance.id,
        reactive_flow.get_wrapper_entity_instance().unwrap().id
    );
    let second_entity_instance = Arc::new(create_random_entity_instance(r_string()));
    assert!(!reactive_flow.has_entity(second_entity_instance.clone()));
    assert!(!reactive_flow.has_entity_by_id(second_entity_instance.id));
    reactive_flow.add_entity(second_entity_instance.clone());
    assert_eq!(
        second_entity_instance.id,
        reactive_flow
            .get_entity(second_entity_instance.id)
            .unwrap()
            .id
    );
    assert!(reactive_flow.has_entity(second_entity_instance.clone()));
    assert!(reactive_flow.has_entity_by_id(second_entity_instance.id));
    assert_eq!(2, reactive_flow.entity_instances.read().unwrap().len());
    let third_entity_instance = Arc::new(create_random_entity_instance(r_string()));
    reactive_flow.add_entity(third_entity_instance.clone());
    assert_eq!(3, reactive_flow.entity_instances.read().unwrap().len());
    reactive_flow.remove_entity(third_entity_instance.id);
    assert_eq!(2, reactive_flow.entity_instances.read().unwrap().len());

    let reactive_flow_2 = ReactiveFlow::from(wrapper_entity_instance.clone());
    assert_eq!(wrapper_entity_instance.id, reactive_flow_2.id);

    let reactive_flow = Arc::new(reactive_flow);
    let flow: Flow = reactive_flow.clone().try_into().unwrap();
    assert_eq!(reactive_flow.id, flow.id);
    assert_eq!(reactive_flow.type_name, flow.type_name);
    assert_eq!(2, flow.entity_instances.len());
    assert_eq!(0, flow.relation_instances.len());
}

#[test]
fn reactive_flow_test_try_into() {
    let wrapper_entity_instance = Arc::new(create_random_entity_instance_with_type(
        "generic_flow",
        "test",
    ));
    let reactive_flow = ReactiveFlow::new(wrapper_entity_instance.clone());
    let flow: Flow = reactive_flow.try_into().unwrap();
    assert_eq!(wrapper_entity_instance.id, flow.id);
    assert_eq!(wrapper_entity_instance.type_name, flow.type_name);

    let reactive_flow_copy = ReactiveFlow::try_from(flow).unwrap();
    assert_eq!(wrapper_entity_instance.id, reactive_flow_copy.id);
    assert_eq!(
        wrapper_entity_instance.type_name,
        reactive_flow_copy.type_name
    );
}
