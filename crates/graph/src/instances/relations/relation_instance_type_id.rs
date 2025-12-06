use regex::Regex;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;
use std::sync::LazyLock;

use crate::INSTANCE_ID_SEPARATOR;
use crate::InstanceId;
use crate::NamespaceSegment;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::RelationInstanceTypeIdParseError;
use crate::RelationTypeId;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeDefinitionParseError;
use crate::TypeIdParseError;
use crate::TypeIdType;
use crate::namespace::Namespace;

#[cfg(any(test, feature = "test"))]
use crate::NamespacedTypeError;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildType;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildTypeId;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypeId;

pub const RELATION_INSTANCE_TYPE_ID_PATTERN: &str = r"^[a-z_]+(?:::[a-z_]+)*(?:::([A-Z][a-zA-Z0-9]*))(?:__([a-zA-Z0-9_-]+))*$";

pub static RELATION_INSTANCE_TYPE_ID_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(RELATION_INSTANCE_TYPE_ID_PATTERN).expect("Failed to construct RELATION_INSTANCE_TYPE_ID_REGEX!"));

/// Type identifier of a relation instance.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct RelationInstanceTypeId {
    /// The type definition of the relation type.
    #[serde(rename = "type")]
    ty: RelationTypeId,

    /// The instance id.
    instance_id: InstanceId,
}

impl RelationInstanceTypeId {
    pub fn new<RT: Into<RelationTypeId>, ID: Into<InstanceId>>(ty: RT, instance_id: ID) -> Self {
        Self {
            ty: ty.into(),
            instance_id: instance_id.into(),
        }
    }

    /// Between two entity instances there can be only one relation instance.
    pub fn new_singleton<RT: Into<RelationTypeId>>(ty: RT) -> Self {
        Self {
            ty: ty.into(),
            instance_id: InstanceId::Singleton,
        }
    }

    // Between two entity instances there can be only one relation instance with the same instance
    // id.
    //
    // For example, multiple connectors exists between two entity instances. But only one connector
    // is allowed between two properties.

    /// Between two entity instances there can be multiple one relation instances. The instance id
    /// of the relation instance will be generated randomly.
    pub fn new_with_random_instance_id<RT: Into<RelationTypeId>>(ty: RT) -> Self {
        Self {
            ty: ty.into(),
            instance_id: InstanceId::new_with_random_id(),
        }
    }

    /// Returns true, if the relation instance type id is of the given relation type id.
    pub fn is_a(&self, ty: &RelationTypeId) -> bool {
        &self.ty == ty
    }

    /// Returns the inner relation type id.
    pub fn relation_type_id(&self) -> RelationTypeId {
        self.ty.clone()
    }

    /// Returns the instance id.
    pub fn instance_id(&self) -> InstanceId {
        self.instance_id.clone()
    }

    // TODO: BREAKING CHANGE: Replace all calls to type_name()
    // pub fn fully_qualified_type_name(&self) -> String {
    // }
}

impl NamespacedTypeGetter for RelationInstanceTypeId {
    fn namespaced_type(&self) -> NamespacedType {
        self.ty.namespaced_type()
    }

    fn namespace(&self) -> Namespace {
        self.ty.namespace()
    }

    fn path(&self) -> Namespace {
        self.ty.path()
    }

    // TODO: BREAKING CHANGE: Replace all calls to type_name()
    fn type_name(&self) -> NamespaceSegment {
        // todo!();
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for RelationInstanceTypeId {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::RelationType
    }
}

impl AsRef<NamespacedType> for RelationInstanceTypeId {
    fn as_ref(&self) -> &NamespacedType {
        &self.ty.as_ref()
    }
}

impl AsRef<Namespace> for RelationInstanceTypeId {
    fn as_ref(&self) -> &Namespace {
        &self.ty.as_ref()
    }
}

impl PartialOrd<Self> for RelationInstanceTypeId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RelationInstanceTypeId {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.ty.cmp(&other.ty) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.instance_id.cmp(&other.instance_id),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl Display for RelationInstanceTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RelationInstanceTypeId({}{INSTANCE_ID_SEPARATOR}{})", &self.ty, self.instance_id().to_string())
    }
}

impl From<&RelationInstanceTypeId> for RelationInstanceTypeId {
    fn from(ty: &RelationInstanceTypeId) -> Self {
        ty.clone()
    }
}

impl From<&RelationInstanceTypeId> for TypeDefinition {
    fn from(ty: &RelationInstanceTypeId) -> Self {
        ty.ty.type_definition()
    }
}

impl From<&RelationInstanceTypeId> for NamespacedType {
    fn from(ty: &RelationInstanceTypeId) -> Self {
        let relation_ty = &ty.ty;
        relation_ty.into()
    }
}

impl FromStr for RelationInstanceTypeId {
    type Err = RelationInstanceTypeIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if RELATION_INSTANCE_TYPE_ID_REGEX.is_match(s) {
            return match s.split_once(INSTANCE_ID_SEPARATOR) {
                None => Ok(RelationInstanceTypeId::new(
                    RelationTypeId::from_str(s).map_err(RelationInstanceTypeIdParseError::NamespacedTypeParseError)?,
                    InstanceId::Singleton,
                )),
                Some((namespace_part, instance_id_part)) => Ok(RelationInstanceTypeId::new(
                    RelationTypeId::from_str(namespace_part).map_err(RelationInstanceTypeIdParseError::NamespacedTypeParseError)?,
                    InstanceId::from_str(instance_id_part)?,
                )),
            };
        }
        // Parse TypeDefinition Format RelationType(namespace::namespace::TypeName,instance_id)
        let mut s = s.split(&['(', ',', ')'][..]);
        let type_id_type = TypeIdType::from_str(
            s.next()
                .ok_or(RelationInstanceTypeIdParseError::TypeDefinitionParseError(TypeDefinitionParseError::MissingTypeIdType))?,
        )
        .map_err(TypeDefinitionParseError::TypeIdTypeParseError)
        .map_err(RelationInstanceTypeIdParseError::TypeDefinitionParseError)?;
        if TypeIdType::RelationType != type_id_type {
            return Err(TypeIdParseError::InvalidTypeIdType(TypeIdType::RelationType, type_id_type).into());
        }
        println!("{}", type_id_type);
        let namespaced_type = NamespacedType::from_str(
            s.next()
                .ok_or(RelationInstanceTypeIdParseError::TypeDefinitionParseError(TypeDefinitionParseError::MissingNamespace))?,
        )
        .map_err(TypeDefinitionParseError::NamespacedTypeError)
        .map_err(RelationInstanceTypeIdParseError::TypeDefinitionParseError)?;
        println!("{}", namespaced_type);
        let instance_id = match s.next() {
            None => Ok(InstanceId::Singleton),
            Some(id) => InstanceId::from_str(id),
        }
        .map_err(RelationInstanceTypeIdParseError::InstanceIdError)?;
        println!("{}", instance_id);
        Ok(RelationInstanceTypeId::new(RelationTypeId::new(namespaced_type), instance_id))
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomNamespacedTypeId for RelationInstanceTypeId {
    type Error = NamespacedTypeError;

    fn random_type_id() -> Result<Self, NamespacedTypeError> {
        NamespacedType::random_type_id().map(RelationInstanceTypeId::new_with_random_instance_id)
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomChildType for RelationInstanceTypeId {
    type Error = NamespacedTypeError;

    fn random_child_type(namespace: &Namespace) -> Result<Self, Self::Error> {
        NamespacedType::random_child_type_id(namespace).map(RelationInstanceTypeId::new_with_random_instance_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::InstanceId;
    use crate::NamespaceSegment;
    use crate::RELATION_INSTANCE_TYPE_ID_REGEX;
    use crate::RandomNamespacedTypeId;
    use crate::RelationInstanceTypeId;
    use crate::RelationTypeId;
    use schemars::schema_for;
    use std::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn relation_instance_type_id_regex_test() {
        // InstanceId::Singleton
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::Type"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::Type"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::namespace::Type"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::namespace::namespace::Type"));
        // InstanceId::Id
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::Type__2a82b484-7f47-4a76-844f-5a2e686c8680"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::Type__2a82b484-7f47-4a76-844f-5a2e686c8680"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::namespace::Type__2a82b484-7f47-4a76-844f-5a2e686c8680"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::namespace::namespace::Type__2a82b484-7f47-4a76-844f-5a2e686c8680"));
        // InstanceId::Named
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::Type__name"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::Type__name"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::namespace::Type__name"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::namespace::namespace::Type__name"));
        // InstanceId::MultiSegmented
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::Type__segment_1__segment_2"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::Type__segment_1__segment_2__segment_3"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::Type__segment_1__segment_2__segment_3__segment_4"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::Type__segment_1__segment_2"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::Type__segment_1__segment_2__segment_3"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::Type__segment_1__segment_2__segment_3__segment_4"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::namespace::Type__segment_1__segment_2"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::namespace::Type__segment_1__segment_2__segment_3"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::namespace::Type__segment_1__segment_2__segment_3__segment_4"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::namespace::namespace::Type__segment_1__segment_2"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::namespace::namespace::Type__segment_1__segment_2__segment_3"));
        assert!(RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::namespace::namespace::Type__segment_1__segment_2__segment_3__segment_4"));

        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("Namespace::Type"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("Namespace::Type__2a82b484-7f47-4a76-844f-5a2e686c8680"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("Namespace::Type__name"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("Namespace::Type__segment_1__segment_2"));

        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::type"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::type__2a82b484-7f47-4a76-844f-5a2e686c8680"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::type__name"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::type__segment_1__segment_2"));

        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::Namespace::Type"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::Namespace::Type__2a82b484-7f47-4a76-844f-5a2e686c8680"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::Namespace::Type__name"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::Namespace::Type__segment_1__segment_2"));

        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::type"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::type__2a82b484-7f47-4a76-844f-5a2e686c8680"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::type__name"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::namespace::type__segment_1__segment_2"));

        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("Namespace::namespace::type"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("Namespace::namespace::type__2a82b484-7f47-4a76-844f-5a2e686c8680"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("Namespace::namespace::type__name"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("Namespace::namespace::type__segment_1__segment_2"));

        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::Namespace::type"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::Namespace::type__2a82b484-7f47-4a76-844f-5a2e686c8680"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::Namespace::type__name"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("namespace::Namespace::type__segment_1__segment_2"));

        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("::Namespace::Type"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("::Namespace::Type__2a82b484-7f47-4a76-844f-5a2e686c8680"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("::Namespace::Type__name"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("::Namespace::Type__segment_1__segment_2"));

        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("::Namespace::Type::"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("::Namespace::Type::__2a82b484-7f47-4a76-844f-5a2e686c8680"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("::Namespace::Type::__name"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("::Namespace::Type::__segment_1__segment_2"));

        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("Namespace::Type::"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("Namespace::Type::__2a82b484-7f47-4a76-844f-5a2e686c8680"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("Namespace::Type::__name"));
        assert!(!RELATION_INSTANCE_TYPE_ID_REGEX.is_match("Namespace::Type::__segment_1__segment_2"));
    }

    #[test]
    fn relation_instance_type_id_from_string_test() {
        assert_eq!(InstanceId::Singleton, RelationInstanceTypeId::from_str("namespace::namespace::Type").unwrap().instance_id);
        assert_eq!(
            InstanceId::Singleton,
            RelationInstanceTypeId::from_str("namespace::namespace::namespace::Type").unwrap().instance_id
        );
        assert_eq!(
            InstanceId::Singleton,
            RelationInstanceTypeId::from_str("namespace::namespace::namespace::namespace::Type")
                .unwrap()
                .instance_id
        );

        let id = Uuid::from_str("2a82b484-7f47-4a76-844f-5a2e686c8680").unwrap();
        assert_eq!(
            InstanceId::Id(id),
            RelationInstanceTypeId::from_str("namespace::namespace::Type__2a82b484-7f47-4a76-844f-5a2e686c8680")
                .expect("Failed to parse RelationInstanceTypeId with UUID")
                .instance_id
        );
        assert_eq!(
            InstanceId::Id(id),
            RelationInstanceTypeId::from_str("namespace::namespace::namespace::Type__2a82b484-7f47-4a76-844f-5a2e686c8680")
                .unwrap()
                .instance_id
        );
        assert_eq!(
            InstanceId::Id(id),
            RelationInstanceTypeId::from_str("namespace::namespace::namespace::namespace::Type__2a82b484-7f47-4a76-844f-5a2e686c8680")
                .unwrap()
                .instance_id
        );

        let named_segment = NamespaceSegment::from_str("name").unwrap();
        assert_eq!(
            InstanceId::Named(named_segment.clone()),
            RelationInstanceTypeId::from_str("namespace::namespace::Type__name").unwrap().instance_id
        );
        assert_eq!(
            InstanceId::Named(named_segment.clone()),
            RelationInstanceTypeId::from_str("namespace::namespace::namespace::Type__name")
                .unwrap()
                .instance_id
        );
        assert_eq!(
            InstanceId::Named(named_segment.clone()),
            RelationInstanceTypeId::from_str("namespace::namespace::namespace::namespace::Type__name")
                .unwrap()
                .instance_id
        );

        let segment_1 = NamespaceSegment::from_str("segment_1").unwrap();
        let segment_2 = NamespaceSegment::from_str("segment_2").unwrap();
        let segments = vec![segment_1, segment_2];
        assert_eq!(
            InstanceId::MultiSegmented(segments.clone()),
            RelationInstanceTypeId::from_str("namespace::namespace::Type__segment_1__segment_2")
                .unwrap()
                .instance_id
        );
        assert_eq!(
            InstanceId::MultiSegmented(segments.clone()),
            RelationInstanceTypeId::from_str("namespace::namespace::namespace::Type__segment_1__segment_2")
                .unwrap()
                .instance_id
        );
        assert_eq!(
            InstanceId::MultiSegmented(segments.clone()),
            RelationInstanceTypeId::from_str("namespace::namespace::namespace::namespace::Type__segment_1__segment_2")
                .unwrap()
                .instance_id
        );

        assert!(RelationInstanceTypeId::from_str("Namespace::Type").is_err());
        assert!(RelationInstanceTypeId::from_str("Namespace::Type__2a82b484-7f47-4a76-844f-5a2e686c8680").is_err());
        assert!(RelationInstanceTypeId::from_str("Namespace::Type__name").is_err());
        assert!(RelationInstanceTypeId::from_str("Namespace::Type__segment_1__segment_2").is_err());

        assert!(RelationInstanceTypeId::from_str("namespace::type").is_err());
        assert!(RelationInstanceTypeId::from_str("namespace::type__2a82b484-7f47-4a76-844f-5a2e686c8680").is_err());
        assert!(RelationInstanceTypeId::from_str("namespace::type__name").is_err());
        assert!(RelationInstanceTypeId::from_str("namespace::type__segment_1__segment_2").is_err());

        assert!(RelationInstanceTypeId::from_str("namespace::Namespace::Type").is_err());
        assert!(RelationInstanceTypeId::from_str("namespace::Namespace::Type__2a82b484-7f47-4a76-844f-5a2e686c8680").is_err());
        assert!(RelationInstanceTypeId::from_str("namespace::Namespace::Type__name").is_err());
        assert!(RelationInstanceTypeId::from_str("namespace::Namespace::Type__segment_1__segment_2").is_err());

        assert!(RelationInstanceTypeId::from_str("namespace::namespace::type").is_err());
        assert!(RelationInstanceTypeId::from_str("namespace::namespace::type__2a82b484-7f47-4a76-844f-5a2e686c8680").is_err());
        assert!(RelationInstanceTypeId::from_str("namespace::namespace::type__name").is_err());
        assert!(RelationInstanceTypeId::from_str("namespace::namespace::type__segment_1__segment_2").is_err());

        assert!(RelationInstanceTypeId::from_str("Namespace::namespace::type").is_err());
        assert!(RelationInstanceTypeId::from_str("Namespace::namespace::type__2a82b484-7f47-4a76-844f-5a2e686c8680").is_err());
        assert!(RelationInstanceTypeId::from_str("Namespace::namespace::type__name").is_err());
        assert!(RelationInstanceTypeId::from_str("Namespace::namespace::type__segment_1__segment_2").is_err());

        assert!(RelationInstanceTypeId::from_str("namespace::Namespace::type").is_err());
        assert!(RelationInstanceTypeId::from_str("namespace::Namespace::type__2a82b484-7f47-4a76-844f-5a2e686c8680").is_err());
        assert!(RelationInstanceTypeId::from_str("namespace::Namespace::type__name").is_err());
        assert!(RelationInstanceTypeId::from_str("namespace::Namespace::type__segment_1__segment_2").is_err());

        assert!(RelationInstanceTypeId::from_str("::Namespace::Type").is_err());
        assert!(RelationInstanceTypeId::from_str("::Namespace::Type__2a82b484-7f47-4a76-844f-5a2e686c8680").is_err());
        assert!(RelationInstanceTypeId::from_str("::Namespace::Type__name").is_err());
        assert!(RelationInstanceTypeId::from_str("::Namespace::Type__segment_1__segment_2").is_err());

        assert!(RelationInstanceTypeId::from_str("::Namespace::Type::").is_err());
        assert!(RelationInstanceTypeId::from_str("::Namespace::Type::__2a82b484-7f47-4a76-844f-5a2e686c8680").is_err());
        assert!(RelationInstanceTypeId::from_str("::Namespace::Type::__name").is_err());
        assert!(RelationInstanceTypeId::from_str("::Namespace::Type::__segment_1__segment_2").is_err());

        assert!(RelationInstanceTypeId::from_str("Namespace::Type::").is_err());
        assert!(RelationInstanceTypeId::from_str("Namespace::Type::__2a82b484-7f47-4a76-844f-5a2e686c8680").is_err());
        assert!(RelationInstanceTypeId::from_str("Namespace::Type::__name").is_err());
        assert!(RelationInstanceTypeId::from_str("Namespace::Type::__segment_1__segment_2").is_err());
    }

    #[test]
    fn relation_instance_type_id_singleton_test() {
        let relation_ty = RelationTypeId::random_type_id().unwrap();
        let relation_instance_ty = RelationInstanceTypeId::new_singleton(relation_ty.clone());
        assert_eq!(relation_ty, relation_instance_ty.relation_type_id());
        assert_eq!(InstanceId::Singleton, relation_instance_ty.instance_id());
    }

    #[test]
    fn relation_instance_type_id_random_test() {
        let relation_ty = RelationTypeId::random_type_id().unwrap();
        let relation_instance_ty = RelationInstanceTypeId::new_with_random_instance_id(relation_ty.clone());
        assert_eq!(relation_ty, relation_instance_ty.relation_type_id());
        match relation_instance_ty.instance_id() {
            InstanceId::Id(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn relation_instance_type_id_json_schema() {
        let schema = schema_for!(RelationInstanceTypeId);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
