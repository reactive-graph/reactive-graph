use reactive_graph_graph::PropertyInstances;
use serde_json::Value;
use typed_builder::TypedBuilder;

#[derive(cynic::InputObject, Debug, TypedBuilder)]
#[cynic(schema_path = "schema_graphql.graphql", schema_module = "crate::schema_graphql::schema")]
pub struct PropertyInstanceDefinition {
    pub name: String,
    pub value: Value,
}

pub struct PropertyInstanceDefinitions(pub Vec<PropertyInstanceDefinition>);

impl PropertyInstanceDefinitions {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl From<PropertyInstanceDefinitions> for Vec<PropertyInstanceDefinition> {
    fn from(property_instances: PropertyInstanceDefinitions) -> Self {
        property_instances.0.into_iter().collect()
    }
}

impl From<PropertyInstances> for PropertyInstanceDefinitions {
    fn from(property_instances: PropertyInstances) -> Self {
        PropertyInstanceDefinitions(
            property_instances
                .iter()
                .map(|p| PropertyInstanceDefinition {
                    name: p.key().clone(),
                    value: p.value().clone(),
                })
                .collect(),
        )
    }
}

impl Default for PropertyInstanceDefinitions {
    fn default() -> Self {
        Self::new()
    }
}
