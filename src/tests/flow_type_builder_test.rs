use crate::model::DataType;
use crate::model::PropertyType;
use crate::model::SocketType;
use crate::model::TypeContainer;
use crate::tests::utils::r_string;
use crate::EntityTypeBuilder;
use crate::FlowTypeBuilder;
use serde_json::json;

#[test]
fn flow_type_builder_test() {
    let type_name = r_string();
    let name = r_string();
    let namespace = r_string();
    let description = r_string();
    let variable_name = r_string();
    let extension_1_name = r_string();
    let extension_2_name = r_string();
    let flow_type = FlowTypeBuilder::new(type_name.clone(), name.clone())
        .namespace(namespace.clone())
        .description(description.clone())
        .variable(variable_name.clone(), DataType::String)
        .extension(extension_1_name.clone(), json!(true))
        .extension(extension_2_name.clone(), json!(true))
        .build();
    assert_eq!(type_name, flow_type.type_name);
    assert_eq!(name, flow_type.name);
    assert_eq!(namespace, flow_type.namespace);
    assert_eq!(description, flow_type.description);
    assert!(flow_type.has_own_extension(extension_1_name.clone()));
    assert!(flow_type.has_own_extension(extension_2_name.clone()));
    assert!(!flow_type.has_own_extension(r_string()));
    assert!(entity_type.has_own_property(property_1_name.clone()));
    assert!(entity_type.has_own_property(property_2_name.clone()));
    assert!(entity_type.has_own_property(property_3_name.clone()));
    assert!(entity_type.has_own_property(property_4_name.clone()));
    assert!(entity_type.has_own_property(property_5_name.clone()));
    assert!(entity_type.has_own_property(property_6_name.clone()));
    assert!(entity_type.has_own_property(property_7_name.clone()));
    assert!(entity_type.has_own_property(property_8_name.clone()));
    assert!(entity_type.has_own_property(property_9_name.clone()));
    assert!(!entity_type.has_own_property(r_string()));
    assert_eq!(
        SocketType::Input,
        entity_type.properties.iter().find(|p| p.name == property_8_name.clone()).unwrap().socket_type
    );
    assert_eq!(
        SocketType::Output,
        entity_type.properties.iter().find(|p| p.name == property_9_name.clone()).unwrap().socket_type
    );
}
