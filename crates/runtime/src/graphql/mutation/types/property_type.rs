use async_graphql::*;
use serde::Deserialize;
use serde::Serialize;

use crate::graphql::query::GraphQLDataType;
use crate::graphql::query::GraphQLExtension;
use crate::graphql::query::GraphQLMutability;
use crate::graphql::query::GraphQLSocketType;
use crate::model::PropertyType;
use crate::model::PropertyTypes;

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "PropertyTypeDefinition")]
pub struct PropertyTypeDefinition {
    /// The name of the property
    pub name: String,

    /// The description of the property
    pub description: String,

    /// The data type of the property
    pub data_type: GraphQLDataType,

    /// Specifies which type of socket
    #[serde(default = "GraphQLSocketType::none")]
    pub socket_type: GraphQLSocketType,

    /// The property is mutable or immutable
    #[serde(default = "GraphQLMutability::mutable")]
    pub mutability: GraphQLMutability,

    /// Property specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<GraphQLExtension>,
}

impl From<PropertyTypeDefinition> for PropertyType {
    fn from(property_type: PropertyTypeDefinition) -> Self {
        PropertyType {
            name: property_type.name,
            description: property_type.description,
            data_type: property_type.data_type.into(),
            socket_type: property_type.socket_type.into(),
            mutability: property_type.mutability.into(),
            extensions: property_type.extensions.iter().map(|extension| extension.clone().into()).collect(),
        }
    }
}

pub struct PropertyTypeDefinitions(pub Vec<PropertyTypeDefinition>);

impl PropertyTypeDefinitions {
    pub fn new(tys: Vec<PropertyTypeDefinition>) -> Self {
        Self(tys)
    }
}

impl Default for PropertyTypeDefinitions {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl From<PropertyTypeDefinitions> for PropertyTypes {
    fn from(tys: PropertyTypeDefinitions) -> Self {
        tys.0.into_iter().map(|ty| ty.into()).collect()
    }
}
