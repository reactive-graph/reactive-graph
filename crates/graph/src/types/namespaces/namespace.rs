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
use std::path::PathBuf;
use thiserror::Error;

#[cfg(any(test, feature = "test"))]
use rand_derive3::RandGen;

/// Separator for the string representation of a type definition.
pub static NAMESPACE_SEPARATOR: &str = "::";

#[derive(Debug, Error)]
pub enum NamespaceError {
    #[error("The namespace is invalid because a segment is invalid: {0}")]
    SegmentError(#[from] NamespaceSegmentError),
    #[error("{0} is not a valid namespace because a type must be prefixed with a path")]
    MissingPathForType(NamespaceSegment),
    #[error("Type {0} cannot be appended with {1} because types must be the last segment of a namespace.")]
    TypeCannotBeAppended(Namespace, NamespaceSegment),
}

#[derive(Debug, Error)]
pub enum NamespaceSegmentError {
    #[error("The namespace segment must not be empty")]
    MustNotBeEmpty(String),
    #[error("The namespace segment {0} must not contain the namespace delimiter ::")]
    MustNotContainDelimiter(String),
    #[error("The namespace segment {0} must contain alphanumeric characters only")]
    MustBeAlphanumeric(String),
}

/// A namespace segment only contains these alphanumeric chars and the underscore.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
pub struct NamespaceSegment(String);

impl NamespaceSegment {
    pub fn is_path(&self) -> bool {
        !self.is_type()
    }
    pub fn is_type(&self) -> bool {
        self.0.chars().next().map(char::is_uppercase).unwrap_or(false)
    }
}

impl Display for NamespaceSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for NamespaceSegment {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<&str> for NamespaceSegment {
    type Error = NamespaceSegmentError;

    fn try_from(segment: &str) -> Result<Self, Self::Error> {
        Self::try_from(segment.to_string())
    }
}

impl TryFrom<&String> for NamespaceSegment {
    type Error = NamespaceSegmentError;

    fn try_from(segment: &String) -> Result<Self, Self::Error> {
        Self::try_from(segment.to_string())
    }
}

impl TryFrom<String> for NamespaceSegment {
    type Error = NamespaceSegmentError;

    fn try_from(segment: String) -> Result<Self, Self::Error> {
        if segment.is_empty() {
            return Err(NamespaceSegmentError::MustNotBeEmpty(segment).into());
        }
        if segment.contains(NAMESPACE_SEPARATOR) {
            return Err(NamespaceSegmentError::MustNotContainDelimiter(segment).into());
        }
        if !segment.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(NamespaceSegmentError::MustBeAlphanumeric(segment).into());
        }
        Ok(NamespaceSegment(segment))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, JsonSchema, Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
pub struct Namespace(Vec<NamespaceSegment>);

impl Namespace {
    pub fn new_top_level_from_segment(segment: NamespaceSegment) -> Result<Namespace, NamespaceError> {
        if segment.is_type() {
            return Err(NamespaceError::MissingPathForType(segment));
        }
        Ok(Namespace(vec![segment]))
    }

    pub fn try_new_top_level<S: Into<String>>(segment: S) -> Result<Namespace, NamespaceError> {
        NamespaceSegment::try_from(segment.into())
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
        self.try_append_segment(NamespaceSegment::try_from(segment.into()).map_err(NamespaceError::from)?)
    }

    pub fn parent(&self) -> Option<Namespace> {
        let mut segments = self.get_segments();
        if segments.pop().is_none() {
            return None;
        };
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
        self.to_string().replace("::", "/")
    }

    pub fn relative_path(&self) -> PathBuf {
        PathBuf::from(self.relative_url())
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|y| y.to_string()).collect::<Vec<String>>().join(NAMESPACE_SEPARATOR))
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

impl TryFrom<NamespaceSegment> for Namespace {
    type Error = NamespaceError;

    fn try_from(segment: NamespaceSegment) -> Result<Self, Self::Error> {
        Namespace::new_top_level_from_segment(segment)
    }
}

impl TryFrom<&str> for Namespace {
    type Error = NamespaceError;

    fn try_from(namespace: &str) -> Result<Self, Self::Error> {
        let mut split = namespace.split(NAMESPACE_SEPARATOR);
        let Some(top_level_segment) = split.next() else {
            return Err(NamespaceError::SegmentError(NamespaceSegmentError::MustNotBeEmpty("".to_string())));
        };
        let mut namespace = Namespace::try_new_top_level(top_level_segment)?;
        while let Some(segment) = split.next() {
            namespace = namespace.try_append(segment)?;
        }
        Ok(namespace)
    }
}

impl TryFrom<&String> for Namespace {
    type Error = NamespaceError;

    fn try_from(namespace: &String) -> Result<Self, Self::Error> {
        Self::try_from(namespace.as_str())
    }
}

impl TryFrom<String> for Namespace {
    type Error = NamespaceError;

    fn try_from(namespace: String) -> Result<Self, Self::Error> {
        Self::try_from(namespace.as_str())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Namespaces(DashSet<Namespace>);

impl Namespaces {
    pub fn new() -> Self {
        Self(DashSet::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn to_vec(&self) -> Vec<Namespace> {
        let mut tys: Vec<Namespace> = self.iter().map(|ty| ty.clone()).collect();
        tys.sort();
        tys
    }
}

impl Deref for Namespaces {
    type Target = DashSet<Namespace>;

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
    type IntoIter = OwningIter<Namespace, RandomState>;

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

impl From<DashSet<Namespace>> for Namespaces {
    fn from(tys: DashSet<Namespace>) -> Self {
        Self(tys)
    }
}

impl From<&DashSet<Namespace>> for Namespaces {
    fn from(tys: &DashSet<Namespace>) -> Self {
        Self(tys.clone())
    }
}

impl From<Namespaces> for DashSet<Namespace> {
    fn from(tys: Namespaces) -> Self {
        tys.0
    }
}

impl FromIterator<Namespace> for Namespaces {
    fn from_iter<I: IntoIterator<Item = Namespace>>(iter: I) -> Self {
        let tys = Self::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
    }
}
#[cfg(test)]
pub mod tests {
    use super::Namespace;
    use super::NamespaceSegment;
    use default_test::DefaultTest;
    use rand::Rng;
    use reactive_graph_utils_test::r_namespace_path_segment;
    use reactive_graph_utils_test::r_namespace_type_name;

    impl DefaultTest for NamespaceSegment {
        fn default_test() -> Self {
            NamespaceSegment::try_from(r_namespace_path_segment()).unwrap()
        }
    }

    impl DefaultTest for Namespace {
        fn default_test() -> Self {
            let mut namespace = Namespace::new_top_level_from_segment(NamespaceSegment::default_test()).unwrap();
            let mut rng = rand::rng();
            for _ in 1..rng.random_range(1..5) {
                namespace = namespace.try_append_segment(NamespaceSegment::default_test()).unwrap();
            }
            namespace
                .try_append_segment(NamespaceSegment::try_from(r_namespace_type_name()).unwrap())
                .unwrap()
        }
    }
}
