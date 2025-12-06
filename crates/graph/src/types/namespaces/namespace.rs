use convert_case::Case::Pascal;
use convert_case::Casing;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeSet;
use std::collections::btree_set::IntoIter;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;

use crate::NamespaceSegment;
use crate::NamespaceSegmentError;

#[cfg(any(test, feature = "test"))]
use rand::Rng;

/// Separator for the string representation of a type definition.
pub static NAMESPACE_SEPARATOR: &str = "::";

/// Separator for the relative path representation of a type definition.
pub static RELATIVE_PATH_SEPARATOR: &str = "/";

#[derive(Debug, Error)]
pub enum NamespaceError {
    #[error("The namespace is invalid because a segment is invalid: {0}")]
    SegmentError(#[from] NamespaceSegmentError),
    #[error("\"{0}\" is not a valid namespace because a type must be prefixed with a path")]
    MissingPathForType(NamespaceSegment),
    #[error("Type \"{0}\" cannot be appended with \"{1}\" because types must be the last segment of a namespace.")]
    TypeCannotBeAppended(Namespace, NamespaceSegment),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, JsonSchema, Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
pub struct Namespace(Vec<NamespaceSegment>);

impl Namespace {
    pub fn new_top_level_from_segment(segment: NamespaceSegment) -> Result<Namespace, NamespaceError> {
        if segment.is_type() {
            return Err(NamespaceError::MissingPathForType(segment));
        }
        Ok(Namespace(vec![segment]))
    }

    pub fn try_new_top_level<S: Into<String>>(segment: S) -> Result<Namespace, NamespaceError> {
        let segment = segment.into();
        NamespaceSegment::from_str(&segment)
            .map_err(NamespaceError::from)
            .and_then(Namespace::new_top_level_from_segment)
    }

    pub fn try_append_segment(&self, segment: NamespaceSegment) -> Result<Self, NamespaceError> {
        let mut namespace = self.clone();
        if self.is_type() {
            return Err(NamespaceError::TypeCannotBeAppended(namespace, segment));
        }
        namespace.0.push(segment);
        Ok(namespace)
    }

    pub fn try_append<S: Into<String>>(self, segment: S) -> Result<Self, NamespaceError> {
        let segment = segment.into();
        self.try_append_segment(NamespaceSegment::from_str(&segment).map_err(NamespaceError::from)?)
    }

    pub fn parent(&self) -> Option<Namespace> {
        let mut segments = self.get_segments();
        if segments.len() <= 1 {
            return None;
        }
        if segments.pop().is_none() {
            return None;
        }
        Some(Namespace(segments))
    }

    pub fn get_segments(&self) -> Vec<NamespaceSegment> {
        self.0.clone()
    }

    pub fn last_segment(&self) -> Option<NamespaceSegment> {
        self.0.last().cloned()
    }

    pub fn is_path(&self) -> bool {
        !self.is_type()
    }

    pub fn is_type(&self) -> bool {
        self.last_segment().map(|segment| segment.is_type()).unwrap_or(false)
    }

    pub fn relative_url(&self) -> String {
        self.to_string().replace(NAMESPACE_SEPARATOR, RELATIVE_PATH_SEPARATOR)
    }

    pub fn relative_path(&self) -> PathBuf {
        PathBuf::from(self.relative_url())
    }

    /// Returns the fully qualified type name in pascal case.
    /// For example, namespace1::namespace2::namespace3::TypeName will become
    /// Namespace1Namespace2Namespace3TypeName
    pub fn fully_qualified_type_name(&self) -> String {
        format!("{}", self.0.iter().map(|y| y.to_string().to_case(Pascal)).collect::<Vec<String>>().join(""))
    }

    pub fn parse_optional_namespace(namespace: Option<String>) -> Result<Option<Self>, NamespaceError>
    where
        Self: Sized,
    {
        match namespace {
            None => Ok(None),
            Some(namespace) => Self::from_str(&namespace).map(Some),
        }
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|segment| segment.to_string())
                .collect::<Vec<String>>()
                .join(NAMESPACE_SEPARATOR)
        )
    }
}

impl From<&Namespace> for Namespace {
    fn from(namespace: &Namespace) -> Self {
        namespace.clone()
    }
}

impl From<Namespace> for String {
    fn from(namespace: Namespace) -> Self {
        namespace.to_string()
    }
}

impl FromStr for Namespace {
    type Err = NamespaceError;

    fn from_str(namespace: &str) -> Result<Self, Self::Err> {
        let mut split = namespace.split(NAMESPACE_SEPARATOR);
        let Some(top_level_segment) = split.next() else {
            return Err(NamespaceError::SegmentError(NamespaceSegmentError::MustNotBeEmpty));
        };
        let mut namespace = Namespace::try_new_top_level(top_level_segment)?;
        while let Some(segment) = split.next() {
            namespace = namespace.try_append(segment)?;
        }
        Ok(namespace)
    }
}

impl TryFrom<NamespaceSegment> for Namespace {
    type Error = NamespaceError;

    fn try_from(segment: NamespaceSegment) -> Result<Self, Self::Error> {
        Namespace::new_top_level_from_segment(segment)
    }
}

// Required because of #[serde(try_from = "String")]
impl TryFrom<String> for Namespace {
    type Error = NamespaceError;

    fn try_from(namespace: String) -> Result<Self, Self::Error> {
        Self::from_str(namespace.as_str())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Namespaces(BTreeSet<Namespace>);

impl Namespaces {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn to_vec(&self) -> Vec<Namespace> {
        let mut tys: Vec<Namespace> = self.iter().map(|ty| ty.clone()).collect();
        tys.sort();
        tys
    }

    // TODO: to_sorted_vec()
}

impl Deref for Namespaces {
    type Target = BTreeSet<Namespace>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Namespaces {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for Namespaces {
    type Item = Namespace;
    type IntoIter = IntoIter<Namespace>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for Namespaces {
    fn eq(&self, other: &Self) -> bool {
        let this = self.to_vec();
        let other = other.to_vec();
        this.eq(&other)
    }
}

impl Eq for Namespaces {}

impl Hash for Namespaces {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl From<Vec<Namespace>> for Namespaces {
    fn from(tys: Vec<Namespace>) -> Self {
        Self(tys.into_iter().collect())
    }
}

impl From<Namespaces> for Vec<Namespace> {
    fn from(tys: Namespaces) -> Self {
        tys.to_vec()
    }
}

impl From<&Namespaces> for Vec<Namespace> {
    fn from(tys: &Namespaces) -> Self {
        tys.0.iter().map(|ty| ty.clone()).collect()
    }
}

impl From<BTreeSet<Namespace>> for Namespaces {
    fn from(tys: BTreeSet<Namespace>) -> Self {
        Self(tys)
    }
}

impl From<&BTreeSet<Namespace>> for Namespaces {
    fn from(tys: &BTreeSet<Namespace>) -> Self {
        Self(tys.clone())
    }
}

impl From<Namespaces> for BTreeSet<Namespace> {
    fn from(tys: Namespaces) -> Self {
        tys.0
    }
}

impl FromIterator<Namespace> for Namespaces {
    fn from_iter<I: IntoIterator<Item = Namespace>>(iter: I) -> Self {
        let mut tys = Self::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
    }
}

#[cfg(any(test, feature = "test"))]
impl Namespace {
    pub fn random_path() -> Result<Self, NamespaceError> {
        let mut namespace = Namespace::new_top_level_from_segment(NamespaceSegment::random_path_segment().map_err(NamespaceError::SegmentError)?)?;
        let mut rng = rand::rng();
        for _ in 1..rng.random_range(1..5) {
            namespace = namespace.try_append_segment(NamespaceSegment::random_path_segment().map_err(NamespaceError::SegmentError)?)?;
        }
        Ok(namespace)
    }

    pub fn random_child_path(&self) -> Result<Self, NamespaceError> {
        let mut namespace = self.clone();
        let mut rng = rand::rng();
        for _ in 1..rng.random_range(1..5) {
            namespace = namespace.try_append_segment(NamespaceSegment::random_path_segment().map_err(NamespaceError::SegmentError)?)?;
        }
        Ok(namespace)
    }

    pub fn random_type() -> Result<Self, NamespaceError> {
        Self::random_path()?.try_append_segment(NamespaceSegment::random_type_segment().map_err(NamespaceError::SegmentError)?)
    }

    pub fn random_child_type(&self) -> Result<Self, NamespaceError> {
        self.random_child_path()?
            .try_append_segment(NamespaceSegment::random_type_segment().map_err(NamespaceError::SegmentError)?)
    }
}

#[cfg(any(test, feature = "test"))]
impl Namespaces {
    pub fn random_types() -> Result<Self, NamespaceError> {
        let mut types = Self::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            types.insert(Namespace::random_type()?);
        }
        Ok(types)
    }

    pub fn random_path_tree() -> Result<Self, NamespaceError> {
        let mut types = Self::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            let namespace = Namespace::random_path()?;
            for _ in 0..rng.random_range(0..10) {
                types.insert(namespace.random_child_path()?);
            }
        }
        Ok(types)
    }
    pub fn random_type_tree() -> Result<Self, NamespaceError> {
        let mut types = Self::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            let namespace = Namespace::random_path()?;
            for _ in 0..rng.random_range(0..10) {
                types.insert(namespace.random_child_type()?);
            }
        }
        Ok(types)
    }
}

#[cfg(test)]
mod tests {
    use super::Namespace;
    use std::str::FromStr;

    #[test]
    fn random_path_namespace_test() {
        assert!(Namespace::random_path().is_ok(), "Failed to create random path namespace");
    }

    #[test]
    fn random_type_namespace_test() {
        assert!(Namespace::random_type().is_ok(), "Failed to create random type namespace");
    }

    #[test]
    fn namespace_from_str_test() {
        assert!(Namespace::from_str("namespace").is_ok());
        assert!(Namespace::from_str("namespace::Type").is_ok());
        assert!(Namespace::from_str("name_space::TypeName").is_ok());
        assert!(Namespace::from_str("namespace::namespace").is_ok());
        assert!(Namespace::from_str("namespace::namespace::Type").is_ok());
        assert!(Namespace::from_str("namespace::namespace::namespace").is_ok());
        assert!(Namespace::from_str("namespace::namespace::namespace::Type").is_ok());
        assert!(Namespace::from_str("namespace::namespace::namespace::namespace").is_ok());
        assert!(Namespace::from_str("namespace::namespace::namespace::namespace::Type").is_ok());

        assert!(Namespace::from_str("").is_err());
        assert!(Namespace::from_str("::").is_err());
        assert!(Namespace::from_str("::::").is_err());
        assert!(Namespace::from_str("__").is_err());
        assert!(Namespace::from_str("____").is_err());
        assert!(Namespace::from_str("namespace__Type").is_err());
        assert!(Namespace::from_str("namespace::namespace__Type").is_err());
        assert!(Namespace::from_str("Type").is_err());
        assert!(Namespace::from_str("Namespace::Type").is_err());
        assert!(Namespace::from_str("namespace::Namespace::Type").is_err());
        assert!(Namespace::from_str("Namespace::namespace::type").is_err());
        assert!(Namespace::from_str("namespace::Namespace::type").is_err());
        assert!(Namespace::from_str("::Namespace::Type").is_err());
        assert!(Namespace::from_str("::Namespace::Type::").is_err());
        assert!(Namespace::from_str("namespace::Type::").is_err());
        assert!(Namespace::from_str("Namespace::Type::").is_err());
    }
}
