use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

/// String representation of the type of behaviour types.
pub const TYPE_ID_TYPE_BEHAVIOUR: &str = "b";

/// String representation of the type of component types.
pub const TYPE_ID_TYPE_COMPONENT: &str = "c";

/// String representation of the type of entity types.
pub const TYPE_ID_TYPE_ENTITY_TYPE: &str = "e";

/// String representation of the type of extensions.
pub const TYPE_ID_TYPE_EXTENSION: &str = "x";

/// String representation of the type of relation types.
pub const TYPE_ID_TYPE_RELATION_TYPE: &str = "r";

/// String representation of the type of flow types.
pub const TYPE_ID_TYPE_FLOW_TYPE: &str = "f";

pub static TYPE_ID_TYPE_NAMESPACE_BEHAVIOUR: Uuid = Uuid::from_u128(0x12b7c8109d3d13c180f86c262ff540d9);
pub static TYPE_ID_TYPE_NAMESPACE_COMPONENT: Uuid = Uuid::from_u128(0x1ab7c8109d3d13c180f468262fd540d9);
pub static TYPE_ID_TYPE_NAMESPACE_ENTITY_TYPE: Uuid = Uuid::from_u128(0x6ba7c8109dcd11c180b400d04fd530c7);
pub static TYPE_ID_TYPE_NAMESPACE_EXTENSION: Uuid = Uuid::from_u128(0x6ba7c8109dcd11f586b708d07fd530c7);
pub static TYPE_ID_TYPE_NAMESPACE_RELATION_TYPE: Uuid = Uuid::from_u128(0x1ab7c8109dcd11c180b400d01fd530c7);
pub static TYPE_ID_TYPE_NAMESPACE_FLOW_TYPE: Uuid = Uuid::from_u128(0x62b7c5106d3d18c189f468202fd45230);

/// The type of a type.
#[derive(Clone, Debug, PartialEq, Copy, Eq, Serialize, Deserialize, JsonSchema)]
pub enum TypeIdType {
    Behaviour,
    Component,
    EntityType,
    Extension,
    RelationType,
    FlowType,
}

impl TypeIdType {
    pub fn full_name(&self) -> String {
        match self {
            TypeIdType::Behaviour => "Behaviour",
            TypeIdType::Component => "Component",
            TypeIdType::EntityType => "Entity",
            TypeIdType::Extension => "Extension",
            TypeIdType::RelationType => "Relation",
            TypeIdType::FlowType => "Flow",
        }
        .to_string()
    }

    pub fn relative_url(&self) -> String {
        self.full_name().to_lowercase()
    }

    pub fn relative_path(&self) -> PathBuf {
        PathBuf::from(self.relative_url())
    }
}

/// Converts the type of a type into the uuid namespace representation.
impl From<TypeIdType> for Uuid {
    fn from(t: TypeIdType) -> Self {
        match t {
            TypeIdType::Behaviour => TYPE_ID_TYPE_NAMESPACE_BEHAVIOUR,
            TypeIdType::Component => TYPE_ID_TYPE_NAMESPACE_COMPONENT,
            TypeIdType::EntityType => TYPE_ID_TYPE_NAMESPACE_ENTITY_TYPE,
            TypeIdType::Extension => TYPE_ID_TYPE_NAMESPACE_EXTENSION,
            TypeIdType::RelationType => TYPE_ID_TYPE_NAMESPACE_RELATION_TYPE,
            TypeIdType::FlowType => TYPE_ID_TYPE_NAMESPACE_FLOW_TYPE,
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
                TypeIdType::Behaviour => TYPE_ID_TYPE_BEHAVIOUR,
                TypeIdType::Component => TYPE_ID_TYPE_COMPONENT,
                TypeIdType::EntityType => TYPE_ID_TYPE_ENTITY_TYPE,
                TypeIdType::Extension => TYPE_ID_TYPE_EXTENSION,
                TypeIdType::RelationType => TYPE_ID_TYPE_RELATION_TYPE,
                TypeIdType::FlowType => TYPE_ID_TYPE_FLOW_TYPE,
            }
        )
    }
}

#[derive(Debug, Error)]
pub enum TypeIdTypeParseError {
    #[error("The type id type {0} is unknown")]
    UnknownTypeIdType(String),
}

impl TryFrom<&str> for TypeIdType {
    type Error = TypeIdTypeParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            TYPE_ID_TYPE_BEHAVIOUR => Ok(TypeIdType::Behaviour),
            TYPE_ID_TYPE_COMPONENT => Ok(TypeIdType::Component),
            TYPE_ID_TYPE_ENTITY_TYPE => Ok(TypeIdType::EntityType),
            TYPE_ID_TYPE_EXTENSION => Ok(TypeIdType::Extension),
            TYPE_ID_TYPE_RELATION_TYPE => Ok(TypeIdType::RelationType),
            TYPE_ID_TYPE_FLOW_TYPE => Ok(TypeIdType::FlowType),
            _ => Err(TypeIdTypeParseError::UnknownTypeIdType(s.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TYPE_ID_TYPE_NAMESPACE_BEHAVIOUR;
    use crate::TYPE_ID_TYPE_NAMESPACE_COMPONENT;
    use crate::TYPE_ID_TYPE_NAMESPACE_ENTITY_TYPE;
    use crate::TYPE_ID_TYPE_NAMESPACE_EXTENSION;
    use crate::TYPE_ID_TYPE_NAMESPACE_FLOW_TYPE;
    use crate::TYPE_ID_TYPE_NAMESPACE_RELATION_TYPE;
    use crate::TypeIdType;
    use schemars::schema_for;
    use uuid::Uuid;

    #[test]
    fn type_id_type_to_string_test() {
        let tidt_b = TypeIdType::Behaviour;
        assert_eq!("b", tidt_b.to_string());

        let tidt_c = TypeIdType::Component;
        assert_eq!("c", tidt_c.to_string());

        let tidt_e = TypeIdType::EntityType;
        assert_eq!("e", tidt_e.to_string());

        let tidt_x = TypeIdType::Extension;
        assert_eq!("x", tidt_x.to_string());

        let tidt_r = TypeIdType::RelationType;
        assert_eq!("r", tidt_r.to_string());

        let tidt_f = TypeIdType::FlowType;
        assert_eq!("f", tidt_f.to_string());
    }

    #[test]
    fn type_id_type_uuid_test() {
        let tidt_b_uuid: Uuid = TypeIdType::Behaviour.into();
        assert_eq!(TYPE_ID_TYPE_NAMESPACE_BEHAVIOUR, tidt_b_uuid);

        let tidt_c_uuid: Uuid = TypeIdType::Component.into();
        assert_eq!(TYPE_ID_TYPE_NAMESPACE_COMPONENT, tidt_c_uuid);

        let tidt_e_uuid: Uuid = TypeIdType::EntityType.into();
        assert_eq!(TYPE_ID_TYPE_NAMESPACE_ENTITY_TYPE, tidt_e_uuid);

        let tidt_x_uuid: Uuid = TypeIdType::Extension.into();
        assert_eq!(TYPE_ID_TYPE_NAMESPACE_EXTENSION, tidt_x_uuid);

        let tidt_r_uuid: Uuid = TypeIdType::RelationType.into();
        assert_eq!(TYPE_ID_TYPE_NAMESPACE_RELATION_TYPE, tidt_r_uuid);

        let tidt_f_uuid: Uuid = TypeIdType::FlowType.into();
        assert_eq!(TYPE_ID_TYPE_NAMESPACE_FLOW_TYPE, tidt_f_uuid);
    }

    #[test]
    fn type_id_type_from_str_test() {
        assert_eq!(TypeIdType::Behaviour, TypeIdType::try_from("b").unwrap());
        assert_eq!(TypeIdType::Component, TypeIdType::try_from("c").unwrap());
        assert_eq!(TypeIdType::EntityType, TypeIdType::try_from("e").unwrap());
        assert_eq!(TypeIdType::Extension, TypeIdType::try_from("x").unwrap());
        assert_eq!(TypeIdType::RelationType, TypeIdType::try_from("r").unwrap());
        assert_eq!(TypeIdType::FlowType, TypeIdType::try_from("f").unwrap());
        assert!(TypeIdType::try_from("a").is_err());
        assert!(TypeIdType::try_from("abc").is_err());
    }

    #[test]
    fn type_id_type_json_schema() {
        let schema = schema_for!(TypeIdType);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
