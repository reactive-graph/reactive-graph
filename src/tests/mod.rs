mod data_type_test;
mod socket_type_test;

mod component_test;
mod entity_type_test;
mod flow_type_test;
mod property_type_test;
mod relation_type_test;

mod entity_instance_test;
mod relation_instance_test;

mod flow_instance_test;

mod reactive_entity_instance_test;
mod reactive_property_instance_test;
mod reactive_relation_instance_test;

mod reactive_flow_instance_test;

mod component_type_id_test;
mod entity_type_id_test;
mod flow_type_id_test;
mod relation_instance_type_id_test;
mod relation_type_id_test;
mod type_definition_test;
mod type_id_type_test;
mod type_namespaced_type_test;

#[tarpaulin::skip]
pub mod utils;
