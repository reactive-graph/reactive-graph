use async_graphql::scalar;
use inexor_rgf_core_model::PropertyType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// The named property stores a value/document as JSON representation.
///
/// Each property is represented by it's name (String) and it's value. The value is
/// a representation of a JSON value/document. Therefore the value can be boolean,
/// number, string, array or an object. For more information about the data types
/// please look at https://docs.serde.rs/serde_json/value/enum.Value.html
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GraphQLPropertyInstance {
    /// The name of the property.
    pub name: String,

    /// The value of the property as JSON representation.
    pub value: Value,
}
scalar!(GraphQLPropertyInstance, "Property");

impl GraphQLPropertyInstance {
    pub fn to_map(properties: Option<Vec<GraphQLPropertyInstance>>) -> HashMap<String, Value> {
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

    pub fn to_map_with_defaults(properties: Option<Vec<GraphQLPropertyInstance>>, property_types: Vec<PropertyType>) -> HashMap<String, Value> {
        let mut props = HashMap::new();
        for property_type in property_types {
            props.insert(property_type.name.clone(), property_type.data_type.default_value());
        }
        match properties {
            Some(properties) => {
                for property in properties {
                    props.insert(property.name.clone(), property.value.clone());
                }
            }
            None => {}
        }
        props
    }
}
