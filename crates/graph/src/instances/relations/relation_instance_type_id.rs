use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use thiserror::Error;
use uuid::Uuid;

use crate::NamespaceSegment;
use crate::NamespaceSegmentError;
use crate::NamespacedType;
use crate::NamespacedTypeError;
use crate::NamespacedTypeGetter;
use crate::RelationTypeId;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeDefinitionParseError;
use crate::TypeIdParseError;
use crate::TypeIdType;
use crate::namespace::Namespace;

#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand_derive3::RandGen;

/// Separator for the string representation of a property connector.
pub static INSTANCE_ID_SEPARATOR: &str = "__";

#[derive(Debug, Error)]
pub enum InstanceIdError {
    #[error("The instance id {0} is invalid")]
    InvalidInstanceId(String),
    #[error("The instance id is invalid: {0}")]
    NamespaceSegmentError(#[from] NamespaceSegmentError),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(into = "String", try_from = "String")]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
pub enum InstanceId {
    Singleton,
    Id(Uuid),
    Named(NamespaceSegment),
    MultiSegmented(Vec<NamespaceSegment>),
}

impl InstanceId {
    pub fn new_with_random_id() -> Self {
        InstanceId::Id(Uuid::new_v4())
    }

    pub fn new_segmented(segments: Vec<NamespaceSegment>) -> Self {
        InstanceId::MultiSegmented(segments)
    }

    pub fn parse_named(instance_id: &str) -> Result<Self, InstanceIdError> {
        Ok(InstanceId::Named(NamespaceSegment::try_from(instance_id).map_err(InstanceIdError::NamespaceSegmentError)?))
    }
}

impl Display for InstanceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let id = match self {
            InstanceId::Singleton => "".to_string(),
            InstanceId::Id(id) => id.to_string(),
            InstanceId::Named(id) => id.to_string(),
            InstanceId::MultiSegmented(segments) => segments.iter().map(ToString::to_string).collect::<Vec<_>>().join(INSTANCE_ID_SEPARATOR),
        };
        write!(f, "{id}")
    }
}

impl From<InstanceId> for String {
    fn from(instance_id: InstanceId) -> Self {
        instance_id.to_string()
    }
}

impl TryFrom<&str> for InstanceId {
    type Error = InstanceIdError;

    fn try_from(instance_id: &str) -> Result<Self, Self::Error> {
        if instance_id.is_empty() {
            return Ok(InstanceId::Singleton);
        }
        if let Ok(id) = Uuid::parse_str(instance_id) {
            return Ok(InstanceId::Id(id));
        }
        let instance_id = NamespaceSegment::try_from(instance_id).map_err(InstanceIdError::NamespaceSegmentError)?;
        if !instance_id.as_ref().contains(INSTANCE_ID_SEPARATOR) {
            return Ok(InstanceId::Named(instance_id));
        }
        let mut segments = vec![];
        for segment in instance_id.as_ref().split(INSTANCE_ID_SEPARATOR) {
            segments.push(NamespaceSegment::try_from(segment).map_err(|e| InstanceIdError::NamespaceSegmentError(e))?);
        }
        Ok(InstanceId::MultiSegmented(segments))
    }
}

impl TryFrom<&String> for InstanceId {
    type Error = InstanceIdError;

    fn try_from(instance_id: &String) -> Result<Self, Self::Error> {
        Self::try_from(instance_id.as_str())
    }
}

impl TryFrom<String> for InstanceId {
    type Error = InstanceIdError;

    fn try_from(instance_id: String) -> Result<Self, Self::Error> {
        Self::try_from(instance_id.as_str())
    }
}

#[derive(Debug, Error)]
pub enum RelationInstanceTypeIdError {
    #[error("Failed to construct relation instance type id because of an error with the instance id: {0}")]
    InstanceIdError(#[from] InstanceIdError),
    #[error("Failed to construct relation instance type id because of an error with the namespaced type: {0}")]
    NamespacedTypeError(#[from] NamespacedTypeError),
}

/// Type identifier of a relation instance.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
pub struct RelationInstanceTypeId {
    /// The type definition of the relation type.
    #[serde(flatten)]
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

    // TODO: BREAKING CHANGE: Aufrufe auf type_name() umschreiben zu fully_qualified_type_name()
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

    /// BREAKING CHANGE
    fn type_name(&self) -> NamespaceSegment {
        todo!();
        // self.ty.type_name()
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

#[derive(Debug, Error)]
pub enum RelationInstanceTypeIdParseError {
    #[error("Failed to parse type definition: {0}")]
    TypeDefinitionParseError(#[from] TypeDefinitionParseError),
    #[error("Failed to parse type id: {0}")]
    TypeIdParseError(#[from] TypeIdParseError),
    #[error("Failed to parse instance id: {0}")]
    InstanceIdError(#[from] InstanceIdError),
}

impl TryFrom<&str> for RelationInstanceTypeId {
    type Error = RelationInstanceTypeIdParseError;

    fn try_from(relation_type_definition: &str) -> Result<Self, Self::Error> {
        let mut s = relation_type_definition.split(&['(', ',', ')'][..]);
        let type_id_type: TypeIdType = s
            .next()
            .ok_or(RelationInstanceTypeIdParseError::TypeDefinitionParseError(TypeDefinitionParseError::MissingTypeIdType))?
            .try_into()
            .map_err(TypeDefinitionParseError::TypeIdTypeParseError)
            .map_err(RelationInstanceTypeIdParseError::TypeDefinitionParseError)?;
        if TypeIdType::RelationType != type_id_type {
            return Err(TypeIdParseError::InvalidTypeIdType(TypeIdType::RelationType, type_id_type).into());
        }
        let namespaced_type: NamespacedType = s
            .next()
            .map(|s| s.to_owned())
            .ok_or(RelationInstanceTypeIdParseError::TypeDefinitionParseError(TypeDefinitionParseError::MissingNamespace))?
            .try_into()
            .map_err(TypeDefinitionParseError::NamespacedTypeError)
            .map_err(RelationInstanceTypeIdParseError::TypeDefinitionParseError)?;
        let instance_id = match s.next() {
            None => Ok(InstanceId::Singleton),
            Some(id) => InstanceId::try_from(id),
        }
        .map_err(RelationInstanceTypeIdParseError::InstanceIdError)?;
        Ok(RelationInstanceTypeId::new(RelationTypeId::new(namespaced_type), instance_id))
    }
}

impl TryFrom<&String> for RelationInstanceTypeId {
    type Error = RelationInstanceTypeIdParseError;

    fn try_from(relation_instance_type_id: &String) -> Result<Self, Self::Error> {
        RelationInstanceTypeId::try_from(relation_instance_type_id.as_str())
    }
}

impl TryFrom<String> for RelationInstanceTypeId {
    type Error = RelationInstanceTypeIdParseError;

    fn try_from(relation_instance_type_id: String) -> Result<Self, Self::Error> {
        RelationInstanceTypeId::try_from(relation_instance_type_id.as_str())
    }
}

impl Display for RelationInstanceTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
        // TODO: 2023-09-03
        // if self.instance_id.is_empty() {
        //     write!(f, "{}", &self.type_definition().to_string())
        // } else {
        //     write!(f, "{}{}{}", &self.type_definition().to_string(), &TYPE_ID_TYPE_SEPARATOR, self.instance_id)
        // }
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for RelationInstanceTypeId {
    fn default_test() -> Self {
        RelationInstanceTypeId::new_with_random_instance_id(NamespacedType::generate_random())
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use crate::NamespacedType;
    use crate::NamespacedTypeGetter;
    use crate::RelationInstanceTypeId;
    use crate::RelationTypeId;
    use crate::TypeDefinition;
    use crate::TypeDefinitionGetter;
    use crate::TypeIdType;
    use reactive_graph_utils_test::r_string;

    #[test]
    fn relation_instance_type_id_unique_id_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let rty = RelationTypeId::new_from_type(&namespace, &type_name);
        let ty = RelationInstanceTypeId::new_singleton(rty.clone());
        assert_eq!(namespace, ty.namespace());
        assert_eq!(nt.namespace, ty.namespace());
        assert_eq!(nt.type_name, ty.type_name());
        assert_eq!(format!("r__{namespace}__{type_name}"), format!("{}", ty));
        let type_definition = ty.type_definition();
        assert_eq!(TypeIdType::RelationType, type_definition.type_id_type);
        assert_eq!(namespace, type_definition.namespace());
        assert_eq!(type_name, type_definition.type_name());

        let type_definition_3 = TypeDefinition::from(&ty);
        assert_eq!(TypeIdType::RelationType, type_definition_3.type_id_type);
        assert_eq!(namespace, type_definition_3.namespace());
        assert_eq!(type_name, type_definition_3.type_name());

        let ty2 = RelationInstanceTypeId::new_singleton(rty.clone());
        assert_eq!(ty, ty2);
        assert_eq!(ty.namespace(), ty2.namespace());
        assert_eq!(ty.type_name(), ty2.type_name());
        assert_eq!(ty.instance_id(), ty2.instance_id());
        assert_eq!(ty.to_string(), ty2.to_string());
    }

    #[test]
    fn relation_instance_type_id_unique_for_instance_id_test() {
        let namespace = r_string();
        let type_name = r_string();
        let instance_id = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let rty = RelationTypeId::new_from_type(&namespace, &type_name);
        let ty = RelationInstanceTypeId::new(rty.clone(), &instance_id);
        assert_eq!(namespace, ty.namespace());
        assert_eq!(nt.namespace, ty.namespace());
        assert_eq!(format!("{}__{}", type_name, instance_id), ty.type_name());
        assert_eq!(instance_id, ty.instance_id());
        assert_eq!(rty, ty.relation_type_id());
        assert_eq!(format!("r__{namespace}__{type_name}__{instance_id}"), format!("{}", ty));
        let type_definition = ty.type_definition();
        assert_eq!(TypeIdType::RelationType, type_definition.type_id_type);
        assert_eq!(namespace, type_definition.namespace());
        assert_eq!(format!("{}__{}", type_name, instance_id), type_definition.type_name());

        let type_definition_3 = TypeDefinition::from(&ty);
        assert_eq!(TypeIdType::RelationType, type_definition_3.type_id_type);
        assert_eq!(namespace, type_definition_3.namespace());
        assert_eq!(format!("{}__{}", type_name, instance_id), type_definition_3.type_name());

        let instance_id_2 = r_string();
        let ty2 = RelationInstanceTypeId::new(rty.clone(), &instance_id_2);
        assert_eq!(namespace, ty2.namespace());
        assert_eq!(nt.namespace, ty2.namespace());
        assert_eq!(format!("{}__{}", type_name, instance_id_2), ty2.type_name());
        assert_eq!(instance_id_2, ty2.instance_id());
        assert_eq!(rty, ty2.relation_type_id());
        assert_eq!(format!("r__{namespace}__{type_name}__{instance_id_2}"), format!("{}", ty2));
        assert_ne!(ty, ty2);
        assert_eq!(ty.namespace(), ty2.namespace());
        assert_ne!(ty.type_name(), ty2.type_name());
        assert_eq!(ty.relation_type_id(), ty2.relation_type_id());
        assert_ne!(ty.instance_id(), ty2.instance_id());
        assert_ne!(ty.to_string(), ty2.to_string());
    }

    #[test]
    fn relation_instance_type_id_with_random_instance_id_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let rty = RelationTypeId::new_from_type(&namespace, &type_name);
        let ty = RelationInstanceTypeId::new_with_random_instance_id(rty.clone());
        assert_eq!(namespace, ty.namespace());
        assert_eq!(nt.namespace, ty.namespace());
        assert_eq!(format!("{}__{}", type_name, ty.instance_id()), ty.type_name());
        assert_eq!(rty, ty.relation_type_id());
        assert_eq!(format!("r__{namespace}__{type_name}__{}", ty.instance_id()), format!("{}", ty));
        let type_definition = ty.type_definition();
        assert_eq!(TypeIdType::RelationType, type_definition.type_id_type);
        assert_eq!(namespace, type_definition.namespace());
        assert_eq!(format!("{}__{}", type_name, ty.instance_id()), type_definition.type_name());

        let type_definition_3 = TypeDefinition::from(&ty);
        assert_eq!(TypeIdType::RelationType, type_definition_3.type_id_type);
        assert_eq!(namespace, type_definition_3.namespace());
        assert_eq!(format!("{}__{}", type_name, ty.instance_id()), type_definition_3.type_name());

        let ty2 = RelationInstanceTypeId::new_with_random_instance_id(rty.clone());
        assert_eq!(namespace, ty2.namespace());
        assert_eq!(nt.namespace, ty2.namespace());
        assert_eq!(format!("{}__{}", type_name, ty2.instance_id()), ty2.type_name());
        assert_ne!(ty.instance_id(), ty2.instance_id());
        assert_eq!(rty, ty2.relation_type_id());
        assert_eq!(format!("r__{namespace}__{type_name}__{}", ty2.instance_id()), format!("{}", ty2));
        assert_ne!(ty, ty2);
        assert_eq!(ty.namespace(), ty2.namespace());
        assert_ne!(ty.type_name(), ty2.type_name());
        assert_eq!(ty.relation_type_id(), ty2.relation_type_id());
        assert_ne!(ty.instance_id(), ty2.instance_id());
        assert_ne!(ty.to_string(), ty2.to_string());
    }

    #[test]
    fn relation_instance_type_id_from_string_test() {
        let t1 = String::from("r__ns__ty");
        let ty1 = RelationInstanceTypeId::try_from(&t1).unwrap();
        assert_eq!("ns", ty1.namespace());
        assert_eq!("ty", ty1.relation_type_id().type_name());
        assert_eq!("ty", ty1.type_name());
        assert!(ty1.instance_id().is_empty());

        let t2 = String::from("r__ns__ty__instance");
        let ty2 = RelationInstanceTypeId::try_from(&t2).unwrap();
        assert_eq!("ns", ty2.namespace());
        assert_eq!("ty", ty2.relation_type_id().type_name());
        assert_eq!("ty__instance", ty2.type_name());
        assert_eq!("instance", ty2.instance_id());

        let t3 = String::from("r__ns__ty__outbound__inbound");
        let ty3 = RelationInstanceTypeId::try_from(&t3).unwrap();
        assert_eq!("ns", ty3.namespace());
        assert_eq!("ty", ty3.relation_type_id().type_name());
        assert_eq!("ty__outbound__inbound", ty3.type_name());
        assert_eq!("outbound__inbound", ty3.instance_id());

        let t4 = String::from("e__ns__ty");
        let ty4 = RelationInstanceTypeId::try_from(&t4);
        assert!(ty4.is_err());

        let t5 = String::from("r__");
        let ty5 = RelationInstanceTypeId::try_from(&t5);
        assert!(ty5.is_err());

        let t6 = String::from("r__ns");
        let ty6 = RelationInstanceTypeId::try_from(&t6);
        assert!(ty6.is_err());

        let t7 = String::from("r__ns__");
        let ty7 = RelationInstanceTypeId::try_from(&t7);
        assert!(ty7.is_err());
    }

    #[test]
    fn relation_instance_type_id_json_schema() {
        let schema = schema_for!(RelationInstanceTypeId);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
