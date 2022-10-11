use serde_json::json;

use crate::model::DataType;
use crate::model::PropertyType;
use crate::model::SocketType;
use crate::tests::utils::r_string;
use crate::ComponentBuilder;
use crate::ComponentsBuilder;

#[test]
fn component_builder_test() {
    let namespace = r_string();
    let name = r_string();
    let description = r_string();
    let extension_1_name = r_string();
    let extension_2_name = r_string();
    let property_1_name = r_string();
    let property_2_name = r_string();
    let property_3_name = r_string();
    let property_4_name = r_string();
    let property_5_name = r_string();
    let property_6_name = r_string();
    let property_7_name = r_string();
    let property_8_name = r_string();
    let property_9_name = r_string();
    let component = ComponentBuilder::new(namespace.clone(), name.clone())
        .description(description.clone())
        .property(property_1_name.clone(), DataType::String)
        .property_from(PropertyType::new(property_2_name.clone(), DataType::Bool))
        .string_property(property_3_name.clone())
        .bool_property(property_4_name.clone())
        .number_property(property_5_name.clone())
        .array_property(property_6_name.clone())
        .object_property(property_7_name.clone())
        .input_property(property_8_name.clone(), DataType::Bool)
        .output_property(property_9_name.clone(), DataType::Bool)
        .extension(extension_1_name.clone(), json!(true))
        .extension(extension_2_name.clone(), json!(true))
        .build();
    assert_eq!(namespace, component.namespace);
    assert_eq!(name, component.name);
    assert_eq!(description, component.description);
    assert!(component.has_extension(extension_1_name.clone()));
    assert!(component.has_extension(extension_2_name.clone()));
    assert!(!component.has_extension(r_string()));
    assert!(component.has_property(property_1_name.clone()));
    assert!(component.has_property(property_2_name.clone()));
    assert!(component.has_property(property_3_name.clone()));
    assert!(component.has_property(property_4_name.clone()));
    assert!(component.has_property(property_5_name.clone()));
    assert!(component.has_property(property_6_name.clone()));
    assert!(component.has_property(property_7_name.clone()));
    assert!(component.has_property(property_8_name.clone()));
    assert!(component.has_property(property_9_name.clone()));
    assert!(!component.has_property(r_string()));
    assert_eq!(
        SocketType::Input,
        component.properties.iter().find(|p| p.name == property_8_name.clone()).unwrap().socket_type
    );
    assert_eq!(
        SocketType::Output,
        component.properties.iter().find(|p| p.name == property_9_name.clone()).unwrap().socket_type
    );
}

#[test]
fn components_builder_test() {
    let namespace = r_string();
    let name_1 = r_string();
    let description_1 = r_string();
    let property_1_name = r_string();
    let property_2_name = r_string();
    let property_3_name = r_string();
    let property_4_name = r_string();
    let property_5_name = r_string();
    let property_6_name = r_string();
    let property_7_name = r_string();
    let property_8_name = r_string();
    let property_9_name = r_string();
    let extension_1_name = r_string();
    let extension_2_name = r_string();
    let name_2 = r_string();
    let description_2 = r_string();
    let property_2_1_name = r_string();
    let name_3 = r_string();
    let description_3 = r_string();
    let property_3_1_name = r_string();
    let components = ComponentsBuilder::new(namespace.clone())
        .next(&name_1)
        .description(&description_1)
        .property(property_1_name.clone(), DataType::String)
        .property_from(PropertyType::new(property_2_name.clone(), DataType::Bool))
        .string_property(property_3_name.clone())
        .bool_property(property_4_name.clone())
        .number_property(property_5_name.clone())
        .array_property(property_6_name.clone())
        .object_property(property_7_name.clone())
        .input_property(property_8_name.clone(), DataType::Bool)
        .output_property(property_9_name.clone(), DataType::Bool)
        .extension(extension_1_name.clone(), json!(true))
        .extension(extension_2_name.clone(), json!(true))
        .done() // Explicit done()
        .next(&name_2)
        .description(&description_2)
        .bool_property(&property_2_1_name)
        .next(&name_3) // allows to skip .done()
        .description(&description_3)
        .bool_property(&property_3_1_name)
        .build(); // allows to skip .done()
    assert_eq!(3, components.len());
    assert_eq!(namespace, components[0].namespace);
    assert_eq!(name_1, components[0].name);
    assert_eq!(description_1, components[0].description);
    assert!(components[0].has_property(property_1_name.clone()));
    assert!(components[0].has_property(property_2_name.clone()));
    assert!(components[0].has_property(property_3_name.clone()));
    assert!(components[0].has_property(property_4_name.clone()));
    assert!(components[0].has_property(property_5_name.clone()));
    assert!(components[0].has_property(property_6_name.clone()));
    assert!(components[0].has_property(property_7_name.clone()));
    assert!(components[0].has_property(property_8_name.clone()));
    assert!(components[0].has_property(property_9_name.clone()));
    assert!(components[0].has_extension(extension_1_name.clone()));
    assert!(components[0].has_extension(extension_2_name.clone()));
    assert!(!components[0].has_extension(r_string()));
    assert_eq!(namespace, components[1].namespace);
    assert_eq!(name_2, components[1].name);
    assert_eq!(description_2, components[1].description);
}
