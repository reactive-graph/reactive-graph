use crate::model::DataType;
use crate::tests::utils::r_string;
use crate::RelationTypeBuilder;

#[test]
fn relation_type_builder_test() {
    let type_name = r_string();
    let outbound_type = r_string();
    let inbound_type = r_string();
    let component_1_name = r_string();
    let component_2_name = r_string();
    let behaviour_1_name = r_string();
    let behaviour_2_name = r_string();
    let property_1_name = r_string();
    let property_2_name = r_string();
    let relation_type = RelationTypeBuilder::new(
        outbound_type.clone(),
        type_name.clone(),
        inbound_type.clone(),
    )
    .property(property_1_name.clone(), DataType::String)
    .string_property(property_2_name.clone())
    .component(component_1_name.clone())
    .component(component_2_name.clone())
    .behaviour(behaviour_1_name.clone())
    .behaviour(behaviour_2_name.clone())
    .build();
    assert_eq!(outbound_type, relation_type.outbound_type.clone());
    assert_eq!(inbound_type, relation_type.inbound_type.clone());
    assert_eq!(type_name, relation_type.type_name);
    assert_eq!(type_name, relation_type.t.0);
    assert!(relation_type.is_a(component_1_name.clone()));
    assert!(relation_type.is_a(component_2_name.clone()));
    assert!(!relation_type.is_a(r_string()));
    assert!(relation_type.behaves_as(behaviour_1_name.clone()));
    assert!(relation_type.behaves_as(behaviour_2_name.clone()));
    assert!(!relation_type.behaves_as(r_string()));
    assert!(relation_type.has_own_property(property_1_name.clone()));
    assert!(relation_type.has_own_property(property_2_name.clone()));
    assert!(!relation_type.has_own_property(r_string()));
}
