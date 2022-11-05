extern crate test;

use std::process::Termination;
use std::sync::Arc;
use std::sync::RwLock;
use test::Bencher;

use dashmap::DashMap;
use dashmap::DashSet;
use indradb::NamedProperty;
use indradb::Vertex;
use indradb::VertexProperties;
use serde_json::json;
use uuid::Uuid;

use crate::property_identifier;
use crate::tests::utils::create_random_entity_instance;
use crate::tests::utils::r_json_string;
use crate::tests::utils::r_string;
use crate::BehaviourTypeId;
use crate::Component;
use crate::ComponentContainer;
use crate::ComponentTypeId;
use crate::DataType;
use crate::EntityInstance;
use crate::EntityTypeId;
use crate::NamespacedTypeGetter;
use crate::PropertyInstanceGetter;
use crate::PropertyInstanceSetter;
use crate::PropertyType;
use crate::ReactiveBehaviourContainer;
use crate::ReactiveEntityInstance;
use crate::ReactivePropertyContainer;
use crate::ReactivePropertyInstance;
use crate::TypeDefinitionGetter;

#[test]
fn reactive_entity_instance_test() {
    let uuid = Uuid::new_v4();
    let namespace = r_string();
    let type_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let property_value = r_json_string();

    let properties = DashMap::new();
    properties.insert(
        property_name.clone(),
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), property_value.clone()),
    );

    let component_namespace = r_string();
    let component_name = r_string();
    let component_ty = ComponentTypeId::new_from_type(&component_namespace, &component_name);
    let component_name_2 = r_string();
    let component_ty_2 = ComponentTypeId::new_from_type(&component_namespace, &component_name_2);
    let components = DashSet::new();
    components.insert(component_ty.clone());

    let behaviour_name = r_string();
    let behaviour_name_2 = r_string();
    let behaviour_ty = BehaviourTypeId::new_from_type(&namespace, &behaviour_name);
    let behaviour_ty_2 = BehaviourTypeId::new_from_type(&namespace, &behaviour_name_2);
    let behaviours = DashSet::new();
    behaviours.insert(behaviour_ty.clone());

    let ty = EntityTypeId::new_from_type(&namespace, &type_name);
    let reactive_entity_instance = Arc::new(ReactiveEntityInstance {
        ty: ty.clone(),
        id: uuid.clone(),
        description: description.clone(),
        properties,
        components,
        behaviours,
    });
    assert_eq!(namespace.clone(), reactive_entity_instance.namespace());
    assert_eq!(type_name.clone(), reactive_entity_instance.type_name());
    assert_eq!(uuid.clone(), reactive_entity_instance.id.clone());
    assert_eq!(description.clone(), reactive_entity_instance.description.clone());

    assert_eq!(1, reactive_entity_instance.get_components().len());
    assert!(reactive_entity_instance.is_a(&component_ty));
    assert!(!reactive_entity_instance.is_a(&component_ty_2));
    assert!(!reactive_entity_instance.is_a(&ComponentTypeId::new_from_type(&component_namespace, &r_string())));
    reactive_entity_instance.add_component(component_ty_2.clone());
    assert!(reactive_entity_instance.is_a(&component_ty_2));
    assert_eq!(2, reactive_entity_instance.get_components().len());
    reactive_entity_instance.remove_component(&component_ty);
    assert!(!reactive_entity_instance.is_a(&component_ty));
    assert_eq!(1, reactive_entity_instance.get_components().len());

    let component_2_property_name = r_string();
    let component_2_properties = vec![PropertyType::string(&component_2_property_name)];
    let component_2_ty = ComponentTypeId::new_from_type(&namespace, &component_name);
    let component_2 = Component::new_without_extensions(component_2_ty.clone(), &r_string(), component_2_properties);
    reactive_entity_instance.add_component_with_properties(&component_2);
    assert_eq!(2, reactive_entity_instance.get_components().len());
    assert!(reactive_entity_instance.has_property(&component_2_property_name));

    assert!(reactive_entity_instance.behaves_as(&behaviour_ty));
    assert!(!reactive_entity_instance.behaves_as(&behaviour_ty_2));
    assert!(!reactive_entity_instance.behaves_as(&BehaviourTypeId::new_from_type(r_string(), r_string())));
    reactive_entity_instance.add_behaviour(behaviour_ty_2.clone());
    assert!(reactive_entity_instance.behaves_as(&behaviour_ty_2));
    reactive_entity_instance.remove_behaviour(&behaviour_ty);
    assert!(!reactive_entity_instance.behaves_as(&behaviour_ty));

    assert!(reactive_entity_instance.has_property(&property_name));
    let new_property_name = r_string();
    let new_property_value = json!(r_string());
    assert!(!reactive_entity_instance.has_property(&new_property_name));
    reactive_entity_instance.add_property(&new_property_name, new_property_value);
    assert!(reactive_entity_instance.has_property(&new_property_name));

    let new_property_name = r_string();
    let new_property_type = PropertyType::new(&new_property_name, DataType::Number);
    assert!(!reactive_entity_instance.has_property(&new_property_name));
    reactive_entity_instance.add_property_by_type(&new_property_type);
    assert!(reactive_entity_instance.has_property(&new_property_name));
    reactive_entity_instance.remove_property(&new_property_name);
    assert!(!reactive_entity_instance.has_property(&new_property_name));

    let entity_instance: EntityInstance = reactive_entity_instance.into();
    assert_eq!(namespace.clone(), entity_instance.namespace());
    assert_eq!(type_name.clone(), entity_instance.type_name());
    assert_eq!(uuid.clone(), entity_instance.id.clone());
    assert_eq!(description.clone(), entity_instance.description.clone());
    assert!(entity_instance.properties.contains_key(property_name.as_str()));
    assert_eq!(property_value, *entity_instance.properties.get(property_name.as_str()).unwrap());
}

#[test]
fn reactive_entity_instance_from_vertex_properties_test() {
    let uuid = Uuid::new_v4();
    let namespace = r_string();
    let type_name = r_string();
    let ty = EntityTypeId::new_from_type(&namespace, &type_name);
    let property_name = r_string();
    let property_value = r_string();
    let property_value_json = json!(property_value);
    let property = NamedProperty {
        name: property_identifier(&property_name),
        value: property_value_json.clone(),
    };
    let properties = vec![property];
    let vertex_properties = VertexProperties {
        vertex: Vertex { id: uuid, t: ty.type_id() },
        props: properties.clone(),
    };
    let reactive_entity_instance = Arc::new(ReactiveEntityInstance::try_from(vertex_properties).unwrap());
    assert_eq!(type_name.clone(), reactive_entity_instance.type_name());
    assert_eq!(uuid.clone(), reactive_entity_instance.id.clone());
    assert_eq!(property_name.clone(), reactive_entity_instance.properties.get(property_name.as_str()).unwrap().name);
    assert_eq!(property_value.clone(), reactive_entity_instance.properties.get(property_name.as_str()).unwrap().get());

    let entity_instance: EntityInstance = reactive_entity_instance.into();
    assert_eq!(type_name.clone(), entity_instance.type_name());
    assert_eq!(type_name.clone(), entity_instance.type_name());
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
    i.set(property_name.clone(), json!([1.23]));
    assert_eq!(vec![json!(1.23)], i.as_array(property_name.clone()).unwrap());
    i.set(
        property_name.clone(),
        json!({
            "test": 1.23
        }),
    );
    assert_eq!(1.23, i.as_object(property_name.clone()).unwrap().get("test").unwrap().as_f64().unwrap());
    let s = r_string();
    i.set(property_name.clone(), json!(s.clone()));
    assert_eq!(s, i.as_string(property_name.clone()).unwrap());
    assert_eq!(json!(s), i.get(property_name.clone()).unwrap());
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

#[test]
fn reactive_entity_instance_stream_test() {
    let namespace = r_string();
    let type_name = r_string();
    let ty = EntityTypeId::new_from_type(&namespace, &type_name);
    let reactive_entity_instance = Arc::new(ReactiveEntityInstance {
        ty: ty.clone(),
        id: Uuid::new_v4(),
        description: r_string(),
        properties: DashMap::new(),
        components: DashSet::new(),
        behaviours: DashSet::new(),
    });
    let property_name = r_string();
    let initial_property_value = r_string();
    let initial_outer_value = r_string();
    reactive_entity_instance.add_property(&property_name, json!(&initial_property_value));

    assert_eq!(initial_property_value, reactive_entity_instance.as_string(&property_name).unwrap());

    let handle_id = Uuid::new_v4().as_u128();

    // This is the important part:
    // The value can be shared by encapsulate in a RwLock which is encapsulated in an Arc
    let outer_value = Arc::new(RwLock::new(json!(&initial_outer_value)));

    assert_eq!(initial_outer_value.as_str(), outer_value.read().unwrap().as_str().unwrap());

    let inner_value = outer_value.clone();
    reactive_entity_instance.observe_with_handle(
        &property_name,
        move |v| {
            // Mutable writer on the rwlock
            let mut writer = inner_value.write().unwrap();
            *writer = v.clone();
            // Unlock write lock automatically when reaching end of the lambda scope
        },
        handle_id,
    );

    // Test tick(): The property value will be propagated to the stream
    // The observer will be called
    // The inner_value will be modified by the observer
    // The outer_value shares the same data as the inner_value
    // Expected: The reactive property value hasn't changed
    // Expected: The outer_value is equal to the original property value
    reactive_entity_instance.tick();
    assert_eq!(initial_property_value, reactive_entity_instance.as_string(&property_name).unwrap());
    // outer value must have changed after tick() because of the observer
    assert_eq!(initial_property_value, outer_value.read().unwrap().as_str().unwrap());

    // Test set_no_propagate(): The modified property value will be set but not propagated to the stream
    // No observer will be called
    // The inner_value wont be modified
    // The outer_value shares the same data as the inner_value
    // Expected: The reactive property value has changed
    // Expected: The outer_value didn't change
    let modified_value_1 = r_string();
    reactive_entity_instance.set_no_propagate(&property_name, json!(modified_value_1));
    assert_eq!(modified_value_1, reactive_entity_instance.as_string(&property_name).unwrap());
    // outer value must not have changed after set_no_propagate() because the observer wasn't called
    assert_eq!(initial_property_value, outer_value.read().unwrap().as_str().unwrap());

    // Test set(): The modified property value will be propagated to the stream
    // The observer will be called
    // The inner_value will be modified by the observer
    // The outer_value shares the same data as the inner_value
    // Expected: The reactive property value has changed
    // Expected: The outer_value is equal to the modified property value
    let modified_value = r_string();
    reactive_entity_instance.set(&property_name, json!(modified_value));
    assert_eq!(modified_value, reactive_entity_instance.as_string(&property_name).unwrap());
    // outer value must have changed after set() because of the observer
    assert_eq!(modified_value, outer_value.read().unwrap().as_str().unwrap());

    // Test set() after removing the observer: The modified property value will be propagated to the stream
    // No observer will be called
    // The inner_value wont be modified
    // The outer_value shares the same data as the inner_value
    // Expected: The reactive property value has changed
    // Expected: The outer_value hasn't changed
    reactive_entity_instance.remove_observer(&property_name, handle_id);
    let modified_value_2 = r_string();
    reactive_entity_instance.set(&property_name, json!(modified_value_2));
    assert_eq!(modified_value_2, reactive_entity_instance.as_string(&property_name).unwrap());
    // outer value must not be changed after set() because there is no observer anymore
    assert_eq!(modified_value, outer_value.read().unwrap().as_str().unwrap());
}

#[bench]
fn create_reactive_entity_instance_benchmark(bencher: &mut Bencher) -> impl Termination {
    let uuid = Uuid::new_v4();
    let namespace = r_string();
    let type_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let property_value = r_json_string();

    let ty = EntityTypeId::new_from_type(&namespace, &type_name);

    bencher.iter(move || {
        let properties = DashMap::new();
        properties.insert(
            property_name.clone(),
            ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), property_value.clone()),
        );

        let component_name = r_string();
        let component_ty = ComponentTypeId::new_from_type(&namespace, &component_name);
        let components = DashSet::new();
        components.insert(component_ty);

        let behaviour_name = r_string();
        let behaviour_ty = BehaviourTypeId::new_from_type(&namespace, &behaviour_name);
        let behaviours = DashSet::new();
        behaviours.insert(behaviour_ty);

        let _reactive_entity_instance = Arc::new(ReactiveEntityInstance {
            ty: ty.clone(),
            id: uuid.clone(),
            description: description.clone(),
            properties,
            components,
            behaviours,
        });
    })
}
