use std::collections::HashMap;

use async_graphql::InputObject;
use async_graphql::Object;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use inexor_rgf_graph::PropertyInstances;
use inexor_rgf_graph::PropertyTypes;

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "CommandResultDefinition")]
pub struct GraphQLCommandResult {
    /// The name of the property.
    pub name: String,

    /// The value of the property as JSON representation.
    pub value: Value,
}

/// The named property stores a value/document as JSON representation.
///
/// Each property is represented by it's name (String) and it's value. The value is
/// a representation of a JSON value/document. Therefore the value can be boolean,
/// number, string, array or an object. For more information about the data types
/// please look at https://docs.serde.rs/serde_json/value/enum.Value.html
#[Object(name = "CommandResult")]
impl GraphQLCommandResult {
    /// The name of the property.
    async fn name(&self) -> String {
        self.name.clone()
    }

    /// The value of the property as JSON representation.
    async fn value(&self) -> Value {
        self.value.clone()
    }
}

impl GraphQLCommandResult {
    pub fn new(name: String, value: Value) -> Self {
        GraphQLCommandResult { name, value }
    }

    pub fn to_map(properties: Option<Vec<GraphQLCommandResult>>) -> HashMap<String, Value> {
        match properties {
            Some(properties) => {
                let mut props = HashMap::new();
                for property in properties {
                    props.insert(property.name.clone(), property.value.clone());
                }
                props
            }
            None => HashMap::new(),
        }
    }

    pub fn to_map_with_defaults(properties: Option<Vec<GraphQLCommandResult>>, property_types: PropertyTypes) -> HashMap<String, Value> {
        let mut props = HashMap::new();
        for property_type in property_types.iter() {
            props.insert(property_type.name.clone(), property_type.data_type.default_value());
        }
        if let Some(properties) = properties {
            for property in properties {
                props.insert(property.name.clone(), property.value.clone());
            }
        }
        props
    }

    pub fn to_property_instances_with_defaults(properties: Option<Vec<GraphQLCommandResult>>, property_types: PropertyTypes) -> PropertyInstances {
        let property_instances = PropertyInstances::new();
        for property_type in property_types.iter() {
            property_instances.insert(property_type.name.clone(), property_type.data_type.default_value());
        }
        if let Some(properties) = properties {
            for property in properties {
                property_instances.insert(property.name.clone(), property.value.clone());
            }
        }
        property_instances
    }
}
