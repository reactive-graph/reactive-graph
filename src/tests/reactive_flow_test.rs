use std::convert::TryFrom;
use std::convert::TryInto;
use std::sync::Arc;

use serde_json::json;
use uuid::Uuid;

use crate::tests::utils::{create_random_entity_instance, create_random_entity_instance_with_type, create_random_relation_instance, r_string};
use crate::Flow;
use crate::PropertyInstanceGetter;
use crate::PropertyInstanceSetter;
use crate::ReactiveFlow;

#[test]
fn reactive_flow_test() {
    let wrapper_entity_instance = Arc::new(create_random_entity_instance_with_type("generic_flow", "test"));
    let reactive_flow = ReactiveFlow::new(wrapper_entity_instance.clone());
    assert_eq!(wrapper_entity_instance.id, reactive_flow.id);
    assert_eq!(wrapper_entity_instance.type_name, reactive_flow.type_name);
    assert!(reactive_flow.has_entity(wrapper_entity_instance.clone()));
    assert!(reactive_flow.has_entity_by_id(wrapper_entity_instance.id));
    assert!(!reactive_flow.has_entity_by_id(Uuid::new_v4()));
    assert_eq!(wrapper_entity_instance.id, reactive_flow.get_entity(wrapper_entity_instance.id).unwrap().id);
    assert!(reactive_flow.get_entity(Uuid::new_v4()).is_none());
    assert_eq!(wrapper_entity_instance.id, reactive_flow.get_wrapper_entity_instance().unwrap().id);
    let second_entity_instance = Arc::new(create_random_entity_instance(r_string()));
    assert!(!reactive_flow.has_entity(second_entity_instance.clone()));
    assert!(!reactive_flow.has_entity_by_id(second_entity_instance.id));

    // Property accessors (wrapper entity instance)
    assert!(reactive_flow.get("test").is_some());
    assert!(reactive_flow.get(r_string()).is_none());
    wrapper_entity_instance.set("test", json!("def"));
    assert_eq!("def", reactive_flow.get("test").unwrap().as_str().unwrap());
    assert_eq!("def", reactive_flow.as_string("test").unwrap());
    reactive_flow.set("test", json!("ghi"));
    assert_eq!("ghi", reactive_flow.as_string("test").unwrap());
    reactive_flow.set("test", json!(1));
    assert_eq!(1, reactive_flow.as_i64("test").unwrap());
    reactive_flow.set("test", json!(1.1));
    assert_eq!(1.1, reactive_flow.as_f64("test").unwrap());
    reactive_flow.set("test", json!(2));
    assert_eq!(2, reactive_flow.as_u64("test").unwrap());
    reactive_flow.set_no_propagate("test", json!(false));
    assert_eq!(false, reactive_flow.as_bool("test").unwrap());

    reactive_flow.add_entity(second_entity_instance.clone());
    assert_eq!(second_entity_instance.id, reactive_flow.get_entity(second_entity_instance.id).unwrap().id);
    assert!(reactive_flow.has_entity(second_entity_instance.clone()));
    assert!(reactive_flow.has_entity_by_id(second_entity_instance.id));
    assert_eq!(2, reactive_flow.entity_instances.read().unwrap().len());
    let third_entity_instance = Arc::new(create_random_entity_instance(r_string()));
    reactive_flow.add_entity(third_entity_instance.clone());
    assert_eq!(3, reactive_flow.entity_instances.read().unwrap().len());
    reactive_flow.remove_entity(third_entity_instance.id);
    assert_eq!(2, reactive_flow.entity_instances.read().unwrap().len());
    let fourth_entity_instance = Arc::new(create_random_entity_instance(r_string()));
    reactive_flow.add_entity(fourth_entity_instance.clone());
    assert_eq!(3, reactive_flow.entity_instances.read().unwrap().len());

    let relation_instance = Arc::new(create_random_relation_instance(second_entity_instance.clone(), fourth_entity_instance.clone(), r_string()));
    assert_eq!(0, reactive_flow.relation_instances.read().unwrap().len());
    reactive_flow.add_relation(relation_instance.clone());
    assert_eq!(1, reactive_flow.relation_instances.read().unwrap().len());
    assert!(reactive_flow.has_relation(relation_instance.clone()));
    assert!(reactive_flow.has_relation_by_key(relation_instance.get_key().unwrap()));
    let second_relation_instance = Arc::new(create_random_relation_instance(wrapper_entity_instance.clone(), second_entity_instance.clone(), r_string()));
    reactive_flow.add_relation(second_relation_instance.clone());
    assert_eq!(2, reactive_flow.relation_instances.read().unwrap().len());
    assert!(reactive_flow.has_relation(second_relation_instance.clone()));
    assert!(reactive_flow.has_relation_by_key(second_relation_instance.get_key().unwrap()));
    assert_eq!(
        second_relation_instance.get_key().unwrap(),
        reactive_flow
            .get_relation(second_relation_instance.get_key().unwrap())
            .unwrap()
            .get_key()
            .unwrap()
    );
    reactive_flow.remove_relation(second_relation_instance.get_key().unwrap());
    assert_eq!(1, reactive_flow.relation_instances.read().unwrap().len());
    assert!(!reactive_flow.has_relation(second_relation_instance.clone()));
    assert!(!reactive_flow.has_relation_by_key(second_relation_instance.get_key().unwrap()));

    let reactive_flow_2 = ReactiveFlow::from(wrapper_entity_instance.clone());
    assert_eq!(wrapper_entity_instance.id, reactive_flow_2.id);

    let reactive_flow = Arc::new(reactive_flow);
    let flow: Flow = reactive_flow.clone().try_into().unwrap();
    assert_eq!(reactive_flow.id, flow.id);
    assert_eq!(reactive_flow.type_name, flow.type_name);

    assert_eq!(3, flow.entity_instances.len());
    assert_eq!(1, flow.relation_instances.len());
    let flow_str = serde_json::to_string_pretty(&flow).unwrap_or("Failed".into());
    println!("{}", flow_str);
}

#[test]
fn reactive_flow_test_try_into() {
    let wrapper_entity_instance = Arc::new(create_random_entity_instance_with_type("generic_flow", "test"));
    let reactive_flow = ReactiveFlow::new(wrapper_entity_instance.clone());
    let second_entity_instance = Arc::new(create_random_entity_instance(r_string()));
    let third_entity_instance = Arc::new(create_random_entity_instance(r_string()));
    reactive_flow.add_entity(second_entity_instance.clone());
    reactive_flow.add_entity(third_entity_instance.clone());
    let relation_instance = Arc::new(create_random_relation_instance(second_entity_instance.clone(), third_entity_instance.clone(), r_string()));
    reactive_flow.add_relation(relation_instance.clone());

    let flow: Flow = reactive_flow.try_into().unwrap();
    assert_eq!(wrapper_entity_instance.id, flow.id);
    assert_eq!(wrapper_entity_instance.type_name, flow.type_name);

    let reactive_flow_copy = ReactiveFlow::try_from(flow).unwrap();
    assert_eq!(wrapper_entity_instance.id, reactive_flow_copy.id);
    assert_eq!(wrapper_entity_instance.type_name, reactive_flow_copy.type_name);
}
