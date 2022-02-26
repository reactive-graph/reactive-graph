use crate::model::DataType;
use crate::model::PropertyType;
use crate::model::SocketType;
use crate::tests::utils::r_string;
use crate::EntityTypeBuilder;
use serde_json::json;

#[test]
fn entity_type_builder_test() {
    let type_name = r_string();
    let group = r_string();
    let description = r_string();
    let component_1_name = r_string();
    let component_2_name = r_string();
    let extension_1_name = r_string();
    let extension_2_name = r_string();
    let property_1_name = r_string();
    let property_2_name = r_string();
    let property_3_name = r_string();
    let property_4_name = r_string();
    let property_5_name = r_string();
    let property_6_name = r_string();
    let property_7_name = r_string();
    let entity_type = EntityTypeBuilder::new(type_name.clone())
        .group(group.clone())
        .description(description.clone())
        .property(property_1_name.clone(), DataType::String)
        .property_from(PropertyType::new(property_2_name.clone(), DataType::Bool))
        .string_property(property_3_name.clone())
        .bool_property(property_4_name.clone())
        .number_property(property_5_name.clone())
        .input_property(property_6_name.clone(), DataType::Bool)
        .output_property(property_7_name.clone(), DataType::Bool)
        .component(component_1_name.clone())
        .component(component_2_name.clone())
        .extension(extension_1_name.clone(), json!(true))
        .extension(extension_2_name.clone(), json!(true))
        .build();
    assert_eq!(type_name, entity_type.name);
    assert_eq!(type_name, entity_type.t.to_string());
    assert_eq!(group, entity_type.group);
    // TODO: assert_eq!(description, entity_type.description);
    assert!(entity_type.is_a(component_1_name.clone()));
    assert!(entity_type.is_a(component_2_name.clone()));
    assert!(!entity_type.is_a(r_string()));
    assert!(entity_type.has_own_extension(extension_1_name.clone()));
    assert!(entity_type.has_own_extension(extension_2_name.clone()));
    assert!(!entity_type.has_own_extension(r_string()));
    assert!(entity_type.has_own_property(property_1_name.clone()));
    assert!(entity_type.has_own_property(property_2_name.clone()));
    assert!(entity_type.has_own_property(property_3_name.clone()));
    assert!(entity_type.has_own_property(property_4_name.clone()));
    assert!(entity_type.has_own_property(property_5_name.clone()));
    assert!(entity_type.has_own_property(property_6_name.clone()));
    assert!(entity_type.has_own_property(property_7_name.clone()));
    assert!(!entity_type.has_own_property(r_string()));
    assert_eq!(
        SocketType::Input,
        entity_type
            .properties
            .iter()
            .find(|p| p.name == property_6_name.clone())
            .unwrap()
            .socket_type
    );
    assert_eq!(
        SocketType::Output,
        entity_type
            .properties
            .iter()
            .find(|p| p.name == property_7_name.clone())
            .unwrap()
            .socket_type
    );
}
