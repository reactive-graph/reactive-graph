use std::collections::HashMap;
use std::sync::Arc;

use indradb::{EdgeKey, Type};
use serde_json::Value;
use uuid::Uuid;

use crate::api::{RelationEdgeCreationError, RelationEdgeManager};

#[allow(dead_code)]
pub struct RelationEdgeBuilder {
    outbound_id: Uuid,
    type_name: String,
    inbound_id: Uuid,
    properties: HashMap<String, Value>,
}

#[allow(dead_code)]
impl RelationEdgeBuilder {
    pub fn new<S: Into<String>>(
        outbound_id: Uuid,
        type_name: S,
        inbound_id: Uuid,
    ) -> RelationEdgeBuilder {
        RelationEdgeBuilder {
            outbound_id,
            type_name: type_name.into(),
            inbound_id,
            properties: HashMap::new(),
        }
    }

    pub fn property<'a, S: Into<String>>(
        &'a mut self,
        property_name: S,
        value: Value,
    ) -> &'a mut RelationEdgeBuilder {
        self.properties.insert(property_name.into(), value);
        self
    }

    pub fn create<'a>(
        &'a mut self,
        relation_edge_manager: Arc<dyn RelationEdgeManager>,
    ) -> Result<EdgeKey, RelationEdgeCreationError> {
        let edge_key = EdgeKey::new(
            self.outbound_id,
            Type::new(self.type_name.clone()).unwrap(),
            self.inbound_id,
        );
        relation_edge_manager.create(edge_key, self.properties.clone())
    }
}

impl From<EdgeKey> for RelationEdgeBuilder {
    fn from(edge_key: EdgeKey) -> Self {
        RelationEdgeBuilder {
            outbound_id: edge_key.outbound_id,
            type_name: edge_key.t.0.clone(),
            inbound_id: edge_key.inbound_id,
            properties: HashMap::new(),
        }
    }
}
