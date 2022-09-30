use inexor_rgf_core_model::TypeContainer;
use std::env;

use crate::model::{DataType, EntityType, PropertyType};
use crate::tests::utils::application::init_application;
use crate::tests::utils::r_string;

#[test]
fn test_register_entity_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();

    let namespace = r_string();
    let type_name = r_string();
    let description = r_string();

    entity_type_manager.register(EntityType::new(
        &namespace,
        &type_name,
        &description,
        vec![String::from("positionable")],
        vec![crate::model::PropertyType::new(String::from("x"), DataType::String)],
        vec![],
    ));
    assert!(entity_type_manager.has(type_name.as_str()));

    let entity_type: Option<EntityType> = entity_type_manager.get(type_name.as_str());
    assert_eq!(type_name, entity_type.unwrap().name);
}

#[test]
fn test_create_and_delete_entity_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();

    let namespace = r_string();
    let type_name = r_string();
    let description = r_string();

    entity_type_manager.create(
        namespace.as_str(),
        type_name.as_str(),
        description.as_str(),
        vec![String::from("positionable")],
        vec![PropertyType::new(String::from("x"), DataType::String)],
        vec![],
    );
    assert!(entity_type_manager.has(type_name.as_str()));

    let entity_type: Option<EntityType> = entity_type_manager.get(type_name.as_str());
    assert_eq!(type_name, entity_type.unwrap().name);

    entity_type_manager.delete(type_name.as_str());

    assert!(!entity_type_manager.has(type_name.as_str()));

    let entity_type: Option<EntityType> = entity_type_manager.get(type_name.as_str());
    assert!(entity_type.is_none());
}

#[test]
fn test_get_entity_types() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let namespace = r_string();
    let type_name = r_string();
    let description = r_string();
    entity_type_manager.create(namespace.as_str(), type_name.as_str(), description.as_str(), vec![], vec![], vec![]);
    let entity_types = entity_type_manager.get_entity_types();
    assert_eq!(1, entity_types.len());
    for entity_type in entity_types {
        assert!(entity_type_manager.has(entity_type.name.as_str()));
    }
}

#[test]
fn test_register_entity_type_has_component() {
    let application = init_application();
    let component_manager = application.get_component_manager();
    let entity_type_manager = application.get_entity_type_manager();

    let namespace = r_string();
    let component_name = r_string();

    component_manager.register(crate::model::Component::new(
        namespace.clone(),
        component_name.clone(),
        String::new(),
        vec![crate::model::PropertyType::new(String::from("x"), DataType::String)],
        Vec::new(),
    ));

    let entity_type_name = r_string();
    let description = r_string();

    entity_type_manager.register(crate::model::EntityType::new(
        &namespace,
        &entity_type_name,
        &description,
        vec![component_name.clone()],
        vec![crate::model::PropertyType::new(String::from("y"), DataType::String)],
        vec![],
    ));
    let entity_type: EntityType = entity_type_manager.get(entity_type_name.as_str()).unwrap();
    assert!(entity_type.components.contains(&component_name.clone()));
    assert!(entity_type.is_a(component_name.clone()));
}

#[test]
fn test_register_entity_type_has_property() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();

    let property_name = String::from("x");
    let property_type = PropertyType::new(property_name.clone(), DataType::String);

    let entity_type_name = r_string();
    let namespace = r_string();

    entity_type_manager.register(EntityType::new(
        namespace.clone(),
        entity_type_name.clone(),
        String::new(),
        vec![],
        vec![property_type],
        vec![],
    ));
    let entity_type: Option<EntityType> = entity_type_manager.get(entity_type_name.as_str());
    assert!(entity_type.unwrap().has_own_property(property_name.as_str()));
}

#[test]
fn test_export_import_entity_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();

    let namespace = r_string();
    let type_name = r_string();
    let description = r_string();

    let mut path = env::temp_dir();
    path.push(format!("{}.json", type_name));
    let path = path.into_os_string().into_string().unwrap();

    entity_type_manager.create(
        namespace.as_str(),
        type_name.as_str(),
        description.as_str(),
        vec![String::from("positionable")],
        vec![PropertyType::new(String::from("x"), DataType::String)],
        vec![],
    );
    entity_type_manager.export(type_name.as_str(), path.as_str());
    assert!(entity_type_manager.has(type_name.as_str()));
    entity_type_manager.delete(type_name.as_str());
    assert!(!entity_type_manager.has(type_name.as_str()));
    let result = entity_type_manager.import(path.as_str());
    assert!(entity_type_manager.has(type_name.as_str()));
    assert!(result.is_ok());
}
