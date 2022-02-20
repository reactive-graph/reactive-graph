use std::collections::HashMap;

use indradb::EdgeKey;
use serde_json::Value;
use uuid::Uuid;

use crate::model::RelationInstance;

#[allow(dead_code)]
pub struct RelationInstanceBuilder {
    outbound_id: Uuid,
    type_name: String,
    inbound_id: Uuid,
    properties: HashMap<String, Value>,
}

#[allow(dead_code)]
impl RelationInstanceBuilder {
    pub fn new<S: Into<String>>(
        outbound_id: Uuid,
        type_name: S,
        inbound_id: Uuid,
    ) -> RelationInstanceBuilder {
        RelationInstanceBuilder {
            outbound_id,
            type_name: type_name.into(),
            inbound_id,
            properties: HashMap::new(),
        }
    }

    pub fn property<S: Into<String>>(
        &mut self,
        property_name: S,
        value: Value,
    ) -> &mut RelationInstanceBuilder {
        self.properties.insert(property_name.into(), value);
        self
    }

    pub fn get(&mut self) -> RelationInstance {
        RelationInstance::new(
            self.outbound_id,
            self.type_name.clone(),
            self.inbound_id,
            self.properties.clone(),
        )
    }
}

impl From<EdgeKey> for RelationInstanceBuilder {
    fn from(edge_key: EdgeKey) -> Self {
        RelationInstanceBuilder {
            outbound_id: edge_key.outbound_id,
            type_name: edge_key.t.to_string(),
            inbound_id: edge_key.inbound_id,
            properties: HashMap::new(),
        }
    }
}
