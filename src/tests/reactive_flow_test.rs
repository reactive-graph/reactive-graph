// TODO: move unit test to plugin


use std::sync::Arc;

use serde_json::json;

use crate::behaviour::{AndGate, DefaultConnector, EntityBehaviour, RelationBehaviour};
use crate::builder::{EntityInstanceBuilder, EntityTypeBuilder, DefaultConnectorBuilder, FlowBuilder};
use std::convert::{TryFrom, TryInto};
use crate::reactive::LogicalGateProperties;
use uuid::Uuid;
use indradb::{EdgeKey, Type};
use std::str::FromStr;

const LHS: LogicalGateProperties = LogicalGateProperties::LHS;
const RHS: LogicalGateProperties = LogicalGateProperties::RHS;
const BIT_3: &'static str = "bit_3";
const RESULT: LogicalGateProperties = LogicalGateProperties::RESULT;

#[test]
fn reactive_flow_test () {
    let e_wrapper = EntityInstanceBuilder::new("and_3".to_string())
        .property(LHS.to_string(), json!(false))
        .property(RHS.to_string(), json!(false))
        .property(BIT_3.to_string(), json!(false))
        .property(RESULT.to_string(), json!(false))
        .get();
    let wrapper = Arc::new(ReactiveEntityInstance::from(e_wrapper));

    let and1 = AndGate::new().unwrap();
    let and2 = AndGate::new().unwrap();

    let r_wrapper_bit_1_and1_bit_1 = Arc::new(create_default_connector(wrapper.clone(), and1.entity.clone(), LHS.to_string(), LHS.to_string()));
    let r_wrapper_bit_2_and1_bit_2 = Arc::new(create_default_connector(wrapper.clone(), and1.entity.clone(), RHS.to_string(), RHS.to_string()));
    let r_wrapper_bit_3_and2_bit_1 = Arc::new(create_default_connector(wrapper.clone(), and2.entity.clone(), BIT_3.to_string(), LHS.to_string()));
    let r_and1_result_1_and2_bit_2 = Arc::new(create_default_connector(and1.entity.clone(), and2.entity.clone(), RESULT.to_string(), RHS.to_string()));
    let r_and2_result_1_wrapper_result_1 = Arc::new(create_default_connector(and2.entity.clone(), wrapper.clone(), RESULT.to_string(), RESULT.to_string()));

    let c_wrapper_bit_1_and1_bit_1 = DefaultConnector::from_relation_instance(r_wrapper_bit_1_and1_bit_1.clone()).unwrap();
    let c_wrapper_bit_2_and1_bit_2 = DefaultConnector::from_relation_instance(r_wrapper_bit_2_and1_bit_2.clone()).unwrap();
    let c_wrapper_bit_3_and2_bit_1 = DefaultConnector::from_relation_instance(r_wrapper_bit_3_and2_bit_1.clone()).unwrap();
    let c_and1_result_1_and2_bit_2 = DefaultConnector::from_relation_instance(r_and1_result_1_and2_bit_2.clone()).unwrap();
    let c_and2_result_1_wrapper_result_1 = DefaultConnector::from_relation_instance(r_and2_result_1_wrapper_result_1.clone()).unwrap();

    let flow = ReactiveFlow::new(wrapper.clone());
    flow.add_entity(and1.entity.clone());
    flow.add_entity(and2.entity.clone());
    flow.add_relation(c_wrapper_bit_1_and1_bit_1.relation.clone());
    flow.add_relation(c_wrapper_bit_2_and1_bit_2.relation.clone());
    flow.add_relation(c_wrapper_bit_3_and2_bit_1.relation.clone());
    flow.add_relation(c_and1_result_1_and2_bit_2.relation.clone());
    flow.add_relation(c_and2_result_1_wrapper_result_1.relation.clone());

    assert!(flow.has_entity(and1.entity.clone()));
    assert!(flow.has_entity(and2.entity.clone()));
    assert!(flow.has_entity(wrapper.clone()));

    assert!(flow.get_entity(and1.entity.id).is_some());
    assert!(flow.get_entity(and2.entity.id).is_some());
    assert!(!flow.get_entity(Uuid::new_v4()).is_some());

    assert!(flow.has_relation(r_wrapper_bit_1_and1_bit_1.clone()));
    assert!(flow.has_relation(r_wrapper_bit_2_and1_bit_2.clone()));
    assert!(flow.has_relation(r_wrapper_bit_3_and2_bit_1.clone()));
    assert!(flow.has_relation(r_and1_result_1_and2_bit_2.clone()));
    assert!(flow.has_relation(r_and2_result_1_wrapper_result_1.clone()));

    assert!(flow.get_relation(r_wrapper_bit_1_and1_bit_1.get_key().unwrap()).is_some());
    assert!(flow.get_relation(r_wrapper_bit_2_and1_bit_2.get_key().unwrap()).is_some());
    assert!(flow.get_relation(r_wrapper_bit_3_and2_bit_1.get_key().unwrap()).is_some());
    assert!(flow.get_relation(r_and1_result_1_and2_bit_2.get_key().unwrap()).is_some());
    assert!(flow.get_relation(r_and2_result_1_wrapper_result_1.get_key().unwrap()).is_some());

    // Non-existing key
    let edge_key = EdgeKey::new(Uuid::new_v4(), Type::from_str(r_string().as_str()).unwrap(), Uuid::new_v4());
    assert!(!flow.get_relation(edge_key).is_some());

    // From "outside" we only see the wrapper
    wrapper.set(LHS.to_string(), json!(true));
    assert_eq!(false, wrapper.as_bool(RESULT.to_string()).unwrap());
    wrapper.set(RHS.to_string(), json!(true));
    assert_eq!(false, wrapper.as_bool(RESULT.to_string()).unwrap());
    wrapper.set(BIT_3.to_string(), json!(true));
    assert_eq!(true, wrapper.as_bool(RESULT.to_string()).unwrap());

}

#[test]
fn reactive_flow_from_flow_test () {
    let t_and = EntityTypeBuilder::new("and".to_string())
        .property(LHS.to_string(), DataType::Bool)
        .property(RHS.to_string(), DataType::Bool)
        .property(RESULT.to_string(), DataType::Bool)
        .build();

    let t_and3 = EntityTypeBuilder::new("and3".to_string())
        .property(LHS.to_string(), DataType::Bool)
        .property(RHS.to_string(), DataType::Bool)
        .property(BIT_3.to_string(), DataType::Bool)
        .property(RESULT.to_string(), DataType::Bool)
        .build();

    let e_wrapper = EntityInstanceBuilder::from(t_and3.clone())
        .property(BIT_3.to_string(), json!(false))
        .get();
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
        .outbound(e_wrapper.clone(), BIT_3.to_string())
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


    let flow = FlowBuilder::new(e_wrapper.clone())
        .name("AND-3".to_string())
        .entity(e_and1.clone())
        .entity(e_and2.clone())
        .relation(r_wrapper_bit_1_and1_bit_1.clone())
        .relation(r_wrapper_bit_2_and1_bit_2.clone())
        .relation(r_wrapper_bit_3_and2_bit_1.clone())
        .relation(r_and1_result_1_and2_bit_2.clone())
        .relation(r_and2_result_1_wrapper_result_1.clone())
        .get();
    // println!("{}", serde_json::to_string_pretty(&flow.clone()).unwrap());

    let reactive_flow = ReactiveFlow::try_from(flow.clone());
    assert!(reactive_flow.is_ok());
    let reactive_flow = Arc::new(reactive_flow.unwrap());

    let flow2: Flow = reactive_flow.clone().try_into().unwrap();
    // println!("{}", serde_json::to_string_pretty(&flow2.clone()).unwrap());
    assert_eq!(flow.id, flow2.id);

    // Check if the reactive entity instances have been created
    assert!(reactive_flow.has_entity_by_id(e_wrapper.id));
    assert!(reactive_flow.has_entity_by_id(e_and1.id));
    assert!(reactive_flow.has_entity_by_id(e_and2.id));

    // Get the reactive entity instances
    let re_wrapper = reactive_flow.get_entity(e_wrapper.id);
    assert!(re_wrapper.is_some());
    let re_wrapper = re_wrapper.unwrap();
    let re_and1 = reactive_flow.get_entity(e_and1.id);
    assert!(re_and1.is_some());
    let re_and1 = re_and1.unwrap();
    let re_and2 = reactive_flow.get_entity(e_and2.id);
    assert!(re_and2.is_some());
    let re_and2 = re_and2.unwrap();

    // Check if the reactive relation instances have been created
    assert!(reactive_flow.has_relation_by_key(r_wrapper_bit_1_and1_bit_1.get_key().unwrap()));
    assert!(reactive_flow.has_relation_by_key(r_wrapper_bit_2_and1_bit_2.get_key().unwrap()));
    assert!(reactive_flow.has_relation_by_key(r_wrapper_bit_3_and2_bit_1.get_key().unwrap()));
    assert!(reactive_flow.has_relation_by_key(r_and1_result_1_and2_bit_2.get_key().unwrap()));
    assert!(reactive_flow.has_relation_by_key(r_and2_result_1_wrapper_result_1.get_key().unwrap()));

    // Get the reactive relation instances
    let rr_wrapper_bit_1_and1_bit_1 = reactive_flow.get_relation(r_wrapper_bit_1_and1_bit_1.get_key().unwrap());
    assert!(rr_wrapper_bit_1_and1_bit_1.is_some());
    let rr_wrapper_bit_1_and1_bit_1 = rr_wrapper_bit_1_and1_bit_1.unwrap();
    assert!(Arc::ptr_eq(&re_wrapper, &rr_wrapper_bit_1_and1_bit_1.outbound));
    assert!(Arc::ptr_eq(&re_and1, &rr_wrapper_bit_1_and1_bit_1.inbound));

    let rr_wrapper_bit_2_and1_bit_2 = reactive_flow.get_relation(r_wrapper_bit_2_and1_bit_2.get_key().unwrap());
    assert!(rr_wrapper_bit_2_and1_bit_2.is_some());
    let rr_wrapper_bit_2_and1_bit_2 = rr_wrapper_bit_2_and1_bit_2.unwrap();
    assert!(Arc::ptr_eq(&re_wrapper, &rr_wrapper_bit_2_and1_bit_2.outbound));
    assert!(Arc::ptr_eq(&re_and1, &rr_wrapper_bit_2_and1_bit_2.inbound));

    let rr_wrapper_bit_3_and2_bit_1 = reactive_flow.get_relation(r_wrapper_bit_3_and2_bit_1.get_key().unwrap());
    assert!(rr_wrapper_bit_3_and2_bit_1.is_some());
    let rr_wrapper_bit_3_and2_bit_1 = rr_wrapper_bit_3_and2_bit_1.unwrap();
    assert!(Arc::ptr_eq(&re_wrapper, &rr_wrapper_bit_3_and2_bit_1.outbound));
    assert!(Arc::ptr_eq(&re_and2, &rr_wrapper_bit_3_and2_bit_1.inbound));

    let rr_and1_result_1_and2_bit_2 = reactive_flow.get_relation(r_and1_result_1_and2_bit_2.get_key().unwrap());
    assert!(rr_and1_result_1_and2_bit_2.is_some());
    let rr_and1_result_1_and2_bit_2 = rr_and1_result_1_and2_bit_2.unwrap();
    assert!(Arc::ptr_eq(&re_and1, &rr_and1_result_1_and2_bit_2.outbound));
    assert!(Arc::ptr_eq(&re_and2, &rr_and1_result_1_and2_bit_2.inbound));

    let rr_and2_result_1_wrapper_result_1 = reactive_flow.get_relation(r_and2_result_1_wrapper_result_1.get_key().unwrap());
    assert!(rr_and2_result_1_wrapper_result_1.is_some());
    let rr_and2_result_1_wrapper_result_1 = rr_and2_result_1_wrapper_result_1.unwrap();
    assert!(Arc::ptr_eq(&re_and2, &rr_and2_result_1_wrapper_result_1.outbound));
    assert!(Arc::ptr_eq(&re_wrapper, &rr_and2_result_1_wrapper_result_1.inbound));

    // The reactive flow doesn't have behaviours yet

    // Create the entity behaviours
    let b_and1 = AndGate::from_entity_instance(re_and1.clone()).ok();
    assert!(b_and1.is_some());
    let b_and1 = b_and1.unwrap();
    assert!(Arc::ptr_eq(&re_and1, &b_and1.entity));


    let b_and2 = AndGate::from_entity_instance(re_and2.clone()).ok();
    assert!(b_and2.is_some());
    let b_and2 = b_and2.unwrap();
    assert!(Arc::ptr_eq(&re_and2, &b_and2.entity));

    // Create the relation behaviours
    let b_wrapper_bit_1_and1_bit_1 = DefaultConnector::from_relation_instance(rr_wrapper_bit_1_and1_bit_1.clone()).ok();
    assert!(b_wrapper_bit_1_and1_bit_1.is_some());
    let b_wrapper_bit_1_and1_bit_1 = b_wrapper_bit_1_and1_bit_1.unwrap();
    b_wrapper_bit_1_and1_bit_1.relation.outbound.set(LHS.to_string(), json!(true));

    let b_wrapper_bit_2_and1_bit_2 = DefaultConnector::from_relation_instance(rr_wrapper_bit_2_and1_bit_2.clone()).ok();
    assert!(b_wrapper_bit_2_and1_bit_2.is_some());
    let _b_wrapper_bit_2_and1_bit_2 = b_wrapper_bit_2_and1_bit_2.unwrap();
    let b_wrapper_bit_3_and2_bit_1 = DefaultConnector::from_relation_instance(rr_wrapper_bit_3_and2_bit_1.clone()).ok();
    assert!(b_wrapper_bit_3_and2_bit_1.is_some());
    let _b_wrapper_bit_3_and2_bit_1 = b_wrapper_bit_3_and2_bit_1.unwrap();
    let b_and1_result_1_and2_bit_2 = DefaultConnector::from_relation_instance(rr_and1_result_1_and2_bit_2.clone()).ok();
    assert!(b_and1_result_1_and2_bit_2.is_some());
    let _b_and1_result_1_and2_bit_2 = b_and1_result_1_and2_bit_2.unwrap();
    let b_and2_result_1_wrapper_result_1 = DefaultConnector::from_relation_instance(rr_and2_result_1_wrapper_result_1.clone()).ok();
    assert!(b_and2_result_1_wrapper_result_1.is_some());
    let _b_and2_result_1_wrapper_result_1 = b_and2_result_1_wrapper_result_1.unwrap();

    // From "outside" we only see the "wrapper", but we can also inspect the inner of the flow
    reactive_flow.set(LHS.to_string(), json!(true));
    assert_eq!(true, reactive_flow.as_bool(LHS.to_string()).unwrap());
    assert_eq!(true, b_and1.entity.as_bool(LHS.to_string()).unwrap());
    assert_eq!(false, b_and2.entity.as_bool(LHS.to_string()).unwrap());
    assert_eq!(false, reactive_flow.as_bool(RESULT.to_string()).unwrap());
    reactive_flow.set(RHS.to_string(), json!(true));
    assert_eq!(true, reactive_flow.as_bool(RHS.to_string()).unwrap());
    assert_eq!(true, b_and1.entity.as_bool(RHS.to_string()).unwrap());
    assert_eq!(true, b_and1.entity.as_bool(RESULT.to_string()).unwrap());
    assert_eq!(true, b_and2.entity.as_bool(RHS.to_string()).unwrap());
    assert_eq!(false, reactive_flow.as_bool(RESULT.to_string()).unwrap());
    reactive_flow.set(BIT_3.to_string(), json!(true));
    assert_eq!(true, reactive_flow.as_bool(BIT_3.to_string()).unwrap());
    assert_eq!(true, b_and2.entity.as_bool(LHS.to_string()).unwrap());
    assert_eq!(true, b_and2.entity.as_bool(RESULT.to_string()).unwrap());
    assert_eq!(true, reactive_flow.as_bool(RESULT.to_string()).unwrap());

}

#[test]
fn reactive_flow_from_flow_compact_test () {
    let t_and = EntityTypeBuilder::new("and".to_string())
        .property(LHS.to_string(), DataType::Bool)
        .property(RHS.to_string(), DataType::Bool)
        .property(RESULT.to_string(), DataType::Bool)
        .build();

    let t_and3 = EntityTypeBuilder::new("and3".to_string())
        .property(LHS.to_string(), DataType::Bool)
        .property(RHS.to_string(), DataType::Bool)
        .property(BIT_3.to_string(), DataType::Bool)
        .property(RESULT.to_string(), DataType::Bool)
        .build();

    let e_wrapper = EntityInstanceBuilder::from(t_and3.clone())
        .property(BIT_3.to_string(), json!(false))
        .get();
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
        .outbound(e_wrapper.clone(), BIT_3.to_string())
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


    let flow = FlowBuilder::new(e_wrapper.clone())
        .name("AND-3".to_string())
        .entity(e_and1.clone())
        .entity(e_and2.clone())
        .relation(r_wrapper_bit_1_and1_bit_1.clone())
        .relation(r_wrapper_bit_2_and1_bit_2.clone())
        .relation(r_wrapper_bit_3_and2_bit_1.clone())
        .relation(r_and1_result_1_and2_bit_2.clone())
        .relation(r_and2_result_1_wrapper_result_1.clone())
        .get();

    let reactive_flow = ReactiveFlow::try_from(flow.clone());
    assert!(reactive_flow.is_ok());
    let reactive_flow = reactive_flow.unwrap();

    // The reactive flow doesn't have behaviours yet

    // Create the entity behaviours
    let _b_and1 = AndGate::from_entity_instance(reactive_flow.get_entity(e_and1.id).unwrap()).unwrap();
    let _b_and2 = AndGate::from_entity_instance(reactive_flow.get_entity(e_and2.id).unwrap()).unwrap();

    // Create the relation behaviours
    let _b_wrapper_bit_1_and1_bit_1 = DefaultConnector::from_relation_instance(reactive_flow.get_relation(r_wrapper_bit_1_and1_bit_1.get_key().unwrap()).unwrap()).unwrap();
    let _b_wrapper_bit_2_and1_bit_2 = DefaultConnector::from_relation_instance(reactive_flow.get_relation(r_wrapper_bit_2_and1_bit_2.get_key().unwrap()).unwrap()).unwrap();
    let _b_wrapper_bit_3_and2_bit_1 = DefaultConnector::from_relation_instance(reactive_flow.get_relation(r_wrapper_bit_3_and2_bit_1.get_key().unwrap()).unwrap()).unwrap();
    let _b_and1_result_1_and2_bit_2 = DefaultConnector::from_relation_instance(reactive_flow.get_relation(r_and1_result_1_and2_bit_2.get_key().unwrap()).unwrap()).unwrap();
    let _b_and2_result_1_wrapper_result_1 = DefaultConnector::from_relation_instance(reactive_flow.get_relation(r_and2_result_1_wrapper_result_1.get_key().unwrap()).unwrap()).unwrap();

    // From "outside" we only see the "wrapper", but we can also inspect the inner of the flow
    reactive_flow.set(LHS.to_string(), json!(true));
    assert_eq!(true, reactive_flow.as_bool(LHS.to_string()).unwrap());
    assert_eq!(false, reactive_flow.as_bool(RESULT.to_string()).unwrap());
    reactive_flow.set(RHS.to_string(), json!(true));
    assert_eq!(true, reactive_flow.as_bool(RHS.to_string()).unwrap());
    assert_eq!(false, reactive_flow.as_bool(RESULT.to_string()).unwrap());
    reactive_flow.set(BIT_3.to_string(), json!(true));
    assert_eq!(true, reactive_flow.as_bool(BIT_3.to_string()).unwrap());
    assert_eq!(true, reactive_flow.as_bool(RESULT.to_string()).unwrap());

}
