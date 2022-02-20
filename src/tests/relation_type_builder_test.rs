use crate::model::DataType;
use crate::model::PropertyType;
use crate::tests::utils::r_string;
use crate::RelationTypeBuilder;
use serde_json::json;

#[test]
fn relation_type_builder_test() {
    let type_name = r_string();
    let outbound_type = r_string();
    let inbound_type = r_string();
    let component_1_name = r_string();
    let component_2_name = r_string();
    let behaviour_1_name = r_string();
    let behaviour_2_name = r_string();
    let extension_1_name = r_string();
    let extension_2_name = r_string();
    let property_1_name = r_string();
    let property_2_name = r_string();
    let property_3_name = r_string();
    let property_4_name = r_string();
    let property_5_name = r_string();
    let relation_type = RelationTypeBuilder::new(
        outbound_type.clone(),
        type_name.clone(),
        inbound_type.clone(),
    )
    .property(property_1_name.clone(), DataType::String)
    .property_from(PropertyType::new(property_2_name.clone(), DataType::Bool))
    .string_property(property_3_name.clone())
    .bool_property(property_4_name.clone())
    .number_property(property_5_name.clone())
    .component(component_1_name.clone())
    .component(component_2_name.clone())
    .behaviour(behaviour_1_name.clone())
    .behaviour(behaviour_2_name.clone())
    .extension(extension_1_name.clone(), json!(true))
    .extension(extension_2_name.clone(), json!(true))
    .build();
    assert_eq!(outbound_type, relation_type.outbound_type.clone());
    assert_eq!(inbound_type, relation_type.inbound_type.clone());
    assert_eq!(type_name, relation_type.type_name);
    assert_eq!(type_name, relation_type.t.to_string());
    assert!(relation_type.is_a(component_1_name.clone()));
    assert!(relation_type.is_a(component_2_name.clone()));
    assert!(!relation_type.is_a(r_string()));
    assert!(relation_type.behaves_as(behaviour_1_name.clone()));
    assert!(relation_type.behaves_as(behaviour_2_name.clone()));
    assert!(!relation_type.behaves_as(r_string()));
    assert!(relation_type.has_own_extension(extension_1_name.clone()));
    assert!(relation_type.has_own_extension(extension_2_name.clone()));
    assert!(!relation_type.has_own_extension(r_string()));
    assert!(relation_type.has_own_property(property_1_name.clone()));
    assert!(relation_type.has_own_property(property_2_name.clone()));
    assert!(relation_type.has_own_property(property_3_name.clone()));
    assert!(relation_type.has_own_property(property_4_name.clone()));
    assert!(relation_type.has_own_property(property_5_name.clone()));
    assert!(!relation_type.has_own_property(r_string()));
}
