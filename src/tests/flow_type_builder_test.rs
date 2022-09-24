use crate::model::DataType;
use crate::tests::utils::r_string;
use crate::EntityInstanceBuilder;
use crate::FlowTypeBuilder;
use crate::RelationInstanceBuilder;
use serde_json::json;

#[test]
fn flow_type_builder_test() {
    let type_name = r_string();
    let entity_type_name_1 = r_string();
    let entity_type_name_2 = r_string();
    let relation_type_name = r_string();
    let name = r_string();
    let namespace = r_string();
    let description = r_string();
    let entity_instance_1 = EntityInstanceBuilder::new(entity_type_name_1.clone()).get();
    let entity_instance_2 = EntityInstanceBuilder::new(entity_type_name_2.clone()).get();
    let relation_instance = RelationInstanceBuilder::new(entity_instance_1.id, relation_type_name.clone(), entity_instance_2.id).get();
    let variable_1_name = r_string();
    let variable_2_name = r_string();
    let extension_1_name = r_string();
    let extension_2_name = r_string();
    let flow_type = FlowTypeBuilder::new(type_name.clone(), name.clone())
        .namespace(namespace.clone())
        .description(description.clone())
        .entity_instance(entity_instance_1)
        .entity_instance(entity_instance_2)
        .relation_instance(relation_instance)
        .variable(variable_1_name.clone(), DataType::Bool)
        .variable(variable_2_name.clone(), DataType::String)
        .extension(extension_1_name.clone(), json!(true))
        .extension(extension_2_name.clone(), json!(true))
        .build();
    assert_eq!(type_name, flow_type.type_name);
    assert_eq!(name, flow_type.name);
    assert_eq!(namespace, flow_type.namespace);
    assert_eq!(description, flow_type.description);
    assert_eq!(2, flow_type.entity_instances.len());
    assert_eq!(1, flow_type.relation_instances.len());
    assert!(flow_type.has_extension(extension_1_name.clone()));
    assert!(flow_type.has_extension(extension_2_name.clone()));
    assert!(!flow_type.has_extension(r_string()));
    assert!(flow_type.has_variable(variable_1_name.clone()));
    assert!(flow_type.has_variable(variable_2_name.clone()));
    assert!(!flow_type.has_variable(r_string()));
}
