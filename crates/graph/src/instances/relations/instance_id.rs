use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

use crate::NamespaceSegment;
use crate::NamespaceSegmentError;

/// Separator for the string representation of a property connector.
pub static INSTANCE_ID_SEPARATOR: &str = "__";

#[derive(Debug, Error, PartialEq)]
pub enum InstanceIdError {
    #[error("The instance id {0} is invalid")]
    InvalidInstanceId(String),
    #[error("The instance id is invalid: {0}")]
    NamespaceSegmentError(#[from] NamespaceSegmentError),
    #[error("At least one segment is required")]
    AtLeastOneSegmentRequired,
    #[error("Named instance ids must not contain the instance id separator")]
    MustNotContainDelimiter,
    #[error("Named instance ids must not be empty")]
    NamedInstanceIdMustNotBeEmpty,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(into = "String", try_from = "String")]
pub enum InstanceId {
    Singleton,
    Id(Uuid),
    Named(NamespaceSegment),
    MultiSegmented(Vec<NamespaceSegment>),
}

impl InstanceId {
    pub fn new_singleton() -> Self {
        InstanceId::Singleton
    }

    pub fn new_with_id(id: Uuid) -> Self {
        InstanceId::Id(id)
    }

    pub fn new_with_random_id() -> Self {
        InstanceId::Id(Uuid::new_v4())
    }

    pub fn new_segmented(segments: Vec<NamespaceSegment>) -> Result<Self, InstanceIdError> {
        if segments.is_empty() {
            return Err(InstanceIdError::AtLeastOneSegmentRequired);
        }
        if segments.len() == 1
            && let Some(segment) = segments.first()
        {
            return Ok(InstanceId::Named(segment.clone()));
        }
        Ok(InstanceId::MultiSegmented(segments))
    }

    pub fn parse_named(instance_id: &str) -> Result<Self, InstanceIdError> {
        if instance_id.is_empty() {
            return Err(InstanceIdError::NamedInstanceIdMustNotBeEmpty);
        }
        if instance_id.contains(INSTANCE_ID_SEPARATOR) {
            return Err(InstanceIdError::MustNotContainDelimiter);
        }
        Ok(InstanceId::Named(NamespaceSegment::from_str(instance_id).map_err(InstanceIdError::NamespaceSegmentError)?))
    }
}

impl Display for InstanceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let id = match self {
            InstanceId::Singleton => "".to_string(),
            InstanceId::Id(id) => id.to_string(),
            InstanceId::Named(name) => name.to_string(),
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

impl FromStr for InstanceId {
    type Err = InstanceIdError;

    fn from_str(instance_id: &str) -> Result<Self, Self::Err> {
        if instance_id.is_empty() {
            return Ok(InstanceId::Singleton);
        }
        if let Ok(id) = Uuid::parse_str(instance_id) {
            return Ok(InstanceId::Id(id));
        }
        if !instance_id.contains(INSTANCE_ID_SEPARATOR) {
            return Ok(InstanceId::Named(NamespaceSegment::from_str(instance_id).map_err(InstanceIdError::NamespaceSegmentError)?));
            // let instance_id = NamespaceSegment::from_str(instance_id).map_err(InstanceIdError::NamespaceSegmentError)?;
        }
        // let instance_id = NamespaceSegment::from_str(instance_id).map_err(InstanceIdError::NamespaceSegmentError)?;
        // if !instance_id.as_ref().contains(INSTANCE_ID_SEPARATOR) {
        //     return Ok(InstanceId::Named(instance_id));
        // }
        let mut segments = vec![];
        for segment in instance_id.split(INSTANCE_ID_SEPARATOR) {
            segments.push(NamespaceSegment::from_str(segment).map_err(|e| InstanceIdError::NamespaceSegmentError(e))?);
        }
        Ok(InstanceId::MultiSegmented(segments))
    }
}

// Required because of #[serde(try_from = "String")]
impl TryFrom<String> for InstanceId {
    type Error = InstanceIdError;

    fn try_from(instance_id: String) -> Result<Self, Self::Error> {
        Self::from_str(instance_id.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::InstanceId;
    use super::InstanceIdError;
    use crate::NamespaceSegment;
    use crate::NamespaceSegmentError;
    use std::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn new_singleton_test() {
        let instance_id = InstanceId::Singleton;
        assert_eq!("".to_string(), instance_id.to_string());
        assert_eq!(InstanceId::Singleton, InstanceId::from_str("").expect("Failed to parse empty instance id"));
    }

    #[test]
    fn new_with_id_test() {
        let uuid = Uuid::new_v4();
        let instance_id = InstanceId::new_with_id(uuid);
        let uuid_2 = Uuid::parse_str(&instance_id.to_string()).expect("Failed to parse UUID from instance id ");
        assert_eq!(uuid, uuid_2);
    }

    #[test]
    fn new_with_random_id_test() {
        let instance_id = InstanceId::new_with_random_id();
        let uuid = Uuid::parse_str(&instance_id.to_string()).expect("Failed to parse UUID from instance id ");
        assert_eq!(uuid.to_string(), format!("{instance_id}"));
    }

    #[test]
    fn new_segmented_test() {
        assert_eq!(Err(InstanceIdError::AtLeastOneSegmentRequired), InstanceId::new_segmented(vec![]));
        let instance_id = InstanceId::new_segmented(vec![NamespaceSegment::from_str("asdf").unwrap()]).unwrap();
        assert_eq!("asdf", format!("{instance_id}"));
        let instance_id = InstanceId::new_segmented(vec![NamespaceSegment::from_str("asdf").unwrap(), NamespaceSegment::from_str("asdf").unwrap()]).unwrap();
        assert_eq!("asdf__asdf", format!("{instance_id}"));
        let instance_id = InstanceId::new_segmented(vec![
            NamespaceSegment::from_str("asdf").unwrap(),
            NamespaceSegment::from_str("asdf").unwrap(),
            NamespaceSegment::from_str("asdf").unwrap(),
        ])
        .unwrap();
        assert_eq!("asdf__asdf__asdf", format!("{instance_id}"));
    }

    #[test]
    fn parse_named_test() {
        assert_eq!("asdf", InstanceId::parse_named("asdf").unwrap().to_string());
        assert_eq!("asdf_asdf", InstanceId::parse_named("asdf_asdf").unwrap().to_string());
        assert_eq!(Err(InstanceIdError::MustNotContainDelimiter), InstanceId::parse_named("asdf__asdf"));
        assert_eq!(
            Err(InstanceIdError::NamespaceSegmentError(NamespaceSegmentError::MustNotContainDelimiter(
                "asdf::asdf".to_string()
            ))),
            InstanceId::parse_named("asdf::asdf")
        );
    }
}
