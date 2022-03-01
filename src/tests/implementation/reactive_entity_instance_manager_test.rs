// TODO: fix these unit test

use indradb::Datastore;
use serde_json::json;

use crate::builder::EntityTypeBuilder;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::PropertyInstanceGetter;
use crate::tests::utils::application::init_application;
use crate::tests::utils::r_json_string;
use crate::tests::utils::r_string;

#[test]
fn test_register_reactive_entity_instance() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let reactive_entity_instance_manager = application.get_reactive_entity_instance_manager();
    let graph_database = application.get_graph_database();
    let datastore = graph_database.get_datastore();

    let type_name = r_string();
    let property_name = r_string();
    let property_value = r_json_string();

    let entity_type = EntityTypeBuilder::new(type_name.clone()).string_property(property_name.clone()).build();
    entity_type_manager.register(entity_type.clone());

    // Check that we cannot create an entity instance with a type which doesn't exist
    let reactive_entity_instance = ReactiveEntityInstanceBuilder::new(type_name.clone())
        .property(property_name.clone(), property_value.clone())
        .get();
    reactive_entity_instance_manager.register_reactive_instance(reactive_entity_instance.clone());
    assert_eq!(1, datastore.get_vertex_count().unwrap());
    assert!(reactive_entity_instance_manager.has(reactive_entity_instance.id));

    let id = reactive_entity_instance.id;

    let o_r = reactive_entity_instance_manager.get(id);
    assert!(o_r.is_some());
    let r = o_r.unwrap();
    assert_eq!(id, r.id);
    assert!(r.properties.contains_key(property_name.as_str()));
    assert_eq!(json!(property_value.clone()), r.get(property_name.as_str()).unwrap());
}

#[test]
fn test_unregister_reactive_entity_instance() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let reactive_entity_instance_manager = application.get_reactive_entity_instance_manager();
    let graph_database = application.get_graph_database();
    let datastore = graph_database.get_datastore();

    let type_name = r_string();
    let property_name = r_string();
    let property_value = r_json_string();

    let entity_type = EntityTypeBuilder::new(type_name.clone()).string_property(property_name.clone()).build();
    entity_type_manager.register(entity_type.clone());

    // Check that we cannot create an entity instance with a type which doesn't exist
    let reactive_entity_instance = ReactiveEntityInstanceBuilder::new(type_name.clone())
        .property(property_name.clone(), property_value.clone())
        .get();
    reactive_entity_instance_manager.register_reactive_instance(reactive_entity_instance.clone());
    assert_eq!(1, datastore.get_vertex_count().unwrap());
    assert!(reactive_entity_instance_manager.has(reactive_entity_instance.id));
    reactive_entity_instance_manager.unregister_reactive_instance(reactive_entity_instance.id);
    assert_eq!(1, datastore.get_vertex_count().unwrap());
    assert!(!reactive_entity_instance_manager.has(reactive_entity_instance.id));
}

#[test]
fn test_not_register_twice_reactive_entity_instance() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let reactive_entity_instance_manager = application.get_reactive_entity_instance_manager();
    let graph_database = application.get_graph_database();
    let datastore = graph_database.get_datastore();

    let type_name = r_string();
    let property_name = r_string();
    let property_value = r_json_string();

    let entity_type = EntityTypeBuilder::new(type_name.clone()).string_property(property_name.clone()).build();
    entity_type_manager.register(entity_type.clone());

    // Check that we cannot create an entity instance with a type which doesn't exist
    let reactive_entity_instance = ReactiveEntityInstanceBuilder::new(type_name.clone())
        .property(property_name.clone(), property_value.clone())
        .get();
    assert_eq!(0, datastore.get_vertex_count().unwrap());
    reactive_entity_instance_manager.register_reactive_instance(reactive_entity_instance.clone());
    assert_eq!(1, datastore.get_vertex_count().unwrap());
    reactive_entity_instance_manager.register_reactive_instance(reactive_entity_instance.clone());
    assert_eq!(1, datastore.get_vertex_count().unwrap());
}
