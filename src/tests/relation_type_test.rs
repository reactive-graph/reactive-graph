use crate::{RelationType, PropertyType, DataType, Extension};
use crate::tests::utils::r_string;
use serde_json::json;

#[test]
fn create_relation_type_test() {
    let type_name = r_string();
    let outbound_type = r_string();
    let inbound_type = r_string();
    let component_name = r_string();
    let behaviour_name = r_string();
    let property_name = r_string();
    let extension_name = r_string();
    let extension_value = json!("JSON");
    let mut component_names = Vec::new();
    component_names.push(component_name.clone());
    let mut behaviour_names = Vec::new();
    behaviour_names.push(behaviour_name.clone());
    let mut property_types = Vec::new();
    let property_type = PropertyType::new(property_name.clone(), DataType::String);
    property_types.push(property_type.clone());
    let mut extensions = Vec::new();
    let extension = Extension { name: extension_name.clone(), extension: extension_value.clone() };
    extensions.push(extension.clone());
    let relation_type = RelationType::new(
        outbound_type.clone(),
        type_name.clone(),
        inbound_type.clone(),
        component_names,
        behaviour_names,
        property_types,
        extensions
    );

    assert_eq!(type_name, relation_type.type_name);
    assert_eq!(outbound_type, relation_type.outbound_type);
    assert_eq!(inbound_type, relation_type.inbound_type);
    assert_eq!(component_name, *relation_type.components.first().unwrap());
    assert!(relation_type.is_a(component_name.clone()));
    assert_eq!(property_name, *relation_type.properties.first().unwrap().name);
    assert!(relation_type.has_own_property(property_name.clone()));
    assert_eq!(extension_name, relation_type.extensions.get(0).unwrap().name);
    assert_eq!(extension_value, relation_type.extensions.get(0).unwrap().extension);
    assert!(relation_type.behaves_as(behaviour_name.clone()));
    assert!(!relation_type.behaves_as(r_string()));
}
