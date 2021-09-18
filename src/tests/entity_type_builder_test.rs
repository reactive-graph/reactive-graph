use crate::model::DataType;
use crate::tests::utils::r_string;
use crate::EntityTypeBuilder;
use inexor_rgf_core_model::PropertyType;
use serde_json::json;

#[test]
fn entity_type_builder_test() {
    let type_name = r_string();
    let group_name = r_string();
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
    let entity_type = EntityTypeBuilder::new(type_name.clone())
        .group(group_name.clone())
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
    assert_eq!(type_name, entity_type.name);
    assert_eq!(type_name, entity_type.t.0);
    assert_eq!(group_name.clone(), entity_type.group);
    assert!(entity_type.is_a(component_1_name.clone()));
    assert!(entity_type.is_a(component_2_name.clone()));
    assert!(!entity_type.is_a(r_string()));
    assert!(entity_type.behaves_as(behaviour_1_name.clone()));
    assert!(entity_type.behaves_as(behaviour_2_name.clone()));
    assert!(!entity_type.behaves_as(r_string()));
    assert!(entity_type.has_own_property(property_1_name.clone()));
    assert!(entity_type.has_own_property(property_2_name.clone()));
    assert!(entity_type.has_own_property(property_3_name.clone()));
    assert!(entity_type.has_own_property(property_4_name.clone()));
    assert!(entity_type.has_own_property(property_5_name.clone()));
    assert!(!entity_type.has_own_property(r_string()));
}
