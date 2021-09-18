use crate::model::DataType;
use crate::tests::utils::r_string;
use crate::EntityTypeBuilder;

#[test]
fn entity_type_builder_test() {
    let type_name = r_string();
    let component_1_name = r_string();
    let component_2_name = r_string();
    let behaviour_1_name = r_string();
    let behaviour_2_name = r_string();
    let property_1_name = r_string();
    let property_2_name = r_string();
    let entity_type = EntityTypeBuilder::new(type_name.clone())
        .property(property_1_name.clone(), DataType::String)
        .string_property(property_2_name.clone())
        .component(component_1_name.clone())
        .component(component_2_name.clone())
        .behaviour(behaviour_1_name.clone())
        .behaviour(behaviour_2_name.clone())
        .build();
    assert_eq!(type_name, entity_type.name);
    assert_eq!(type_name, entity_type.t.0);
    assert!(entity_type.is_a(component_1_name.clone()));
    assert!(entity_type.is_a(component_2_name.clone()));
    assert!(!entity_type.is_a(r_string()));
    assert!(entity_type.behaves_as(behaviour_1_name.clone()));
    assert!(entity_type.behaves_as(behaviour_2_name.clone()));
    assert!(!entity_type.behaves_as(r_string()));
    assert!(entity_type.has_own_property(property_1_name.clone()));
    assert!(entity_type.has_own_property(property_2_name.clone()));
    assert!(!entity_type.has_own_property(r_string()));
}
