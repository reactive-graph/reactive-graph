use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

/// Separator for the string representation of a type definition.
pub static TYPE_ID_TYPE_SEPARATOR: &str = "__";

/// String representation of the type of component types.
pub const TYPE_ID_TYPE_COMPONENT: &str = "c";

/// String representation of the type of entity types.
pub const TYPE_ID_TYPE_ENTITY_TYPE: &str = "e";

/// String representation of the type of relation types.
pub const TYPE_ID_TYPE_RELATION_TYPE: &str = "r";

/// String representation of the type of flow types.
pub const TYPE_ID_TYPE_FLOW_TYPE: &str = "f";

pub static TYPE_ID_TYPE_NAMESPACE_COMPONENT: Uuid = Uuid::from_u128(0x1ab7c8109d3d13c180f468262fd540d9);
pub static TYPE_ID_TYPE_NAMESPACE_ENTITY_TYPE: Uuid = Uuid::from_u128(0x6ba7c8109dcd11c180b400d04fd530c7);
pub static TYPE_ID_TYPE_NAMESPACE_RELATION_TYPE: Uuid = Uuid::from_u128(0x1ab7c8109dcd11c180b400d01fd530c7);
pub static TYPE_ID_TYPE_NAMESPACE_FLOW_TYPE: Uuid = Uuid::from_u128(0x62b7c5106d3d18c189f468202fd45230);

/// The type of a type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy, Eq)]
pub enum TypeIdType {
    Component,
    EntityType,
    RelationType,
    FlowType,
}

/// Converts the type of a type into the uuid namespace representation.
impl From<TypeIdType> for Uuid {
    fn from(t: TypeIdType) -> Self {
        match t {
            TypeIdType::Component => TYPE_ID_TYPE_NAMESPACE_COMPONENT,
            TypeIdType::EntityType => TYPE_ID_TYPE_NAMESPACE_ENTITY_TYPE,
            TypeIdType::RelationType => TYPE_ID_TYPE_NAMESPACE_RELATION_TYPE,
            TypeIdType::FlowType => TYPE_ID_TYPE_NAMESPACE_FLOW_TYPE,
        }
    }
}

/// Converts the type of a type into a one letter string representation.
impl ToString for TypeIdType {
    fn to_string(&self) -> String {
        match self {
            TypeIdType::Component => TYPE_ID_TYPE_COMPONENT.to_string(),
            TypeIdType::EntityType => TYPE_ID_TYPE_ENTITY_TYPE.to_string(),
            TypeIdType::RelationType => TYPE_ID_TYPE_RELATION_TYPE.to_string(),
            TypeIdType::FlowType => TYPE_ID_TYPE_FLOW_TYPE.to_string(),
        }
    }
}

impl TryFrom<&str> for TypeIdType {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            TYPE_ID_TYPE_COMPONENT => Ok(TypeIdType::Component),
            TYPE_ID_TYPE_ENTITY_TYPE => Ok(TypeIdType::EntityType),
            TYPE_ID_TYPE_RELATION_TYPE => Ok(TypeIdType::RelationType),
            TYPE_ID_TYPE_FLOW_TYPE => Ok(TypeIdType::FlowType),
            _ => Err(()),
        }
    }
}
