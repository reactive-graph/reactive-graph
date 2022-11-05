use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use dashmap::DashMap;
use dashmap::DashSet;
use indradb::Edge;
use indradb::EdgeKey;
use indradb::EdgeProperties;
use indradb::NamedProperty;
use serde_json::json;
use uuid::Uuid;

use crate::property_identifier;
use crate::tests::utils::create_random_entity_instance::create_random_entity_instance;
use crate::tests::utils::r_json_string;
use crate::tests::utils::r_string;
use crate::BehaviourTypeId;
use crate::Component;
use crate::ComponentContainer;
use crate::ComponentTypeId;
use crate::DataType;
use crate::EntityTypeId;
use crate::NamespacedTypeGetter;
use crate::PropertyInstanceGetter;
use crate::PropertyInstanceSetter;
use crate::PropertyType;
use crate::ReactiveBehaviourContainer;
use crate::ReactiveEntityInstance;
use crate::ReactivePropertyContainer;
use crate::ReactivePropertyInstance;
use crate::ReactiveRelationInstance;
use crate::RelationInstance;
use crate::RelationInstanceTypeId;
use crate::RelationTypeId;
use crate::TypeDefinitionGetter;

#[test]
fn reactive_relation_instance_test() {
    let namespace = r_string();
    let relation_type_name = r_string();
    let relation_description = r_string();
    let property_name = r_string();
    let property_value = r_json_string();
    let outbound_entity = Arc::new(create_random_entity_instance(property_name.clone()));
    let inbound_entity = Arc::new(create_random_entity_instance(property_name.clone()));

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

    let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &relation_type_name);
    let reactive_relation_instance = Arc::new(ReactiveRelationInstance {
        outbound: outbound_entity.clone(),
        ty: ty.clone(),
        inbound: inbound_entity.clone(),
        description: relation_description.clone(),
        properties,
        components,
        behaviours,
    });
    assert_eq!(namespace.clone(), reactive_relation_instance.namespace());
    assert_eq!(relation_type_name.clone(), reactive_relation_instance.type_name());
    assert_eq!(outbound_entity.id, reactive_relation_instance.outbound.id);
    assert_eq!(inbound_entity.id, reactive_relation_instance.inbound.id);
    assert_eq!(relation_description.clone(), reactive_relation_instance.description.clone());

    assert_eq!(1, reactive_relation_instance.get_components().len());
    assert!(reactive_relation_instance.is_a(&component_ty));
    assert!(!reactive_relation_instance.is_a(&component_ty_2));
    assert!(!reactive_relation_instance.is_a(&ComponentTypeId::new_from_type(&component_namespace, &r_string())));
    reactive_relation_instance.add_component(component_ty_2.clone());
    assert!(reactive_relation_instance.is_a(&component_ty_2));
    assert_eq!(2, reactive_relation_instance.get_components().len());
    reactive_relation_instance.remove_component(&component_ty);
    assert!(!reactive_relation_instance.is_a(&component_ty));
    assert_eq!(1, reactive_relation_instance.get_components().len());

    let component_2_property_name = r_string();
    let component_2_properties = vec![PropertyType::string(&component_2_property_name)];
    let component_2_ty = ComponentTypeId::new_from_type(&namespace, &component_name);
    let component_2 = Component::new_without_extensions(component_2_ty.clone(), &r_string(), component_2_properties);
    reactive_relation_instance.add_component_with_properties(&component_2);
    assert_eq!(2, reactive_relation_instance.get_components().len());
    assert!(reactive_relation_instance.has_property(&component_2_property_name));

    assert!(reactive_relation_instance.behaves_as(&behaviour_ty));
    assert!(!reactive_relation_instance.behaves_as(&behaviour_ty_2));
    assert!(!reactive_relation_instance.behaves_as(&BehaviourTypeId::new_from_type(r_string(), r_string())));
    reactive_relation_instance.add_behaviour(behaviour_ty_2.clone());
    assert!(reactive_relation_instance.behaves_as(&behaviour_ty_2));
    reactive_relation_instance.remove_behaviour(&behaviour_ty);
    assert!(!reactive_relation_instance.behaves_as(&behaviour_ty));

    assert!(reactive_relation_instance.has_property(&property_name));
    let new_property_name = r_string();
    let new_property_value = json!(r_string());
    assert!(!reactive_relation_instance.has_property(&new_property_name));
    reactive_relation_instance.add_property(&new_property_name, new_property_value);
    assert!(reactive_relation_instance.has_property(&new_property_name));

    let new_property_name = r_string();
    let new_property_type = PropertyType::new(&new_property_name, DataType::Number);
    assert!(!reactive_relation_instance.has_property(&new_property_name));
    reactive_relation_instance.add_property_by_type(&new_property_type);
    assert!(reactive_relation_instance.has_property(&new_property_name));
    reactive_relation_instance.remove_property(&new_property_name);
    assert!(!reactive_relation_instance.has_property(&new_property_name));

    let relation_instance: RelationInstance = reactive_relation_instance.into();
    assert_eq!(outbound_entity.id, relation_instance.outbound_id);
    assert_eq!(namespace.clone(), relation_instance.namespace());
    assert_eq!(relation_type_name.clone(), relation_instance.type_name());
    assert_eq!(inbound_entity.id, relation_instance.inbound_id);
    assert_eq!(relation_description.clone(), relation_instance.description.clone());
    assert!(relation_instance.properties.contains_key(property_name.as_str()));
    assert_eq!(property_value, *relation_instance.properties.get(property_name.as_str()).unwrap());
}

#[test]
fn reactive_relation_instance_from_edge_properties_test() {
    let namespace = r_string();
    let outbound_id = Uuid::new_v4();
    let outbound_type_name = r_string();
    let outbound_description = r_string();
    let outbound_ty = EntityTypeId::new_from_type(&namespace, &outbound_type_name);
    let outbound_entity = Arc::new(ReactiveEntityInstance {
        ty: outbound_ty.clone(),
        id: outbound_id.clone(),
        description: outbound_description.clone(),
        properties: DashMap::new(),
        components: DashSet::new(),
        behaviours: DashSet::new(),
    });

    let inbound_id = Uuid::new_v4();
    let inbound_type_name = r_string();
    let inbound_description = r_string();
    let inbound_ty = EntityTypeId::new_from_type(&namespace, &inbound_type_name);
    let inbound_entity = Arc::new(ReactiveEntityInstance {
        ty: inbound_ty.clone(),
        id: inbound_id.clone(),
        description: inbound_description.clone(),
        properties: DashMap::new(),
        components: DashSet::new(),
        behaviours: DashSet::new(),
    });

    let relation_type_name = r_string();
    let relation_description = r_string();
    let relation_ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &relation_type_name);
    let reactive_relation_instance = Arc::new(ReactiveRelationInstance {
        outbound: outbound_entity.clone(), // Arc::clone -> Reference Counted
        ty: relation_ty.clone(),
        inbound: inbound_entity.clone(), // Arc::clone -> Reference Counted
        description: relation_description.clone(),
        properties: DashMap::new(),
        components: DashSet::new(),
        behaviours: DashSet::new(),
    });

    assert_eq!(namespace.clone(), reactive_relation_instance.namespace());
    assert_eq!(relation_type_name.clone(), reactive_relation_instance.type_name());
    assert_eq!(outbound_id, reactive_relation_instance.outbound.id);
    assert_eq!(inbound_id, reactive_relation_instance.inbound.id);
    assert_eq!(relation_description.clone(), reactive_relation_instance.description.clone());

    let relation_instance: RelationInstance = reactive_relation_instance.into();
    assert_eq!(outbound_entity.id, relation_instance.outbound_id);
    assert_eq!(namespace.clone(), relation_instance.namespace());
    assert_eq!(relation_type_name.clone(), relation_instance.type_name());
    assert_eq!(inbound_entity.id, relation_instance.inbound_id);
    assert_eq!(relation_description.clone(), relation_instance.description.clone());
}

#[test]
fn reactive_relation_instance_typed_getter_test() {
    let property_name = r_string();
    let outbound_entity = Arc::new(create_random_entity_instance(property_name.clone()));
    let inbound_entity = Arc::new(create_random_entity_instance(property_name.clone()));
    let i = create_random_relation_instance(outbound_entity.clone(), inbound_entity.clone(), property_name.clone());
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
fn reactive_relation_instance_with_properties_typed_getter_test() {
    let property_name = r_string();
    let outbound_entity = Arc::new(create_random_entity_instance(property_name.clone()));
    let inbound_entity = Arc::new(create_random_entity_instance(property_name.clone()));
    let i = create_random_relation_instance_with_properties(outbound_entity.clone(), inbound_entity.clone(), property_name.clone());
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

fn create_random_relation_instance(
    outbound_entity: Arc<ReactiveEntityInstance>,
    inbound_entity: Arc<ReactiveEntityInstance>,
    property_name: String,
) -> ReactiveRelationInstance {
    let namespace = r_string();
    let type_name = r_string();
    let ty = RelationTypeId::new_from_type(namespace, type_name);
    let property_value = r_string();
    let property_value_json = json!(property_value);
    let property = NamedProperty {
        name: property_identifier(&property_name),
        value: property_value_json,
    };
    let properties = vec![property];
    let outbound_id = outbound_entity.id;
    let inbound_id = inbound_entity.id;
    let edge_key = EdgeKey::new(outbound_id, ty.type_id(), inbound_id);
    let edge_properties = EdgeProperties::new(Edge::new_with_current_datetime(edge_key), properties.clone());
    let outbound_entity = outbound_entity.clone();
    let inbound_entity = outbound_entity.clone();
    ReactiveRelationInstance::new_from_properties(outbound_entity, inbound_entity, edge_properties).unwrap()
}

pub fn create_random_relation_instance_with_properties(
    outbound_entity: Arc<ReactiveEntityInstance>,
    inbound_entity: Arc<ReactiveEntityInstance>,
    property_name: String,
) -> ReactiveRelationInstance {
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(), json!(r_string()));
    ReactiveRelationInstance::new_from_type_with_properties(r_string(), outbound_entity.clone(), r_string(), inbound_entity.clone(), properties)
}

#[test]
fn reactive_relation_instance_stream_test() {
    let property_name_x = r_string();
    let outbound_entity = Arc::new(create_random_entity_instance(property_name_x.clone()));
    let inbound_entity = Arc::new(create_random_entity_instance(property_name_x.clone()));
    let reactive_relation_instance = create_random_relation_instance_with_properties(outbound_entity.clone(), inbound_entity.clone(), property_name_x.clone());

    let property_name = r_string();
    let initial_property_value = r_string();
    let initial_outer_value = r_string();
    reactive_relation_instance.add_property(&property_name, json!(&initial_property_value));

    assert_eq!(initial_property_value, reactive_relation_instance.as_string(&property_name).unwrap());

    let handle_id = Uuid::new_v4().as_u128();

    // This is the important part:
    // The value can be shared by encapsulate in a RwLock which is encapsulated in an Arc
    let outer_value = Arc::new(RwLock::new(json!(&initial_outer_value)));

    assert_eq!(initial_outer_value.as_str(), outer_value.read().unwrap().as_str().unwrap());

    let inner_value = outer_value.clone();
    reactive_relation_instance.observe_with_handle(
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
    reactive_relation_instance.tick();
    assert_eq!(initial_property_value, reactive_relation_instance.as_string(&property_name).unwrap());
    // outer value must have changed after tick() because of the observer
    assert_eq!(initial_property_value, outer_value.read().unwrap().as_str().unwrap());

    // Test set_no_propagate(): The modified property value will be set but not propagated to the stream
    // No observer will be called
    // The inner_value wont be modified
    // The outer_value shares the same data as the inner_value
    // Expected: The reactive property value has changed
    // Expected: The outer_value didn't change
    let modified_value_1 = r_string();
    reactive_relation_instance.set_no_propagate(&property_name, json!(modified_value_1));
    assert_eq!(modified_value_1, reactive_relation_instance.as_string(&property_name).unwrap());
    // outer value must not have changed after set_no_propagate() because the observer wasn't called
    assert_eq!(initial_property_value, outer_value.read().unwrap().as_str().unwrap());

    // Test set(): The modified property value will be propagated to the stream
    // The observer will be called
    // The inner_value will be modified by the observer
    // The outer_value shares the same data as the inner_value
    // Expected: The reactive property value has changed
    // Expected: The outer_value is equal to the modified property value
    let modified_value = r_string();
    reactive_relation_instance.set(&property_name, json!(modified_value));
    assert_eq!(modified_value, reactive_relation_instance.as_string(&property_name).unwrap());
    // outer value must have changed after set() because of the observer
    assert_eq!(modified_value, outer_value.read().unwrap().as_str().unwrap());

    // Test set() after removing the observer: The modified property value will be propagated to the stream
    // No observer will be called
    // The inner_value wont be modified
    // The outer_value shares the same data as the inner_value
    // Expected: The reactive property value has changed
    // Expected: The outer_value hasn't changed
    reactive_relation_instance.remove_observer(&property_name, handle_id);
    let modified_value_2 = r_string();
    reactive_relation_instance.set(&property_name, json!(modified_value_2));
    assert_eq!(modified_value_2, reactive_relation_instance.as_string(&property_name).unwrap());
    // outer value must not be changed after set() because there is no observer anymore
    assert_eq!(modified_value, outer_value.read().unwrap().as_str().unwrap());
}
