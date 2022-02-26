use serde_json::json;

use crate::tests::utils::r_string;
use crate::{DataType, EntityType, Extension, PropertyType};

#[test]
fn create_entity_type_test() {
    let entity_type_name = "entity_type_name";

    let group = "group_name";
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
    extensions.push(extension);

    let entity_type = EntityType::new(entity_type_name, group, description, component_names, property_types, extensions);

    assert_eq!(entity_type_name, entity_type.name);

    assert_eq!(group, entity_type.group);

    assert_eq!(description, entity_type.description);

    assert_eq!(component_name, *entity_type.components.first().unwrap());

    assert!(entity_type.is_a(component_name.clone()));

    assert_eq!(property_name, entity_type.properties.first().unwrap().name);

    assert!(entity_type.has_own_property(property_name));

    assert_eq!(extension_name.clone(), entity_type.extensions.first().unwrap().name);

    assert_eq!(extension_value, entity_type.extensions.first().unwrap().extension);
    assert!(entity_type.has_own_extension(extension_name));
    assert!(!entity_type.has_own_extension(r_string()));
}
