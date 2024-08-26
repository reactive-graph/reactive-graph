use crate::DataType;
use crate::Extension;
use crate::Extensions;
use crate::Mutability;
use crate::SocketType;
use std::fmt;
use std::fmt::Formatter;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema_path = "schema_graphql.graphql", schema_module = "crate::schema_graphql::schema")]
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

impl From<PropertyType> for reactive_graph_graph::PropertyType {
    fn from(property_type: PropertyType) -> Self {
        reactive_graph_graph::PropertyType {
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

impl From<PropertyTypes> for reactive_graph_graph::PropertyTypes {
    fn from(property_types: PropertyTypes) -> Self {
        property_types.0.into_iter().map(|property_type| property_type.into()).collect()
    }
}

impl fmt::Display for PropertyTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)
    }
}
