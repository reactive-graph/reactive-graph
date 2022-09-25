use inexor_rgf_core_model::TypeContainer;
use std::env;

use crate::builder::EntityTypeBuilder;
use crate::model::{DataType, PropertyType, RelationType};
use crate::tests::utils::application::init_application;
use crate::tests::utils::r_string;

#[test]
fn test_register_relation_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();

    let entity_type = EntityTypeBuilder::new(outbound_type_name.clone()).build();
    entity_type_manager.register(entity_type.clone());
    let entity_type = EntityTypeBuilder::new(inbound_type_name.clone()).build();
    entity_type_manager.register(entity_type.clone());
    relation_type_manager.register(crate::model::RelationType::new(
        outbound_type_name.clone(),
        type_name.clone(),
        inbound_type_name.clone(),
        String::new(),
        String::new(),
        vec![String::from("named")],
        vec![crate::model::PropertyType::new(String::from("x"), DataType::String)],
        Vec::new(),
    ));
    assert!(relation_type_manager.has(type_name.as_str()));

    let relation_type: Option<RelationType> = relation_type_manager.get(type_name.as_str());
    assert_eq!(type_name, relation_type.unwrap().type_name);
}

#[test]
fn test_create_and_delete_relation_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();

    let entity_type = EntityTypeBuilder::new(outbound_type_name.clone()).build();
    entity_type_manager.register(entity_type.clone());
    let entity_type = EntityTypeBuilder::new(inbound_type_name.clone()).build();
    entity_type_manager.register(entity_type.clone());
    relation_type_manager.create(
        outbound_type_name.clone(),
        type_name.clone(),
        inbound_type_name.clone(),
        vec![String::from("positionable")],
        vec![PropertyType::new(String::from("x"), DataType::String)],
        Vec::new(),
    );
    assert!(relation_type_manager.has(type_name.as_str()));

    let relation_type: Option<RelationType> = relation_type_manager.get(type_name.as_str());
    assert_eq!(type_name, relation_type.unwrap().type_name);

    relation_type_manager.delete(type_name.as_str());

    assert!(!relation_type_manager.has(type_name.as_str()));

    let relation_type: Option<RelationType> = relation_type_manager.get(type_name.as_str());
    assert!(relation_type.is_none());
}

#[test]
fn test_get_relation_types() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let outbound_type_name = r_string();
    let inbound_type_name = r_string();

    let entity_type = EntityTypeBuilder::new(outbound_type_name.clone()).build();
    entity_type_manager.register(entity_type.clone());
    let entity_type = EntityTypeBuilder::new(inbound_type_name.clone()).build();
    entity_type_manager.register(entity_type.clone());

    relation_type_manager.create(outbound_type_name.clone(), r_string(), inbound_type_name.clone(), vec![], vec![], vec![]);
    let relation_types = relation_type_manager.get_relation_types();
    assert_eq!(1, relation_types.len());
    for relation_type in relation_types {
        assert!(relation_type_manager.has(relation_type.type_name.as_str()));
    }
}

#[test]
fn test_register_relation_type_has_component() {
    let application = init_application();
    let component_manager = application.get_component_manager();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let component_name = r_string();

    component_manager.register(crate::model::Component::new(
        component_name.clone(),
        vec![crate::model::PropertyType::new(String::from("x"), DataType::String)],
    ));

    let relation_type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();

    let entity_type = EntityTypeBuilder::new(outbound_type_name.clone()).build();
    entity_type_manager.register(entity_type.clone());
    let entity_type = EntityTypeBuilder::new(inbound_type_name.clone()).build();
    entity_type_manager.register(entity_type.clone());
    relation_type_manager.register(crate::model::RelationType::new(
        outbound_type_name.clone(),
        relation_type_name.clone(),
        inbound_type_name.clone(),
        String::new(),
        String::new(),
        vec![component_name.clone()],
        vec![crate::model::PropertyType::new(String::from("y"), DataType::String)],
        Vec::new(),
    ));
    let relation_type: RelationType = relation_type_manager.get(relation_type_name.as_str()).unwrap();
    assert!(relation_type.components.contains(&component_name.clone()));
    assert!(relation_type.is_a(component_name.clone()));
}

#[test]
fn test_register_relation_type_has_property() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let property_name = String::from("x");
    let property_type = PropertyType::new(property_name.clone(), DataType::String);

    let relation_type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();

    let entity_type = EntityTypeBuilder::new(outbound_type_name.clone()).build();
    entity_type_manager.register(entity_type.clone());
    let entity_type = EntityTypeBuilder::new(inbound_type_name.clone()).build();
    entity_type_manager.register(entity_type.clone());
    relation_type_manager.register(RelationType::new(
        outbound_type_name.clone(),
        relation_type_name.clone(),
        inbound_type_name.clone(),
        String::new(),
        String::new(),
        Vec::new(),
        vec![property_type],
        Vec::new(),
    ));
    let relation_type: Option<RelationType> = relation_type_manager.get(relation_type_name.as_str());
    assert!(relation_type.unwrap().has_own_property(property_name.clone()));
}

#[test]
fn test_export_import_relation_type() {
    let application = init_application();
    let entity_type_manager = application.get_entity_type_manager();
    let relation_type_manager = application.get_relation_type_manager();

    let type_name = r_string();
    let outbound_type_name = r_string();
    let inbound_type_name = r_string();

    let mut path = env::temp_dir();
    path.push(format!("{}.json", type_name));
    let path = path.into_os_string().into_string().unwrap();

    let entity_type = EntityTypeBuilder::new(outbound_type_name.clone()).build();
    entity_type_manager.register(entity_type.clone());
    let entity_type = EntityTypeBuilder::new(inbound_type_name.clone()).build();
    entity_type_manager.register(entity_type.clone());
    relation_type_manager.create(
        outbound_type_name.clone(),
        type_name.clone(),
        inbound_type_name.clone(),
        vec![String::from("positionable")],
        vec![PropertyType::new(String::from("x"), DataType::String)],
        Vec::new(),
    );
    relation_type_manager.export(type_name.as_str(), path.as_str());
    assert!(relation_type_manager.has(type_name.as_str()));
    relation_type_manager.delete(type_name.as_str());
    assert!(!relation_type_manager.has(type_name.as_str()));
    let result = relation_type_manager.import(path.as_str());
    assert!(relation_type_manager.has(type_name.as_str()));
    assert!(result.is_ok());
}
