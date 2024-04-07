// TODO: Should this unit test moved to somewhere else ?
// TODO: where are the builders ?
// TODO: Mock the ConnectorProperties ?
// TODO: Mock the LogicalGateProperties ?

use crate::builder::DefaultConnectorBuilder;
use crate::builder::EntityInstanceBuilder;
use crate::builder::EntityTypeBuilder;
use crate::builder::FlowInstanceBuilder;
use crate::builder::RelationInstanceBuilder;
use crate::connector::Connector;
use crate::entity::logical_gate::LogicalGateProperties;
use crate::relation::connector::ConnectorProperties;
use crate::tests::utils::r_string;
use reactive_graph_graph::DataType;
use reactive_graph_graph::Flow;
use serde_json::json;

const LHS: LogicalGateProperties = LogicalGateProperties::LHS;
const RHS: LogicalGateProperties = LogicalGateProperties::RHS;
const RESULT: LogicalGateProperties = LogicalGateProperties::RESULT;

const DEFAULT_CONNECTOR_TYPE_NAME: &str = "default_connector";

#[test]
fn flow_test() {
    // AND-3: Wraps two AND-Gates
    //
    // The flow wrapper entity has 3 input-bits and 1 output-bit.
    //
    // Within the flow there are 2 AND-Gates.
    //
    // The wrapper entity acts like an AND-Gate with 3 bits.
    //
    //    ________________________________________
    //   |                                       |
    //   |    _______       _____       _____    |
    //   |   |      |      |    |      |    |    |
    //   |   | FLOW |B1--B1| && |R1--B1|    |    |
    //   |   | WRAP |B2--B2|____|      | && |R1--|
    //   |   | PER  |                  |    |
    //   --R1|      |B3--------------B2|    |
    //       |______|                  |____|
    //
    //
    // This is intentional very low level - on higher levels it's of course more convenient and
    // not that much boilerplate. It just represents the entities and relations on a very low
    // level.

    let e_wrapper = EntityInstanceBuilder::new("and_3".to_string())
        .property(LHS.to_string(), json!(false))
        .property(RHS.to_string(), json!(false))
        .property("bit_3".to_string(), json!(false))
        .property(RESULT.to_string(), json!(false))
        .get();

    let e_and1 = EntityInstanceBuilder::new("and".to_string())
        .property(LHS.to_string(), json!(false))
        .property(RHS.to_string(), json!(false))
        .property(RESULT.to_string(), json!(false))
        .get();

    let e_and2 = EntityInstanceBuilder::new("and".to_string())
        .property(LHS.to_string(), json!(false))
        .property(RHS.to_string(), json!(false))
        .property(RESULT.to_string(), json!(false))
        .get();

    let r_wrapper_bit_1_and1_bit_1 =
        RelationInstanceBuilder::new(e_wrapper.id, Connector::type_name(DEFAULT_CONNECTOR_TYPE_NAME, LHS.as_ref(), LHS.as_ref()), e_and1.id)
            .property(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string(), json!(LHS.as_ref()))
            .property(ConnectorProperties::INBOUND_PROPERTY_NAME.to_string(), json!(LHS.as_ref()))
            .get();

    let r_wrapper_bit_2_and1_bit_2 =
        RelationInstanceBuilder::new(e_wrapper.id, Connector::type_name(DEFAULT_CONNECTOR_TYPE_NAME, RHS.as_ref(), RHS.as_ref()), e_and1.id)
            .property(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string(), json!(RHS.as_ref()))
            .property(ConnectorProperties::INBOUND_PROPERTY_NAME.to_string(), json!(RHS.as_ref()))
            .get();

    let r_wrapper_bit_3_and2_bit_1 =
        RelationInstanceBuilder::new(e_wrapper.id, Connector::type_name(DEFAULT_CONNECTOR_TYPE_NAME, "bit_3", LHS.as_ref()), e_and2.id)
            .property(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string(), json!("bit_3"))
            .property(ConnectorProperties::INBOUND_PROPERTY_NAME.to_string(), json!(LHS.as_ref()))
            .get();

    let r_and1_result_1_and2_bit_2 =
        RelationInstanceBuilder::new(e_and1.id, Connector::type_name(DEFAULT_CONNECTOR_TYPE_NAME, RESULT.as_ref(), RHS.as_ref()), e_and2.id)
            .property(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string(), json!(RESULT.as_ref()))
            .property(ConnectorProperties::INBOUND_PROPERTY_NAME.to_string(), json!(RHS.as_ref()))
            .get();

    let r_and2_result_1_wrapper_result_1 =
        RelationInstanceBuilder::new(e_and2.id, Connector::type_name(DEFAULT_CONNECTOR_TYPE_NAME, RESULT.as_ref(), RESULT.as_ref()), e_wrapper.id)
            .property(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string(), json!(RESULT.as_ref()))
            .property(ConnectorProperties::INBOUND_PROPERTY_NAME.to_string(), json!(RESULT.as_ref()))
            .get();

    let mut flow = Flow::from(e_wrapper.clone());
    flow.entity_instances.push(e_and1.clone());
    flow.entity_instances.push(e_and2.clone());
    flow.relation_instances.push(r_wrapper_bit_1_and1_bit_1.clone());
    flow.relation_instances.push(r_wrapper_bit_2_and1_bit_2.clone());
    flow.relation_instances.push(r_wrapper_bit_3_and2_bit_1.clone());
    flow.relation_instances.push(r_and1_result_1_and2_bit_2.clone());
    flow.relation_instances.push(r_and2_result_1_wrapper_result_1.clone());

    // TODO: unit tests
}

#[test]
fn flow_builder_test() {
    let t_and = EntityTypeBuilder::new("and".to_string())
        .property(LHS.to_string(), DataType::Bool)
        .property(RHS.to_string(), DataType::Bool)
        .property(RESULT.to_string(), DataType::Bool)
        .build();

    let e_wrapper = EntityInstanceBuilder::from(t_and.clone()).property("bit_3".to_string(), json!(false)).get();
    let e_and1 = EntityInstanceBuilder::from(t_and.clone()).get();
    let e_and2 = EntityInstanceBuilder::from(t_and.clone()).get();
    let r_wrapper_bit_1_and1_bit_1 = DefaultConnectorBuilder::new()
        .outbound(e_wrapper.clone(), LHS.to_string())
        .inbound(e_and1.clone(), LHS.to_string())
        .get();
    let r_wrapper_bit_2_and1_bit_2 = DefaultConnectorBuilder::new()
        .outbound(e_wrapper.clone(), RHS.to_string())
        .inbound(e_and1.clone(), RHS.to_string())
        .get();
    let r_wrapper_bit_3_and2_bit_1 = DefaultConnectorBuilder::new()
        .outbound(e_wrapper.clone(), "bit_3".to_string())
        .inbound(e_and2.clone(), LHS.to_string())
        .get();
    let r_and1_result_1_and2_bit_2 = DefaultConnectorBuilder::new()
        .outbound(e_and1.clone(), RESULT.to_string())
        .inbound(e_and2.clone(), RHS.to_string())
        .get();
    let r_and2_result_1_wrapper_result_1 = DefaultConnectorBuilder::new()
        .outbound(e_and2.clone(), RESULT.to_string())
        .inbound(e_wrapper.clone(), RESULT.to_string())
        .get();

    let _flow = FlowInstanceBuilder::new(e_wrapper.clone())
        .name("AND-3".to_string())
        .entity(e_and1.clone())
        .entity(e_and2.clone())
        .relation(r_wrapper_bit_1_and1_bit_1.clone())
        .relation(r_wrapper_bit_2_and1_bit_2.clone())
        .relation(r_wrapper_bit_3_and2_bit_1.clone())
        .relation(r_and1_result_1_and2_bit_2.clone())
        .relation(r_and2_result_1_wrapper_result_1.clone())
        .get();

    // TODO: unit tests
}

#[test]
fn flow_from_instance_with_name_test() {
    let t_and = EntityTypeBuilder::new("and".to_string())
        .property(LHS.to_string(), DataType::Bool)
        .property(RHS.to_string(), DataType::Bool)
        .property(RESULT.to_string(), DataType::Bool)
        .build();

    let e_wrapper = EntityInstanceBuilder::from(t_and.clone()).property("bit_3".to_string(), json!(false)).get();

    let flow_name = r_string();
    let flow = Flow::from_instance_with_name(e_wrapper.clone(), flow_name.clone());
    assert_eq!(flow_name, flow.name);
    assert_eq!(1, flow.entity_instances.len());
    assert_eq!(e_wrapper.id, flow.id);
}
