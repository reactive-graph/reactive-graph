use async_graphql::Enum;
use serde::Deserialize;
use serde::Serialize;
use strum::Display;

/// The mutability of a property.
#[derive(Enum, Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Display)]
#[serde(rename_all = "lowercase")]
#[graphql(name = "Mutability", remote = "inexor_rgf_graph::Mutability")]
pub enum GraphQLMutability {
    /// The property is mutable.
    Mutable,

    /// The property is immutable.
    Immutable,
}

impl GraphQLMutability {
    pub fn mutable() -> Self {
        GraphQLMutability::Mutable
    }
    pub fn immutable() -> Self {
        GraphQLMutability::Immutable
    }
}
