use async_graphql::InputObject;
use serde::Deserialize;
use serde::Serialize;

use crate::mutation::GraphQLExtensionDefinition;
use crate::mutation::GraphQLExtensionDefinitions;
use crate::query::GraphQLDataType;
use crate::query::GraphQLMutability;
use crate::query::GraphQLSocketType;
use reactive_graph_graph::NamespacedTypeError;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypes;

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
    pub extensions: Vec<GraphQLExtensionDefinition>,
}

impl TryFrom<PropertyTypeDefinition> for PropertyType {
    type Error = NamespacedTypeError;

    fn try_from(property_type: PropertyTypeDefinition) -> Result<Self, Self::Error> {
        Ok(PropertyType {
            name: property_type.name,
            description: property_type.description,
            data_type: property_type.data_type.into(),
            socket_type: property_type.socket_type.into(),
            mutability: property_type.mutability.into(),
            extensions: GraphQLExtensionDefinitions::parse_definitions(property_type.extensions)?,
        })
    }
}

#[derive(Default)]
pub struct PropertyTypeDefinitions(PropertyTypes);

impl PropertyTypeDefinitions {
    pub fn new(property_types: PropertyTypes) -> Self {
        Self(property_types)
    }

    pub fn parse_definitions(property_type_definitions: Vec<PropertyTypeDefinition>) -> Result<PropertyTypes, NamespacedTypeError> {
        PropertyTypeDefinitions::try_from(property_type_definitions).map(|p| p.0)
    }

    pub fn parse_optional_definitions(property_type_definitions: Option<Vec<PropertyTypeDefinition>>) -> Result<PropertyTypes, NamespacedTypeError> {
        match property_type_definitions {
            Some(property_type_definitions) => {
                PropertyTypeDefinitions::try_from(property_type_definitions).map(|property_type_definition| property_type_definition.0)
            }
            None => Ok(PropertyTypes::new()),
        }
    }
}

impl From<PropertyTypeDefinitions> for PropertyTypes {
    fn from(property_types: PropertyTypeDefinitions) -> Self {
        property_types.0
        // tys.0.into_iter().map(|ty| ty.into()).collect()
    }
}

impl TryFrom<Vec<PropertyTypeDefinition>> for PropertyTypeDefinitions {
    type Error = NamespacedTypeError;

    fn try_from(property_type_definitions: Vec<PropertyTypeDefinition>) -> Result<Self, Self::Error> {
        let property_types = PropertyTypes::new();
        for property_type_definition in property_type_definitions.into_iter() {
            property_types.push(PropertyType::try_from(property_type_definition)?);
        }
        Ok(PropertyTypeDefinitions::new(property_types))
    }
}
