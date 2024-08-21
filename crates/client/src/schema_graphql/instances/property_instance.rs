use crate::schema_graphql::scalar::Json;
use crate::schema_graphql::types::property_type::PropertyType;
use std::fmt;
use std::fmt::Formatter;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema_path = "schema_graphql.graphql", schema_module = "crate::schema_graphql::schema")]
pub struct PropertyInstance {
    /// The name of the extension.
    pub name: String,

    /// Textual description of the extension.
    pub type_: Option<PropertyType>,

    /// The extension as JSON representation.
    pub value: Json,
}

#[derive(Clone, Debug)]
pub struct PropertyInstances(pub Vec<PropertyInstance>);

impl From<PropertyInstances> for reactive_graph_graph::PropertyInstances {
    fn from(property_instances: PropertyInstances) -> Self {
        property_instances
            .0
            .into_iter()
            .map(|property_instance| {
                let value: serde_json::Value = property_instance.value.into();
                (property_instance.name, value)
                // let x = property_instance.into();
                // x
            })
            .collect()
    }
}

impl fmt::Display for PropertyInstances {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)
    }
}
