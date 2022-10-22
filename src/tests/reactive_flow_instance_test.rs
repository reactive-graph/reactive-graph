use std::convert::TryFrom;
use std::convert::TryInto;
use std::sync::Arc;

use serde_json::json;
use uuid::Uuid;

use crate::tests::utils::create_random_entity_instance;
use crate::tests::utils::create_random_entity_instance_with_type;
use crate::tests::utils::create_random_relation_instance;
use crate::tests::utils::r_string;
use crate::FlowInstance;
use crate::FlowInstanceCreationError;
use crate::FlowInstanceDao;
use crate::NamespacedTypeGetter;
use crate::PropertyInstanceGetter;
use crate::PropertyInstanceSetter;
use crate::ReactiveFlowInstance;

#[test]
fn reactive_flow_test() {
    let wrapper_entity_instance = Arc::new(create_random_entity_instance_with_type("namespace", "generic_flow", "test"));
    let reactive_flow_instance = ReactiveFlowInstance::new(wrapper_entity_instance.clone());
    assert_eq!(wrapper_entity_instance.id, reactive_flow_instance.id);
    assert_eq!(wrapper_entity_instance.namespace(), reactive_flow_instance.namespace());
    assert_eq!(wrapper_entity_instance.type_name(), reactive_flow_instance.type_name());
    assert!(reactive_flow_instance.has_entity(wrapper_entity_instance.clone()));
    assert!(reactive_flow_instance.has_entity_by_id(wrapper_entity_instance.id));
    assert!(!reactive_flow_instance.has_entity_by_id(Uuid::new_v4()));
    assert_eq!(wrapper_entity_instance.id, reactive_flow_instance.get_entity(wrapper_entity_instance.id).unwrap().id);
    assert!(reactive_flow_instance.get_entity(Uuid::new_v4()).is_none());
    assert_eq!(wrapper_entity_instance.id, reactive_flow_instance.get_wrapper_entity_instance().unwrap().id);
    let second_entity_instance = Arc::new(create_random_entity_instance(r_string()));
    assert!(!reactive_flow_instance.has_entity(second_entity_instance.clone()));
    assert!(!reactive_flow_instance.has_entity_by_id(second_entity_instance.id));

    // Property accessors (wrapper entity instance)
    assert!(reactive_flow_instance.get("test").is_some());
    assert!(reactive_flow_instance.get(r_string()).is_none());
    wrapper_entity_instance.set("test", json!("def"));
    assert_eq!("def", reactive_flow_instance.get("test").unwrap().as_str().unwrap());
    assert_eq!("def", reactive_flow_instance.as_string("test").unwrap());
    reactive_flow_instance.set("test", json!("ghi"));
    assert_eq!("ghi", reactive_flow_instance.as_string("test").unwrap());
    reactive_flow_instance.set("test", json!(1));
    assert_eq!(1, reactive_flow_instance.as_i64("test").unwrap());
    reactive_flow_instance.set("test", json!(1.1));
    assert_eq!(1.1, reactive_flow_instance.as_f64("test").unwrap());
    reactive_flow_instance.set("test", json!(2));
    assert_eq!(2, reactive_flow_instance.as_u64("test").unwrap());
    reactive_flow_instance.set("test", json!([1.23]));
    assert_eq!(vec![json!(1.23)], reactive_flow_instance.as_array("test").unwrap());
    reactive_flow_instance.set(
        "test",
        json!({
            "test": 1.23
        }),
    );
    assert_eq!(1.23, reactive_flow_instance.as_object("test").unwrap().get("test").unwrap().as_f64().unwrap());
    reactive_flow_instance.set_no_propagate("test", json!(false));
    assert_eq!(false, reactive_flow_instance.as_bool("test").unwrap());

    reactive_flow_instance.add_entity(second_entity_instance.clone());
    assert_eq!(second_entity_instance.id, reactive_flow_instance.get_entity(second_entity_instance.id).unwrap().id);
    assert!(reactive_flow_instance.has_entity(second_entity_instance.clone()));
    assert!(reactive_flow_instance.has_entity_by_id(second_entity_instance.id));
    assert_eq!(2, reactive_flow_instance.entity_instances.read().unwrap().len());
    let third_entity_instance = Arc::new(create_random_entity_instance(r_string()));
    reactive_flow_instance.add_entity(third_entity_instance.clone());
    assert_eq!(3, reactive_flow_instance.entity_instances.read().unwrap().len());
    reactive_flow_instance.remove_entity(third_entity_instance.id);
    assert_eq!(2, reactive_flow_instance.entity_instances.read().unwrap().len());
    let fourth_entity_instance = Arc::new(create_random_entity_instance(r_string()));
    reactive_flow_instance.add_entity(fourth_entity_instance.clone());
    assert_eq!(3, reactive_flow_instance.entity_instances.read().unwrap().len());

    let relation_instance = Arc::new(create_random_relation_instance(second_entity_instance.clone(), fourth_entity_instance.clone(), r_string()));
    assert_eq!(0, reactive_flow_instance.relation_instances.read().unwrap().len());
    reactive_flow_instance.add_relation(relation_instance.clone());
    assert_eq!(1, reactive_flow_instance.relation_instances.read().unwrap().len());
    assert!(reactive_flow_instance.has_relation(relation_instance.clone()));
    assert!(reactive_flow_instance.has_relation_by_key(&relation_instance.get_key()));
    let second_relation_instance = Arc::new(create_random_relation_instance(wrapper_entity_instance.clone(), second_entity_instance.clone(), r_string()));
    reactive_flow_instance.add_relation(second_relation_instance.clone());
    assert_eq!(2, reactive_flow_instance.relation_instances.read().unwrap().len());
    assert!(reactive_flow_instance.has_relation(second_relation_instance.clone()));
    assert!(reactive_flow_instance.has_relation_by_key(&second_relation_instance.get_key()));
    assert_eq!(
        second_relation_instance.get_key(),
        reactive_flow_instance.get_relation(&second_relation_instance.get_key()).unwrap().get_key()
    );
    reactive_flow_instance.remove_relation(&second_relation_instance.get_key());
    assert_eq!(1, reactive_flow_instance.relation_instances.read().unwrap().len());
    assert!(!reactive_flow_instance.has_relation(second_relation_instance.clone()));
    assert!(!reactive_flow_instance.has_relation_by_key(&second_relation_instance.get_key()));

    let reactive_flow_instance_2 = ReactiveFlowInstance::from(wrapper_entity_instance.clone());
    assert_eq!(wrapper_entity_instance.id, reactive_flow_instance_2.id);

    let reactive_flow_instance = Arc::new(reactive_flow_instance);
    let flow_instance: FlowInstance = reactive_flow_instance.clone().try_into().unwrap();
    assert_eq!(reactive_flow_instance.id, flow_instance.id);
    assert_eq!(reactive_flow_instance.namespace(), flow_instance.namespace());
    assert_eq!(reactive_flow_instance.type_name(), flow_instance.type_name());

    assert_eq!(3, flow_instance.entity_instances.len());
    assert_eq!(1, flow_instance.relation_instances.len());
    let flow_instance_dao: FlowInstanceDao = (&flow_instance).into();
    let flow_str = serde_json::to_string_pretty(&flow_instance_dao).unwrap_or("Failed".into());
    println!("{}", flow_str);
}

#[test]
fn reactive_flow_test_try_into() {
    let wrapper_entity_instance = Arc::new(create_random_entity_instance_with_type("namespace", "generic_flow", "test"));
    let reactive_flow_instance = ReactiveFlowInstance::new(wrapper_entity_instance.clone());
    let second_entity_instance = Arc::new(create_random_entity_instance(r_string()));
    let third_entity_instance = Arc::new(create_random_entity_instance(r_string()));
    reactive_flow_instance.add_entity(second_entity_instance.clone());
    reactive_flow_instance.add_entity(third_entity_instance.clone());
    let relation_instance = Arc::new(create_random_relation_instance(second_entity_instance.clone(), third_entity_instance.clone(), r_string()));
    reactive_flow_instance.add_relation(relation_instance.clone());

    let flow_instance: FlowInstance = reactive_flow_instance.try_into().unwrap();
    assert_eq!(wrapper_entity_instance.id, flow_instance.id);
    assert_eq!(wrapper_entity_instance.namespace(), flow_instance.namespace());
    assert_eq!(wrapper_entity_instance.type_name(), flow_instance.type_name());

    let reactive_flow_instance_copy = ReactiveFlowInstance::try_from(flow_instance).unwrap();
    assert_eq!(wrapper_entity_instance.id, reactive_flow_instance_copy.id);
    assert_eq!(wrapper_entity_instance.namespace(), reactive_flow_instance_copy.namespace());
    assert_eq!(wrapper_entity_instance.type_name(), reactive_flow_instance_copy.type_name());
}

#[test]
fn reactive_flow_test_try_fail() {
    let wrapper_entity_instance = Arc::new(create_random_entity_instance_with_type("namespace", "generic_flow", "test"));
    let mut reactive_flow_instance = ReactiveFlowInstance::new(wrapper_entity_instance.clone());
    // Poisoning by altering the id
    reactive_flow_instance.id = Uuid::new_v4();
    let result: Result<FlowInstance, FlowInstanceCreationError> = reactive_flow_instance.try_into();
    assert!(result.is_err());
}

#[test]
fn reactive_flow_test_try_fail_2() {
    let wrapper_entity_instance = Arc::new(create_random_entity_instance_with_type("namespace", "generic_flow", "test"));
    let reactive_flow_instance = Arc::new(ReactiveFlowInstance::new(wrapper_entity_instance.clone()));
    // Poisoning by removing the wrapper entity instance
    reactive_flow_instance.remove_entity(reactive_flow_instance.id);
    let result: Result<FlowInstance, FlowInstanceCreationError> = reactive_flow_instance.try_into();
    assert!(result.is_err());
}
