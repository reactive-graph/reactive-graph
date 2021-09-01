use crate::model::{
    PropertyInstanceGetter, PropertyInstanceSetter, ReactiveEntityInstance,
    ReactiveRelationInstance,
};
use crate::relation::connector::ConnectorProperties;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;

// TODO: move into ConnectorProperties
pub static TYPE_NAME_CONNECTOR: &'static str = "connector";
// pub static PROPERTY_NAME_OUTBOUND: &'static str = "outbound_property_name";
// pub static PROPERTY_NAME_INBOUND: &'static str = "inbound_property_name";

pub type ConnectorFunction = fn(Value) -> Value;

/// A connector connects a property of the outbound entity instance with
/// a property of the inbound entity instance.
///
/// In theory it's also possible to connect two properties of the same entity instance.
///
/// On construction the streams are connected. No type checks are performed.
///
/// On destruction of the connector, the stream will be removed.
pub struct Connector {
    /// The connector is a wrapper of a reactive relation instance.
    pub relation: Arc<ReactiveRelationInstance>,

    pub f: ConnectorFunction,

    /// The handle id is the numeric representation (u128) of the UUID of the inbound property
    // TODO: make it a tuple (outbound_id, inbound_id) or use two handle_ids.
    // TODO: This would result in a unique handle_id
    pub handle_id: u128,
}

impl Connector {
    pub fn from_relation<'a>(
        relation: Arc<ReactiveRelationInstance>,
        f: ConnectorFunction,
    ) -> Connector {
        let mut connector = Connector {
            relation: relation.clone(),
            f,
            handle_id: 0,
        };
        connector.connect();
        connector
    }

    /// Constructs a new connector using an outbound entity (+ name of the property) and
    /// an inbound entity (+ name of the property)
    pub fn new(
        outbound: Arc<ReactiveEntityInstance>,
        outbound_property_name: String,
        inbound: Arc<ReactiveEntityInstance>,
        inbound_property_name: String,
    ) -> Connector {
        let properties =
            get_connector_relation_properties(outbound_property_name, inbound_property_name);
        let relation = Arc::new(ReactiveRelationInstance::create_with_properties(
            outbound.clone(),
            TYPE_NAME_CONNECTOR.to_string(),
            inbound.clone(),
            properties,
        ));
        Connector::from_relation(relation, |v| v.clone())
    }

    /// TODO: Add guard: disconnect only if connected
    /// TODO: Fail fast
    /// TODO: Return Result
    pub fn connect(&mut self) {
        let outbound_property_name = self
            .relation
            .as_string(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string());
        let inbound_property_name = self
            .relation
            .as_string(ConnectorProperties::INBOUND_PROPERTY_NAME.to_string());
        if outbound_property_name.is_some() && inbound_property_name.is_some() {
            let outbound_property_name = outbound_property_name.unwrap();
            let inbound_property_name = inbound_property_name.unwrap();
            let outbound_property = self
                .relation
                .outbound
                .properties
                .get(&outbound_property_name.clone());
            let inbound_property = self
                .relation
                .inbound
                .properties
                .get(&inbound_property_name.clone());
            if outbound_property.is_some() && inbound_property.is_some() {
                let inbound = self.relation.inbound.clone();
                self.handle_id = inbound_property.unwrap().id.as_u128();
                // println!("connecting {} {} --> {} {}", self.relation.outbound.id, outbound_property_name.clone(), self.relation.inbound.id, inbound_property_name.clone());
                let f = self.f;
                outbound_property
                    .unwrap()
                    .stream
                    .read()
                    .unwrap()
                    .observe_with_handle(
                        move |value: &Value| {
                            inbound.set(inbound_property_name.clone(), f(value.clone()));
                        },
                        self.handle_id,
                    );
            }
        }
    }

    /// TODO: Add guard: disconnect only if connected
    pub fn disconnect(&self) {
        let outbound_property_name = self
            .relation
            .as_string(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string());
        if outbound_property_name.is_some() {
            let outbound_property = self
                .relation
                .outbound
                .properties
                .get(&outbound_property_name.unwrap().clone());
            if outbound_property.is_some() {
                outbound_property
                    .unwrap()
                    .stream
                    .read()
                    .unwrap()
                    .remove(self.handle_id);
            }
        }
    }

    // pub fn type_name(type_name: String, outbound_property_name: String, inbound_property_name: String) -> String {
    //     format!("{}--{}--{}", type_name, outbound_property_name.as_str(), inbound_property_name.as_str())
    // }

    pub fn type_name(
        type_name: &str,
        outbound_property_name: &str,
        inbound_property_name: &str,
    ) -> String {
        format!(
            "{}--{}--{}",
            type_name, outbound_property_name, inbound_property_name
        )
    }
}

/// Automatically disconnect streams on destruction
impl Drop for Connector {
    fn drop(&mut self) {
        self.disconnect();
    }
}

/// The relation instance of type connector contains exactly two properties
/// which contains the names of the entity properties.
fn get_connector_relation_properties(
    outbound_property_name: String,
    inbound_property_name: String,
) -> HashMap<String, Value> {
    let mut properties = HashMap::new();
    properties.insert(
        ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string(),
        json!(outbound_property_name),
    );
    properties.insert(
        ConnectorProperties::INBOUND_PROPERTY_NAME.to_string(),
        json!(inbound_property_name),
    );
    properties
}
