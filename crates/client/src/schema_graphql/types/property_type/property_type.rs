use crate::DataType;
use crate::Extension;
use crate::Extensions;
use crate::Mutability;
use crate::SocketType;
use reactive_graph_graph::InvalidPropertyTypeError;
use std::fmt;
use std::fmt::Formatter;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
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

impl TryFrom<PropertyType> for reactive_graph_graph::PropertyType {
    type Error = InvalidPropertyTypeError;

    fn try_from(property_type: PropertyType) -> Result<Self, Self::Error> {
        Ok(reactive_graph_graph::PropertyType {
            name: property_type.name,
            description: property_type.description,
            data_type: property_type.data_type.into(),
            socket_type: property_type.socket_type.into(),
            mutability: property_type.mutability.into(),
            extensions: reactive_graph_graph::Extensions::try_from(Extensions(property_type.extensions)).map_err(InvalidPropertyTypeError::InvalidExtension)?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct PropertyTypes(pub Vec<PropertyType>);

impl TryFrom<PropertyTypes> for reactive_graph_graph::PropertyTypes {
    type Error = InvalidPropertyTypeError;

    fn try_from(property_types: PropertyTypes) -> Result<Self, Self::Error> {
        let property_types_2 = reactive_graph_graph::PropertyTypes::new();
        for property_type in property_types.0 {
            property_types_2.push(reactive_graph_graph::PropertyType::try_from(property_type)?);
        }
        Ok(property_types_2)
    }
}

impl fmt::Display for PropertyTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)
    }
}
