use crate::tests::utils::r_string;
use crate::Component;
use crate::DataType;
use crate::Extension;
use crate::PropertyType;
use serde_json::json;

#[test]
fn component_test() {
    let namespace = r_string();
    let component_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let mut property_types = Vec::new();
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

    let mut component = Component {
        namespace: namespace.clone(),
        name: component_name.clone(),
        description: description.clone(),
        properties: property_types,
        extensions,
    };
    let component_name_2 = r_string();

    assert_eq!(namespace, component.namespace);
    assert_eq!(component_name, component.name);
    assert_eq!(description, component.description);
    component.name = component_name_2.clone();
    assert_ne!(component_name, component.name);
    assert_eq!(component_name_2, component.name);
    assert_eq!(extension_name.clone(), component.extensions.first().unwrap().name);
    assert_eq!(extension_value, component.extensions.first().unwrap().extension);
    assert!(component.has_extension(extension_name));
    assert!(!component.has_extension(r_string()));

    let component_2 = component.clone();
    assert_eq!(component_2.name, component.name);
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
    let component = Component::new(namespace.clone(), component_name.clone(), description.clone(), property_types.clone(), extensions);
    assert_eq!(namespace, component.namespace);
    assert_eq!(component_name, component.name);
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

    let component = Component::new_without_properties(namespace.clone(), component_name.clone(), r_string(), extensions.clone());
    assert_eq!(namespace, component.namespace);
    assert_eq!(component_name, component.name);
    assert_eq!(extension_name.clone(), component.extensions.first().unwrap().name);
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

    let component = Component::new_without_extensions(namespace.clone(), component_name.clone(), r_string(), property_types);
    assert_eq!(namespace, component.namespace);
    assert_eq!(component_name, component.name);
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
    let component = Component::new(namespace.clone(), component_name.clone(), r_string(), property_types.clone(), Vec::new());
    assert!(component.has_property(property_name));
    assert!(!component.has_property(r_string()));
}
