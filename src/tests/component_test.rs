use serde_json::json;

use crate::tests::utils::r_string;
use crate::Component;
use crate::ComponentType;
use crate::DataType;
use crate::Extension;
use crate::NamespacedTypeGetter;
use crate::PropertyType;
use crate::TypeDefinitionGetter;

#[test]
fn component_test() {
    let namespace = r_string();
    let component_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let mut property_types = Vec::new();
    let property_type = PropertyType::new(&property_name, DataType::String);
    property_types.push(property_type.clone());

    let mut extensions = Vec::new();
    let extension_name = "extension_name";
    let extension_value = json!("extension_value");
    let extension = Extension {
        name: extension_name.to_string(),
        extension: extension_value.clone(),
    };
    extensions.push(extension);
    let extension = Extension::new("other_extension", extension_value.clone());
    extensions.push(extension);

    let ty = ComponentType::new_from_type(&namespace, &component_name);
    let component = Component {
        ty,
        description: description.clone(),
        properties: property_types,
        extensions,
    };
    assert_eq!(namespace, component.namespace());
    assert_eq!(component_name, component.type_name());
    assert_eq!(format!("c__{}__{}", &namespace, &component_name), component.type_definition().to_string());
    assert_eq!(description, component.description);
    assert_eq!(&extension_name, &component.extensions.first().unwrap().name);
    assert_eq!(extension_value, component.extensions.first().unwrap().extension);
    assert!(component.has_extension(extension_name));
    assert!(!component.has_extension(r_string()));

    let component_2 = component.clone();
    assert_eq!(component_2.type_name(), component.type_name());
}

#[test]
fn create_new_component_test() {
    let namespace = r_string();
    let component_name = r_string();
    let description = r_string();
    let mut property_types = Vec::new();
    let property_name = r_string();
    let property_type = PropertyType::new(property_name.clone(), DataType::String);
    property_types.push(property_type.clone());
    let mut extensions = Vec::new();
    let extension_name = "extension_name";
    let extension_value = json!("extension_value");
    let extension = Extension {
        name: extension_name.to_string(),
        extension: extension_value.clone(),
    };
    extensions.push(extension);
    let extension = Extension::new("other_extension", extension_value.clone());
    extensions.push(extension);
    let ty = ComponentType::new_from_type(&namespace, &component_name);
    let component = Component::new(ty, description.clone(), property_types.clone(), extensions);
    assert_eq!(namespace, component.namespace());
    assert_eq!(component_name, component.type_name());
    assert_eq!(property_name.clone(), component.properties.first().unwrap().name);
    assert_eq!(property_type.data_type, component.properties.first().unwrap().data_type);
    assert!(!component.properties.iter().filter(|&p| p.name == property_name).collect::<Vec<_>>().is_empty());
    assert!(component.has_property(property_name.clone()));
    assert!(!component.has_property(r_string()));
}

#[test]
fn create_new_component_without_properties_test() {
    let namespace = r_string();
    let component_name = r_string();

    let mut extensions = Vec::new();
    let extension_name = "extension_name";
    let extension_value = json!("extension_value");
    let extension = Extension {
        name: extension_name.to_string(),
        extension: extension_value.clone(),
    };
    extensions.push(extension);
    let extension = Extension::new("other_extension", extension_value.clone());
    extensions.push(extension);

    let ty = ComponentType::new_from_type(&namespace, &component_name);
    let component = Component::new_without_properties(ty, r_string(), extensions.clone());
    assert_eq!(namespace, component.namespace());
    assert_eq!(component_name, component.type_name());
    assert_eq!(&extension_name, &component.extensions.first().unwrap().name);
    assert_eq!(extension_value, component.extensions.first().unwrap().extension);
    assert!(component.has_extension(extension_name));
    assert!(!component.has_extension(r_string()));
}

#[test]
fn create_component_without_extensions_test() {
    let component_name = r_string();
    let namespace = r_string();

    let property_name = r_string();
    let mut property_types = Vec::new();
    let property_type = PropertyType::new(property_name.clone(), DataType::String);
    property_types.push(property_type.clone());

    let ty = ComponentType::new_from_type(&namespace, &component_name);
    let component = Component::new_without_extensions(ty, r_string(), property_types);
    assert_eq!(namespace, component.namespace());
    assert_eq!(component_name, component.type_name());
    assert_eq!(property_name.clone(), component.properties.first().unwrap().name);
    assert_eq!(property_type.data_type, component.properties.first().unwrap().data_type);
    assert!(!component.properties.iter().filter(|&p| p.name == property_name).collect::<Vec<_>>().is_empty());
}

#[test]
fn component_has_property_test() {
    let namespace = r_string();
    let component_name = r_string();
    let mut property_types = Vec::new();
    let property_name = r_string();
    let property_type = PropertyType::new(property_name.clone(), DataType::String);
    property_types.push(property_type.clone());
    let ty = ComponentType::new_from_type(&namespace, &component_name);
    let component = Component::new(ty, r_string(), property_types.clone(), Vec::new());
    assert!(component.has_property(property_name));
    assert!(!component.has_property(r_string()));
}
