// TODO: move unit test to plugin


use std::f64::consts::PI;

use serde_json::json;

use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter};
// use crate::behaviour::{EntityBehaviour, NumericOperationBehaviour};
use crate::model::ReactiveEntityFactory;
use crate::entity::numeric_operation::{NumericOperation, NumericOperationProperties};
use crate::entity::numeric_operation::NumericOperationReactiveEntityFactory as Factory;

const LHS: NumericOperationProperties = NumericOperationProperties::LHS;
const RESULT: NumericOperationProperties = NumericOperationProperties::RESULT;

const SIN_GATE_TYPE_NAME: &str = "sin";
const SIN_GATE_OPERATION: fn(f64) -> f64 = |lhs: f64| lhs.sin();

const COS_GATE_TYPE_NAME: &str = "cos";
const COS_GATE_OPERATION: fn(f64) -> f64 = |lhs: f64| lhs.cos();


#[test]
fn numeric_operation_sin_type_test () {
    let sin_entity = Factory::new(SIN_GATE_TYPE_NAME);
    let sin_gate = NumericOperation::new(sin_entity.clone(), SIN_GATE_OPERATION);
    assert_eq!(SIN_GATE_TYPE_NAME.to_string(), sin_gate.type_name());
}

#[test]
fn numeric_operation_sin_test () {
    let sin_entity = Factory::new(SIN_GATE_TYPE_NAME);
    {
        let sin_gate = NumericOperation::new(sin_entity.clone(), SIN_GATE_OPERATION);
        assert_ne!(0, sin_gate.handle_id);

        sin_entity.set(LHS.to_string(), json!(0.0));
        assert_eq!(0.0, sin_entity.as_f64(RESULT.to_string()).unwrap());

        sin_entity.set(LHS.to_string(), json!(PI / 2.0));
        assert_eq!(1.0, sin_entity.as_f64(RESULT.to_string()).unwrap());

    } // The TrigonometricGate no more alive ...
    sin_entity.set(LHS.to_string(), json!(0.0));
    assert_eq!(1.0, sin_entity.as_f64(RESULT.to_string()).unwrap());
}

#[test]
fn numeric_operation_cos_type_test () {
    let cos_entity = Factory::new(COS_GATE_TYPE_NAME);
    let cos_gate = NumericOperation::new(cos_entity.clone(), COS_GATE_OPERATION);
    assert_eq!(COS_GATE_TYPE_NAME.to_string(), cos_gate.type_name());
}

#[test]
fn numeric_operation_cos_test () {
    let cos_entity = Factory::new(COS_GATE_TYPE_NAME);
    {
        let cos_gate = NumericOperation::new(cos_entity.clone(), COS_GATE_OPERATION);
        assert_ne!(0, cos_gate.handle_id);

        cos_entity.set(LHS.to_string(), json!(0.0));
        assert_eq!(1.0, cos_entity.as_f64(RESULT.to_string()).unwrap());

        cos_entity.set(LHS.to_string(), json!(PI / 2.0));
        assert!(assert_approx(0.0, cos_entity.as_f64(RESULT.to_string()).unwrap()));
    } // The TrigonometricGate no more alive ...
    cos_entity.set(LHS.to_string(), json!(0.0));
    assert!(assert_approx(0.0, cos_entity.as_f64(RESULT.to_string()).unwrap()));
}

fn assert_approx(expected: f64, value: f64) -> bool {
    value > expected - 0.00000001 && value < expected + 0.00000001
}
