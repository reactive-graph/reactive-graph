use dashmap::DashSet;
use dashmap::iter_set::OwningIter;
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
use typed_builder::TypedBuilder;

use crate::NamespaceError;
use crate::namespace::Namespace;
use crate::namespace::NamespaceSegment;

#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use rand_derive3::RandGen;
#[cfg(any(test, feature = "table"))]
use tabled::Tabled;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NamespacedTypeError {
    #[error("{0} is not a valid namespaced type because a type must be prefixed with a path")]
    MissingPathForType(Namespace),
    #[error("The namespace {0} must contain a type name as last segment")]
    TypeNameMissing(Namespace),
    #[error("The namespaced type is not a valid namespace: {0}")]
    NamespaceError(#[from] NamespaceError),
}

pub trait NamespacedTypeConstructor {
    /// Creates a new namespaced type.
    fn new<NT: Into<NamespacedType>>(nt: NT) -> Self;

    fn parse_namespace(namespace: &String) -> Result<Self, NamespacedTypeError>
    where
        Self: Sized,
    {
        Ok(Self::new(NamespacedType::try_from(namespace)?))
    }

    fn parse_optional_namespace(namespace: Option<String>) -> Result<Option<Self>, NamespacedTypeError>
    where
        Self: Sized,
    {
        match namespace {
            None => Ok(None),
            Some(namespace) => Self::parse_namespace(&namespace).map(Some),
        }
        // Ok(namespace.map(|namespace| Self::parse_namespace(&namespace)?))
    }
}

/// Grants access to the namespace and the type name of a type of types.
pub trait NamespacedTypeGetter {
    /// Returns the namespaced type.
    fn namespaced_type(&self) -> NamespacedType;

    /// Returns the fully qualified namespace of the type.
    fn namespace(&self) -> Namespace;

    /// Returns the path of the type without the type name segment.
    fn path(&self) -> Namespace;

    /// Returns the name of the type.
    fn type_name(&self) -> NamespaceSegment;
}

/// Defines the namespace and the name of a type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema, TypedBuilder)]
#[serde(into = "String", try_from = "String")]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
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

impl TryFrom<String> for NamespacedType {
    type Error = NamespacedTypeError;

    fn try_from(namespace: String) -> Result<Self, Self::Error> {
        Self::new(Namespace::try_from(namespace).map_err(NamespacedTypeError::from)?)
    }
}

impl TryFrom<&String> for NamespacedType {
    type Error = NamespacedTypeError;

    fn try_from(namespace: &String) -> Result<Self, Self::Error> {
        Self::try_from(namespace.clone())
    }
}

// impl<N: Into<Namespace>, T: Into<String>> From<(N, T)> for NamespacedType {
//     fn from(ty: (N, T)) -> Self {
//         NamespacedType::new(ty.0.into(), ty.1.into())
//     }
// }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NamespacedTypes(DashSet<NamespacedType>);

impl NamespacedTypes {
    pub fn new() -> Self {
        Self(DashSet::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn to_vec(&self) -> Vec<NamespacedType> {
        let mut tys: Vec<NamespacedType> = self.iter().map(|ty| ty.clone()).collect();
        tys.sort();
        tys
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
impl DefaultTest for NamespacedType {
    fn default_test() -> Self {
        NamespacedType::generate_random()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for NamespacedTypes {
    fn default_test() -> Self {
        let tys = Self::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            tys.insert(NamespacedType::default_test());
        }
        tys
    }
}

#[cfg(test)]
pub mod tests {
    use default_test::DefaultTest;
    use schemars::schema_for;

    use crate::Namespace;
    use crate::NamespacedType;
    use crate::NamespacedTypeGetter;
    use reactive_graph_utils_test::r_namespace_path_segment;
    use reactive_graph_utils_test::r_namespace_type_name;

    impl DefaultTest for NamespacedType {
        fn default_test() -> Self {
            NamespacedType::new(Namespace::default_test()).unwrap()
        }
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
