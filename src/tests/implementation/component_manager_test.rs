use std::env;

use crate::model::{Component, DataType, PropertyType};
use crate::tests::utils::application::init_application;
use crate::tests::utils::r_string;

#[test]
fn test_register_component() {
    let application = init_application();
    let component_manager = application.get_component_manager();
    let component_name = r_string();
    let property_name = r_string();
    component_manager.register(Component::new(
        component_name.clone(),
        vec![PropertyType::new(property_name.clone(), DataType::String)],
    ));
    assert!(component_manager.has(component_name.clone()));

    let component = component_manager.get(component_name.clone()).unwrap();
    assert_eq!(component_name, component.name);
    assert!(component.has_property(property_name.clone()));
    assert!(!component.has_property(r_string()));
}

#[test]
fn test_get_components() {
    let application = init_application();
    let component_manager = application.get_component_manager();
    let components = component_manager.get_components();
    for component in components {
        assert!(component_manager.has(component.name));
    }
    assert!(!component_manager.has(r_string()));
}

#[test]
fn test_export_import_component() {
    let application = init_application();
    let component_manager = application.get_component_manager();
    let component_name = r_string();

    let mut path = env::temp_dir();
    path.push(format!("{}.json", component_name));
    let path = path.into_os_string().into_string().unwrap();

    component_manager.create(
        component_name.clone(),
        vec![PropertyType::new(String::from("x"), DataType::String)],
    );
    component_manager.export(component_name.clone(), path.clone());
    assert!(component_manager.has(component_name.clone()));
    component_manager.delete(component_name.clone());
    assert!(!component_manager.has(component_name.clone()));
    component_manager.import(path.clone());
    assert!(component_manager.has(component_name.clone()));
}
