use crate::INSTANCE_ID_SEPARATOR;
use crate::NAMESPACE_SEPARATOR;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;
use thiserror::Error;

#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::r_namespace_path_segment;

#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::r_namespace_type_name;

#[derive(Debug, Error, PartialEq)]
pub enum NamespaceSegmentError {
    #[error("The namespace segment must not be empty")]
    MustNotBeEmpty,
    #[error("The namespace segment {0} must not contain the namespace delimiter ::")]
    MustNotContainDelimiter(String),
    #[error("The namespace segment {0} must contain alphanumeric characters only")]
    MustBeAlphanumeric(String),
}

/// A namespace segment only contains these alphanumeric chars and the underscore.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NamespaceSegment(String);

impl NamespaceSegment {
    pub fn is_path(&self) -> bool {
        !self.is_type()
    }
    pub fn is_type(&self) -> bool {
        self.0.chars().next().map(char::is_uppercase).unwrap_or_default()
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

impl FromStr for NamespaceSegment {
    type Err = NamespaceSegmentError;

    fn from_str(segment: &str) -> Result<Self, Self::Err> {
        let segment = segment.to_string();
        if segment.is_empty() {
            return Err(NamespaceSegmentError::MustNotBeEmpty.into());
        }
        if segment.contains(NAMESPACE_SEPARATOR) {
            return Err(NamespaceSegmentError::MustNotContainDelimiter(segment).into());
        }
        if segment.contains(INSTANCE_ID_SEPARATOR) {
            return Err(NamespaceSegmentError::MustNotContainDelimiter(segment).into());
        }
        if !segment.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(NamespaceSegmentError::MustBeAlphanumeric(segment).into());
        }
        Ok(NamespaceSegment(segment))
    }
}

#[cfg(any(test, feature = "test"))]
impl NamespaceSegment {
    pub fn random_path_segment() -> Result<Self, NamespaceSegmentError> {
        NamespaceSegment::from_str(&r_namespace_path_segment())
    }

    pub fn random_type_segment() -> Result<Self, NamespaceSegmentError> {
        NamespaceSegment::from_str(&r_namespace_type_name())
    }
}

#[cfg(test)]
mod tests {
    use super::NamespaceSegment;
    use super::NamespaceSegmentError;
    use std::str::FromStr;

    #[test]
    fn segment_from_str_test() {
        assert_eq!(NamespaceSegmentError::MustNotBeEmpty, NamespaceSegment::from_str("").unwrap_err());
        assert_eq!(
            NamespaceSegmentError::MustNotContainDelimiter("namespace::namespace".to_string()),
            NamespaceSegment::from_str("namespace::namespace").unwrap_err()
        );
        assert_eq!(
            NamespaceSegmentError::MustNotContainDelimiter("namespace__namespace".to_string()),
            NamespaceSegment::from_str("namespace__namespace").unwrap_err()
        );
        assert_eq!(NamespaceSegmentError::MustBeAlphanumeric("(".to_string()), NamespaceSegment::from_str("(").unwrap_err());
        assert_eq!(NamespaceSegmentError::MustBeAlphanumeric(")".to_string()), NamespaceSegment::from_str(")").unwrap_err());
        assert!(NamespaceSegment::from_str("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_").is_ok());
    }

    #[test]
    fn segment_type_test() {
        assert!(NamespaceSegment::from_str("TypeNameSegment").unwrap().is_type());
        assert!(NamespaceSegment::from_str("namespace_path_segment").unwrap().is_path());
    }

    #[test]
    fn random_path_segment_test() {
        assert!(NamespaceSegment::random_path_segment().is_ok(), "Failed to create random path segment");
    }

    #[test]
    fn random_type_segment_test() {
        assert!(NamespaceSegment::random_type_segment().is_ok(), "Failed to create random type segment");
    }
}
