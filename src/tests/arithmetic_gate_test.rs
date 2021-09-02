// TODO: move unit test to plugin


use std::sync::Arc;

use serde_json::{json, Value};

use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter};
// use crate::behaviour::{AddGate, ArithmeticGateBehaviour, ConnectorBehaviour, DefaultConnector, EntityBehaviour};
use crate::model::ReactiveEntityInstanceFactory;
use crate::relation::connector::Connector;
use crate::entity::arithmetic_gate::{ArithmeticGate, ArithmeticGateProperties};
use crate::entity::arithmetic_gate::ArithmeticGateReactiveEntityInstanceFactory as Factory;
use crate::tests::utils::create_default_connector;

const LHS: ArithmeticGateProperties = ArithmeticGateProperties::LHS;
const RHS: ArithmeticGateProperties = ArithmeticGateProperties::RHS;
const RESULT: ArithmeticGateProperties = ArithmeticGateProperties::RESULT;

const ADD_GATE_TYPE_NAME: &str = "add";
const ADD_GATE_OPERATION: fn(i64, i64) -> i64 = |lhs, rhs| lhs + rhs;

const DEFAULT_CONNECTOR_OPERATION: fn(Value) -> Value = |v| v.clone();

#[test]
fn add_gate_type_test () {
    let and = Factory::new(ADD_GATE_TYPE_NAME);
    let and_gate = ArithmeticGate::new(and.clone(), ADD_GATE_OPERATION);
    assert_eq!(ADD_GATE_TYPE_NAME.to_string(), and_gate.type_name());
}

#[test]
fn add_gate_test () {
    let add = Factory::new(ADD_GATE_TYPE_NAME);
    {
        // Create the ADD-Gate in scope
        let add_gate = ArithmeticGate::new(add.clone(), ADD_GATE_OPERATION);
        assert_ne!(0, add_gate.handle_id);

        // add_gate.lhs(json!(1));
        add.set(LHS.as_ref(), json!(1));
        // add_gate.rhs(json!(1));
        add.set(RHS.to_string(), json!(1));
        assert_eq!(2, add.as_i64(RESULT.to_string()).unwrap());
        add.set(RHS.to_string(), json!(2));
        assert_eq!(3, add.as_i64(RESULT.to_string()).unwrap());
        add.set(LHS.to_string(), json!(2));
        assert_eq!(4, add.as_i64(RESULT.to_string()).unwrap());
    } // The ArithmeticGate no more alive ...
    // ... so, setting the inputs ...
    add.set(LHS.to_string(), json!(0));
    add.set(RHS.to_string(), json!(0));
    // ... doesn't affect the result anymore (result should have the same value as before)
    assert_eq!(4, add.as_i64(RESULT.to_string()).unwrap());
}

/// The results of two ADD-Gates are the inputs of the third ADD-Gate
#[test]
fn three_add_gates_test () {
    let add_1 = Factory::new(ADD_GATE_TYPE_NAME);
    let add_2 = Factory::new(ADD_GATE_TYPE_NAME);
    let add_3 = Factory::new(ADD_GATE_TYPE_NAME);

    // Reset all states
    add_1.set(LHS.to_string(), json!(0));
    add_1.set(RHS.to_string(), json!(0));
    add_1.set(RESULT.to_string(), json!(0));

    add_2.set(LHS.to_string(), json!(0));
    add_2.set(RHS.to_string(), json!(0));
    add_2.set(RESULT.to_string(), json!(0));

    add_3.set(LHS.to_string(), json!(0));
    add_3.set(RHS.to_string(), json!(0));
    add_3.set(RESULT.to_string(), json!(0));

    // Make the entity instances act as AND-Gates
    let add_gate_1 = ArithmeticGate::new(add_1.clone(), ADD_GATE_OPERATION);
    assert_ne!(0, add_gate_1.handle_id);

    let add_gate_2 = ArithmeticGate::new(add_2.clone(), ADD_GATE_OPERATION);
    assert_ne!(0, add_gate_2.handle_id);

    let add_gate_3 = ArithmeticGate::new(add_3.clone(), ADD_GATE_OPERATION);
    assert_ne!(0, add_gate_3.handle_id);

    // Connect the results of the first two AND-Gates with the inputs of the third AND-Gate
    let r_add_1_add_3 = Arc::new(create_default_connector(
        add_1.clone(),
        add_3.clone(),
        RESULT.to_string(),
        LHS.to_string()
    ));
    let c_add_1_add_3 = Connector::from_relation(r_add_1_add_3.clone(), DEFAULT_CONNECTOR_OPERATION);
    assert_ne!(0, c_add_1_add_3.handle_id);

    let r_add_2_add_3 = Arc::new(create_default_connector(
        add_2.clone(),
        add_3.clone(),
        RESULT.to_string(),
        RHS.to_string()
    ));
    let c_add_2_add_3 = Connector::from_relation(r_add_2_add_3.clone(), DEFAULT_CONNECTOR_OPERATION);
    assert_ne!(0, c_add_2_add_3.handle_id);

    // Starting point
    assert_eq!(0, add_1.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(0, add_2.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(0, add_3.as_i64(RESULT.to_string()).unwrap());

    add_1.set(LHS.as_ref(), json!(1));
    assert_eq!(1, add_1.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(0, add_2.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(1, add_3.as_i64(RESULT.to_string()).unwrap());

    add_1.set(RHS.to_string(), json!(1));
    assert_eq!(2, add_1.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(0, add_2.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(2, add_3.as_i64(RESULT.to_string()).unwrap());

    add_2.set(LHS.as_ref(), json!(1));
    assert_eq!(2, add_1.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(1, add_2.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(3, add_3.as_i64(RESULT.to_string()).unwrap());

    add_2.set(RHS.to_string(), json!(1));
    assert_eq!(2, add_1.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(2, add_2.as_i64(RESULT.to_string()).unwrap());
    assert_eq!(4, add_3.as_i64(RESULT.to_string()).unwrap());
}
