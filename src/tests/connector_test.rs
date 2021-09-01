use std::collections::HashMap;
use std::sync::Arc;

use serde_json::{json, Value};

// use crate::behaviour::{ConnectorBehaviour, DefaultConnector, ParseIntConnector, RelationBehaviour, ToStringConnector};
use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::model::{ReactiveEntityInstance, ReactiveRelationInstance};
use crate::relation::connector::{Connector, ConnectorFunction};
use crate::relation::connector::ConnectorProperties;
use crate::tests::utils::create_random_entity_instance;
use crate::tests::utils::r_string;

const DEFAULT_CONNECTOR_TYPE_NAME: &str = "default_connector";
const DEFAULT_CONNECTOR_OPERATION: fn(Value) -> Value = |v| v.clone();

const TO_STRING_CONNECTOR_TYPE_NAME: &str = "to_string_connector";
const TO_STRING_CONNECTOR_OPERATION: fn(Value) -> Value = |v| json!(v.clone().to_string());

const PARSE_INT_CONNECTOR_TYPE_NAME: &str = "parse_int_connector";
const PARSE_INT_CONNECTOR_OPERATION: fn(Value) -> Value = |v| json!(v.clone().as_str().unwrap().parse::<i64>().unwrap());

#[test]
fn connector_test () {
    let outbound_property_name = r_string();
    let inbound_property_name = r_string();
    let outbound_entity = Arc::new(create_random_entity_instance(outbound_property_name.clone()));
    let inbound_entity = Arc::new(create_random_entity_instance(inbound_property_name.clone()));
    let r = Arc::new(create_default_connector(
        outbound_entity.clone(),
        inbound_entity.clone(),
        outbound_property_name.clone(),
        inbound_property_name.clone()
    ));
    let connector = Connector::from_relation(r.clone(), DEFAULT_CONNECTOR_OPERATION);
    connector.relation.outbound.set(outbound_property_name.clone(), json!(true));
    assert!(connector.relation.inbound.as_bool(inbound_property_name.clone()).unwrap());
    connector.relation.outbound.set(outbound_property_name.clone(), json!(false));
    assert!(!connector.relation.inbound.as_bool(inbound_property_name.clone()).unwrap());
    connector.relation.outbound.set(outbound_property_name.clone(), json!(123));
    assert_eq!(123, connector.relation.inbound.as_u64(inbound_property_name.clone()).unwrap());
    connector.relation.outbound.set(outbound_property_name.clone(), json!(-123));
    assert_eq!(-123, connector.relation.inbound.as_i64(inbound_property_name.clone()).unwrap());
    connector.relation.outbound.set(outbound_property_name.clone(), json!(1.23));
    assert_eq!(1.23, connector.relation.inbound.as_f64(inbound_property_name.clone()).unwrap());
    let s = r_string();
    connector.relation.outbound.set(outbound_property_name.clone(), json!(s.clone()));
    assert_eq!(s, connector.relation.inbound.as_string(inbound_property_name.clone()).unwrap());
}

#[test]
fn connector_multiple_propagation_test () {
    let e1_property_name = r_string();
    let e1 = Arc::new(create_random_entity_instance(e1_property_name.clone()));

    let e2_property_name = r_string();
    let e2 = Arc::new(create_random_entity_instance(e2_property_name.clone()));

    let e3_property_name = r_string();
    let e3 = Arc::new(create_random_entity_instance(e3_property_name.clone()));

    let r_e1_e2 = Arc::new(create_default_connector(
        e1.clone(),
        e2.clone(),
        e1_property_name.clone(),
        e2_property_name.clone()
    ));
    let c_e1_e2 = Connector::from_relation(r_e1_e2.clone(), DEFAULT_CONNECTOR_OPERATION);
    assert_ne!(0, c_e1_e2.handle_id);

    let r_e2_e3 = Arc::new(create_default_connector(
        e2.clone(),
        e3.clone(),
        e2_property_name.clone(),
        e3_property_name.clone()
    ));
    let c_e2_e3 = Connector::from_relation(r_e2_e3.clone(), DEFAULT_CONNECTOR_OPERATION);
    assert_ne!(0, c_e2_e3.handle_id);

    e1.set(e1_property_name.clone(), json!(true));
    assert!(e3.as_bool(e3_property_name.clone()).unwrap());
    e1.set(e1_property_name.clone(), json!(false));
    assert!(!e3.as_bool(e3_property_name.clone()).unwrap());
    e1.set(e1_property_name.clone(), json!(123));
    assert_eq!(123, e3.as_u64(e3_property_name.clone()).unwrap());
    e1.set(e1_property_name.clone(), json!(-123));
    assert_eq!(-123, e3.as_i64(e3_property_name.clone()).unwrap());
    e1.set(e1_property_name.clone(), json!(1.23));
    assert_eq!(1.23, e3.as_f64(e3_property_name.clone()).unwrap());
    let s = r_string();
    e1.set(e1_property_name.clone(), json!(s.clone()));
    assert_eq!(s, e3.as_string(e3_property_name.clone()).unwrap());
}

#[test]
fn connector_destruction_test () {
    let e1_property_name = r_string();
    let e1 = Arc::new(create_random_entity_instance(e1_property_name.clone()));

    let e2_property_name = r_string();
    let e2 = Arc::new(create_random_entity_instance(e2_property_name.clone()));

    let e3_property_name = r_string();
    let e3 = Arc::new(create_random_entity_instance(e3_property_name.clone()));

    let r_e1_e2 = Arc::new(create_default_connector(
        e1.clone(),
        e2.clone(),
        e1_property_name.clone(),
        e2_property_name.clone()
    ));
    let c_e1_e2 = Connector::from_relation(r_e1_e2.clone(), DEFAULT_CONNECTOR_OPERATION);
    assert_ne!(0, c_e1_e2.handle_id);

    let e3_default_string = "unmodified_e3".to_string();
    e1.set(e1_property_name.clone(), json!(e3_default_string.clone()));
    {
        let r_e2_e3 = Arc::new(create_default_connector(
            e2.clone(),
            e3.clone(),
            e2_property_name.clone(),
            e3_property_name.clone()
        ));
        let c_e2_e3 = Connector::from_relation(r_e2_e3.clone(), DEFAULT_CONNECTOR_OPERATION);
        assert_ne!(0, c_e2_e3.handle_id);

        e1.set(e1_property_name.clone(), json!(true));
        assert!(e3.as_bool(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(false));
        assert!(!e3.as_bool(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(123));
        assert_eq!(123, e3.as_u64(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(-123));
        assert_eq!(-123, e3.as_i64(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(1.23));
        assert_eq!(1.23, e3.as_f64(e3_property_name.clone()).unwrap());
        let s = r_string();
        e1.set(e1_property_name.clone(), json!(s.clone()));
        assert_eq!(s, e3.as_string(e3_property_name.clone()).unwrap());

        e1.set(e1_property_name.clone(), json!(e3_default_string.clone()));
    } // Connector c_e2_e3 should be destructed, no more propagation to e3

    assert_eq!(e3_default_string.clone(), e3.as_string(e3_property_name.clone()).unwrap());
    let s = r_string();
    e1.set(e1_property_name.clone(), json!(s.clone()));
    assert_eq!(s, e2.as_string(e2_property_name.clone()).unwrap());
    assert_eq!(e3_default_string.clone().to_string(), e3.as_string(e3_property_name.clone()).unwrap());
}

#[test]
fn connector_connect_test () {
    let e1_property_name = r_string();
    let e1 = Arc::new(create_random_entity_instance(e1_property_name.clone()));
    let e2_property_name = r_string();
    let e2 = Arc::new(create_random_entity_instance(e2_property_name.clone()));
    let e3_property_name = r_string();
    let e3 = Arc::new(create_random_entity_instance(e3_property_name.clone()));
    let c_e1_e2 = Connector::new(
        e1.clone(),
        e1_property_name.clone(),
        e2.clone(),
        e2_property_name.clone()
    );
    assert_ne!(0, c_e1_e2.handle_id);
    let e3_default_string = "unmodified_e3".to_string();
    e1.set(e1_property_name.clone(), json!(e3_default_string.clone()));
    {
        let c_e2_e3 = Connector::new(
            e2.clone(),
            e2_property_name.clone(),
            e3.clone(),
            e3_property_name.clone()
        );
        assert_ne!(0, c_e2_e3.handle_id);
        e1.set(e1_property_name.clone(), json!(true));
        assert!(e3.as_bool(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(false));
        assert!(!e3.as_bool(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(123));
        assert_eq!(123, e3.as_u64(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(-123));
        assert_eq!(-123, e3.as_i64(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(1.23));
        assert_eq!(1.23, e3.as_f64(e3_property_name.clone()).unwrap());
        let s = r_string();
        e1.set(e1_property_name.clone(), json!(s.clone()));
        assert_eq!(s, e3.as_string(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(e3_default_string.clone()));
    } // Connector c_e2_e3 should be destructed, no more propagation to e3

    assert_eq!(e3_default_string.clone(), e3.as_string(e3_property_name.clone()).unwrap());
    let s = r_string();
    e1.set(e1_property_name.clone(), json!(s.clone()));
    assert_eq!(s, e2.as_string(e2_property_name.clone()).unwrap());
    assert_eq!(e3_default_string.clone().to_string(), e3.as_string(e3_property_name.clone()).unwrap());

}

#[test]
fn connector_custom_connector_function_test () {
    let outbound_property_name = r_string();
    let inbound_property_name = r_string();
    let outbound_entity = Arc::new(create_random_entity_instance(outbound_property_name.clone()));
    let inbound_entity = Arc::new(create_random_entity_instance(inbound_property_name.clone()));
    let r = Arc::new(create_default_connector(
        outbound_entity.clone(),
        inbound_entity.clone(),
        outbound_property_name.clone(),
        inbound_property_name.clone()
    ));

    // A custom connector function can be used to transform the data
    // This makes it possible to do:
    // * calculation
    // * data type conversion / casting
    // * simple string operations like trim()
    let custom_connector_function: ConnectorFunction = |v| json!(v.as_i64().unwrap() + 1);

    // The default connector just passes the value:
    // let default_connector_function: ConnectorFunction = |v| v.clone();

    let connector = Connector::from_relation(r.clone(), custom_connector_function);
    connector.relation.outbound.set(outbound_property_name.clone(), json!(-123));
    assert_eq!(-122, connector.relation.inbound.as_i64(inbound_property_name.clone()).unwrap());
}

#[test]
fn connector_to_string_connector_test () {
    let outbound_property_name = r_string();
    let inbound_property_name = r_string();
    let outbound_entity = Arc::new(create_random_entity_instance(outbound_property_name.clone()));
    let inbound_entity = Arc::new(create_random_entity_instance(inbound_property_name.clone()));
    let r = Arc::new(create_relation_instance_with_properties(
        outbound_entity.clone(),
        TO_STRING_CONNECTOR_TYPE_NAME.to_string(),
        inbound_entity.clone(),
        outbound_property_name.clone(),
        inbound_property_name.clone()
    ));
    let connector = Connector::from_relation(r.clone(), TO_STRING_CONNECTOR_OPERATION);
    connector.relation.outbound.set(outbound_property_name.clone(), json!(123));
    assert_eq!("123", connector.relation.inbound.as_string(inbound_property_name.clone()).unwrap());
}

#[test]
fn connector_parse_int_connector_test () {
    let outbound_property_name = r_string();
    let inbound_property_name = r_string();
    let outbound_entity = Arc::new(create_random_entity_instance(outbound_property_name.clone()));
    let inbound_entity = Arc::new(create_random_entity_instance(inbound_property_name.clone()));
    let r = Arc::new(create_relation_instance_with_properties(
        outbound_entity.clone(),
        PARSE_INT_CONNECTOR_TYPE_NAME.to_string(),
        inbound_entity.clone(),
        outbound_property_name.clone(),
        inbound_property_name.clone()
    ));
    let connector = Connector::from_relation(r.clone(), PARSE_INT_CONNECTOR_OPERATION);
    connector.relation.outbound.set(outbound_property_name.clone(), json!("123"));
    assert_eq!(123, connector.relation.inbound.as_i64(inbound_property_name.clone()).unwrap());
}

pub(crate) fn create_relation_instance_with_properties (
    outbound_entity: Arc<ReactiveEntityInstance>,
    type_name: String,
    inbound_entity: Arc<ReactiveEntityInstance>,
    outbound_property_name: String,
    inbound_property_name: String
) -> ReactiveRelationInstance {
    let mut properties = HashMap::new();
    properties.insert(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string(),  json!(outbound_property_name));
    properties.insert(ConnectorProperties::INBOUND_PROPERTY_NAME.to_string(),  json!(inbound_property_name));
    ReactiveRelationInstance::create_with_properties(
        outbound_entity.clone(),
        type_name.clone(),
        inbound_entity.clone(),
        properties
    )
}

pub(crate) fn create_default_connector<S: Into<String>> (
    outbound_entity: Arc<ReactiveEntityInstance>,
    inbound_entity: Arc<ReactiveEntityInstance>,
    outbound_property_name: S,
    inbound_property_name: S
) -> ReactiveRelationInstance {
    create_relation_instance_with_properties(
        outbound_entity,
        Connector::type_name(
            DEFAULT_CONNECTOR_TYPE_NAME,
            "bit_1",
            "bit_1"
        ),
        inbound_entity,
        outbound_property_name.into(),
        inbound_property_name.into()
    )
}
