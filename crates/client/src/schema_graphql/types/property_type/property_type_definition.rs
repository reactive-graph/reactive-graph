use crate::DataType;
use crate::ExtensionDefinition;
use crate::ExtensionDefinitions;
use crate::Mutability;
use crate::SocketType;
use typed_builder::TypedBuilder;

#[derive(cynic::InputObject, Debug, TypedBuilder)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct PropertyTypeDefinition {
    pub data_type: DataType,
    pub description: String,
    pub extensions: Vec<ExtensionDefinition>,
    pub mutability: Mutability,
    pub name: String,
    pub socket_type: SocketType,
}

impl From<reactive_graph_graph::PropertyType> for PropertyTypeDefinition {
    fn from(property_type: reactive_graph_graph::PropertyType) -> Self {
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

impl PropertyTypeDefinitions {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl From<PropertyTypeDefinitions> for Vec<PropertyTypeDefinition> {
    fn from(property_types: PropertyTypeDefinitions) -> Self {
        property_types.0.into_iter().collect()
    }
}

impl From<reactive_graph_graph::PropertyTypes> for PropertyTypeDefinitions {
    fn from(property_types: reactive_graph_graph::PropertyTypes) -> Self {
        property_types.into_iter().map(|(_, property_type)| property_type).collect()
    }
}

impl FromIterator<reactive_graph_graph::PropertyType> for PropertyTypeDefinitions {
    fn from_iter<I: IntoIterator<Item = reactive_graph_graph::PropertyType>>(iter: I) -> Self {
        let mut property_types = PropertyTypeDefinitions::new();
        for property_type in iter {
            property_types.0.push(property_type.into());
        }
        property_types
    }
}

impl Default for PropertyTypeDefinitions {
    fn default() -> Self {
        Self::new()
    }
}
