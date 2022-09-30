use serde_json::json;
use std::collections::HashMap;

use crate::tests::utils::create_entity_instance;
use crate::tests::utils::r_string;
use crate::DataType;
use crate::Extension;
use crate::FlowType;
use crate::PropertyType;
use crate::RelationInstance;

#[test]
fn create_flow_type_test() {
    let flow_type_name = "flow_type_name";

    let entity_type_name = "entity_type_name";

    let relation_type_name = "relation_type_name";

    let namespace = "namespace";
    let description = "Lorem ipsum";

    let entity_instance_1 = create_entity_instance("property");
    let entity_instance_2 = create_entity_instance("property2");
    let mut entity_instances = Vec::new();
    entity_instances.push(entity_instance_1.clone());
    entity_instances.push(entity_instance_2.clone());

    let relation_instance = RelationInstance::new(entity_instance_1.id, relation_type_name.clone(), entity_instance_2.id, HashMap::new());
    let mut relation_instances = Vec::new();
    relation_instances.push(relation_instance.clone());

    let mut variables = Vec::new();
    let variable_name = "variable_name";
    let variable_data_type = DataType::Object;
    let variable = PropertyType::new(variable_name, variable_data_type);
    variables.push(variable.clone());

    let mut extensions = Vec::new();
    let extension_name = "extension_name";
    let extension_value = json!("extension_value");
    let extension = Extension {
        name: extension_name.to_string(),
        extension: extension_value.clone(),
    };
    extensions.push(extension);
    let extension = Extension::new("other_extension", extension_value.clone());
    extensions.push(extension);

    let flow_type = FlowType::new(
        namespace,
        flow_type_name,
        entity_type_name,
        description,
        entity_instances,
        relation_instances,
        variables,
        extensions,
    );

    assert_eq!(namespace, flow_type.namespace);

    assert_eq!(flow_type_name, flow_type.name);

    assert_eq!(entity_type_name, flow_type.type_name);

    assert_eq!(description, flow_type.description);

    assert_eq!(entity_instance_1.id, flow_type.entity_instances.first().unwrap().id);

    assert_eq!(entity_instance_2.id, flow_type.entity_instances.get(1).unwrap().id);

    assert_eq!(relation_instance.type_name, *flow_type.relation_instances.first().unwrap().type_name);

    assert_eq!(relation_instance.outbound_id, flow_type.relation_instances.first().unwrap().outbound_id);
    assert_eq!(entity_instance_1.id, flow_type.relation_instances.first().unwrap().outbound_id);

    assert_eq!(relation_instance.inbound_id, flow_type.relation_instances.first().unwrap().inbound_id);
    assert_eq!(entity_instance_2.id, flow_type.relation_instances.first().unwrap().inbound_id);

    assert_eq!(variable_name, flow_type.variables.first().unwrap().name.as_str());

    assert_eq!(variable_data_type, flow_type.variables.first().unwrap().data_type);

    assert_eq!(extension_name.clone(), flow_type.extensions.first().unwrap().name);

    assert_eq!(3, flow_type.uses_entity_types().len());
    assert!(flow_type.uses_entity_types().contains(&entity_instance_1.type_name));

    assert_eq!(1, flow_type.uses_relation_types().len());
    assert!(flow_type.uses_relation_types().contains(&relation_instance.type_name));

    assert!(flow_type.has_variable(variable_name));
    assert!(!flow_type.has_variable(r_string()));

    assert_eq!(extension_value, flow_type.extensions.first().unwrap().extension);
    assert!(flow_type.has_extension(extension_name));
    assert!(!flow_type.has_extension(r_string()));
}
