use serde_json::json;

use crate::tests::utils::r_string;
use crate::tests::utils::r_string_1000;
use crate::DataType;
use crate::EntityType;
use crate::Extension;
use crate::ExtensionContainer;
use crate::PropertyType;
use crate::TypeContainer;

#[test]
fn create_entity_type_test() {
    let entity_type_name = "entity_type_name";

    let namespace = "namespace";
    let description = "Lorem ipsum";

    let component_name = r_string();
    let mut component_names = Vec::new();
    component_names.push(component_name.clone());

    let mut property_types = Vec::new();
    let property_name = "property_name";
    let property_type = PropertyType::new(property_name, DataType::String);
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
    extensions.push(extension.clone());

    let entity_type = EntityType::new(namespace, entity_type_name, description, component_names, property_types, extensions);

    assert_eq!(namespace, entity_type.namespace);

    assert_eq!(entity_type_name, entity_type.name);

    assert_eq!(format!("{}__{}", &namespace, &entity_type_name), entity_type.fully_qualified_name());

    assert_eq!(format!("{namespace}__{entity_type_name}"), entity_type.t.to_string());

    assert_eq!(description, entity_type.description);

    assert_eq!(component_name, *entity_type.components.first().unwrap());

    assert!(entity_type.is_a(component_name.clone()));

    assert_eq!(property_name, entity_type.properties.first().unwrap().name);

    assert!(entity_type.has_own_property(property_name));
    assert!(!entity_type.has_own_property(r_string()));
    assert_eq!(property_type.data_type, entity_type.get_own_property(property_name).unwrap().data_type);

    assert_eq!(extension_name.clone(), entity_type.extensions.first().unwrap().name);

    assert_eq!(extension_value, entity_type.extensions.first().unwrap().extension);
    assert!(entity_type.has_own_extension(extension_name));
    assert!(!entity_type.has_own_extension(r_string()));
    assert_eq!(extension.extension, entity_type.get_own_extension(extension_name).unwrap().extension);
}

#[test]
fn long_entity_type_test() {
    let namespace = r_string_1000();
    let entity_type_name = r_string_1000();
    let description = r_string();
    let et = EntityType::new(namespace, entity_type_name, description, Vec::new(), Vec::new(), Vec::new());
    assert!(et.t.as_str().len() < 255);
}
