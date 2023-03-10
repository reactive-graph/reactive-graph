use std::env;

use serde_json::json;

use crate::get_runtime;
use crate::model::Component;
use crate::model::ComponentTypeId;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyType;
use crate::tests::utils::r_string;

#[test]
fn test_register_component() {
    let runtime = get_runtime();
    let component_manager = runtime.get_component_manager();
    let namespace = r_string();
    let component_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let extension_name = r_string();
    let component_ty = ComponentTypeId::new_from_type(namespace.clone(), component_name.clone());
    let extension_ty = ExtensionTypeId::new_from_type(&namespace, &extension_name);
    let component = Component::new(
        &component_ty,
        &description,
        vec![PropertyType::string(&property_name)],
        vec![Extension::new(extension_ty.clone(), "", json!(""))],
    );
    assert!(component_manager.register(component).is_ok());
    assert!(component_manager.has(&component_ty));

    let component = component_manager.get(&component_ty).unwrap();
    assert_eq!(namespace, component.namespace());
    assert_eq!(component_name, component.type_name());
    assert!(component.has_property(property_name.clone()));
    assert!(!component.has_property(r_string()));
    assert!(component.has_extension(&extension_ty));
}

#[test]
fn test_get_components() {
    let runtime = get_runtime();
    let component_manager = runtime.get_component_manager();
    let components = component_manager.get_all();
    for component in components {
        assert!(component_manager.has(&component.ty));
    }
    assert!(!component_manager.has(&ComponentTypeId::new_from_type(r_string(), r_string())));
}

#[test]
fn test_export_import_component() {
    let runtime = get_runtime();
    let component_manager = runtime.get_component_manager();
    let namespace = r_string();
    let component_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let extension_name = r_string();

    let mut path = env::temp_dir();
    path.push(format!("{}.json", component_name));
    let path = path.into_os_string().into_string().unwrap();

    let component_ty = ComponentTypeId::new_from_type(&namespace, &component_name);
    let extension_ty = ExtensionTypeId::new_from_type(&namespace, &extension_name);
    let result = component_manager.create(
        &component_ty,
        &description,
        vec![PropertyType::string(&property_name)],
        vec![Extension::new(extension_ty.clone(), "", json!(""))],
    );
    assert!(result.is_ok());
    component_manager.export(&component_ty, path.as_str());
    assert!(component_manager.has(&component_ty));
    component_manager.delete(&component_ty);
    assert!(!component_manager.has(&component_ty));
    let result = component_manager.import(path.as_str());
    assert!(result.is_ok());
    assert!(component_manager.has(&component_ty));
}
