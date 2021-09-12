use crate::tests::utils::{
    create_random_entity_instance, create_random_entity_instance_with_type, r_string,
};
use crate::{ReactiveFlow, ReactiveFlowConstructionError};
use std::fmt::Display;
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

    let reactive_flow_2 = ReactiveFlow::from(wrapper_entity_instance.clone());
    assert_eq!(wrapper_entity_instance.id, reactive_flow_2.id);
}
