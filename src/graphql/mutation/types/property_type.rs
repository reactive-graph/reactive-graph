use async_graphql::*;
use serde::{Deserialize, Serialize};

use crate::graphql::query::GraphQLExtension;
use crate::model::{DataType, PropertyType, SocketType};

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
pub struct PropertyTypeDefinition {
    /// The name of the property
    pub name: String,

    /// The data type of the property
    pub data_type: DataType,

    /// Specifies which type of socket
    #[serde(default = "SocketType::none")]
    pub socket_type: SocketType,

    /// Property specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<GraphQLExtension>,
}

impl From<PropertyTypeDefinition> for PropertyType {
    fn from(property_type: PropertyTypeDefinition) -> Self {
        PropertyType {
            name: property_type.name,
            data_type: property_type.data_type,
            socket_type: property_type.socket_type,
            extensions: property_type.extensions.iter().map(|extension| extension.clone().into()).collect(),
        }
    }
}
