use crate::NamespaceSegment;
use crate::NamespacedType;
use crate::NamespacedTypeError;
use crate::NamespacedTypeGetter;
use crate::TypeIdType;
use crate::TypeIdTypeParseError;
use crate::namespace::Namespace;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Debug, Error)]
pub enum TypeDefinitionConversionError {
    #[error("The type definition {0} has type id type {1} but the target type id type is {2}")]
    TypeIdTypeMatchError(TypeDefinition, TypeIdType, TypeIdType),
}

/// Definition of a type with the type of the type, the namespace and the name of the type.
#[derive(Clone, Debug, PartialEq, Eq, JsonSchema, TypedBuilder)]
pub struct TypeDefinition {
    pub type_id_type: TypeIdType,
    pub namespaced_type: NamespacedType,
}

impl TypeDefinition {
    /// Constructs a new type definition from the given type id type and the given namespaced type.
    pub fn new<NT: Into<NamespacedType>>(type_id_type: TypeIdType, namespaced_type: NT) -> TypeDefinition {
        TypeDefinition {
            type_id_type,
            namespaced_type: namespaced_type.into(),
        }
    }

    /// Constructs a type definition for a component.
    pub fn component<NT: Into<NamespacedType>>(namespaced_type: NT) -> TypeDefinition {
        TypeDefinition::new(TypeIdType::Component, namespaced_type.into())
    }

    /// Constructs a type definition for a entity type.
    pub fn entity_type<NT: Into<NamespacedType>>(namespaced_type: NT) -> TypeDefinition {
        TypeDefinition::new(TypeIdType::EntityType, namespaced_type.into())
    }

    /// Constructs a type definition for a relation type.
    pub fn relation_type<NT: Into<NamespacedType>>(namespaced_type: NT) -> TypeDefinition {
        TypeDefinition::new(TypeIdType::RelationType, namespaced_type.into())
    }

    /// Constructs a type definition for a flow type.
    pub fn flow_type<NT: Into<NamespacedType>>(namespaced_type: NT) -> TypeDefinition {
        TypeDefinition::new(TypeIdType::FlowType, namespaced_type.into())
    }

    pub fn relative_url(&self) -> String {
        format!("{}/{}", self.type_id_type, self.namespaced_type.namespace.relative_url())
    }

    pub fn relative_path(&self) -> PathBuf {
        let mut relative_path = self.type_id_type.relative_path();
        relative_path.push(self.namespaced_type.namespace.relative_path());
        relative_path
    }
}

impl NamespacedTypeGetter for TypeDefinition {
    fn namespaced_type(&self) -> NamespacedType {
        self.namespaced_type.clone()
    }

    fn namespace(&self) -> Namespace {
        self.namespaced_type.namespace()
    }

    fn path(&self) -> Namespace {
        self.namespaced_type.path()
    }

    fn type_name(&self) -> NamespaceSegment {
        self.namespaced_type.type_name()
    }
}

impl Serialize for TypeDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{self}").as_str())
    }
}

impl<'de> Deserialize<'de> for TypeDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = String::deserialize(deserializer)?;
        let type_definition = Self::try_from(&v).map_err(Error::custom)?;
        Ok(type_definition)
    }
}

/// Returns the fully qualified type name.
impl Display for TypeDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.type_id_type.full_name(), &self.namespaced_type)
    }
}

/// Returns the type of the type.
impl From<&TypeDefinition> for TypeIdType {
    fn from(type_definition: &TypeDefinition) -> Self {
        type_definition.type_id_type
    }
}

/// Returns the type of the type.
impl From<&TypeDefinition> for NamespacedType {
    fn from(type_definition: &TypeDefinition) -> Self {
        type_definition.namespaced_type.clone()
    }
}

#[derive(Debug, Error)]
pub enum TypeDefinitionParseError {
    #[error("The type id type is missing")]
    MissingTypeIdType,
    #[error("The type id type is invalid: {0}")]
    TypeIdTypeParseError(#[from] TypeIdTypeParseError),
    #[error("The namespace is missing")]
    MissingNamespace,
    #[error("The namespaced type is invalid: {0}")]
    NamespacedTypeError(#[from] NamespacedTypeError),
}

#[derive(Debug, Error)]
pub enum TypeIdParseError {
    #[error("Failed to parse type id: {0}")]
    TypeDefinitionParseError(TypeDefinitionParseError),
    #[error("The type id type must be {0} but was {1}")]
    InvalidTypeIdType(TypeIdType, TypeIdType),
}

impl TryFrom<&str> for TypeDefinition {
    type Error = TypeDefinitionParseError;

    fn try_from(type_definition: &str) -> Result<Self, Self::Error> {
        let mut s = type_definition.split(&['(', ')'][..]);
        let type_id_type: TypeIdType = s
            .next()
            .ok_or(TypeDefinitionParseError::MissingTypeIdType)?
            .try_into()
            .map_err(TypeDefinitionParseError::TypeIdTypeParseError)?;
        let namespaced_type: NamespacedType = s
            .next()
            .map(|s| s.to_owned())
            .ok_or(TypeDefinitionParseError::MissingNamespace)?
            .try_into()
            .map_err(TypeDefinitionParseError::NamespacedTypeError)?;
        Ok(Self::new(type_id_type, namespaced_type))
    }
}

impl TryFrom<&String> for TypeDefinition {
    type Error = TypeDefinitionParseError;

    fn try_from(type_definition: &String) -> Result<Self, Self::Error> {
        TypeDefinition::try_from(type_definition.as_str())
    }
}

impl TryFrom<String> for TypeDefinition {
    type Error = TypeDefinitionParseError;

    fn try_from(type_definition: String) -> Result<Self, Self::Error> {
        TypeDefinition::try_from(&type_definition)
    }
}

/// Grants access to the type definition of a type of types.
pub trait TypeDefinitionGetter {
    /// Returns the type definition of the type.
    fn type_definition(&self) -> TypeDefinition;

    /// Returns the type id type.
    fn type_id_type() -> TypeIdType;
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use crate::Namespace;
    use crate::NamespacedType;
    use crate::NamespacedTypeGetter;
    use crate::TypeDefinition;
    use crate::TypeIdType;
    use reactive_graph_utils_test::r_namespace_path_segment;
    use reactive_graph_utils_test::r_namespace_type_name;

    #[test]
    fn type_definition_component_test() {
        let path_segment_1 = r_namespace_path_segment();
        let path_segment_2 = r_namespace_path_segment();
        let type_name = r_namespace_type_name();
        let namespace = Namespace::try_new_top_level(&path_segment_1)
            .and_then(|namespace| namespace.try_append(&path_segment_2))
            .and_then(|namespace| namespace.try_append(&type_name))
            .expect("Failed to construct namespace");
        let nt = NamespacedType::new(&namespace).expect("Failed to create Namespaced type");
        let td = TypeDefinition::new(TypeIdType::Component, nt.clone());
        assert_eq!(TypeIdType::Component, td.type_id_type);
        assert_eq!(namespace, td.namespace());
        assert_eq!(type_name, td.type_name().to_string());
        assert_eq!(format!("Component({namespace})"), td.to_string());

        let tid: TypeIdType = TypeIdType::from(&td);
        assert_eq!(TypeIdType::Component, tid);

        let nt2: NamespacedType = NamespacedType::from(&td);
        assert_eq!(nt, nt2);

        let t = format!("Component({})", namespace);

        let td2 = TypeDefinition::try_from(&t).unwrap();
        assert_eq!(TypeIdType::Component, td2.type_id_type);
        assert_eq!(namespace, td2.namespace());
        assert_eq!(type_name, td2.type_name().to_string());
        assert_eq!(format!("Component({})", nt2.namespace), td2.to_string());
        assert_eq!(td, td2);
    }

    #[test]
    fn type_definition_component_2_test() {
        let path_segment_1 = r_namespace_path_segment();
        let path_segment_2 = r_namespace_path_segment();
        let type_name = r_namespace_type_name();
        let namespace = Namespace::try_new_top_level(&path_segment_1)
            .and_then(|namespace| namespace.try_append(&path_segment_2))
            .and_then(|namespace| namespace.try_append(&type_name))
            .expect("Failed to construct namespace");
        let nt = NamespacedType::new(&namespace).expect("Failed to create Namespaced type");
        let td = TypeDefinition::new(TypeIdType::Component, &nt);
        assert_eq!(TypeIdType::Component, td.type_id_type);
        assert_eq!(namespace, td.namespace());
        assert_eq!(type_name, td.type_name().to_string());
        assert_eq!(format!("Component({namespace})"), td.to_string());
    }

    #[test]
    fn type_definition_component_3_test() {
        let path_segment_1 = r_namespace_path_segment();
        let path_segment_2 = r_namespace_path_segment();
        let type_name = r_namespace_type_name();
        let namespace = Namespace::try_new_top_level(&path_segment_1)
            .and_then(|namespace| namespace.try_append(&path_segment_2))
            .and_then(|namespace| namespace.try_append(&type_name))
            .expect("Failed to construct namespace");
        let nt = NamespacedType::new(&namespace).expect("Failed to create namespaced type");
        let td = TypeDefinition::component(&nt);
        assert_eq!(TypeIdType::Component, td.type_id_type);
        assert_eq!(namespace, td.namespace());
        assert_eq!(format!("Component({namespace})"), td.to_string());
    }

    #[test]
    fn type_definition_entity_type_test() {
        let path_segment_1 = r_namespace_path_segment();
        let path_segment_2 = r_namespace_path_segment();
        let type_name = r_namespace_type_name();
        let namespace = Namespace::try_new_top_level(&path_segment_1)
            .and_then(|namespace| namespace.try_append(&path_segment_2))
            .and_then(|namespace| namespace.try_append(&type_name))
            .expect("Failed to construct namespace");
        let nt = NamespacedType::new(&namespace).expect("Failed to create namespaced type");
        let td = TypeDefinition::entity_type(&nt);
        assert_eq!(TypeIdType::EntityType, td.type_id_type);
        assert_eq!(namespace, td.namespace());
        assert_eq!(type_name, td.type_name().to_string());
        assert_eq!(format!("EntityType({namespace})"), td.to_string());

        let tid: TypeIdType = TypeIdType::from(&td);
        assert_eq!(TypeIdType::EntityType, tid);
    }

    #[test]
    fn type_definition_relation_type_test() {
        let path_segment_1 = r_namespace_path_segment();
        let path_segment_2 = r_namespace_path_segment();
        let type_name = r_namespace_type_name();
        let namespace = Namespace::try_new_top_level(&path_segment_1)
            .and_then(|namespace| namespace.try_append(&path_segment_2))
            .and_then(|namespace| namespace.try_append(&type_name))
            .expect("Failed to construct namespace");
        let nt = NamespacedType::new(&namespace).expect("Failed to create namespaced type");
        let td = TypeDefinition::relation_type(&nt);
        assert_eq!(TypeIdType::RelationType, td.type_id_type);
        assert_eq!(namespace, td.namespace());
        assert_eq!(type_name, td.type_name().to_string());
        assert_eq!(format!("RelationType({namespace})"), td.to_string());

        let tid: TypeIdType = TypeIdType::from(&td);
        assert_eq!(TypeIdType::RelationType, tid);
    }

    #[test]
    fn type_definition_flow_type_test() {
        let path_segment_1 = r_namespace_path_segment();
        let path_segment_2 = r_namespace_path_segment();
        let type_name = r_namespace_type_name();
        let namespace = Namespace::try_new_top_level(&path_segment_1)
            .and_then(|namespace| namespace.try_append(&path_segment_2))
            .and_then(|namespace| namespace.try_append(&type_name))
            .expect("Failed to construct namespace");
        let nt = NamespacedType::new(&namespace).expect("Failed to create namespaced type");
        let td = TypeDefinition::flow_type(&nt);
        assert_eq!(TypeIdType::FlowType, td.type_id_type);
        assert_eq!(namespace, td.namespace());
        assert_eq!(type_name, td.type_name().to_string());
        assert_eq!(format!("FlowType({namespace})"), td.to_string());

        let tid: TypeIdType = TypeIdType::from(&td);
        assert_eq!(TypeIdType::FlowType, tid);
    }

    #[test]
    fn type_definition_component_from_string_test() {
        let path_segment_1 = r_namespace_path_segment();
        let path_segment_2 = r_namespace_path_segment();
        let type_name = r_namespace_type_name();
        let t = format!("Component({path_segment_1}::{path_segment_2}::{type_name})");
        let td = TypeDefinition::try_from(&t).expect("Failed to parse type definition");
        assert_eq!(TypeIdType::Component, td.type_id_type);
        assert_eq!(format!("{path_segment_1}::{path_segment_2}"), td.path().to_string());
        assert_eq!(type_name, td.type_name().to_string());
        assert_eq!(format!("Component({path_segment_1}::{path_segment_2}::{type_name})"), td.to_string());
    }

    #[test]
    fn type_definition_json_schema() {
        let schema = schema_for!(TypeDefinition);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
