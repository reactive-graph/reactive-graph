use std::collections::HashMap;

use crate::model::RelationInstanceTypeId;
use indradb::EdgeKey;
use serde_json::Value;
use uuid::Uuid;

use crate::model::RelationInstance;
use crate::model::RelationTypeId;

#[allow(dead_code)]
pub struct RelationInstanceBuilder {
    outbound_id: Uuid,
    ty: RelationInstanceTypeId,
    inbound_id: Uuid,
    properties: HashMap<String, Value>,
}

#[allow(dead_code)]
impl RelationInstanceBuilder {
    pub fn new<RIT: Into<RelationInstanceTypeId>>(outbound_id: Uuid, ty: RIT, inbound_id: Uuid) -> RelationInstanceBuilder {
        RelationInstanceBuilder {
            outbound_id,
            ty: ty.into(),
            inbound_id,
            properties: HashMap::new(),
        }
    }

    pub fn new_unique_id<RT: Into<RelationTypeId>>(outbound_id: Uuid, ty: RT, inbound_id: Uuid) -> RelationInstanceBuilder {
        RelationInstanceBuilder {
            outbound_id,
            ty: RelationInstanceTypeId::new_unique_id(ty.into()),
            inbound_id,
            properties: HashMap::new(),
        }
    }

    pub fn new_unique_for_instance_id<RT: Into<RelationTypeId>, S: Into<String>>(
        outbound_id: Uuid,
        ty: RT,
        instance_id: S,
        inbound_id: Uuid,
    ) -> RelationInstanceBuilder {
        RelationInstanceBuilder {
            outbound_id,
            ty: RelationInstanceTypeId::new_unique_for_instance_id(ty.into(), instance_id),
            inbound_id,
            properties: HashMap::new(),
        }
    }

    pub fn new_with_random_instance_id<RT: Into<RelationTypeId>>(outbound_id: Uuid, ty: RT, inbound_id: Uuid) -> RelationInstanceBuilder {
        RelationInstanceBuilder {
            outbound_id,
            ty: RelationInstanceTypeId::new_with_random_instance_id(ty.into()),
            inbound_id,
            properties: HashMap::new(),
        }
    }

    pub fn new_from_type_unique_id<S: Into<String>>(outbound_id: Uuid, namespace: S, type_name: S, inbound_id: Uuid) -> RelationInstanceBuilder {
        RelationInstanceBuilder::new(outbound_id, RelationInstanceTypeId::new_from_type_unique_id(namespace, type_name), inbound_id)
    }

    pub fn new_from_type_unique_for_instance_id<S: Into<String>>(
        outbound_id: Uuid,
        namespace: S,
        type_name: S,
        instance_id: S,
        inbound_id: Uuid,
    ) -> RelationInstanceBuilder {
        RelationInstanceBuilder::new(
            outbound_id,
            RelationInstanceTypeId::new_from_type_unique_for_instance_id(namespace, type_name, instance_id),
            inbound_id,
        )
    }

    pub fn new_from_type_with_random_instance_id<S: Into<String>>(outbound_id: Uuid, namespace: S, type_name: S, inbound_id: Uuid) -> RelationInstanceBuilder {
        RelationInstanceBuilder::new(outbound_id, RelationInstanceTypeId::new_from_type_with_random_instance_id(namespace, type_name), inbound_id)
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
            RelationInstanceTypeId::try_from(&edge_key.t)?,
            edge_key.inbound_id,
        ))
    }
}
