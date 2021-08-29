use crate::{Component, PropertyType, DataType};
use crate::tests::utils::r_string;

#[test]
fn component_test() {
    let component_name = r_string();
    let description = r_string();
    let property_name = r_string();
    let mut property_types = Vec::new();
    let property_type = PropertyType::new(property_name.clone(), DataType::String);
    property_types.push(property_type.clone());
    let mut component = Component {
        name: component_name.clone(),
        description: description.clone(),
        properties: property_types
    };
    let component_name_2 = r_string();

    assert_eq!(component_name, component.name);
    assert_eq!(description, component.description);
    component.name = component_name_2.clone();
    assert_ne!(component_name, component.name);
    assert_eq!(component_name_2, component.name);

    let component_2 = component.clone();
    assert_eq!(component_2.name, component.name);
}

#[test]
fn create_component_test() {
    let component_name =  r_string();
    let mut property_types = Vec::new();
    let property_name =  r_string();
    let property_type = PropertyType::new(property_name.clone(), DataType::String);
    property_types.push(property_type.clone());
    let component = Component::new(component_name.clone(), property_types.clone());
    assert_eq!(component_name, component.name);
    assert_eq!(property_name.clone(), component.properties.first().unwrap().name);
    assert_eq!(
        property_type.data_type,
        component.properties.first().unwrap().data_type
    );
    assert!(!component.properties
        .iter()
        .filter(|&p| p.name == property_name)
        .collect::<Vec<_>>()
        .is_empty());
    assert!(component.has_property(property_name.clone()));
    assert!(!component.has_property(r_string()));
}
