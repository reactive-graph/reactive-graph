use std::env;

use crate::model::{DataType, EntityType, PropertyType};
use crate::tests::utils::application::init_application;
use crate::tests::utils::r_string;

#[test]
fn test_register_entity_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();

    let type_name = r_string();
    let group_name = r_string();

    entity_type_manager.register(crate::model::EntityType::new(
        type_name.clone(),
        group_name.clone(),
        vec![String::from("positionable")],
        vec![],
        vec![crate::model::PropertyType::new(
            String::from("x"),
            DataType::String,
        )],
        vec![],
    ));
    assert!(entity_type_manager.has(type_name.clone()));

    let entity_type: Option<EntityType> = entity_type_manager.get(type_name.clone());
    assert_eq!(type_name, entity_type.unwrap().name);
}

#[test]
fn test_create_and_delete_entity_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();

    let type_name = r_string();
    let group = r_string();

    entity_type_manager.create(
        type_name.clone(),
        group.clone(),
        vec![String::from("positionable")],
        vec![],
        vec![PropertyType::new(String::from("x"), DataType::String)],
        vec![],
    );
    assert!(entity_type_manager.has(type_name.clone()));

    let entity_type: Option<EntityType> = entity_type_manager.get(type_name.clone());
    assert_eq!(type_name, entity_type.unwrap().name);

    entity_type_manager.delete(type_name.clone());

    assert!(!entity_type_manager.has(type_name.clone()));

    let entity_type: Option<EntityType> = entity_type_manager.get(type_name.clone());
    assert!(entity_type.is_none());
}

#[test]
fn test_get_entity_types() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    entity_type_manager.create(r_string(), r_string(), vec![], vec![], vec![], vec![]);
    let entity_types = entity_type_manager.get_entity_types();
    assert_eq!(1, entity_types.len());
    for entity_type in entity_types {
        assert!(entity_type_manager.has(entity_type.name));
    }
}

#[test]
fn test_register_entity_type_has_component() {
    let application = init_application();
    let component_manager = application.get_component_manager();
    let entity_type_manager = application.get_entity_type_manager();

    let component_name = r_string();

    component_manager.register(crate::model::Component::new(
        component_name.clone(),
        vec![crate::model::PropertyType::new(
            String::from("x"),
            DataType::String,
        )],
    ));

    let entity_type_name = r_string();
    let group_name = r_string();

    entity_type_manager.register(crate::model::EntityType::new(
        entity_type_name.clone(),
        group_name.clone(),
        vec![component_name.clone()],
        vec![],
        vec![crate::model::PropertyType::new(
            String::from("y"),
            DataType::String,
        )],
        vec![],
    ));
    let entity_type: EntityType = entity_type_manager.get(entity_type_name.clone()).unwrap();
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
    let group_name = r_string();

    entity_type_manager.register(EntityType::new(
        entity_type_name.clone(),
        group_name.clone(),
        vec![],
        vec![],
        vec![property_type],
        vec![],
    ));
    let entity_type: Option<EntityType> = entity_type_manager.get(entity_type_name.clone());
    assert!(entity_type.unwrap().has_own_property(property_name.clone()));
}

#[test]
fn test_export_import_entity_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();

    let type_name = r_string();
    let group = r_string();

    let mut path = env::temp_dir();
    path.push(format!("{}.json", type_name));
    let path = path.into_os_string().into_string().unwrap();

    entity_type_manager.create(
        type_name.clone(),
        group.clone(),
        vec![String::from("positionable")],
        vec![],
        vec![PropertyType::new(String::from("x"), DataType::String)],
        vec![],
    );
    entity_type_manager.export(type_name.clone(), path.clone());
    assert!(entity_type_manager.has(type_name.clone()));
    entity_type_manager.delete(type_name.clone());
    assert!(!entity_type_manager.has(type_name.clone()));
    let result = entity_type_manager.import(path.clone());
    assert!(entity_type_manager.has(type_name.clone()));
    assert!(result.is_ok());
}
