use serde_json::json;
use uuid::Uuid;

use crate::behaviour::relation::{DefaultConnector, RelationBehaviour};
use crate::builder::RelationInstanceBuilder;
use crate::model::{EntityInstance, RelationInstance};
use crate::reactive::relation::connector::{Connector, ConnectorProperties};

#[allow(dead_code)]
pub struct DefaultConnectorBuilder {
    outbound: Option<(Uuid, String)>,
    inbound: Option<(Uuid, String)>,
}

#[allow(dead_code)]
impl DefaultConnectorBuilder {
    pub fn new() -> DefaultConnectorBuilder {
        DefaultConnectorBuilder {
            outbound: None,
            inbound: None,
        }
    }

    pub fn outbound<'a, S: Into<String>>(
        &'a mut self,
        outbound_entity: EntityInstance,
        property_name: S,
    ) -> &'a mut DefaultConnectorBuilder {
        self.outbound = Some((outbound_entity.id, property_name.into().clone()));
        self
    }

    pub fn inbound<'a, S: Into<String>>(
        &'a mut self,
        inbound_entity: EntityInstance,
        property_name: S,
    ) -> &'a mut DefaultConnectorBuilder {
        self.inbound = Some((inbound_entity.id, property_name.into().clone()));
        self
    }

    pub fn get(&mut self) -> RelationInstance {
        let outbound = self.outbound.clone().unwrap();
        let inbound = self.inbound.clone().unwrap();
        // The edge_key must be different even if there are multiple connectors between two entities
        let type_name = Connector::type_name(
            DefaultConnector::TYPE_NAME,
            outbound.1.as_str(),
            inbound.1.as_str(),
        );
        RelationInstanceBuilder::new(outbound.0, type_name, inbound.0)
            .property(
                ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string(),
                json!(outbound.1),
            )
            .property(
                ConnectorProperties::INBOUND_PROPERTY_NAME.to_string(),
                json!(inbound.1),
            )
            .get()
    }
}
