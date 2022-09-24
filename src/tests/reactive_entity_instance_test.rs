extern crate test;

use std::process::Termination;
use std::str::FromStr;
use std::sync::Arc;
use test::Bencher;

use dashmap::DashMap;
use dashmap::DashSet;
use indradb::Identifier;
use indradb::NamedProperty;
use indradb::Vertex;
use indradb::VertexProperties;
use serde_json::json;
use uuid::Uuid;

use crate::tests::utils::{create_random_entity_instance, r_json_string, r_string};
use crate::ComponentContainer;
use crate::EntityInstance;
use crate::PropertyInstanceGetter;
use crate::PropertyInstanceSetter;
use crate::ReactiveBehaviourContainer;
use crate::ReactiveEntityInstance;
use crate::ReactivePropertyInstance;

#[test]
fn reactive_entity_instance_test() {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let property_value = r_json_string();

    let properties = DashMap::new();
    properties.insert(
        property_name.clone(),
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), property_value.clone()),
    );

    let component_name = r_string();
    let component_name_2 = r_string();
    let components = DashSet::new();
    components.insert(component_name.clone());

    let behaviour_name = r_string();
    let behaviour_name_2 = r_string();
    let behaviours = DashSet::new();
    behaviours.insert(behaviour_name.clone());

    let reactive_entity_instance = Arc::new(ReactiveEntityInstance {
        type_name: type_name.clone(),
        id: uuid.clone(),
        description: description.clone(),
        properties,
        components,
        behaviours,
    });
    assert_eq!(type_name.clone(), reactive_entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), reactive_entity_instance.id.clone());
    assert_eq!(description.clone(), reactive_entity_instance.description.clone());

    assert!(reactive_entity_instance.is_a(component_name.clone()));
    assert!(!reactive_entity_instance.is_a(component_name_2.clone()));
    assert!(!reactive_entity_instance.is_a(r_string()));
    reactive_entity_instance.add_component(component_name_2.clone());
    assert!(reactive_entity_instance.is_a(component_name_2.clone()));
    reactive_entity_instance.remove_component(component_name.clone());
    assert!(!reactive_entity_instance.is_a(component_name.clone()));

    assert!(reactive_entity_instance.behaves_as(behaviour_name.clone()));
    assert!(!reactive_entity_instance.behaves_as(behaviour_name_2.clone()));
    assert!(!reactive_entity_instance.behaves_as(r_string()));
    reactive_entity_instance.add_behaviour(behaviour_name_2.clone());
    assert!(reactive_entity_instance.behaves_as(behaviour_name_2.clone()));
    reactive_entity_instance.remove_behaviour(behaviour_name.clone());
    assert!(!reactive_entity_instance.behaves_as(behaviour_name.clone()));

    let entity_instance: EntityInstance = reactive_entity_instance.into();
    assert_eq!(type_name.clone(), entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), entity_instance.id.clone());
    assert_eq!(description.clone(), entity_instance.description.clone());
    assert!(entity_instance.properties.contains_key(property_name.as_str()));
    assert_eq!(property_value, *entity_instance.properties.get(property_name.as_str()).unwrap());
}

#[test]
fn reactive_entity_instance_from_vertex_properties_test() {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let t = Identifier::from_str(type_name.as_str()).unwrap();
    let property_name = r_string();
    let property_value = r_string();
    let property_value_json = json!(property_value);
    let property = NamedProperty {
        name: Identifier::new(&property_name).unwrap(),
        value: property_value_json.clone(),
    };
    let properties = vec![property];
    let vertex_properties = VertexProperties {
        vertex: Vertex { id: uuid, t: t.clone() },
        props: properties.clone(),
    };
    let reactive_entity_instance = Arc::new(ReactiveEntityInstance::from(vertex_properties));
    assert_eq!(type_name.clone(), reactive_entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), reactive_entity_instance.id.clone());
    assert_eq!(property_name.clone(), reactive_entity_instance.properties.get(property_name.as_str()).unwrap().name);
    assert_eq!(property_value.clone(), reactive_entity_instance.properties.get(property_name.as_str()).unwrap().get());

    let entity_instance: EntityInstance = reactive_entity_instance.into();
    assert_eq!(type_name.clone(), entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), entity_instance.id.clone());
    assert!(entity_instance.properties.contains_key(property_name.as_str()));
    assert_eq!(property_value_json, *entity_instance.properties.get(property_name.as_str()).unwrap());
}

#[test]
fn reactive_entity_instance_typed_getter_test() {
    let property_name = r_string();
    let i = create_random_entity_instance(property_name.clone());
    i.set(property_name.clone(), json!(true));
    assert!(i.as_bool(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(false));
    assert!(!i.as_bool(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(123));
    assert_eq!(123, i.as_u64(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(-123));
    assert_eq!(-123, i.as_i64(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(1.23));
    assert_eq!(1.23, i.as_f64(property_name.clone()).unwrap());
    let s = r_string();
    i.set(property_name.clone(), json!(s.clone()));
    assert_eq!(s, i.as_string(property_name.clone()).unwrap());
}

#[test]
fn reactive_entity_instance_typed_eq_bool_test() {
    let property_name = r_string();
    let instance1 = create_random_entity_instance(property_name.clone());
    let instance2 = create_random_entity_instance(property_name.clone());

    instance1.set(property_name.clone(), json!(false));
    instance2.set(property_name.clone(), json!(false));
    assert_eq!(instance1.get(property_name.clone()), instance2.get(property_name.clone()));

    instance1.set(property_name.clone(), json!(true));
    instance2.set(property_name.clone(), json!(false));
    assert_ne!(instance1.get(property_name.clone()), instance2.get(property_name.clone()));

    instance1.set(property_name.clone(), json!(false));
    instance2.set(property_name.clone(), json!(true));
    assert_ne!(instance1.get(property_name.clone()), instance2.get(property_name.clone()));

    instance1.set(property_name.clone(), json!(true));
    instance2.set(property_name.clone(), json!(true));
    assert_eq!(instance1.get(property_name.clone()), instance2.get(property_name.clone()));
}

#[bench]
fn create_reactive_entity_instance_benchmark(bencher: &mut Bencher) -> impl Termination {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let property_value = r_json_string();

    bencher.iter(move || {
        let properties = DashMap::new();
        properties.insert(
            property_name.clone(),
            ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), property_value.clone()),
        );

        let component_name = r_string();
        let components = DashSet::new();
        components.insert(component_name.clone());

        let behaviour_name = r_string();
        let behaviours = DashSet::new();
        behaviours.insert(behaviour_name.clone());

        let _reactive_entity_instance = Arc::new(ReactiveEntityInstance {
            type_name: type_name.clone(),
            id: uuid.clone(),
            description: description.clone(),
            properties,
            components,
            behaviours,
        });
    })
}
