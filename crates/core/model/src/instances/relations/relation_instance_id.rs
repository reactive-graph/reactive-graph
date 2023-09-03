use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::NamespacedType;
use crate::RelationInstanceTypeId;
use crate::NamespacedTypeGetter;
use crate::RelationTypeId;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;

/// Separator for the string representation of a relation instance.
pub static RELATION_INSTANCE_ID_SEPARATOR: &str = "--";

/// Unique edge key
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TypedBuilder)]
pub struct RelationInstanceId {

    /// The id of the outbound entity.
    pub outbound_id: Uuid,

    /// The relation instance type id containing the relation type and the instance id.
    #[builder(setter(into))]
    pub ty: RelationInstanceTypeId,

    /// The id of the inbound entity.
    pub inbound_id: Uuid,
}

impl RelationInstanceId {
    pub fn new <RIT: Into<RelationInstanceTypeId>> (outbound_id: Uuid, ty: RIT, inbound_id: Uuid) -> Self {
        RelationInstanceId {
            outbound_id: outbound_id.into(),
            ty: ty.into(),
            inbound_id: inbound_id.into()
        }
    }

    pub fn new_unique<RT: Into<RelationTypeId>>(outbound_id: Uuid, ty: RT, inbound_id: Uuid) -> Self {
        RelationInstanceId {
            outbound_id,
            ty: RelationInstanceTypeId::new_unique_id(ty),
            inbound_id
        }
    }

    pub fn new_unique_for_instance_id<RT: Into<RelationTypeId>>(outbound_id: Uuid, ty: RT, instance_id: String, inbound_id: Uuid) -> Self {
        RelationInstanceId {
            outbound_id,
            ty: RelationInstanceTypeId::new_unique_for_instance_id(ty, instance_id),
            inbound_id
        }
    }

    pub fn new_with_random_instance_id<RT: Into<RelationTypeId>>(outbound_id: Uuid, ty: RT, inbound_id: Uuid) -> Self {
        RelationInstanceId {
            outbound_id,
            ty: RelationInstanceTypeId::new_with_random_instance_id(ty),
            inbound_id
        }
    }
}

impl NamespacedTypeGetter for RelationInstanceId {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    /// Returns the full instance type name (relation type name + instance id)
    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for RelationInstanceId {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }
}

impl PartialOrd<Self> for RelationInstanceId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RelationInstanceId {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.ty.cmp(&other.ty) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                match self.outbound_id.cmp(&other.outbound_id) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => self.inbound_id.cmp(&other.inbound_id),
                    Ordering::Greater => Ordering::Greater,
                }
            }
            Ordering::Greater => Ordering::Greater
        }
    }
}

impl From<&RelationInstanceId> for RelationInstanceId {
    fn from(ty: &RelationInstanceId) -> Self {
        ty.clone()
    }
}

impl From<&RelationInstanceId> for TypeDefinition {
    fn from(ty: &RelationInstanceId) -> Self {
        TypeDefinition::new(TypeIdType::RelationType, ty.into())
    }
}

impl From<&RelationInstanceId> for NamespacedType {
    fn from(ty: &RelationInstanceId) -> Self {
        // Returns the namespaced type with the full instance type name (relation type name + instance id)
        NamespacedType::from(&ty.ty)
    }
}

impl TryFrom<&String> for RelationInstanceId {
    type Error = ();

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let mut s = s.splitn(3, &RELATION_INSTANCE_ID_SEPARATOR);
        let outbound_id = s.next().ok_or(())?.try_into().map_err(|_| ())?;
        let ty: RelationInstanceTypeId = s.next().ok_or(())?.to_string().try_into().map_err(|_| ())?;
        let inbound_id = s.next().ok_or(())?.try_into().map_err(|_| ())?;
        return Ok(RelationInstanceId::new(outbound_id, ty, inbound_id));
    }
}

impl Display for RelationInstanceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-[{}]->{}", self.outbound_id, &self.ty, self.inbound_id)
    }
}
