use std::collections::HashMap;

use indradb::EdgeKey;
use serde_json::Value;
use uuid::Uuid;

use crate::model::RelationInstance;
use crate::model::RelationTypeType;

#[allow(dead_code)]
pub struct RelationInstanceBuilder {
    outbound_id: Uuid,
    ty: RelationTypeType,
    inbound_id: Uuid,
    properties: HashMap<String, Value>,
}

#[allow(dead_code)]
impl RelationInstanceBuilder {
    pub fn new(outbound_id: Uuid, ty: RelationTypeType, inbound_id: Uuid) -> RelationInstanceBuilder {
        RelationInstanceBuilder {
            outbound_id,
            ty,
            inbound_id,
            properties: HashMap::new(),
        }
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, outbound_id: Uuid, type_name: S, inbound_id: Uuid) -> RelationInstanceBuilder {
        RelationInstanceBuilder::new(outbound_id, RelationTypeType::new_from_type(namespace, type_name), inbound_id)
    }

    pub fn property<S: Into<String>>(&mut self, property_name: S, value: Value) -> &mut RelationInstanceBuilder {
        self.properties.insert(property_name.into(), value);
        self
    }

    pub fn build(&self) -> RelationInstance {
        RelationInstance::new(self.outbound_id, self.ty.clone(), self.inbound_id, self.properties.clone())
    }
}

impl TryFrom<&EdgeKey> for RelationInstanceBuilder {
    type Error = ();

    fn try_from(edge_key: &EdgeKey) -> Result<Self, Self::Error> {
        Ok(RelationInstanceBuilder::new(
            edge_key.outbound_id,
            RelationTypeType::try_from(&edge_key.t)?,
            edge_key.inbound_id,
        ))
    }
}
