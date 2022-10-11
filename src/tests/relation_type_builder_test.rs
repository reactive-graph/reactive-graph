use serde_json::json;

use crate::model::DataType;
use crate::model::ExtensionContainer;
use crate::model::PropertyType;
use crate::model::SocketType;
use crate::model::TypeContainer;
use crate::tests::utils::r_string;
use crate::RelationTypeBuilder;

#[test]
fn relation_type_builder_test() {
    let namespace = r_string();
    let type_name = r_string();
    let outbound_type = r_string();
    let inbound_type = r_string();
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
    let property_8_name = r_string();
    let property_9_name = r_string();
    let relation_type = RelationTypeBuilder::new(namespace.clone(), outbound_type.clone(), type_name.clone(), inbound_type.clone())
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
        .component(component_1_name.clone())
        .component(component_2_name.clone())
        .extension(extension_1_name.clone(), json!(true))
        .extension(extension_2_name.clone(), json!(true))
        .build();
    assert_eq!(namespace, relation_type.namespace);
    assert_eq!(outbound_type, relation_type.outbound_type.clone());
    assert_eq!(inbound_type, relation_type.inbound_type.clone());
    assert_eq!(type_name, relation_type.type_name);
    assert_eq!(format!("{namespace}__{type_name}"), relation_type.t.to_string());
    assert_eq!(description, relation_type.description);
    assert!(relation_type.is_a(component_1_name.clone()));
    assert!(relation_type.is_a(component_2_name.clone()));
    assert!(!relation_type.is_a(r_string()));
    assert!(relation_type.has_own_extension(extension_1_name.clone()));
    assert!(relation_type.has_own_extension(extension_2_name.clone()));
    assert!(!relation_type.has_own_extension(r_string()));
    assert!(relation_type.has_own_property(property_1_name.clone()));
    assert!(relation_type.has_own_property(property_2_name.clone()));
    assert!(relation_type.has_own_property(property_3_name.clone()));
    assert!(relation_type.has_own_property(property_4_name.clone()));
    assert!(relation_type.has_own_property(property_5_name.clone()));
    assert!(relation_type.has_own_property(property_6_name.clone()));
    assert!(relation_type.has_own_property(property_7_name.clone()));
    assert!(relation_type.has_own_property(property_8_name.clone()));
    assert!(relation_type.has_own_property(property_9_name.clone()));
    assert!(!relation_type.has_own_property(r_string()));
    assert_eq!(
        SocketType::Input,
        relation_type.properties.iter().find(|p| p.name == property_8_name.clone()).unwrap().socket_type
    );
    assert_eq!(
        SocketType::Output,
        relation_type.properties.iter().find(|p| p.name == property_9_name.clone()).unwrap().socket_type
    );
}
