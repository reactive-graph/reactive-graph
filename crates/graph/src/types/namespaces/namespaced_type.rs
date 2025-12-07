use dashmap::DashSet;
use dashmap::iter_set::OwningIter;
use regex::Regex;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::hash::RandomState;
use std::ops::Deref;
use std::ops::DerefMut;
use std::str::FromStr;
use std::sync::LazyLock;

use crate::Namespace;
use crate::NamespaceSegment;
use crate::NamespacedTypeError;
use crate::NamespacedTypeParseError;
use crate::Namespaces;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildTypeId;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildTypeIds;

#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypeId;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypeIds;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "table"))]
use tabled::Tabled;

pub const NAMESPACED_TYPE_PATTERN: &str = r"^[a-z_]+(?:::[a-z_]+)*(?:::([A-Z][a-zA-Z0-9]*))$";

pub static NAMESPACED_TYPE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(NAMESPACED_TYPE_PATTERN).expect("Failed to construct NAMESPACED_TYPE_REGEX!"));

pub trait NamespacedTypeConstructor: FromStr<Err = NamespacedTypeParseError> {
    /// Creates a new namespaced type.
    fn new<NT: Into<NamespacedType>>(nt: NT) -> Self;

    fn parse_namespace(namespace: &str) -> Result<Self, NamespacedTypeError>
    where
        Self: Sized,
    {
        Ok(Self::new(NamespacedType::from_str(namespace)?))
    }

    fn parse_optional_namespace(namespace: Option<String>) -> Result<Option<Self>, NamespacedTypeError>
    where
        Self: Sized,
    {
        match namespace {
            None => Ok(None),
            Some(namespace) => Self::parse_namespace(&namespace).map(Some),
        }
    }
}

#[cfg(any(test, feature = "test"))]
impl<TY: NamespacedTypeConstructor> RandomNamespacedTypeId for TY {
    type Error = NamespacedTypeError;

    fn random_type_id() -> Result<Self, NamespacedTypeError> {
        NamespacedType::random_type_id().map(Self::new)
    }
}

#[cfg(any(test, feature = "test"))]
impl<TY: NamespacedTypeConstructor> RandomChildTypeId for TY {
    type Error = NamespacedTypeError;

    fn random_child_type_id(namespace: &Namespace) -> Result<Self, Self::Error> {
        NamespacedType::random_child_type_id(namespace).map(Self::new)
    }
}

/// Grants access to the namespace and the type name of a type of types.
pub trait NamespacedTypeGetter {
    // : AsRef<Namespace>
    /// Returns the namespaced type.
    fn namespaced_type(&self) -> NamespacedType;

    /// Returns the fully qualified namespace of the type.
    fn namespace(&self) -> Namespace;

    /// Returns the path of the type without the type name segment.
    fn path(&self) -> Namespace;

    /// Returns the name of the type.
    fn type_name(&self) -> NamespaceSegment;

    /// Returns the fully qualified type name in pascal case.
    ///
    /// For example, namespace1::namespace2::namespace3::TypeName will become
    /// Namespace1Namespace2Namespace3TypeName
    fn fully_qualified_type_name(&self) -> String {
        self.namespace().fully_qualified_type_name()
    }
    // fn fully_qualified_type_name(&self) -> String
    // where
    //     Self: AsRef<Namespace>,
    // {
    //     AsRef::<Namespace>::as_ref(self).fully_qualified_type_name()
    // }
}

/// Defines the namespace and the name of a type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(into = "String", try_from = "String")]
#[cfg_attr(any(test, feature = "table"), derive(Tabled))]
pub struct NamespacedType {
    /// The fully qualified namespace of the type.
    #[serde(alias = "name")]
    #[schemars(required)]
    #[cfg_attr(any(test, feature = "table"), tabled(rename = "FQN"))]
    pub namespace: Namespace,

    /// The path of the type without the type name segment.
    #[serde(skip)]
    #[schemars(skip)]
    #[cfg_attr(any(test, feature = "table"), tabled(rename = "Path"))]
    pub path: Namespace,

    /// The name of the type.
    #[serde(skip)]
    #[schemars(skip)]
    #[cfg_attr(any(test, feature = "table"), tabled(rename = "Type Name"))]
    pub type_name: NamespaceSegment,
}

impl NamespacedType {
    /// Constructs a new namespaced type.
    pub fn new<N: Into<Namespace>>(namespace: N) -> Result<NamespacedType, NamespacedTypeError> {
        let namespace = namespace.into();
        if !namespace.is_type() {
            return Err(NamespacedTypeError::TypeNameMissing(namespace));
        }
        let Some(path) = namespace.parent() else {
            return Err(NamespacedTypeError::MissingPathForType(namespace));
        };
        let Some(type_name) = namespace.last_segment() else {
            return Err(NamespacedTypeError::TypeNameMissing(namespace));
        };
        Ok(NamespacedType { namespace, path, type_name })
    }

    /// Returns the fully qualified namespace of the namespaced type.
    #[inline]
    pub fn namespace(&self) -> Namespace {
        NamespacedTypeGetter::namespace(self)
    }

    /// Returns the path of the namespaced type without the type name segment.
    #[inline]
    pub fn path(&self) -> Namespace {
        NamespacedTypeGetter::path(self)
    }

    /// Returns the name of the namespaced type.
    #[inline]
    pub fn type_name(&self) -> NamespaceSegment {
        NamespacedTypeGetter::type_name(self)
    }

    /// Returns the fully qualified type name in pascal case.
    ///
    /// For example, namespace1::namespace2::namespace3::TypeName will become
    /// Namespace1Namespace2Namespace3TypeName
    #[inline]
    pub fn fully_qualified_type_name(&self) -> String {
        NamespacedTypeGetter::fully_qualified_type_name(self)
    }
}

impl NamespacedTypeGetter for NamespacedType {
    /// Returns the namespaced type.
    fn namespaced_type(&self) -> NamespacedType {
        self.clone()
    }

    /// Returns the fully qualified namespace of the namespaced type.
    fn namespace(&self) -> Namespace {
        self.namespace.clone()
    }

    /// Returns the path of the namespaced type without the type name segment.
    fn path(&self) -> Namespace {
        self.path.clone()
    }

    /// Returns the name of the namespaced type.
    fn type_name(&self) -> NamespaceSegment {
        self.type_name.clone()
    }
}

impl AsRef<Namespace> for NamespacedType {
    fn as_ref(&self) -> &Namespace {
        &self.namespace
    }
}

impl Display for NamespacedType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.namespace)
    }
}

impl From<NamespacedType> for String {
    fn from(namespaced_type: NamespacedType) -> Self {
        namespaced_type.to_string()
    }
}

impl FromStr for NamespacedType {
    type Err = NamespacedTypeError;

    fn from_str(namespace: &str) -> Result<Self, Self::Err> {
        Self::new(Namespace::from_str(namespace).map_err(NamespacedTypeError::from)?)
    }
}

// Required because of #[serde(try_from = "String")]
impl TryFrom<String> for NamespacedType {
    type Error = NamespacedTypeError;

    fn try_from(namespace: String) -> Result<Self, Self::Error> {
        Self::from_str(namespace.as_str())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NamespacedTypes(DashSet<NamespacedType>);

impl NamespacedTypes {
    pub fn new() -> Self {
        Self(DashSet::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn to_vec(&self) -> Vec<NamespacedType> {
        let mut tys: Vec<NamespacedType> = self.iter().map(|ty| ty.clone()).collect();
        tys.sort();
        tys
    }

    pub fn to_sorted_vec(&self) -> Vec<NamespacedType> {
        let mut namespaced_types = self.to_vec();
        namespaced_types.sort();
        namespaced_types
    }

    pub fn get_namespaces(&self) -> Namespaces {
        let mut namespaces = Namespaces::new();
        let namespaced_types = self.to_sorted_vec();
        for namespaced_type in namespaced_types {
            let path = namespaced_type.path();
            namespaces.insert(path);
        }
        namespaces
    }

    /// Returns the paths and their parent paths of all types.
    pub fn get_all_parent_paths_recursively(&self) -> Namespaces {
        let mut namespaces = Namespaces::new();
        for namespaced_type in self.0.iter() {
            let mut path = namespaced_type.path();
            loop {
                namespaces.insert(path.clone());
                path = match path.parent() {
                    None => break,
                    Some(path) => path,
                };
            }
        }
        namespaces
    }
}

impl Deref for NamespacedTypes {
    type Target = DashSet<NamespacedType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NamespacedTypes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for NamespacedTypes {
    type Item = NamespacedType;
    type IntoIter = OwningIter<NamespacedType, RandomState>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for NamespacedTypes {
    fn eq(&self, other: &Self) -> bool {
        let this = self.to_vec();
        let other = other.to_vec();
        this.eq(&other)
    }
}

impl Eq for NamespacedTypes {}

impl Hash for NamespacedTypes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl From<Vec<NamespacedType>> for NamespacedTypes {
    fn from(tys: Vec<NamespacedType>) -> Self {
        Self(tys.into_iter().collect())
    }
}

impl From<NamespacedTypes> for Vec<NamespacedType> {
    fn from(tys: NamespacedTypes) -> Self {
        tys.to_vec()
    }
}

impl From<&NamespacedTypes> for Vec<NamespacedType> {
    fn from(tys: &NamespacedTypes) -> Self {
        tys.0.iter().map(|ty| ty.clone()).collect()
    }
}

impl From<DashSet<NamespacedType>> for NamespacedTypes {
    fn from(tys: DashSet<NamespacedType>) -> Self {
        Self(tys)
    }
}

impl From<&DashSet<NamespacedType>> for NamespacedTypes {
    fn from(tys: &DashSet<NamespacedType>) -> Self {
        Self(tys.clone())
    }
}

impl From<NamespacedTypes> for DashSet<NamespacedType> {
    fn from(tys: NamespacedTypes) -> Self {
        tys.0
    }
}

impl FromIterator<NamespacedType> for NamespacedTypes {
    fn from_iter<I: IntoIterator<Item = NamespacedType>>(iter: I) -> Self {
        let tys = Self::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomNamespacedTypeId for NamespacedType {
    type Error = NamespacedTypeError;

    fn random_type_id() -> Result<Self, Self::Error> {
        NamespacedType::new(Namespace::random_type()?)
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomChildTypeId for NamespacedType {
    type Error = NamespacedTypeError;

    fn random_child_type_id(namespace: &Namespace) -> Result<Self, Self::Error> {
        NamespacedType::new(namespace.random_child_type()?)
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomNamespacedTypeIds for NamespacedTypes {
    type Error = NamespacedTypeError;

    fn random_type_ids() -> Result<Self, NamespacedTypeError> {
        let tys = Self::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            tys.insert(NamespacedType::random_type_id()?);
        }
        Ok(tys)
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomChildTypeIds for NamespacedTypes {
    type Error = NamespacedTypeError;

    fn random_child_type_ids(namespace: &Namespace) -> Result<Self, Self::Error> {
        let tys = Self::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            tys.insert(NamespacedType::random_child_type_id(namespace)?);
        }
        Ok(tys)
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use crate::NAMESPACED_TYPE_REGEX;
    use crate::Namespace;
    use crate::NamespacedType;
    use crate::RandomNamespacedTypeId;
    use reactive_graph_utils_test::r_namespace_path_segment;
    use reactive_graph_utils_test::r_namespace_type_name;

    #[test]
    fn random_namespaced_type_test() {
        assert!(NamespacedType::random_type_id().is_ok(), "Failed to create random type");
    }

    #[test]
    fn namespaced_type_regex_test() {
        assert!(NAMESPACED_TYPE_REGEX.is_match("namespace::Type"));
        assert!(NAMESPACED_TYPE_REGEX.is_match("namespace::namespace::Type"));
        assert!(NAMESPACED_TYPE_REGEX.is_match("namespace::namespace::namespace::Type"));
        assert!(NAMESPACED_TYPE_REGEX.is_match("namespace::namespace::namespace::namespace::Type"));

        assert!(!NAMESPACED_TYPE_REGEX.is_match("Type"), "NamespacedType must have a namespace");
        assert!(!NAMESPACED_TYPE_REGEX.is_match("Namespace::Type"), "namespace must be lowercase");
        assert!(!NAMESPACED_TYPE_REGEX.is_match("namespace::type"), "Type name must be uppercase");
        assert!(!NAMESPACED_TYPE_REGEX.is_match("namespace::namespace"), "NamespacedType must have a type name");
        assert!(!NAMESPACED_TYPE_REGEX.is_match("namespace::namespace::namespace"), "NamespacedType must have a type name");
        assert!(!NAMESPACED_TYPE_REGEX.is_match("::namespace::namespace::Type"), "First segment must not be empty");
        assert!(!NAMESPACED_TYPE_REGEX.is_match("namespace::namespace::Type::"), "Last segment must not be empty");
        assert!(!NAMESPACED_TYPE_REGEX.is_match("namespace::"), "Last segment must be a type name and must not be empty");
    }

    #[test]
    fn namespaced_type_from_str_test() {
        let path_segment_1 = r_namespace_path_segment();
        let path_segment_2 = r_namespace_path_segment();
        let type_name = r_namespace_type_name();
        let ns = Namespace::try_new_top_level(&path_segment_1)
            .and_then(|namespace| namespace.try_append(&path_segment_2))
            .and_then(|namespace| namespace.try_append(&type_name))
            .expect("Failed to construct namespace");
        let nt = NamespacedType::new(&ns).expect("Failed to create Namespaced type");
        assert_eq!(ns, nt.namespace());
        assert_eq!(ns.parent().unwrap(), nt.path());
        assert_eq!(ns.parent().unwrap().to_string(), nt.path().to_string());
        assert_eq!(ns.last_segment().unwrap(), nt.type_name());
        assert_eq!(ns.last_segment().unwrap().to_string(), nt.type_name().to_string());
        assert_eq!(format!("{path_segment_1}::{path_segment_2}::{type_name}"), format!("{}", nt));
        assert_eq!(ns.to_string(), nt.to_string());
    }

    #[test]
    fn namespaced_type_json_schema() {
        let schema = schema_for!(NamespacedType);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
