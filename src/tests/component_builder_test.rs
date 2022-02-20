use crate::model::DataType;
use crate::model::PropertyType;
use crate::tests::utils::r_string;
use crate::ComponentBuilder;
use serde_json::json;

#[test]
fn component_builder_test() {
    let name = r_string();
    let extension_1_name = r_string();
    let extension_2_name = r_string();
    let property_1_name = r_string();
    let property_2_name = r_string();
    let property_3_name = r_string();
    let property_4_name = r_string();
    let property_5_name = r_string();
    let component = ComponentBuilder::new(name.clone())
        .property(property_1_name.clone(), DataType::String)
        .property_from(PropertyType::new(property_2_name.clone(), DataType::Bool))
        .string_property(property_3_name.clone())
        .bool_property(property_4_name.clone())
        .number_property(property_5_name.clone())
        .extension(extension_1_name.clone(), json!(true))
        .extension(extension_2_name.clone(), json!(true))
        .build();
    assert_eq!(name, component.name);
    assert!(component.has_extension(extension_1_name.clone()));
    assert!(component.has_extension(extension_2_name.clone()));
    assert!(!component.has_extension(r_string()));
    assert!(component.has_property(property_1_name.clone()));
    assert!(component.has_property(property_2_name.clone()));
    assert!(component.has_property(property_3_name.clone()));
    assert!(component.has_property(property_4_name.clone()));
    assert!(component.has_property(property_5_name.clone()));
    assert!(!component.has_property(r_string()));
}
