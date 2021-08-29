use crate::{EntityType, PropertyType, DataType, Extension};
use crate::tests::utils::r_string;
use serde_json::json;

#[test]
fn create_entity_type_test() {
    let entity_type_name = "entity_type_name";

    let group = "group_name";

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
    let extension = Extension{ name: extension_name.to_string(), extension: extension_value.clone()};
    extensions.push(extension);

    let entity_type = EntityType::new(entity_type_name, group, component_names, Vec::new(), property_types, extensions);

    assert_eq!(entity_type_name, entity_type.name);

    assert_eq!(group, entity_type.group);

    assert_eq!(component_name, *entity_type.components.first().unwrap());

    assert!(entity_type.is_a(component_name.clone()));

    assert_eq!(property_name, entity_type.properties.first().unwrap().name);

    assert!(entity_type.has_own_property(property_name));

    assert_eq!(extension_name, entity_type.extensions.first().unwrap().name);

    assert_eq!(extension_value, entity_type.extensions.first().unwrap().extension);

}
