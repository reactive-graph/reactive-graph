use crate::NamespaceSegment;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::TypeDefinitionParseError;
use crate::TypeIdType;
use crate::namespace::Namespace;
use regex::Regex;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::LazyLock;
use typed_builder::TypedBuilder;

#[cfg(any(test, feature = "test"))]
use crate::RandomChildTypeId;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypeId;

#[cfg(any(test, feature = "test"))]
use crate::NamespacedTypeError;

pub const TYPE_DEFINITION_PATTERN: &str =
    r"^(Behaviour|Component|EntityType|Extension|FlowType|RelationType|b|c|e|x|f|r)\(([a-z_]+(?:::[a-z_]+)*(?:::([A-Z][a-zA-Z0-9]*)))\)$";

pub static TYPE_DEFINITION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(TYPE_DEFINITION_PATTERN).expect("Failed to construct TYPE_DEFINITION_REGEX!"));

//
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
        format!("{}/{}", self.type_id_type.relative_url(), self.namespaced_type.namespace.relative_url())
    }

    pub fn relative_path(&self) -> PathBuf {
        let mut relative_path = self.type_id_type.relative_path();
        relative_path.push(self.namespaced_type.namespace.relative_path());
        relative_path
    }

    /// Returns the fully qualified type name in pascal case.
    ///
    /// For example, namespace1::namespace2::namespace3::TypeName will become
    /// Namespace1Namespace2Namespace3TypeName
    pub fn fully_qualified_type_name(&self) -> String {
        self.namespaced_type.namespace.fully_qualified_type_name()
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
        let type_definition = Self::from_str(&v).map_err(Error::custom)?;
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

impl FromStr for TypeDefinition {
    type Err = TypeDefinitionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if s.starts_with('{') && s.ends_with('}') {
        //     return serde_json::from_str::<TypeDefinition>(s).map_err(|_| TypeDefinitionParseError::ParseJsonError);
        // }
        let mut s = s.split(&['(', ')'][..]);
        let type_id_type =
            TypeIdType::from_str(s.next().ok_or(TypeDefinitionParseError::MissingTypeIdType)?).map_err(TypeDefinitionParseError::TypeIdTypeParseError)?;
        let namespaced_type =
            NamespacedType::from_str(s.next().ok_or(TypeDefinitionParseError::MissingNamespace)?).map_err(TypeDefinitionParseError::NamespacedTypeError)?;
        Ok(Self::new(type_id_type, namespaced_type))
    }
}

/// Grants access to the type definition of a type of types.
pub trait TypeDefinitionGetter {
    /// Returns the type definition of the type.
    fn type_definition(&self) -> TypeDefinition;

    /// Returns the type id type.
    fn type_id_type() -> TypeIdType;
}

#[cfg(any(test, feature = "test"))]
impl RandomNamespacedTypeId for TypeDefinition {
    type Error = NamespacedTypeError;

    fn random_type_id() -> Result<Self, NamespacedTypeError> {
        Ok(Self::new(TypeIdType::generate_random(), NamespacedType::random_type_id()?))
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomChildTypeId for TypeDefinition {
    type Error = NamespacedTypeError;

    fn random_child_type_id(namespace: &Namespace) -> Result<Self, Self::Error> {
        Ok(Self::new(TypeIdType::generate_random(), NamespacedType::random_child_type_id(namespace)?))
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;
    use std::path::PathBuf;
    use std::str::FromStr;

    use crate::Namespace;
    use crate::NamespacedType;
    use crate::NamespacedTypeGetter;
    use crate::RandomNamespacedTypeId;
    use crate::TYPE_DEFINITION_REGEX;
    use crate::TypeDefinition;
    use crate::TypeIdType;
    use reactive_graph_utils_test::r_namespace_path_segment;
    use reactive_graph_utils_test::r_namespace_type_name;

    #[test]
    fn type_definition_regex_test() {
        assert!(TYPE_DEFINITION_REGEX.is_match("Component(namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("Component(namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("Component(namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("Component(namespace::namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("EntityType(namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("EntityType(namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("EntityType(namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("EntityType(namespace::namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("Extension(namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("Extension(namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("Extension(namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("Extension(namespace::namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("FlowType(namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("FlowType(namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("FlowType(namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("FlowType(namespace::namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("RelationType(namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("RelationType(namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("RelationType(namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("RelationType(namespace::namespace::namespace::namespace::Type)"));

        assert!(TYPE_DEFINITION_REGEX.is_match("c(namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("c(namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("c(namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("c(namespace::namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("e(namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("e(namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("e(namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("e(namespace::namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("x(namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("x(namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("x(namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("x(namespace::namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("f(namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("f(namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("f(namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("f(namespace::namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("r(namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("r(namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("r(namespace::namespace::namespace::Type)"));
        assert!(TYPE_DEFINITION_REGEX.is_match("r(namespace::namespace::namespace::namespace::Type)"));
    }

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

        let td2 = TypeDefinition::from_str(&t).unwrap();
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
        let td = TypeDefinition::new(TypeIdType::Component, nt);
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
        let td = TypeDefinition::component(nt);
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
        let td = TypeDefinition::entity_type(nt);
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
        let td = TypeDefinition::relation_type(nt);
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
        let td = TypeDefinition::flow_type(nt);
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
        let td = TypeDefinition::from_str(&t).expect("Failed to parse type definition");
        assert_eq!(TypeIdType::Component, td.type_id_type);
        assert_eq!(format!("{path_segment_1}::{path_segment_2}"), td.path().to_string());
        assert_eq!(type_name, td.type_name().to_string());
        assert_eq!(format!("Component({path_segment_1}::{path_segment_2}::{type_name})"), td.to_string());
    }

    #[test]
    fn type_definition_relative_url_test() {
        let td = TypeDefinition::random_type_id().unwrap();
        assert_eq!(format!("{}/{}", td.type_id_type.relative_url(), td.namespace().relative_url()), td.relative_url());
    }

    #[test]
    fn type_definition_relative_path_test() {
        let td = TypeDefinition::random_type_id().unwrap();
        assert_eq!(
            PathBuf::from(format!("{}/{}", td.type_id_type.relative_url(), td.namespace().relative_url())),
            td.relative_path()
        );
    }

    #[test]
    fn type_definition_json_schema() {
        let schema = schema_for!(TypeDefinition);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
