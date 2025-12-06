use crate::TypeIdTypeParseError;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;
use std::str::FromStr;
use uuid::Uuid;

#[cfg(any(test, feature = "test"))]
use rand_derive3::RandGen;

/// String representation of the type of behaviour types.
pub const TYPE_ID_TYPE_BEHAVIOUR_SHORT: &str = "b";
pub const TYPE_ID_TYPE_BEHAVIOUR_FULL_NAME: &str = "Behaviour";
pub static TYPE_ID_TYPE_BEHAVIOUR_NAMESPACE: Uuid = Uuid::from_u128(0x12b7c8109d3d13c180f86c262ff540d9);

/// String representation of the type of component types.
pub const TYPE_ID_TYPE_COMPONENT_SHORT: &str = "c";
pub const TYPE_ID_TYPE_COMPONENT_FULL_NAME: &str = "Component";
pub static TYPE_ID_TYPE_COMPONENT_NAMESPACE: Uuid = Uuid::from_u128(0x1ab7c8109d3d13c180f468262fd540d9);

/// String representation of the type of entity types.
pub const TYPE_ID_TYPE_ENTITY_TYPE_SHORT: &str = "e";
pub const TYPE_ID_TYPE_ENTITY_TYPE_FULL_NAME: &str = "EntityType";
pub static TYPE_ID_TYPE_ENTITY_TYPE_NAMESPACE: Uuid = Uuid::from_u128(0x6ba7c8109dcd11c180b400d04fd530c7);

/// String representation of the type of extensions.
pub const TYPE_ID_TYPE_EXTENSION_SHORT: &str = "x";
pub const TYPE_ID_TYPE_EXTENSION_BASE_NAME: &str = "Extension";
pub const TYPE_ID_TYPE_EXTENSION_FULL_NAME: &str = "Extension";
pub static TYPE_ID_TYPE_EXTENSION_NAMESPACE: Uuid = Uuid::from_u128(0x6ba7c8109dcd11f586b708d07fd530c7);

/// String representation of the type of relation types.
pub const TYPE_ID_TYPE_RELATION_TYPE_SHORT: &str = "r";
pub const TYPE_ID_TYPE_RELATION_TYPE_FULL_NAME: &str = "RelationType";
pub static TYPE_ID_TYPE_RELATION_TYPE_NAMESPACE: Uuid = Uuid::from_u128(0x1ab7c8109dcd11c180b400d01fd530c7);

/// String representation of the type of flow types.
pub const TYPE_ID_TYPE_FLOW_TYPE_SHORT: &str = "f";
pub const TYPE_ID_TYPE_FLOW_TYPE_FULL_NAME: &str = "FlowType";
pub static TYPE_ID_TYPE_FLOW_TYPE_NAMESPACE: Uuid = Uuid::from_u128(0x62b7c5106d3d18c189f468202fd45230);

/// The type of a type.
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
#[derive(Clone, Debug, PartialEq, Copy, Eq, Serialize, Deserialize, JsonSchema)]
pub enum TypeIdType {
    // TODO: rename to TypeTypeId
    Behaviour,
    Component,
    EntityType, // TODO: rename to Entity
    Extension,
    RelationType, // TODO: rename to Relation
    FlowType,     // TODO: rename to Flow
}

impl TypeIdType {
    pub fn full_name(&self) -> String {
        match self {
            TypeIdType::Behaviour => TYPE_ID_TYPE_BEHAVIOUR_FULL_NAME,
            TypeIdType::Component => TYPE_ID_TYPE_COMPONENT_FULL_NAME,
            TypeIdType::EntityType => TYPE_ID_TYPE_ENTITY_TYPE_FULL_NAME,
            TypeIdType::Extension => TYPE_ID_TYPE_EXTENSION_FULL_NAME,
            TypeIdType::RelationType => TYPE_ID_TYPE_RELATION_TYPE_FULL_NAME,
            TypeIdType::FlowType => TYPE_ID_TYPE_FLOW_TYPE_FULL_NAME,
        }
        .to_string()
    }

    pub fn relative_url(&self) -> String {
        match self {
            TypeIdType::Behaviour => "types/behaviour",
            TypeIdType::Component => "types/component",
            TypeIdType::EntityType => "types/entity",
            TypeIdType::Extension => "types/extension",
            TypeIdType::RelationType => "types/relation",
            TypeIdType::FlowType => "types/flow",
        }
        .to_string()
    }

    pub fn relative_path(&self) -> PathBuf {
        PathBuf::from(self.relative_url())
    }
}

/// Converts the type of a type into the uuid namespace representation.
impl From<TypeIdType> for Uuid {
    fn from(t: TypeIdType) -> Self {
        match t {
            TypeIdType::Behaviour => TYPE_ID_TYPE_BEHAVIOUR_NAMESPACE,
            TypeIdType::Component => TYPE_ID_TYPE_COMPONENT_NAMESPACE,
            TypeIdType::EntityType => TYPE_ID_TYPE_ENTITY_TYPE_NAMESPACE,
            TypeIdType::Extension => TYPE_ID_TYPE_EXTENSION_NAMESPACE,
            TypeIdType::RelationType => TYPE_ID_TYPE_RELATION_TYPE_NAMESPACE,
            TypeIdType::FlowType => TYPE_ID_TYPE_FLOW_TYPE_NAMESPACE,
        }
    }
}

impl FromStr for TypeIdType {
    type Err = TypeIdTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            TYPE_ID_TYPE_BEHAVIOUR_SHORT => Ok(TypeIdType::Behaviour),
            TYPE_ID_TYPE_BEHAVIOUR_FULL_NAME => Ok(TypeIdType::Behaviour),
            TYPE_ID_TYPE_COMPONENT_SHORT => Ok(TypeIdType::Component),
            TYPE_ID_TYPE_COMPONENT_FULL_NAME => Ok(TypeIdType::Component),
            TYPE_ID_TYPE_ENTITY_TYPE_SHORT => Ok(TypeIdType::EntityType),
            TYPE_ID_TYPE_ENTITY_TYPE_FULL_NAME => Ok(TypeIdType::EntityType),
            TYPE_ID_TYPE_EXTENSION_SHORT => Ok(TypeIdType::Extension),
            TYPE_ID_TYPE_EXTENSION_FULL_NAME => Ok(TypeIdType::Extension),
            TYPE_ID_TYPE_RELATION_TYPE_SHORT => Ok(TypeIdType::RelationType),
            TYPE_ID_TYPE_RELATION_TYPE_FULL_NAME => Ok(TypeIdType::RelationType),
            TYPE_ID_TYPE_FLOW_TYPE_SHORT => Ok(TypeIdType::FlowType),
            TYPE_ID_TYPE_FLOW_TYPE_FULL_NAME => Ok(TypeIdType::FlowType),
            _ => Err(TypeIdTypeParseError::UnknownTypeIdType(s.to_owned())),
        }
    }
}

/// Converts the type of a type into a one letter string representation.
impl Display for TypeIdType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TypeIdType::Behaviour => TYPE_ID_TYPE_BEHAVIOUR_SHORT,
                TypeIdType::Component => TYPE_ID_TYPE_COMPONENT_SHORT,
                TypeIdType::EntityType => TYPE_ID_TYPE_ENTITY_TYPE_SHORT,
                TypeIdType::Extension => TYPE_ID_TYPE_EXTENSION_SHORT,
                TypeIdType::RelationType => TYPE_ID_TYPE_RELATION_TYPE_SHORT,
                TypeIdType::FlowType => TYPE_ID_TYPE_FLOW_TYPE_SHORT,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::TYPE_ID_TYPE_BEHAVIOUR_NAMESPACE;
    use crate::TYPE_ID_TYPE_COMPONENT_NAMESPACE;
    use crate::TYPE_ID_TYPE_ENTITY_TYPE_NAMESPACE;
    use crate::TYPE_ID_TYPE_EXTENSION_NAMESPACE;
    use crate::TYPE_ID_TYPE_FLOW_TYPE_NAMESPACE;
    use crate::TYPE_ID_TYPE_RELATION_TYPE_NAMESPACE;
    use crate::TypeIdType;
    use schemars::schema_for;
    use std::path::PathBuf;
    use std::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn type_id_type_to_string_test() {
        assert_eq!("b", TypeIdType::Behaviour.to_string());
        assert_eq!("c", TypeIdType::Component.to_string());
        assert_eq!("e", TypeIdType::EntityType.to_string());
        assert_eq!("x", TypeIdType::Extension.to_string());
        assert_eq!("r", TypeIdType::RelationType.to_string());
        assert_eq!("f", TypeIdType::FlowType.to_string());
    }

    #[test]
    fn type_id_type_uuid_test() {
        assert_eq!(TYPE_ID_TYPE_BEHAVIOUR_NAMESPACE, Uuid::from(TypeIdType::Behaviour));
        assert_eq!(TYPE_ID_TYPE_COMPONENT_NAMESPACE, Uuid::from(TypeIdType::Component));
        assert_eq!(TYPE_ID_TYPE_ENTITY_TYPE_NAMESPACE, Uuid::from(TypeIdType::EntityType));
        assert_eq!(TYPE_ID_TYPE_EXTENSION_NAMESPACE, Uuid::from(TypeIdType::Extension));
        assert_eq!(TYPE_ID_TYPE_RELATION_TYPE_NAMESPACE, Uuid::from(TypeIdType::RelationType));
        assert_eq!(TYPE_ID_TYPE_FLOW_TYPE_NAMESPACE, Uuid::from(TypeIdType::FlowType));
    }

    #[test]
    fn type_id_type_from_str_test() {
        assert_eq!(TypeIdType::Behaviour, TypeIdType::from_str("b").unwrap());
        assert_eq!(TypeIdType::Behaviour, TypeIdType::from_str("Behaviour").unwrap());
        assert_eq!(TypeIdType::Component, TypeIdType::from_str("c").unwrap());
        assert_eq!(TypeIdType::Component, TypeIdType::from_str("Component").unwrap());
        assert_eq!(TypeIdType::EntityType, TypeIdType::from_str("e").unwrap());
        assert_eq!(TypeIdType::EntityType, TypeIdType::from_str("EntityType").unwrap());
        assert_eq!(TypeIdType::Extension, TypeIdType::from_str("x").unwrap());
        assert_eq!(TypeIdType::Extension, TypeIdType::from_str("Extension").unwrap());
        assert_eq!(TypeIdType::RelationType, TypeIdType::from_str("r").unwrap());
        assert_eq!(TypeIdType::RelationType, TypeIdType::from_str("RelationType").unwrap());
        assert_eq!(TypeIdType::FlowType, TypeIdType::from_str("f").unwrap());
        assert_eq!(TypeIdType::FlowType, TypeIdType::from_str("FlowType").unwrap());
        assert!(TypeIdType::from_str("a").is_err());
        assert!(TypeIdType::from_str("abc").is_err());
    }

    #[test]
    fn type_id_type_full_name_test() {
        assert_eq!("Behaviour", TypeIdType::Behaviour.full_name());
        assert_eq!("Component", TypeIdType::Component.full_name());
        assert_eq!("EntityType", TypeIdType::EntityType.full_name());
        assert_eq!("Extension", TypeIdType::Extension.full_name());
        assert_eq!("RelationType", TypeIdType::RelationType.full_name());
        assert_eq!("FlowType", TypeIdType::FlowType.full_name());
    }

    #[test]
    fn type_id_type_relative_url_test() {
        assert_eq!("types/behaviour", TypeIdType::Behaviour.relative_url());
        assert_eq!("types/component", TypeIdType::Component.relative_url());
        assert_eq!("types/entity", TypeIdType::EntityType.relative_url());
        assert_eq!("types/extension", TypeIdType::Extension.relative_url());
        assert_eq!("types/relation", TypeIdType::RelationType.relative_url());
        assert_eq!("types/flow", TypeIdType::FlowType.relative_url());
    }

    #[test]
    fn type_id_type_relative_path_test() {
        assert_eq!(PathBuf::from("types/behaviour"), TypeIdType::Behaviour.relative_path());
        assert_eq!(PathBuf::from("types/component"), TypeIdType::Component.relative_path());
        assert_eq!(PathBuf::from("types/entity"), TypeIdType::EntityType.relative_path());
        assert_eq!(PathBuf::from("types/extension"), TypeIdType::Extension.relative_path());
        assert_eq!(PathBuf::from("types/relation"), TypeIdType::RelationType.relative_path());
        assert_eq!(PathBuf::from("types/flow"), TypeIdType::FlowType.relative_path());
    }

    #[test]
    fn type_id_type_json_schema() {
        let schema = schema_for!(TypeIdType);
        let schema_str = serde_json::to_string_pretty(&schema).expect("Failed to create JSON schema");
        assert!(!schema_str.is_empty());
    }
}
