use std::fmt;
use std::fmt::Formatter;

use crate::schema::types::data_type::DataType;
use crate::schema::types::extension::Extension;
use crate::schema::types::extension::ExtensionDefinition;
use crate::schema::types::extension::ExtensionDefinitions;
use crate::schema::types::extension::Extensions;
use crate::schema::types::mutability::Mutability;
use crate::schema::types::socket_type::SocketType;

#[derive(cynic::InputObject, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
pub struct PropertyTypeDefinition {
    pub data_type: DataType,
    pub description: String,
    pub extensions: Vec<ExtensionDefinition>,
    pub mutability: Mutability,
    pub name: String,
    pub socket_type: SocketType,
}

impl From<crate::model::PropertyType> for PropertyTypeDefinition {
    fn from(property_type: crate::model::PropertyType) -> Self {
        let extensions: ExtensionDefinitions = property_type.extensions.into();
        PropertyTypeDefinition {
            name: property_type.name,
            description: property_type.description,
            data_type: property_type.data_type.into(),
            socket_type: property_type.socket_type.into(),
            mutability: property_type.mutability.into(),
            extensions: extensions.0,
        }
    }
}

pub struct PropertyTypeDefinitions(pub Vec<PropertyTypeDefinition>);

impl From<PropertyTypeDefinitions> for Vec<PropertyTypeDefinition> {
    fn from(property_types: PropertyTypeDefinitions) -> Self {
        property_types.0.into_iter().collect()
    }
}

impl From<Vec<crate::model::PropertyType>> for PropertyTypeDefinitions {
    fn from(property_types: Vec<crate::model::PropertyType>) -> Self {
        PropertyTypeDefinitions(property_types.into_iter().map(|property_type| property_type.into()).collect())
    }
}

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
pub struct PropertyType {
    /// The name of the extension.
    pub name: String,

    /// Textual description of the extension.
    pub description: String,

    /// The extension as JSON representation.
    pub data_type: DataType,

    /// Specifies the type of socket - either input socket or output socket or none
    pub socket_type: SocketType,

    /// Specifies if the property is mutable.
    pub mutability: Mutability,

    /// Property specific extensions
    pub extensions: Vec<Extension>,
}

impl From<PropertyType> for crate::model::PropertyType {
    fn from(property_type: PropertyType) -> Self {
        crate::model::PropertyType {
            name: property_type.name,
            description: property_type.description,
            data_type: property_type.data_type.into(),
            socket_type: property_type.socket_type.into(),
            mutability: property_type.mutability.into(),
            extensions: Extensions(property_type.extensions).into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PropertyTypes(pub Vec<PropertyType>);

impl From<PropertyTypes> for Vec<crate::model::PropertyType> {
    fn from(property_types: PropertyTypes) -> Self {
        property_types.0.into_iter().map(|property_type| property_type.into()).collect()
    }
}

impl fmt::Display for PropertyTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // let x = Table::new(&self.0).to_string();
        // writeln!(f, "{}", Table::new(self.0.clone()).to_string())
        writeln!(f)
    }
}
