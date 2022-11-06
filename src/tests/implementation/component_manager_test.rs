use std::env;

use serde_json::json;

use crate::model::Component;
use crate::model::ComponentTypeId;
use crate::model::Extension;
use crate::model::NamespacedTypeGetter;
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
    let ty = ComponentTypeId::new_from_type(namespace.clone(), component_name.clone());
    let component = Component::new(
        &ty,
        &description,
        vec![PropertyType::string(&property_name)],
        vec![Extension::new(extension_name.clone(), json!(""))],
    );
    assert!(component_manager.register(component).is_ok());
    assert!(component_manager.has(&ty));

    let component = component_manager.get(&ty).unwrap();
    assert_eq!(namespace, component.namespace());
    assert_eq!(component_name, component.type_name());
    assert!(component.has_property(property_name.clone()));
    assert!(!component.has_property(r_string()));
}

#[test]
fn test_get_components() {
    let application = init_application();
    let component_manager = application.get_component_manager();
    let components = component_manager.get_all();
    for component in components {
        assert!(component_manager.has(&component.ty));
    }
    assert!(!component_manager.has(&ComponentTypeId::new_from_type(r_string(), r_string())));
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

    let ty = ComponentTypeId::new_from_type(&namespace, &component_name);
    let result = component_manager.create(
        &ty,
        &description,
        vec![PropertyType::string(&property_name)],
        vec![Extension::new(extension_name.as_str(), json!(""))],
    );
    assert!(result.is_ok());
    component_manager.export(&ty, path.as_str());
    assert!(component_manager.has(&ty));
    component_manager.delete(&ty);
    assert!(!component_manager.has(&ty));
    let result = component_manager.import(path.as_str());
    assert!(result.is_ok());
    assert!(component_manager.has(&ty));
}
