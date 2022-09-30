use std::env;

use serde_json::json;

use crate::model::Component;
use crate::model::DataType;
use crate::model::Extension;
use crate::model::PropertyType;
use crate::tests::utils::application::init_application;
use crate::tests::utils::r_string;

#[test]
fn test_register_component() {
    let application = init_application();
    let component_manager = application.get_component_manager();
    let namespace = r_string();
    let component_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let extension_name = r_string();
    component_manager.register(Component::new(
        namespace.clone(),
        component_name.clone(),
        description.clone(),
        vec![PropertyType::new(property_name.clone(), DataType::String)],
        vec![Extension::new(extension_name.clone(), json!(""))],
    ));
    assert!(component_manager.has(component_name.as_str()));

    let component = component_manager.get(component_name.as_str()).unwrap();
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
        assert!(component_manager.has(component.name.as_str()));
    }
    assert!(!component_manager.has(r_string().as_str()));
}

#[test]
fn test_export_import_component() {
    let application = init_application();
    let component_manager = application.get_component_manager();
    let namespace = r_string();
    let component_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let extension_name = r_string();

    let mut path = env::temp_dir();
    path.push(format!("{}.json", component_name));
    let path = path.into_os_string().into_string().unwrap();

    component_manager.create(
        namespace.as_str(),
        component_name.as_str(),
        description.as_str(),
        vec![PropertyType::new(property_name.as_str(), DataType::String)],
        vec![Extension::new(extension_name.as_str(), json!(""))],
    );
    component_manager.export(component_name.as_str(), path.as_str());
    assert!(component_manager.has(component_name.as_str()));
    component_manager.delete(component_name.as_str());
    assert!(!component_manager.has(component_name.as_str()));
    component_manager.import(path.as_str());
    assert!(component_manager.has(component_name.as_str()));
}
