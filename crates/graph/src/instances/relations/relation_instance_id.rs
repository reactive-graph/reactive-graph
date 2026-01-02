use regex::Regex;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;
use std::sync::LazyLock;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::INSTANCE_ID_SEPARATOR;
use crate::InstanceId;
use crate::NamespaceSegment;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::RelationInstanceIdParseError;
use crate::RelationInstanceTypeId;
use crate::RelationTypeId;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;
use crate::namespace::Namespace;

/// Separator for the string representation of a relation instance.
pub static RELATION_INSTANCE_ID_SEPARATOR: &str = "--";

pub static RELATION_INSTANCE_ID_OUTBOUND_SEPARATOR: &str = "--[";

pub static RELATION_INSTANCE_ID_INBOUND_SEPARATOR: &str = "]-->";

// pub const RELATION_INSTANCE_ID_PATTERN: &str = r"^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})--\[(Behaviour|Component|EntityType|Extension|RelationType|FlowType|b|c|e|r|f|x)\(([a-z_]+(?:::[a-z_]+)*(?:::([A-Z][a-zA-Z0-9]*)))\)\]-->([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})$";
pub const RELATION_INSTANCE_ID_PATTERN: &str = r"^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})--\[([a-z_]+(?:::[a-z_]+)*(?:::([A-Z][a-zA-Z0-9]*))(?:__([a-zA-Z0-9_-]+))*)]-->([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})$";

pub static RELATION_INSTANCE_ID_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(RELATION_INSTANCE_ID_PATTERN).expect("Failed to construct RELATION_INSTANCE_ID_REGEX!"));

/// Unique ID of the relation instance
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
    pub fn new<RIT: Into<RelationInstanceTypeId>>(outbound_id: Uuid, ty: RIT, inbound_id: Uuid) -> Self {
        RelationInstanceId {
            outbound_id,
            ty: ty.into(),
            inbound_id,
        }
    }

    pub fn new_singleton<RT: Into<RelationTypeId>>(outbound_id: Uuid, ty: RT, inbound_id: Uuid) -> Self {
        RelationInstanceId {
            outbound_id,
            ty: RelationInstanceTypeId::new_singleton(ty),
            inbound_id,
        }
    }

    pub fn new_unique_for_instance_id<RT: Into<RelationTypeId>, ID: Into<InstanceId>>(outbound_id: Uuid, ty: RT, instance_id: ID, inbound_id: Uuid) -> Self {
        RelationInstanceId {
            outbound_id,
            ty: RelationInstanceTypeId::new(ty, instance_id),
            inbound_id,
        }
    }

    pub fn new_with_random_instance_id<RT: Into<RelationTypeId>>(outbound_id: Uuid, ty: RT, inbound_id: Uuid) -> Self {
        RelationInstanceId {
            outbound_id,
            ty: RelationInstanceTypeId::new_with_random_instance_id(ty),
            inbound_id,
        }
    }
}

impl NamespacedTypeGetter for RelationInstanceId {
    fn namespaced_type(&self) -> NamespacedType {
        self.ty.namespaced_type()
    }

    fn namespace(&self) -> Namespace {
        self.ty.namespace()
    }

    fn path(&self) -> Namespace {
        self.ty.path()
    }

    /// Returns the full instance type name (relation type name + instance id)
    /// TODO: Special Handling of RIIDs
    fn type_name(&self) -> NamespaceSegment {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for RelationInstanceId {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::RelationType
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
            Ordering::Equal => match self.outbound_id.cmp(&other.outbound_id) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => self.inbound_id.cmp(&other.inbound_id),
                Ordering::Greater => Ordering::Greater,
            },
            Ordering::Greater => Ordering::Greater,
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
        ty.type_definition()
    }
}

impl From<&RelationInstanceId> for NamespacedType {
    fn from(ty: &RelationInstanceId) -> Self {
        // Returns the namespaced type with the full instance type name (relation type name + instance id)
        NamespacedType::from(&ty.ty)
    }
}

impl FromStr for RelationInstanceId {
    type Err = RelationInstanceIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((outbound_id_part, s)) = s.split_once(RELATION_INSTANCE_ID_OUTBOUND_SEPARATOR) else {
            return Err(RelationInstanceIdParseError::MissingOutboundId);
        };
        let outbound_id = Uuid::from_str(outbound_id_part).map_err(|_| RelationInstanceIdParseError::InvalidOutboundId(outbound_id_part.to_string()))?;
        let Some((relation_instance_type_id_part, inbound_id_part)) = s.split_once(RELATION_INSTANCE_ID_INBOUND_SEPARATOR) else {
            return Err(RelationInstanceIdParseError::MissingInboundId);
        };
        let inbound_id = Uuid::from_str(inbound_id_part).map_err(|_| RelationInstanceIdParseError::InvalidInboundId(inbound_id_part.to_string()))?;
        let ty = RelationInstanceTypeId::from_str(relation_instance_type_id_part).map_err(RelationInstanceIdParseError::RelationInstanceTypeIdParseError)?;
        Ok(RelationInstanceId::new(outbound_id, ty, inbound_id))
    }
}

impl Display for RelationInstanceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.ty.instance_id() {
            InstanceId::Singleton => {
                write!(f, "{}--[{}]-->{}", self.outbound_id, &self.namespace().to_string(), self.inbound_id)
            }
            _ => {
                write!(
                    f,
                    "{}--[{}{INSTANCE_ID_SEPARATOR}{}]-->{}",
                    self.outbound_id,
                    &self.namespace().to_string(),
                    &self.ty.instance_id().to_string(),
                    self.inbound_id
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RELATION_INSTANCE_ID_REGEX;
    use super::RelationInstanceId;
    use crate::InstanceId;
    use crate::NamespaceSegment;
    use crate::RelationInstanceTypeId;
    use crate::RelationTypeId;
    use std::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn relation_instance_id_regex_test() {
        assert!(RELATION_INSTANCE_ID_REGEX.is_match("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"));
        assert!(
            RELATION_INSTANCE_ID_REGEX.is_match("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05")
        );
        assert!(
            RELATION_INSTANCE_ID_REGEX
                .is_match("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::namespace::Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05")
        );
        assert!(
            RELATION_INSTANCE_ID_REGEX
                .is_match("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::namespace::namespace::Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05")
        );

        assert!(!RELATION_INSTANCE_ID_REGEX.is_match("--[namespace::Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"));
        assert!(!RELATION_INSTANCE_ID_REGEX.is_match("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::Type]-->"));
        assert!(!RELATION_INSTANCE_ID_REGEX.is_match("--[namespace::Type]-->"));

        assert!(!RELATION_INSTANCE_ID_REGEX.is_match("Test--[namespace::Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"));
        assert!(!RELATION_INSTANCE_ID_REGEX.is_match("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::Type]-->Test"));
        assert!(!RELATION_INSTANCE_ID_REGEX.is_match("Test--[namespace::Type]-->Test"));

        assert!(
            !RELATION_INSTANCE_ID_REGEX.is_match("8f816c4e-b08a-472f-b37e-457a712656d5--[Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"),
            "NamespacedType must have a namespace"
        );
        assert!(
            !RELATION_INSTANCE_ID_REGEX.is_match("8f816c4e-b08a-472f-b37e-457a712656d5--[Namespace::Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"),
            "namespace must be lowercase"
        );
        assert!(
            !RELATION_INSTANCE_ID_REGEX.is_match("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"),
            "Type name must be uppercase"
        );
        assert!(
            !RELATION_INSTANCE_ID_REGEX.is_match("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"),
            "NamespacedType must have a type name"
        );
        assert!(
            !RELATION_INSTANCE_ID_REGEX
                .is_match("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::namespace]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"),
            "NamespacedType must have a type name"
        );
        assert!(
            !RELATION_INSTANCE_ID_REGEX.is_match("8f816c4e-b08a-472f-b37e-457a712656d5--[::namespace::namespace::Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"),
            "First segment must not be empty"
        );
        assert!(
            !RELATION_INSTANCE_ID_REGEX.is_match("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::Type::]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"),
            "Last segment must not be empty"
        );
        assert!(
            !RELATION_INSTANCE_ID_REGEX.is_match("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"),
            "Last segment must be a type name and must not be empty"
        );
    }

    #[test]
    fn relation_instance_id_from_str_test() {
        assert_eq!(
            InstanceId::Singleton,
            RelationInstanceId::from_str("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05")
                .unwrap()
                .ty
                .instance_id()
        );
        assert_eq!(
            InstanceId::Singleton,
            RelationInstanceId::from_str(
                "8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::namespace::Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"
            )
            .unwrap()
            .ty
            .instance_id()
        );
        assert_eq!(
            InstanceId::Singleton,
            RelationInstanceId::from_str(
                "8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::namespace::namespace::Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"
            )
            .unwrap()
            .ty
            .instance_id()
        );

        assert_eq!(
            InstanceId::Id(Uuid::from_str("8fe1dd09-5878-4c6e-be33-7cfc95c60ce2").unwrap()),
            RelationInstanceId::from_str("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::Type__8fe1dd09-5878-4c6e-be33-7cfc95c60ce2]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05")
                .unwrap()
                .ty
                .instance_id()
        );
        assert_eq!(
            InstanceId::Id(Uuid::from_str("8fe1dd09-5878-4c6e-be33-7cfc95c60ce2").unwrap()),
            RelationInstanceId::from_str(
                "8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::namespace::Type__8fe1dd09-5878-4c6e-be33-7cfc95c60ce2]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"
            )
            .unwrap()
            .ty
            .instance_id()
        );
        assert_eq!(
            InstanceId::Id(Uuid::from_str("8fe1dd09-5878-4c6e-be33-7cfc95c60ce2").unwrap()),
            RelationInstanceId::from_str(
                "8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::namespace::namespace::Type__8fe1dd09-5878-4c6e-be33-7cfc95c60ce2]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"
            )
            .unwrap()
            .ty
            .instance_id()
        );

        assert_eq!(
            InstanceId::Named(NamespaceSegment::from_str("test").unwrap()),
            RelationInstanceId::from_str("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::Type__test]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05")
                .unwrap()
                .ty
                .instance_id()
        );
        assert_eq!(
            InstanceId::Named(NamespaceSegment::from_str("test").unwrap()),
            RelationInstanceId::from_str(
                "8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::namespace::Type__test]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"
            )
            .unwrap()
            .ty
            .instance_id()
        );
        assert_eq!(
            InstanceId::Named(NamespaceSegment::from_str("test").unwrap()),
            RelationInstanceId::from_str(
                "8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::namespace::namespace::Type__test]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"
            )
            .unwrap()
            .ty
            .instance_id()
        );

        assert_eq!(
            InstanceId::MultiSegmented(vec![
                NamespaceSegment::from_str("segment_1").unwrap(),
                NamespaceSegment::from_str("segment_2").unwrap(),
                NamespaceSegment::from_str("segment_3").unwrap()
            ]),
            RelationInstanceId::from_str(
                "8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::Type__segment_1__segment_2__segment_3]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"
            )
            .unwrap()
            .ty
            .instance_id()
        );
        assert_eq!(
            InstanceId::MultiSegmented(vec![
                NamespaceSegment::from_str("segment_1").unwrap(),
                NamespaceSegment::from_str("segment_2").unwrap(),
                NamespaceSegment::from_str("segment_3").unwrap()
            ]),
            RelationInstanceId::from_str(
                "8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::namespace::Type__segment_1__segment_2__segment_3]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"
            )
                .unwrap()
                .ty
                .instance_id()
        );
        assert_eq!(
            InstanceId::MultiSegmented(vec![
                NamespaceSegment::from_str("segment_1").unwrap(),
                NamespaceSegment::from_str("segment_2").unwrap(),
                NamespaceSegment::from_str("segment_3").unwrap()
            ]),
            RelationInstanceId::from_str(
                "8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::namespace::namespace::Type__segment_1__segment_2__segment_3]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05"
            )
                .unwrap()
                .ty
                .instance_id()
        );

        assert!(
            RelationInstanceId::from_str("--[namespace::Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05").is_err(),
            "Missing outbound id"
        );
        assert!(
            RelationInstanceId::from_str("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::Type]-->").is_err(),
            "Missing inbound id"
        );
        assert!(RelationInstanceId::from_str("--[namespace::Type]-->").is_err(), "Missing outbound and inbound id");

        assert!(
            RelationInstanceId::from_str("Test--[namespace::Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05").is_err(),
            "Outbound id must be a valid UUID"
        );
        assert!(
            RelationInstanceId::from_str("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::Type]-->Test").is_err(),
            "Inbound id must be a valid UUID"
        );
        assert!(
            RelationInstanceId::from_str("Test--[namespace::Type]-->Test").is_err(),
            "Outbound id and Inbound id must be a valid UUID"
        );

        assert!(
            RelationInstanceId::from_str("8f816c4e-b08a-472f-b37e-457a712656d5--[Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05").is_err(),
            "NamespacedType must have a namespace"
        );
        assert!(
            RelationInstanceId::from_str("8f816c4e-b08a-472f-b37e-457a712656d5--[Namespace::Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05").is_err(),
            "namespace must be lowercase"
        );
        assert!(
            RelationInstanceId::from_str("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05").is_err(),
            "Type name must be uppercase"
        );
        assert!(
            RelationInstanceId::from_str("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05").is_err(),
            "NamespacedType must have a type name"
        );
        assert!(
            RelationInstanceId::from_str("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::namespace]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05")
                .is_err(),
            "NamespacedType must have a type name"
        );
        assert!(
            RelationInstanceId::from_str("8f816c4e-b08a-472f-b37e-457a712656d5--[::namespace::namespace::Type]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05")
                .is_err(),
            "First segment must not be empty"
        );
        assert!(
            RelationInstanceId::from_str("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::namespace::Type::]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05")
                .is_err(),
            "Last segment must not be empty"
        );
        assert!(
            RelationInstanceId::from_str("8f816c4e-b08a-472f-b37e-457a712656d5--[namespace::]-->f35395e1-d5ea-4e84-8dbd-331e1aaf3d05").is_err(),
            "Last segment must be a type name and must not be empty"
        );
    }

    #[test]
    fn relation_instance_id_to_string_test() {
        let outbound_id = Uuid::from_str("65b3aab3-82ad-4605-a9e7-227f4da73c28").unwrap();
        let relation_ty = RelationTypeId::from_str("namespace::namespace::Type").unwrap();
        let inbound_id = Uuid::from_str("a99b7f91-5bdb-44b9-9a87-3a6e24a54b3f").unwrap();

        let relation_instance_id = RelationInstanceId::builder()
            .outbound_id(outbound_id)
            .ty(RelationInstanceTypeId::new(relation_ty.clone(), InstanceId::Singleton))
            .inbound_id(inbound_id)
            .build();

        let string_representation = relation_instance_id.to_string();
        println!("{}", string_representation);
        assert_eq!(relation_instance_id, RelationInstanceId::from_str(&string_representation).unwrap());

        let relation_instance_id = RelationInstanceId::builder()
            .outbound_id(outbound_id)
            .ty(RelationInstanceTypeId::new(
                relation_ty.clone(),
                InstanceId::Id(Uuid::from_str("f9271804-3f5d-495a-84af-a4a782aec160").unwrap()),
            ))
            .inbound_id(inbound_id)
            .build();

        let string_representation = relation_instance_id.to_string();
        assert_eq!(relation_instance_id, RelationInstanceId::from_str(&string_representation).unwrap());

        let relation_instance_id = RelationInstanceId::builder()
            .outbound_id(outbound_id)
            .ty(RelationInstanceTypeId::new(
                relation_ty.clone(),
                InstanceId::Named(NamespaceSegment::from_str("test").unwrap()),
            ))
            .inbound_id(inbound_id)
            .build();

        let string_representation = relation_instance_id.to_string();
        assert_eq!(relation_instance_id, RelationInstanceId::from_str(&string_representation).unwrap());

        let relation_instance_id = RelationInstanceId::builder()
            .outbound_id(outbound_id)
            .ty(RelationInstanceTypeId::new(
                relation_ty,
                InstanceId::MultiSegmented(vec![
                    NamespaceSegment::from_str("segment_1").unwrap(),
                    NamespaceSegment::from_str("segment_2").unwrap(),
                    NamespaceSegment::from_str("segment_3").unwrap(),
                ]),
            ))
            .inbound_id(inbound_id)
            .build();

        let string_representation = relation_instance_id.to_string();
        assert_eq!(relation_instance_id, RelationInstanceId::from_str(&string_representation).unwrap());
    }
}
